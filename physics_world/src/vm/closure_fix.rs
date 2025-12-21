/// This file contains fixes and enhancements for closure execution
/// It will be integrated into the main state.rs file
use crate::types::{HeapPtr, OpCode, Value};
use crate::vm::state::{CallFrame, VmState};
use bincode;

// Enhanced error types for better debugging
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
    ClosureFormatError(String), // Enhanced error for closure issues
    SerializationError(String), // Better serialization errors
}

// Enhanced closure execution with proper error handling
pub fn execute_closure_call(
    vm: &mut VmState,
    closure_ptr: HeapPtr,
    arg_count: u16,
) -> Result<(), VmError> {
    // 1. Validate closure pointer
    if closure_ptr.get() == 0 {
        return Err(VmError::ClosureFormatError("Null closure pointer".into()));
    }

    // 2. Get closure data from memory
    let closure_data = unsafe { vm.memory.get_data(closure_ptr) };
    if closure_data.len() < 8 {
        // Need at least 4 bytes code idx + 4 bytes capture
        return Err(VmError::ClosureFormatError(format!(
            "Invalid closure data length: {}",
            closure_data.len()
        )));
    }

    // 3. Extract code index from closure
    let code_index = u32::from_le_bytes(closure_data[0..4].try_into().unwrap());

    // 4. Get closure body from constant pool
    let closure_body_value = match vm.constant_pool.get(code_index as usize) {
        Some(Value::Closure(body_ptr)) => body_ptr,
        Some(other) => {
            return Err(VmError::ClosureFormatError(format!(
                "Expected Closure in constant pool, found: {:?}",
                other
            )))
        }
        None => {
            return Err(VmError::ClosureFormatError(format!(
                "Invalid code index: {}",
                code_index
            )))
        }
    };

    // 5. Get closure body data from memory
    let body_data = unsafe { vm.memory.get_data(*closure_body_value) };
    if body_data.len() < 4 {
        // Need at least 4 bytes for size
        return Err(VmError::ClosureFormatError(format!(
            "Invalid closure body length: {}",
            body_data.len()
        )));
    }

    // 6. Extract bytecode length and body
    let bytecode_length = u32::from_le_bytes(body_data[0..4].try_into().unwrap());
    let bytecode_bytes = &body_data[4..4 + bytecode_length as usize];

    // 7. Deserialize bytecode
    let closure_body: Vec<OpCode> = bincode::deserialize(bytecode_bytes).map_err(|e| {
        VmError::SerializationError(format!("Failed to deserialize closure: {}", e))
    })?;

    // 8. Set up call frame
    let stack_start = vm.stack.len() - arg_count as usize;
    let recursion_depth = if vm.call_stack.is_empty() {
        1 // First call in the stack
    } else {
        vm.call_stack.last().unwrap().recursion_depth + 1
    };
    let call_frame = CallFrame {
        return_ip: vm.ip + 1,
        stack_start,
        saved_instructions: Some(vm.instructions.clone()),
        recursion_depth,
        locals: Vec::new(),
        closed_over: std::collections::HashMap::new(),
        is_tail_call: false,
        frame_id: 0, // TODO: Add frame_id generation
    };
    vm.call_stack.push(call_frame);

    // 9. Replace instructions and reset IP
    vm.instructions = closure_body;
    vm.ip = 0;

    Ok(())
}

