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

    // Try to get from call frame first, fall back to top-level locals
    let value = if let Some(call_frame) = vm.call_stack.last() {
        if offset_usize >= call_frame.locals.len() {
            return Err(VmError::StackUnderflow);
        }
        call_frame.locals[offset_usize].clone()
    } else {
        // No call frame - use top-level locals for standalone execution
        if offset_usize >= vm.top_level_locals.len() {
            return Err(VmError::StackUnderflow);
        }
        vm.top_level_locals[offset_usize].clone()
    };

    vm.stack.push(value);
    Ok(())
}

/// Handles SetLocal opcode - TCO FIX
///
/// IMPORTANT: For TCO to work, we cannot pop from the stack after Call truncates it.
/// Instead, we read from the stack at the position where the value should be
/// (relative to frame.stack_start) and write to locals.
///
/// When no call frame exists (top-level execution), uses top_level_locals instead.
pub fn handle_set_local(vm: &mut VmState, offset: u16) -> Result<(), VmError> {
    let offset_usize = offset as usize;

    // Get the value to store (pop from top of stack)
    if vm.stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }
    let value = vm.stack.pop().unwrap();

    // Try to store in call frame first, fall back to top-level locals
    if let Some(call_frame) = vm.call_stack.last_mut() {
        // Expand locals vector if needed
        while offset_usize >= call_frame.locals.len() {
            call_frame.locals.push(Value::Nil);
        }
        call_frame.locals[offset_usize] = value;
    } else {
        // No call frame - use top-level locals for standalone execution
        while offset_usize >= vm.top_level_locals.len() {
            vm.top_level_locals.push(Value::Nil);
        }
        vm.top_level_locals[offset_usize] = value;
    }

    Ok(())
}
