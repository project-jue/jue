/// Send operation handler for inter-actor communication
use crate::types::Value;
use crate::vm::state::{InstructionResult, VmError, VmState};

/// Send a message to another actor
pub fn handle_send(vm: &mut VmState) -> Result<InstructionResult, VmError> {
    let message = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let target_actor = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match target_actor {
        Value::ActorId(actor_id) => {
            // In a real implementation, this would:
            // 1. Check if the current actor has permission to send to target_actor
            // 2. Deliver the message to the target actor's mailbox
            // 3. Update the sender's message log/statistics

            // For now, we'll just log that a message was sent
            eprintln!(
                "Send: Actor {} sending message {:?} to actor {}",
                vm.actor_id, message, actor_id
            );

            // Continue execution - message sending is non-blocking
            Ok(InstructionResult::Continue)
        }
        _ => Err(VmError::TypeMismatch),
    }
}
