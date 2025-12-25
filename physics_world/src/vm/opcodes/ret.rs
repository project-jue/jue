/// Ret opcode handler - implements proper function return system
/// Handles stack frame cleanup and return value propagation
use crate::types::Value;
use crate::vm::state::{InstructionResult, VmError, VmState};

/// Handles the Ret opcode with proper stack frame management
///
/// # Implementation Details (Unified Calling Convention)
/// 1. Function execution leaves return value on stack
/// 2. Get return value from stack
/// 3. Pop current frame (caller's state is already preserved on stack)
/// 4. Push return value for caller
/// 5. Restore caller's instructions and IP
pub fn handle_ret(vm: &mut VmState) -> Result<InstructionResult, VmError> {
    eprintln!(
        "handle_ret called - call_stack depth: {}",
        vm.call_stack.len()
    );
    eprintln!("Current stack: {:?}", vm.stack);

    // 1. Check if there's a call frame
    if vm.call_stack.is_empty() {
        eprintln!("Returning from main program - completing execution");
        let return_value = vm.stack.pop().unwrap_or(Value::Nil);
        return Ok(InstructionResult::Finished(return_value));
    }

    // 2. Get the call frame
    let call_frame = vm.call_stack.pop().unwrap();
    eprintln!(
        "DEBUG RET: return_ip={}, original_stack_size={}",
        call_frame.return_ip, call_frame.original_stack_size
    );

    // 3. Get return value from stack
    // The caller's locals are already on the stack at positions [0, original_stack_size)
    // The return value is at position original_stack_size (or top of stack if no locals)
    let return_value = if !vm.stack.is_empty() {
        vm.stack.pop().unwrap()
    } else {
        Value::Nil
    };
    eprintln!("DEBUG RET: return_value = {:?}", return_value);
    eprintln!("DEBUG RET: Stack after pop: {:?}", vm.stack);

    // 4. Push return value back for caller
    // This goes on TOP of the preserved caller state
    vm.stack.push(return_value.clone());

    // 5. Set instruction pointer for continuation
    // Check if this was the last frame
    if vm.call_stack.is_empty() {
        eprintln!("Returning to top-level - signaling completion");
        return Ok(InstructionResult::Finished(return_value));
    }

    // Restore caller's execution context
    vm.ip = call_frame.return_ip;

    if let Some(saved_instructions) = call_frame.saved_instructions {
        vm.instructions = saved_instructions;
    }

    eprintln!(
        "After ret: stack={:?}, ip={}, call_stack_depth={}",
        vm.stack,
        vm.ip,
        vm.call_stack.len()
    );

    Ok(InstructionResult::Continue)
}
