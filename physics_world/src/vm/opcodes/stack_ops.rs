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
    // GetLocal should access arguments/locals relative to the call frame
    // For a function with N arguments, GetLocal(0) gets the first argument, etc.
    if vm.call_stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }

    let call_frame = vm.call_stack.last().unwrap();
    let offset_usize = offset as usize;

    // Get the value from the call frame locals
    if offset_usize >= call_frame.locals.len() {
        return Err(VmError::StackUnderflow);
    }

    let value = call_frame.locals[offset_usize].clone();
    vm.stack.push(value);
    Ok(())
}

/// Handles SetLocal opcode
pub fn handle_set_local(vm: &mut VmState, offset: u16) -> Result<(), VmError> {
    if vm.stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }

    // SetLocal should modify locals in the current call frame
    if vm.call_stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }

    // Save the value to set first
    let value = vm.stack.pop().unwrap();

    // Access the mutable call frame
    let call_frame = vm.call_stack.last_mut().unwrap();
    let offset_usize = offset as usize;

    // If the local index is beyond current locals, expand the locals vector
    while offset_usize >= call_frame.locals.len() {
        call_frame.locals.push(Value::Nil);
    }

    // Set the local variable in the call frame
    call_frame.locals[offset_usize] = value;
    Ok(())
}
