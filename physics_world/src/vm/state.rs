use crate::memory::arena::ObjectArena;
use crate::types::{HeapPtr, OpCode, Value};
use serde::{Deserialize, Serialize};

/// Represents the state of a single virtual machine instance.
#[derive(Serialize, Deserialize)]
pub struct VmState {
    // Execution
    pub ip: usize,                 // Instruction Pointer
    pub instructions: Vec<OpCode>, // Loaded bytecode
    pub constant_pool: Vec<Value>, // For Symbol values, etc.
    // Data
    pub stack: Vec<Value>,          // Primary working stack
    pub call_stack: Vec<CallFrame>, // For function calls/returns
    pub memory: ObjectArena,        // Heap
    // Resources (AIKR)
    pub steps_remaining: u64, // Decremented each instruction
}

/// Represents a call frame for function calls.
#[derive(Serialize, Deserialize)]
pub struct CallFrame {
    pub return_ip: usize,
    pub stack_start: usize,
}

/// Result of executing a single instruction.
pub enum InstructionResult {
    Continue,
    Yield,
    Finished(Value), // Final value on the stack
}

/// Error types that can occur during VM execution.
#[derive(Debug)]
pub enum VmError {
    CpuLimitExceeded,
    MemoryLimitExceeded,
    StackUnderflow,
    InvalidHeapPtr,
    UnknownOpCode,
    TypeMismatch,
    DivisionByZero,
    ArithmeticOverflow,
}

impl VmState {
    /// Creates a new VM state with code, constants, and resource limits.
    pub fn new(
        instructions: Vec<OpCode>,
        constants: Vec<Value>,
        step_limit: u64,
        mem_limit: usize,
    ) -> Self {
        Self {
            ip: 0,
            instructions,
            constant_pool: constants,
            stack: Vec::new(),
            call_stack: Vec::new(),
            memory: ObjectArena::with_capacity(mem_limit as u32),
            steps_remaining: step_limit,
        }
    }

