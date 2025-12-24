/// Call opcode handler - implements proper function call/return system
/// This is a critical Phase 1 feature for the Physics World VM
use crate::types::{HeapPtr, OpCode, Value};
use crate::vm::state::VmError;
use crate::vm::state::{CallFrame, VmState};
use bincode;
use std::collections::HashMap;

/// Handles the Call opcode with proper closure execution
///
/// # Arguments
/// * `vm` - The VM state
/// * `arg_count` - Number of arguments to pass to the function
///
/// # Returns
/// Result indicating success or error
///
/// # Implementation Details
/// 1. Validates stack has enough arguments + function
/// 2. Checks recursion depth limit
/// 3. Extracts closure from stack
/// 4. Sets up call frame with return address and stack state
/// 5. Extracts closure body from constant pool
/// 6. Replaces current instructions with closure body
/// 7. Resets instruction pointer to start of closure
/// 8. Arguments remain on stack for function to access via GetLocal
pub fn handle_call(vm: &mut VmState, arg_count: u16) -> Result<(), VmError> {
    // 1. Validate stack has at least the closure (function)
    // Note: arg_count can be 0, so we just need the closure on the stack
    if vm.stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }

    // 2. Check recursion depth limit
    let current_depth = if vm.call_stack.is_empty() {
        1 // First call in the stack
    } else {
        vm.call_stack.last().unwrap().recursion_depth + 1
    };

    if current_depth > vm.max_recursion_depth {
        return Err(VmError::RecursionLimitExceeded);
    }

    // 3. Get the function (closure) from stack
    // The function should be the last item on the stack
    let func_pos = vm.stack.len() - 1;
    let func = &vm.stack[func_pos];

    // 4. Handle different function types
    match func {
        Value::Closure(closure_ptr) => execute_closure_call(vm, *closure_ptr, arg_count),
        _ => Err(VmError::TypeMismatch),
    }
}

/// Executes a closure call with proper error handling and validation
fn execute_closure_call(
    vm: &mut VmState,
    closure_ptr: HeapPtr,
    arg_count: u16,
) -> Result<(), VmError> {
    // 1. Validate closure pointer
    if closure_ptr.get() == 0 {
        return Err(VmError::InvalidHeapPtr);
    }

    // 2. Get closure data from memory
    let closure_data = unsafe { vm.memory.get_data(closure_ptr) };
    if closure_data.len() < 4 {
        // Need at least 4 bytes for body pointer
        return Err(VmError::InvalidHeapPtr);
    }

    // 3. The closure data contains the closure body pointer at bytes 0-4
    //    This matches what MakeClosure stores
    let body_ptr_bytes = u32::from_le_bytes(closure_data[0..4].try_into().unwrap());
    let body_ptr = HeapPtr::new(body_ptr_bytes);

    // 4. Get closure body from memory directly
    let body_data = unsafe { vm.memory.get_data(body_ptr) };
    if body_data.len() >= 4 {
        // Extract bytecode length and body
        let bytecode_length =
            u32::from_le_bytes([body_data[0], body_data[1], body_data[2], body_data[3]]);
        if body_data.len() >= 4 + bytecode_length as usize {
            let bytecode_bytes = &body_data[4..4 + bytecode_length as usize];
            match bincode::deserialize::<Vec<OpCode>>(bytecode_bytes) {
                Ok(closure_body) => {
                    return execute_closure_body(vm, closure_body, arg_count);
                }
                Err(_) => {
                    return Err(VmError::TypeMismatch);
                }
            }
        }
    }
    return Err(VmError::InvalidHeapPtr);
}
/// Helper function to execute a closure body
fn execute_closure_body(
    vm: &mut VmState,
    closure_body: Vec<OpCode>,
    arg_count: u16,
) -> Result<(), VmError> {
    // 1. Calculate stack_start BEFORE popping anything
    // stack_start should point to the first argument (NOT including the closure)
    let stack_start = vm.stack.len() - arg_count as usize;

    // 2. Pop the closure from the stack first
    let _closure = vm.stack.pop().unwrap();

    // 3. Pop arguments from the stack (they're at the end, in correct order)
    // No need to reverse - they're already in order
    let mut args: Vec<Value> = Vec::with_capacity(arg_count as usize);
    for _ in 0..arg_count {
        if let Some(arg) = vm.stack.pop() {
            args.push(arg);
        }
    }

    // 4. Set up call frame for proper return handling
    let recursion_depth = if vm.call_stack.is_empty() {
        1 // First call in the stack
    } else {
        vm.call_stack.last().unwrap().recursion_depth + 1
    };

    // Arguments are stored in locals for GetLocal access
    let locals = args;

    let call_frame = CallFrame {
        return_ip: vm.ip + 1,
        stack_start,
        saved_instructions: Some(vm.instructions.clone()),
        recursion_depth,
        locals,
        closed_over: HashMap::new(),
        is_tail_call: false,
        frame_id: vm.next_frame_id(),
    };
    vm.call_stack.push(call_frame);

    // 5. Replace instructions and reset IP
    vm.instructions = closure_body;
    vm.ip = 0;

    Ok(())
}
/// NEW: Basic tail call handler
pub fn handle_tail_call(vm: &mut VmState, arg_count: u16) -> Result<(), VmError> {
    // 1. Validate stack has at least the closure (function)
    // Note: arg_count can be 0, so we just need the closure on the stack
    if vm.stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }

    // 2. Check that we have a call frame to reuse
    if vm.call_stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }

    // 3. Get the function (closure) from stack
    let func_pos = vm.stack.len() - 1;
    let func = &vm.stack[func_pos];

    // 4. Handle different function types
    match func {
        Value::Closure(closure_ptr) => execute_tail_call_closure(vm, *closure_ptr, arg_count),
        _ => Err(VmError::TypeMismatch),
    }
}

