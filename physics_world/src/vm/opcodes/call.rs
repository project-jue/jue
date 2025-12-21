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
    // 1. Validate stack has enough arguments + function
    if vm.stack.len() < arg_count as usize + 1 {
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
    // 1. Validate we have enough arguments + function on stack
    if vm.stack.len() < arg_count as usize + 1 {
        return Err(VmError::StackUnderflow);
    }

    // 2. The closure should be at the top of the stack
    //    Arguments should be below it: [args..., closure]
    //    We need to remove the closure from the stack and set up the call frame
    //    to point to the arguments

    // Remove the closure from stack (it's been validated already)
    let _closure = vm.stack.pop().unwrap();

    // 3. Set up call frame for proper return handling
    // stack_start should point to the first argument
    let stack_start = vm.stack.len() - arg_count as usize;
    let recursion_depth = if vm.call_stack.is_empty() {
        1 // First call in the stack
    } else {
        vm.call_stack.last().unwrap().recursion_depth + 1
    };

    // Preserve local variables (arguments) for access after recursive calls
    let mut locals = Vec::new();
    for i in 0..arg_count as usize {
        if stack_start + i < vm.stack.len() {
            locals.push(vm.stack[stack_start + i].clone());
        }
    }

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

    // 4. Replace instructions and reset IP
    vm.instructions = closure_body;
    vm.ip = 0;

    Ok(())
}

/// NEW: Basic tail call handler
pub fn handle_tail_call(vm: &mut VmState, arg_count: u16) -> Result<(), VmError> {
    // 1. Validate stack has enough arguments + function
    if vm.stack.len() < arg_count as usize + 1 {
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
    // 1. Validate we have enough arguments + function on stack
    if vm.stack.len() < arg_count as usize + 1 {
        return Err(VmError::StackUnderflow);
    }

    // 2. The closure should be at the top of the stack
    //    Arguments should be below it: [args..., closure]
    //    We need to remove the closure from the stack and set up the call frame
    //    to point to the arguments

    // Remove the closure from stack (it's been validated already)
    let _closure = vm.stack.pop().unwrap();

    // 3. Reuse the current call frame for tail call optimization
    if let Some(current_frame) = vm.call_stack.last_mut() {
        current_frame.is_tail_call = true;

        // Preserve local variables for tail call optimization
        let stack_start = vm.stack.len() - arg_count as usize;
        current_frame.locals.clear();
        for i in 0..arg_count as usize {
            if stack_start + i < vm.stack.len() {
                current_frame.locals.push(vm.stack[stack_start + i].clone());
            }
        }

        current_frame.stack_start = stack_start;
        current_frame.saved_instructions = Some(vm.instructions.clone());
    }

    // 4. Replace instructions and reset IP
    vm.instructions = closure_body;
    vm.ip = 0;

    Ok(())
}
