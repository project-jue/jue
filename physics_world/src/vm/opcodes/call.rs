/// Call opcode handler - implements proper function call/return system
/// This is a critical Phase 1 feature for the Physics World VM
use crate::types::{HeapPtr, OpCode, Value};
use crate::vm::call_state::CallFrame;
use crate::vm::state::VmError;
use crate::vm::state::VmState;
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

    eprintln!(
        "DEBUG handle_call: call_stack.len()={}, current_depth={}, max_recursion_depth={}",
        vm.call_stack.len(),
        current_depth,
        vm.max_recursion_depth
    );

    if current_depth > vm.max_recursion_depth {
        eprintln!("DEBUG: Recursion limit exceeded at depth {}", current_depth);
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
                    // Get code_index from closure data (stored at bytes 4-8)
                    let code_index = if closure_data.len() >= 8 {
                        u32::from_le_bytes([
                            closure_data[4],
                            closure_data[5],
                            closure_data[6],
                            closure_data[7],
                        ]) as usize
                    } else {
                        0
                    };
                    return execute_closure_body(vm, closure_body, arg_count, code_index);
                }
                Err(_) => {
                    return Err(VmError::TypeMismatch);
                }
            }
        }
    }
    return Err(VmError::InvalidHeapPtr);
}

/// Helper function to execute a closure body with TCO support
fn execute_closure_body(
    vm: &mut VmState,
    closure_body: Vec<OpCode>,
    arg_count: u16,
    code_index: usize,
) -> Result<(), VmError> {
    eprintln!(
        "DEBUG CALL: stack.len()={}, arg_count={}, stack={:?}",
        vm.stack.len(),
        arg_count,
        vm.stack
    );

    // 1. Pop the closure from the stack first
    let _closure = vm.stack.pop().unwrap();

    // 2. Capture caller's stack state BEFORE truncating arguments
    // original_stack_size must include BOTH the caller's stack AND the caller's locals
    // This is CRITICAL for recursion: we must preserve the caller's complete state
    let original_stack_size = if vm.call_stack.is_empty() {
        // Top-level call: no caller to preserve
        vm.stack.len() - arg_count as usize
    } else {
        // Nested call: preserve the caller's stack_start PLUS the caller's locals count
        // This ensures we keep the caller's arguments on the stack after they return
        let caller = vm.call_stack.last().unwrap();
        caller.stack_start + caller.locals.len()
    };

    // 3. Copy arguments to locals (preserving order: first arg at index 0)
    // NOTE: For TCO, we need to keep arguments on the stack at positions
    // stack_start + offset so that SetLocal can read them
    let args_start = if arg_count > 0 {
        vm.stack.len().saturating_sub(arg_count as usize)
    } else {
        vm.stack.len()
    };

    // Copy to locals for GetLocal access
    let args: Vec<Value> = if arg_count > 0 {
        vm.stack[args_start..].to_vec()
    } else {
        Vec::new()
    };

    eprintln!(
        "DEBUG CALL: args copied to locals: {:?}, original_stack_size={}",
        args, original_stack_size
    );

    // 4. For TCO support, we keep the arguments on the stack at positions
    // stack_start + offset for SetLocal to read them
    // We only truncate values BELOW the arguments (caller's stack)
    // Arguments stay at positions args_start, args_start+1, etc.
    //
    // The stack layout after this will be:
    // [caller's values before call][arg0][arg1]...[argN]
    //                    ^              ^
    //              original_stack_size  stack_start + offset
    //
    // SetLocal(offset) reads from stack[stack_start + offset]
    vm.stack.truncate(original_stack_size);

    eprintln!(
        "DEBUG CALL: After truncate, stack.len()={}, args_start={}",
        vm.stack.len(),
        args_start
    );

    // 4. stack_start points to where this frame's evaluation stack begins
    // After truncation, arguments are at positions stack_start, stack_start+1, etc.
    // SetLocal(offset) reads from stack[stack_start + offset]
    let stack_start = vm.stack.len();

    // 5. Check for tail-recursive call (same code_index as current frame)
    // NOTE: TCO is currently disabled because all closures have code_index=0,
    // which causes ALL recursive calls to be incorrectly identified as tail-recursive.
    // This breaks the call stack and causes incorrect behavior.
    // TODO: Fix TCO by using a unique code_index per closure definition, not per call site
    let is_tail_recursive = false;

    // 6. Set up call frame for proper return handling
    let recursion_depth = if vm.call_stack.is_empty() {
        1 // First call in the stack
    } else {
        vm.call_stack.last().unwrap().recursion_depth + 1
    };

    if is_tail_recursive {
        // Reuse the current frame for tail call optimization
        // Clone args since we still need it for locals assignment
        let args_clone = args.clone();
        if let Some(current_frame) = vm.call_stack.last_mut() {
            current_frame.is_tail_call = true;
            current_frame.stack_start = stack_start;
            current_frame.locals = args_clone;
            // Don't change saved_instructions for TCO - keep caller's instructions
            // Don't push a new frame - reuse the current one
        }
    } else {
        // Arguments are stored in locals for GetLocal access
        let locals = args;
        // Create new frame (normal call)
        // IMPORTANT: Save the caller's instructions BEFORE replacing them
        let caller_instructions = vm.instructions.clone();

        // Determine the return_ip - this is where execution continues after the call
        // For proper continuation, we want to execute the next instruction after Call
        // So return_ip should be vm.ip + 1 (the instruction AFTER the Call)
        let return_ip = vm.ip + 1;

        let call_frame = CallFrame {
            return_ip,
            stack_start,
            original_stack_size,
            saved_instructions: Some(caller_instructions),
            recursion_depth,
            locals,
            closed_over: HashMap::new(),
            is_tail_call: false,
            frame_id: vm.next_frame_id(),
            code_index,
        };
        // vm.call_stack is Vec<CallFrame>, check length manually
        // Use > to allow exactly max_recursion_depth frames (0 to max_recursion_depth-1)
        if vm.call_stack.len() > (vm.max_recursion_depth - 1) as usize {
            return Err(VmError::RecursionLimitExceeded);
        }
        vm.call_stack.push(call_frame);
    }

    // 7. Replace instructions and reset IP
    vm.instructions = closure_body;
    vm.ip = 0;

    Ok(())
}

