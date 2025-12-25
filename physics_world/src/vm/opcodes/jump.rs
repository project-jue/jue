/// Jump opcode handlers - Jmp and JmpIfFalse
///
/// Both jump instructions use RELATIVE offsets (like JVM, WebAssembly):
/// - target_ip = current_ip + 1 + offset
/// - offset can be negative for backward jumps (loops)
/// - offset can be positive for forward jumps (branches)
use crate::types::Value;
use crate::vm::state::VmError;
use crate::vm::state::VmState;

/// Handles the Jmp opcode - unconditional jump
/// Jump offset is RELATIVE to the next instruction: new_ip = current_ip + 1 + offset
pub fn handle_jmp(vm: &mut VmState, offset: i16) -> Result<(), VmError> {
    // Calculate new IP: current_ip + 1 (skip this instruction) + offset
    let new_ip = (vm.ip as i32 + 1 + offset as i32) as usize;

    eprintln!(
        "DEBUG: JMP instruction - current_ip={}, offset={}, new_ip={}",
        vm.ip, offset, new_ip
    );

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
/// Jump offset is RELATIVE to the next instruction: new_ip = current_ip + 1 + offset
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
        // Jump offset is relative to the next instruction: next_ip + offset
        let next_ip = vm.ip + 1;
        let new_ip = (next_ip as i32 + offset as i32) as usize;

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
            "DEBUG: Setting IP from {} to {} (conditional jump, offset={})",
            vm.ip, new_ip, offset
        );
        vm.ip = new_ip;
    } else {
        // Don't jump, just continue to next instruction
        eprintln!("DEBUG: Not jumping, continuing to next instruction");
        vm.ip += 1;
    }

    Ok(())
}