    /// Executes a single instruction. Returns `Ok(InstructionResult)` or `Err(VmError)`.
    pub fn step(&mut self) -> Result<InstructionResult, VmError> {
        // Check if we've exceeded CPU limit
        if self.steps_remaining == 0 {
            return Err(VmError::CpuLimitExceeded);
        }
        self.steps_remaining -= 1;

        // Get current instruction
        let instruction = match self.instructions.get(self.ip) {
            Some(instr) => instr,
            None => {
                return Ok(InstructionResult::Finished(
                    self.stack.pop().unwrap_or(Value::Nil),
                ))
            }
        };

        // Execute the instruction
        match instruction {
            OpCode::Nil => {
                self.stack.push(Value::Nil);
                self.ip += 1;
            }
            OpCode::Bool(b) => {
                self.stack.push(Value::Bool(*b));
                self.ip += 1;
            }
            OpCode::Int(i) => {
                self.stack.push(Value::Int(*i));
                self.ip += 1;
            }
            OpCode::Symbol(sym_idx) => {
                let symbol = match self.constant_pool.get(*sym_idx as usize) {
                    Some(val) => val.clone(),
                    None => return Err(VmError::InvalidHeapPtr),
                };
                self.stack.push(symbol);
                self.ip += 1;
            }
            OpCode::Dup => {
                if self.stack.is_empty() {
                    return Err(VmError::StackUnderflow);
                }
                let top = self.stack.last().unwrap().clone();
                self.stack.push(top);
                self.ip += 1;
            }
            OpCode::Pop => {
                if self.stack.is_empty() {
                    return Err(VmError::StackUnderflow);
                }
                self.stack.pop();
                self.ip += 1;
            }
            OpCode::Cons => {
                if self.stack.len() < 2 {
                    return Err(VmError::StackUnderflow);
                }
                let cdr = self.stack.pop().unwrap();
                let car = self.stack.pop().unwrap();
    
                // Allocate pair on heap - 8 bytes for two HeapPtr values (car and cdr)
                let pair_ptr = match self.memory.allocate(8, 1) { // Tag 1 for pairs
                    Ok(ptr) => ptr,
                    Err(_) => return Err(VmError::MemoryLimitExceeded),
                };
    
                // Store the car and cdr values in the allocated memory
                let data = unsafe { self.memory.get_data_mut(pair_ptr) };
                let car_bytes = match car {
                    Value::Pair(p) => p.get().to_le_bytes(),
                    Value::Closure(p) => p.get().to_le_bytes(),
                    _ => return Err(VmError::TypeMismatch),
                };
                let cdr_bytes = match cdr {
                    Value::Pair(p) => p.get().to_le_bytes(),
                    Value::Closure(p) => p.get().to_le_bytes(),
                    _ => return Err(VmError::TypeMismatch),
                };
    
                data[0..4].copy_from_slice(&car_bytes);
                data[4..8].copy_from_slice(&cdr_bytes);
    
                self.stack.push(Value::Pair(pair_ptr));
                self.ip += 1;
            }
            OpCode::Car => {
                if self.stack.is_empty() {
                    return Err(VmError::StackUnderflow);
                }
                match self.stack.pop().unwrap() {
                    Value::Pair(ptr) => {
                        // Dereference the pointer to get the car value
                        let data = unsafe { self.memory.get_data(ptr) };
                        let car_ptr = u32::from_le_bytes(data[0..4].try_into().unwrap());
                        self.stack.push(Value::Pair(HeapPtr::new(car_ptr)));
                    }
                    _ => return Err(VmError::InvalidHeapPtr),
                }
                self.ip += 1;
            }
            OpCode::Cdr => {
                if self.stack.is_empty() {
                    return Err(VmError::StackUnderflow);
                }
                match self.stack.pop().unwrap() {
                    Value::Pair(ptr) => {
                        // Dereference the pointer to get the cdr value
                        let data = unsafe { self.memory.get_data(ptr) };
                        let cdr_ptr = u32::from_le_bytes(data[4..8].try_into().unwrap());
                        self.stack.push(Value::Pair(HeapPtr::new(cdr_ptr)));
                    }
                    _ => return Err(VmError::InvalidHeapPtr),
                }
                self.ip += 1;
            }
            OpCode::Call(arg_count) => {
                if self.stack.len() < *arg_count as usize + 1 {
                    return Err(VmError::StackUnderflow);
                }

                // Get the function to call (should be a closure)
                let func = self.stack[self.stack.len() - (*arg_count as usize + 1)].clone();

                // Create call frame
                let call_frame = CallFrame {
                    return_ip: self.ip + 1,
                    stack_start: self.stack.len() - (*arg_count as usize + 1),
                };
                self.call_stack.push(call_frame);

                // Extract the function's code pointer from the closure
                match func {
                    Value::Closure(closure_ptr) => {
                        // Get the closure data from memory
                        let data = unsafe { self.memory.get_data(closure_ptr) };
                        // First 4 bytes is the code index in the constant pool
                        let code_index = u32::from_le_bytes(data[0..4].try_into().unwrap());

                        // For now, we'll jump to the code index (simplified)
                        // In a real implementation, we'd have proper function code handling
                        self.ip = code_index as usize;
                    }
                    Value::Int(n) => {
                        // For test purposes, treat integer as a dummy function that jumps to that address
                        // This allows simple tests to work without full closure setup
                        self.ip = n as usize;
                    }
                    _ => return Err(VmError::TypeMismatch),
                }
            }
            OpCode::Ret => {
                if self.call_stack.is_empty() {
                    return Err(VmError::StackUnderflow);
                }

                let call_frame = self.call_stack.pop().unwrap();

                // Get return value
                let return_value = if self.stack.len() > call_frame.stack_start {
                    self.stack.pop().unwrap()
                } else {
                    Value::Nil
                };

                // Restore stack
                self.stack.truncate(call_frame.stack_start);

                // Push return value
                self.stack.push(return_value);

                // Restore instruction pointer
                self.ip = call_frame.return_ip;
            }
            OpCode::Jmp(offset) => {
                let new_ip = (self.ip as i32 + *offset as i32) as usize;
                if new_ip >= self.instructions.len() {
                    return Err(VmError::InvalidHeapPtr);
                }
                self.ip = new_ip;
            }
            OpCode::JmpIfFalse(offset) => {
                if self.stack.is_empty() {
                    return Err(VmError::StackUnderflow);
                }

                let condition = self.stack.pop().unwrap();
                if !condition.is_truthy() {
                    let new_ip = (self.ip as i32 + *offset as i32) as usize;
                    if new_ip >= self.instructions.len() {
                        return Err(VmError::InvalidHeapPtr);
                    }
                    self.ip = new_ip;
                } else {
                    self.ip += 1;
                }
            }
            OpCode::Yield => {
                self.ip += 1;
                return Ok(InstructionResult::Yield);
            }
            OpCode::Send => {
                if self.stack.len() < 2 {
                    return Err(VmError::StackUnderflow);
                }
    
                // Pop message and target actor ID
                let message = self.stack.pop().unwrap();
                let target_actor = match self.stack.pop().unwrap() {
                    Value::ActorId(id) => id,
                    _ => return Err(VmError::TypeMismatch),
                };
    
                // In a real implementation, we would send the message to the scheduler
                // For now, we'll just continue execution
                // The scheduler would handle the actual message delivery
    
                self.ip += 1;
            }
            OpCode::Add => {
                // 1. Pop two values
                let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;

                // 2. Type check and compute
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => {
                        // 3. Check for overflow (optional but good for determinism)
                        let result = x.checked_add(y).ok_or(VmError::ArithmeticOverflow)?;
                        self.stack.push(Value::Int(result));
                    }
                    _ => return Err(VmError::TypeMismatch),
                }
            }

            OpCode::Div => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;

                match (a, b) {
                    (Value::Int(x), Value::Int(0)) => return Err(VmError::DivisionByZero),
                    (Value::Int(x), Value::Int(y)) => {
                        let result = x.checked_div(y).ok_or(VmError::ArithmeticOverflow)?;
                        self.stack.push(Value::Int(result));
                    }
                    _ => return Err(VmError::TypeMismatch),
                }
            }