/// NEW: Basic tail call handler with operation counting (per expert guidance)
pub fn handle_tail_call(vm: &mut VmState, arg_count: u16) -> Result<(), VmError> {
    // 1. Count as operation (per expert guidance - tail calls still consume resources)
    // Use steps_remaining as the CPU limit mechanism
    if vm.steps_remaining == 0 {
        return Err(VmError::CpuLimitExceeded);
    }
    vm.steps_remaining -= 1;

    // 2. Validate stack has at least the closure (function)
    // Note: arg_count can be 0, so we just need the closure on the stack
    if vm.stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }

    // 3. Check that we have a call frame to reuse
    if vm.call_stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }

    // 4. Get the function (closure) from stack
    let func_pos = vm.stack.len() - 1;
    let func = &vm.stack[func_pos];

    // 5. Handle different function types
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

    // 3. Save a COPY of arguments for locals (don't pop them from stack!)
    // The arguments stay on the stack for GetLocal to access them
    let args_start = vm.stack.len() - arg_count as usize;
    let args: Vec<Value> = if args_start < vm.stack.len() {
        vm.stack[args_start..].to_vec()
    } else {
        Vec::new()
    };

    // 4. Reuse the current call frame for tail call optimization
    // Don't change saved_instructions - keep the original caller's instructions
    if let Some(current_frame) = vm.call_stack.last_mut() {
        current_frame.is_tail_call = true;
        current_frame.stack_start = stack_start;
        current_frame.locals = args;
    }

    // 5. Replace instructions and reset IP
    vm.instructions = closure_body;
    vm.ip = 0;

    Ok(())
}