/// Helper function to execute a tail call closure
fn execute_tail_call_closure(
    vm: &mut VmState,
    closure_ptr: HeapPtr,
    arg_count: u16,
) -> Result<(), VmError> {
    // 1. Validate closure pointer
    if closure_ptr.get() == 0 {
        return Err(VmError::InvalidHeapPtr);
    }

    // 2. Get closure data from memory
    let closure_data = unsafe { vm.memory.get_data(closure_ptr) };
    if closure_data.len() < 4 {
        // Need at least 4 bytes for body pointer
        return Err(VmError::InvalidHeapPtr);
    }

    // 3. The closure data contains the closure body pointer at bytes 0-4
    //    This matches what MakeClosure stores
    let body_ptr_bytes = u32::from_le_bytes(closure_data[0..4].try_into().unwrap());
    let body_ptr = HeapPtr::new(body_ptr_bytes);

    // 4. Get closure body from memory directly
    let body_data = unsafe { vm.memory.get_data(body_ptr) };
    if body_data.len() >= 4 {
        // Extract bytecode length and body
        let bytecode_length =
            u32::from_le_bytes([body_data[0], body_data[1], body_data[2], body_data[3]]);
        if body_data.len() >= 4 + bytecode_length as usize {
            let bytecode_bytes = &body_data[4..4 + bytecode_length as usize];
            match bincode::deserialize::<Vec<OpCode>>(bytecode_bytes) {
                Ok(closure_body) => {
                    return execute_tail_call_body(vm, closure_body, arg_count);
                }
                Err(_) => {
                    return Err(VmError::TypeMismatch);
                }
            }
        }
    }
    return Err(VmError::InvalidHeapPtr);
}

/// Helper function to execute a tail call body
fn execute_tail_call_body(
    vm: &mut VmState,
    closure_body: Vec<OpCode>,
    arg_count: u16,
) -> Result<(), VmError> {
    // 1. Calculate stack_start BEFORE popping anything
    let stack_start = vm.stack.len() - arg_count as usize;

    // 2. Pop the closure from the stack first
    let _closure = vm.stack.pop().unwrap();

    // 3. Pop arguments from the stack (in correct order)
    let mut args: Vec<Value> = Vec::with_capacity(arg_count as usize);
    for _ in 0..arg_count {
        if let Some(arg) = vm.stack.pop() {
            args.push(arg);
        }
    }

    // 4. Reuse the current call frame for tail call optimization
    if let Some(current_frame) = vm.call_stack.last_mut() {
        current_frame.is_tail_call = true;
        current_frame.stack_start = stack_start;
        current_frame.locals = args;
        current_frame.saved_instructions = Some(vm.instructions.clone());
    }

    // 5. Replace instructions and reset IP
    vm.instructions = closure_body;
    vm.ip = 0;

    Ok(())
}