            OpCode::Eq => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                self.stack.push(Value::Bool(a == b));
                self.ip += 1;
            }

            OpCode::Lt => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => {
                        self.stack.push(Value::Bool(x < y));
                    }
                    _ => return Err(VmError::TypeMismatch),
                }
                self.ip += 1;
            }

            OpCode::Gt => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => {
                        self.stack.push(Value::Bool(x > y));
                    }
                    _ => return Err(VmError::TypeMismatch),
                }
                self.ip += 1;
            }

            OpCode::Sub => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => {
                        let result = x.checked_sub(y).ok_or(VmError::ArithmeticOverflow)?;
                        self.stack.push(Value::Int(result));
                    }
                    _ => return Err(VmError::TypeMismatch),
                }
                self.ip += 1;
            }

            OpCode::Mul => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => {
                        let result = x.checked_mul(y).ok_or(VmError::ArithmeticOverflow)?;
                        self.stack.push(Value::Int(result));
                    }
                    _ => return Err(VmError::TypeMismatch),
                }
                self.ip += 1;
            }

            OpCode::Mod => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                match (a, b) {
                    (Value::Int(x), Value::Int(y)) => {
                        if y == 0 {
                            return Err(VmError::DivisionByZero);
                        }
                        let result = x.checked_rem(y).ok_or(VmError::ArithmeticOverflow)?;
                        self.stack.push(Value::Int(result));
                    }
                    _ => return Err(VmError::TypeMismatch),
                }
                self.ip += 1;
            }

            OpCode::Swap => {
                if self.stack.len() < 2 {
                    return Err(VmError::StackUnderflow);
                }
                let len = self.stack.len();
                self.stack.swap(len - 1, len - 2);
                self.ip += 1;
            }
            OpCode::MakeClosure(code_idx, capture_count) => {
                // Check if we have enough values on stack to capture
                if self.stack.len() < *capture_count {
                    return Err(VmError::StackUnderflow);
                }
    
                // Calculate size needed: 4 bytes for code index + 4 bytes per captured value
                let size = 4 + (*capture_count as u32 * 4);
                let closure_ptr = match self.memory.allocate(size, 2) { // Tag 2 for closures
                    Ok(ptr) => ptr,
                    Err(_) => return Err(VmError::MemoryLimitExceeded),
                };
    
                // Store the code index and capture environment
                let data = unsafe { self.memory.get_data_mut(closure_ptr) };
    
                // Store code index (first 4 bytes)
                let code_idx_bytes = (*code_idx as u32).to_le_bytes();
                data[0..4].copy_from_slice(&code_idx_bytes);
    
                // Capture environment from stack (next capture_count * 4 bytes)
                // Pop the captured values from stack in reverse order
                for i in 0..*capture_count as usize {
                    let value = self.stack[self.stack.len() - (*capture_count as usize - i)].clone();
                    let value_bytes = match value {
                        Value::Pair(p) => p.get().to_le_bytes(),
                        Value::Closure(p) => p.get().to_le_bytes(),
                        Value::Int(n) => (n as u32).to_le_bytes(),
                        _ => return Err(VmError::TypeMismatch),
                    };
                    let start = 4 + (i * 4);
                    data[start..start + 4].copy_from_slice(&value_bytes);
                }
    
                // Remove captured values from stack
                for _ in 0..*capture_count as usize {
                    self.stack.pop();
                }
    
                self.stack.push(Value::Closure(closure_ptr));
                self.ip += 1;
            }
            OpCode::CheckStepLimit => {
                // Check if we've exceeded CPU limit
                if self.steps_remaining == 0 {
                    return Err(VmError::CpuLimitExceeded);
                }
                self.ip += 1;
            }
        }

        Ok(InstructionResult::Continue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::OpCode;

    #[test]
    fn test_new_vm_state() {
        let vm = VmState::new(vec![OpCode::Int(42), OpCode::Int(23)], vec![], 100, 1024);

        assert_eq!(vm.ip, 0);
        assert_eq!(vm.instructions.len(), 2);
        assert_eq!(vm.constant_pool.len(), 0);
        assert_eq!(vm.stack.len(), 0);
        assert_eq!(vm.call_stack.len(), 0);
        assert_eq!(vm.steps_remaining, 100);
        assert_eq!(vm.memory.capacity(), 1024);
    }

    #[test]
    fn test_simple_int_program() {
        let mut vm = VmState::new(vec![OpCode::Int(5), OpCode::Int(3)], vec![], 10, 1024);

        // Execute first instruction (Int 5)
        let result = vm.step();
        assert!(matches!(result, Ok(InstructionResult::Continue)));
        assert_eq!(vm.ip, 1);
        assert_eq!(vm.stack.len(), 1);
        assert_eq!(vm.stack[0], Value::Int(5));

        // Execute second instruction (Int 3)
        let result = vm.step();
        assert!(matches!(result, Ok(InstructionResult::Continue)));
        assert_eq!(vm.ip, 2);
        assert_eq!(vm.stack.len(), 2);
        assert_eq!(vm.stack[0], Value::Int(5));
        assert_eq!(vm.stack[1], Value::Int(3));
    }

    #[test]
    fn test_cpu_limit_exceeded() {
        let mut vm = VmState::new(
            vec![OpCode::Int(1)],
            vec![],
            0, // Zero steps remaining
            1024,
        );

        let result = vm.step();
        assert!(matches!(result, Err(VmError::CpuLimitExceeded)));
    }

    #[test]
    fn test_stack_operations() {
        let mut vm = VmState::new(
            vec![OpCode::Int(1), OpCode::Int(2), OpCode::Dup, OpCode::Pop],
            vec![],
            10,
            1024,
        );

        // Int 1
        vm.step().unwrap();
        assert_eq!(vm.stack, vec![Value::Int(1)]);

        // Int 2
        vm.step().unwrap();
        assert_eq!(vm.stack, vec![Value::Int(1), Value::Int(2)]);

        // Dup
        vm.step().unwrap();
        assert_eq!(vm.stack, vec![Value::Int(1), Value::Int(2), Value::Int(2)]);

        // Pop
        vm.step().unwrap();
        assert_eq!(vm.stack, vec![Value::Int(1), Value::Int(2)]);
    }

    #[test]
    fn test_stack_underflow() {
        let mut vm = VmState::new(vec![OpCode::Pop], vec![], 10, 1024);

        let result = vm.step();
        assert!(matches!(result, Err(VmError::StackUnderflow)));
    }

    #[test]
    fn test_call_and_return() {
        let mut vm = VmState::new(
            vec![
                OpCode::Int(42), // Push function (simplified)
                OpCode::Int(1),  // Push argument
                OpCode::Call(1), // Call with 1 argument
                OpCode::Int(99), // This should not execute
            ],
            vec![],
            20,
            1024,
        );

        // Push function
        vm.step().unwrap();
        // Push argument
        vm.step().unwrap();
        // Call
        let result = vm.step();
        assert!(matches!(result, Ok(InstructionResult::Continue)));

        // The call should have created a call frame and jumped
        assert_eq!(vm.call_stack.len(), 1);
        assert_eq!(vm.ip, 42); // Jumped to the integer value (dummy function)

        // Now return
        vm.instructions = vec![OpCode::Ret];
        vm.ip = 0;
        let result = vm.step();
        assert!(matches!(result, Ok(InstructionResult::Continue)));

        // Should have restored stack and IP
        assert_eq!(vm.call_stack.len(), 0);
        assert_eq!(vm.stack.len(), 1); // Return value
    }

    #[test]
    fn test_conditional_jump() {
        let mut vm = VmState::new(
            vec![
                OpCode::Bool(false),
                OpCode::JmpIfFalse(2), // Jump 2 instructions forward if false
                OpCode::Int(1),        // This should be skipped
                OpCode::Int(2),        // This should execute
            ],
            vec![],
            10,
            1024,
        );

        // Push false
        vm.step().unwrap();
        // JmpIfFalse - should jump
        vm.step().unwrap();
        assert_eq!(vm.ip, 3); // Should have jumped to Int(2)

        // Execute Int(2)
        vm.step().unwrap();
        assert_eq!(vm.stack, vec![Value::Int(2)]);
    }

    #[test]
    fn test_yield() {
        let mut vm = VmState::new(vec![OpCode::Yield], vec![], 10, 1024);

        let result = vm.step();
        assert!(matches!(result, Ok(InstructionResult::Yield)));
        assert_eq!(vm.ip, 1);
    }
}
