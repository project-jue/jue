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

    // Get from call frame's locals (local variables)
    // This is the canonical location - stack is for expression evaluation only
    let call_frame = vm.call_stack.last().ok_or(VmError::StackUnderflow)?;

    if offset_usize >= call_frame.locals.len() {
        return Err(VmError::StackUnderflow);
    }

    let value = call_frame.locals[offset_usize].clone();
    vm.stack.push(value);
    Ok(())
}

/// Handles SetLocal opcode - TCO FIX
///
/// IMPORTANT: For TCO to work, we cannot pop from the stack after Call truncates it.
/// Instead, we read from the stack at the position where the value should be
/// (relative to frame.stack_start) and write to locals.
pub fn handle_set_local(vm: &mut VmState, offset: u16) -> Result<(), VmError> {
    let offset_usize = offset as usize;

    // Get the current call frame
    let call_frame = vm.call_stack.last_mut().ok_or(VmError::StackUnderflow)?;

    // The value should be at position frame.stack_start + offset in the value_stack
    // This is because:
    // 1. frame.stack_start marks where this frame's evaluation stack begins
    // 2. Local variables (including arguments) are stored at offsets starting from 0
    // 3. After Call truncates the stack, arguments are copied to locals but NOT kept on stack
    //
    // However, for TCO to work with SetLocal, we need the value on the stack at the
    // correct position. The fix is to NOT truncate the stack during Call, but instead
    // keep values at frame.stack_start + offset for SetLocal to read.
    //
    // Alternative approach: Read from the stack position, not pop
    let value_stack_pos = call_frame.stack_start + offset_usize;
    
    if value_stack_pos >= vm.stack.len() {
        // Fallback: If value is not at expected position, try to pop from top
        // This maintains backward compatibility for non-TCO code
        if vm.stack.is_empty() {
            return Err(VmError::StackUnderflow);
        }
        let value = vm.stack.pop().unwrap();
        
        // Expand locals vector if needed
        while offset_usize >= call_frame.locals.len() {
            call_frame.locals.push(Value::Nil);
        }
        call_frame.locals[offset_usize] = value;
    } else {
        // Read from the expected position in the stack
        let value = vm.stack[value_stack_pos].clone();
        
        // Expand locals vector if needed
        while offset_usize >= call_frame.locals.len() {
            call_frame.locals.push(Value::Nil);
        }
        call_frame.locals[offset_usize] = value;
        
        // Remove the value from its position in the stack (shift down)
        // This maintains stack hygiene while keeping the value for SetLocal
        vm.stack.remove(value_stack_pos);
    }
    
    Ok(())
}
