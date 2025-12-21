/// Stack operation opcode handlers - Dup, Pop, Swap, GetLocal, SetLocal
use crate::types::Value;
use crate::vm::state::VmError;
use crate::vm::state::VmState;

/// Handles Dup opcode
pub fn handle_dup(vm: &mut VmState) -> Result<(), VmError> {
    if vm.stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }
    let top = vm.stack.last().unwrap().clone();
    vm.stack.push(top);
    Ok(())
}

/// Handles Pop opcode
pub fn handle_pop(vm: &mut VmState) -> Result<(), VmError> {
    if vm.stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }
    vm.stack.pop();
    Ok(())
}

/// Handles Swap opcode
pub fn handle_swap(vm: &mut VmState) -> Result<(), VmError> {
    if vm.stack.len() < 2 {
        return Err(VmError::StackUnderflow);
    }
    let len = vm.stack.len();
    vm.stack.swap(len - 1, len - 2);
    Ok(())
}

/// Handles GetLocal opcode
pub fn handle_get_local(vm: &mut VmState, offset: u16) -> Result<(), VmError> {
    let offset_usize = offset as usize;

    // Check if we're inside a function call (have a call frame)
    if let Some(call_frame) = vm.call_stack.last() {
        // Inside a function - get from call frame locals
        if offset_usize < call_frame.locals.len() {
            let value = call_frame.locals[offset_usize].clone();
            vm.stack.push(value);
            return Ok(());
        }
        // Fall through to stack-based access if local not found in frame
    }

    // Top-level execution or local not in frame - access from stack directly
    // The offset is relative to the base of the current scope
    if offset_usize >= vm.stack.len() {
        return Err(VmError::StackUnderflow);
    }

    let value = vm.stack[offset_usize].clone();
    vm.stack.push(value);
    Ok(())
}

/// Handles SetLocal opcode
pub fn handle_set_local(vm: &mut VmState, offset: u16) -> Result<(), VmError> {
    if vm.stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }

    let offset_usize = offset as usize;

    // Pop the value to set
    let value = vm.stack.pop().unwrap();

    // Check if we're inside a function call (have a call frame)
    if let Some(call_frame) = vm.call_stack.last_mut() {
        // Inside a function - set in call frame locals
        // Expand locals vector if needed
        while offset_usize >= call_frame.locals.len() {
            call_frame.locals.push(Value::Nil);
        }
        call_frame.locals[offset_usize] = value;
        return Ok(());
    }

    // Top-level execution - store in stack at the specified offset
    // Expand stack if needed
    while offset_usize >= vm.stack.len() {
        vm.stack.push(Value::Nil);
    }
    vm.stack[offset_usize] = value;
    Ok(())
}