// Enhanced closure creation with validation
pub fn create_closure(
    vm: &mut VmState,
    code_idx: usize,
    capture_count: usize,
) -> Result<Value, VmError> {
    // 1. Validate capture count
    if vm.stack.len() < capture_count {
        return Err(VmError::StackUnderflow);
    }

    // 2. Calculate closure size (4 bytes code idx + 4 bytes per captured value)
    let size = 4 + (capture_count as u32 * 4);
    let closure_ptr = vm
        .memory
        .allocate(size, 2) // Tag 2 for closures
        .map_err(|_| VmError::MemoryLimitExceeded)?;

    // 3. Store code index and captured values
    let data = unsafe { vm.memory.get_data_mut(closure_ptr) };

    // Store code index (first 4 bytes)
    let code_idx_bytes = (code_idx as u32).to_le_bytes();
    data[0..4].copy_from_slice(&code_idx_bytes);

    // Store captured values (next capture_count * 4 bytes)
    for i in 0..capture_count {
        let value = &vm.stack[vm.stack.len() - (capture_count - i)];
        let value_bytes = match value {
            Value::Pair(p) => p.get().to_le_bytes(),
            Value::Closure(p) => p.get().to_le_bytes(),
            Value::Int(n) => (*n as u32).to_le_bytes(),
            Value::Float(f) => (*f as u32).to_le_bytes(), // Convert float to u32 for storage
            Value::Bool(b) => (*b as u32).to_le_bytes(),
            Value::Nil => 0u32.to_le_bytes(),
            Value::String(_) => 0u32.to_le_bytes(), // Strings stored in constant pool, not in closure
            Value::Symbol(s) => (*s as u32).to_le_bytes(),
            Value::ActorId(id) => (*id as u32).to_le_bytes(),
            Value::Capability(_) => 0u32.to_le_bytes(), // Placeholder
            Value::GcPtr(p) => (p.0 as u32).to_le_bytes(),
        };
        let start = 4 + (i * 4);
        data[start..start + 4].copy_from_slice(&value_bytes);
    }

    // 4. Remove captured values from stack
    for _ in 0..capture_count {
        vm.stack.pop();
    }

    Ok(Value::Closure(closure_ptr))
}

// Enhanced closure body creation helper
pub fn create_closure_body(vm: &mut VmState, bytecode: Vec<OpCode>) -> Result<HeapPtr, VmError> {
    // 1. Serialize the bytecode
    let serialized = bincode::serialize(&bytecode)
        .map_err(|e| VmError::SerializationError(format!("Failed to serialize: {}", e)))?;

    // 2. Calculate size (4 bytes length + serialized data)
    let size = 4 + serialized.len() as u32;

    // 3. Allocate memory for closure body
    let body_ptr = vm
        .memory
        .allocate(size, 2) // Tag 2 for closure bodies
        .map_err(|_| VmError::MemoryLimitExceeded)?;

    // 4. Store size and bytecode
    let data = unsafe { vm.memory.get_data_mut(body_ptr) };
    let size_bytes = (serialized.len() as u32).to_le_bytes();
    data[0..4].copy_from_slice(&size_bytes);
    data[4..4 + serialized.len()].copy_from_slice(&serialized);

    Ok(body_ptr)
}

// Enhanced Call opcode implementation
pub fn handle_call_opcode(vm: &mut VmState, arg_count: u16) -> Result<(), VmError> {
    // 1. Validate stack has enough arguments + function
    if vm.stack.len() < arg_count as usize + 1 {
        return Err(VmError::StackUnderflow);
    }

    // 2. Get the function (closure) from stack
    let func_pos = vm.stack.len() - (arg_count as usize + 1);
    let func = &vm.stack[func_pos];

    // 3. Handle different function types
    match func {
        Value::Closure(closure_ptr) => execute_closure_call(vm, *closure_ptr, arg_count),
        _ => Err(VmError::TypeMismatch),
    }
}

// Enhanced Ret opcode implementation
pub fn handle_ret_opcode(vm: &mut VmState) -> Result<(), VmError> {
    // 1. Check if there's a call frame
    if vm.call_stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }

    // 2. Get the call frame
    let call_frame = vm.call_stack.pop().unwrap();

    // 3. Get return value (if any)
    let return_value = if vm.stack.len() > call_frame.stack_start {
        Some(vm.stack.pop().unwrap())
    } else {
        None
    };

    // 4. Restore stack to call frame state
    vm.stack.truncate(call_frame.stack_start);

    // 5. Push return value (or Nil if none)
    if let Some(value) = return_value {
        vm.stack.push(value);
    } else {
        vm.stack.push(Value::Nil);
    }

    // 6. Restore instruction pointer and original instructions
    vm.ip = call_frame.return_ip;
    if let Some(saved_instructions) = call_frame.saved_instructions {
        vm.instructions = saved_instructions;
    }

    Ok(())
}
