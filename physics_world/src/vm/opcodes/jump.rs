/// Jump opcode handlers - Jmp and JmpIfFalse
use crate::types::Value;
use crate::vm::state::VmError;
use crate::vm::state::VmState;

/// Handles the Jmp opcode - unconditional jump
pub fn handle_jmp(vm: &mut VmState, offset: i16) -> Result<(), VmError> {
    // Calculate new instruction pointer
    let new_ip = (vm.ip as i32 + offset as i32) as usize;

    // Validate the new IP is within bounds
    if new_ip >= vm.instructions.len() {
        return Err(VmError::UnknownOpCode);
    }

    vm.ip = new_ip;
    Ok(())
}

/// Handles the JmpIfFalse opcode - conditional jump
pub fn handle_jmp_if_false(vm: &mut VmState, offset: i16) -> Result<(), VmError> {
    // Get the condition value from stack
    if vm.stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }

    let condition = vm.stack.pop().unwrap();

    // Check if condition is false (false or 0)
    let should_jump = match condition {
        Value::Bool(false) => true,
        Value::Int(0) => true,
        _ => false,
    };

    if should_jump {
        // Calculate new instruction pointer
        let new_ip = (vm.ip as i32 + offset as i32) as usize;

        // Validate the new IP is within bounds
        if new_ip >= vm.instructions.len() {
            return Err(VmError::UnknownOpCode);
        }

        vm.ip = new_ip;
    } else {
        // Don't jump, just continue to next instruction
        vm.ip += 1;
    }

    Ok(())
}
