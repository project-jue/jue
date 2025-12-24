/// Jump opcode handlers - Jmp and JmpIfFalse
use crate::types::Value;
use crate::vm::state::VmError;
use crate::vm::state::VmState;

/// Handles the Jmp opcode - unconditional jump
pub fn handle_jmp(vm: &mut VmState, offset: i16) -> Result<(), VmError> {
    eprintln!(
        "DEBUG: JMP instruction - current_ip={}, offset={}, new_ip={}",
        vm.ip,
        offset,
        (vm.ip as i32 + offset as i32) as usize
    );

    // Calculate new instruction pointer
    let new_ip = (vm.ip as i32 + offset as i32) as usize;

    // Validate the new IP is within bounds
    if new_ip >= vm.instructions.len() {
        eprintln!(
            "DEBUG: Jump out of bounds! new_ip={} >= instructions.len()={}",
            new_ip,
            vm.instructions.len()
        );
        return Err(VmError::UnknownOpCode);
    }

    eprintln!("DEBUG: Setting IP from {} to {}", vm.ip, new_ip);
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

    eprintln!(
        "DEBUG: JmpIfFalse - condition={:?}, should_jump={}, offset={}",
        condition, should_jump, offset
    );

    if should_jump {
        // Calculate new instruction pointer
        let new_ip = (vm.ip as i32 + offset as i32) as usize;

        // Validate the new IP is within bounds
        if new_ip >= vm.instructions.len() {
            eprintln!(
                "DEBUG: Conditional jump out of bounds! new_ip={} >= instructions.len()={}",
                new_ip,
                vm.instructions.len()
            );
            return Err(VmError::UnknownOpCode);
        }

        eprintln!(
            "DEBUG: Setting IP from {} to {} (conditional jump)",
            vm.ip, new_ip
        );
        vm.ip = new_ip;
    } else {
        // Don't jump, just continue to next instruction
        eprintln!("DEBUG: Not jumping, continuing to next instruction");
        vm.ip += 1;
    }

    Ok(())
}
