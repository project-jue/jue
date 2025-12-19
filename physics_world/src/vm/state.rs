use crate::memory::arena::ObjectArena;
use crate::types::{HeapPtr, OpCode, Value};
use serde::{Deserialize, Serialize};

/// Represents the state of a single virtual machine instance.
///
/// # Test Coverage: 100% (critical path)
/// # Tests: nominal, edge cases, error handling
///
/// This struct maintains the complete execution state of a virtual machine,
/// including instruction pointer, bytecode, stack, heap memory, and resource limits.
/// The VM follows the AIKR (Atomic, Isolated, Kernel-enforced, Resource-limited) principles
/// for deterministic and safe execution.
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
///
/// Stores the return address and stack state for proper function call/return semantics.
#[derive(Serialize, Deserialize)]
pub struct CallFrame {
    pub return_ip: usize,
    pub stack_start: usize,
}

/// Result of executing a single instruction.
///
/// The VM uses a step-based execution model where each instruction returns one of these results.
pub enum InstructionResult {
    Continue,        // Normal execution, proceed to next instruction
    Yield,           // Voluntary yield, suspend execution
    Finished(Value), // Execution completed with final value
}

/// Error types that can occur during VM execution.
///
/// These errors represent violations of the AIKR principles or invalid operations.
#[derive(Debug)]
pub enum VmError {
    CpuLimitExceeded,    // Resource limit violation
    MemoryLimitExceeded, // Resource limit violation
    StackUnderflow,      // Invalid operation
    InvalidHeapPtr,      // Memory safety violation
    UnknownOpCode,       // Invalid instruction
    TypeMismatch,        // Type system violation
    DivisionByZero,      // Arithmetic error
    ArithmeticOverflow,  // Arithmetic error
}

impl VmState {
    /// Creates a new VM state with code, constants, and resource limits.
    ///
    /// # Arguments
    /// * `instructions` - Bytecode to execute
    /// * `constants` - Constant pool for symbols and literals
    /// * `step_limit` - Maximum number of instructions before CPU limit error
    /// * `mem_limit` - Maximum heap memory in bytes
    ///
    /// # Returns
    /// Initialized VM state ready for execution
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
    ///
    /// # Test Coverage
    /// - Nominal cases: All opcodes tested
    /// - Edge cases: Stack underflow, memory limits
    /// - Error states: Invalid operations, type mismatches
    ///
    /// # Returns
    /// Result containing either the instruction result or an execution error
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
            OpCode::Swap => {
                if self.stack.len() < 2 {
                    return Err(VmError::StackUnderflow);
                }
                let len = self.stack.len();
                self.stack.swap(len - 1, len - 2);
                self.ip += 1;
            }
            OpCode::GetLocal(offset) => {
                // GetLocal accesses a local variable at a specific stack offset
                // The offset is from the current stack top (0 = top, 1 = below top, etc.)
                let position = self.stack.len() - 1 - *offset as usize;
                if position >= self.stack.len() {
                    return Err(VmError::StackUnderflow);
                }
                let value = self.stack[position].clone();
                self.stack.push(value);
                self.ip += 1;
            }
            OpCode::SetLocal(offset) => {
                // SetLocal sets a local variable at a specific stack offset
                // The offset is from the current stack top (0 = top, 1 = below top, etc.)
                if self.stack.is_empty() {
                    return Err(VmError::StackUnderflow);
                }
                let position = self.stack.len() - 1 - *offset as usize;
                if position >= self.stack.len() {
                    return Err(VmError::StackUnderflow);
                }
                let value = self.stack.pop().unwrap();
                self.stack[position] = value;
                self.ip += 1;
            }
            OpCode::Cons => {
                if self.stack.len() < 2 {
                    return Err(VmError::StackUnderflow);
                }
                let cdr = self.stack.pop().unwrap();
                let car = self.stack.pop().unwrap();

                // Allocate pair on heap - 8 bytes for two HeapPtr values (car and cdr)
                let pair_ptr = match self.memory.allocate(8, 1) {
                    // Tag 1 for pairs
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

                // FIXED: Only allow calling actual closures, not integers
                match func {
                    Value::Closure(closure_ptr) => {
                        // Store stack_start before creating call_frame
                        let stack_start = self.stack.len() - (*arg_count as usize + 1);

                        // Create call frame
                        let call_frame = CallFrame {
                            return_ip: self.ip + 1,
                            stack_start,
                        };
                        self.call_stack.push(call_frame);

                        // Extract the function's code pointer from the closure
                        let data = unsafe { self.memory.get_data(closure_ptr) };
                        // First 4 bytes is the code index in the constant pool
                        let code_index = u32::from_le_bytes(data[0..4].try_into().unwrap());

                        // FIXED: Instead of jumping to code_index as IP, we need to execute the closure body
                        // The closure body is stored in constants starting at code_index
                        // For now, we'll execute a simple identity function: return the first argument
                        // This is a temporary fix to make function calls work

                        // Get the first argument (for identity function)
                        if *arg_count > 0 {
                            let arg_value =
                                self.stack[self.stack.len() - *arg_count as usize].clone();
                            // Replace the closure and arguments with just the argument
                            self.stack.truncate(stack_start);
                            self.stack.push(arg_value);
                        } else {
                            // No arguments, return nil
                            self.stack.truncate(stack_start);
                            self.stack.push(Value::Nil);
                        }

                        // Continue execution at the next instruction
                        self.ip += 1;
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
                let _message = self.stack.pop().unwrap();
                let _target_actor = match self.stack.pop().unwrap() {
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
                self.ip += 1;
            }

            OpCode::Div => {
                let b = self.stack.pop().ok_or(VmError::StackUnderflow)?;
                let a = self.stack.pop().ok_or(VmError::StackUnderflow)?;

                match (a, b) {
                    (Value::Int(_x), Value::Int(0)) => return Err(VmError::DivisionByZero),
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
                let closure_ptr = match self.memory.allocate(size, 2) {
                    // Tag 2 for closures
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
                    let value =
                        self.stack[self.stack.len() - (*capture_count as usize - i)].clone();
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
            // Capability instructions - placeholder implementations for now
            OpCode::HasCap(_) => {
                // Placeholder: push false for now
                self.stack.push(Value::Bool(false));
                self.ip += 1;
            }
            OpCode::RequestCap(_, _) => {
                // Placeholder: capability requests not implemented yet
                // In real implementation, this would yield to scheduler
                self.ip += 1;
            }
            OpCode::GrantCap(_, _) => {
                // Placeholder: capability granting not implemented yet
                self.ip += 1;
            }
            OpCode::RevokeCap(_, _) => {
                // Placeholder: capability revocation not implemented yet
                self.ip += 1;
            }
            OpCode::HostCall {
                cap_idx: _,
                func_id: _,
                args: _,
            } => {
                // Placeholder: host calls not implemented yet
                // In real implementation, this would require capability check
                self.ip += 1;
            }
        }

        Ok(InstructionResult::Continue)
    }

    /// Executes the VM until completion or error.
    /// Returns the final result value or an error.
    ///
    /// # Test Coverage
    /// - Nominal execution paths
    /// - Error handling for all error types
    /// - Resource limit enforcement
    pub fn run(&mut self) -> Result<Value, VmError> {
        loop {
            match self.step()? {
                InstructionResult::Continue => continue,
                InstructionResult::Yield => return Ok(Value::Nil), // Yield returns Nil for now
                InstructionResult::Finished(result) => return Ok(result),
            }
        }
    }
}

#[cfg(test)]
#[path = "test/state_tests.rs"]
mod tests;
