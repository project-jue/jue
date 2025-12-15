use crate::types::Value;
use crate::vm::state::{InstructionResult, VmError, VmState};
use std::collections::HashMap;

/// Represents an actor in the Physics World.
pub struct Actor {
    pub id: u32,
    pub vm: VmState,
    pub mailbox: Vec<Value>, // Incoming messages
    pub is_waiting: bool,
}

/// Manages multiple actors and enforces fair, deterministic execution.
pub struct PhysicsScheduler {
    pub actors: Vec<Actor>,
    pub current_actor_index: usize,
    pub message_queues: HashMap<u32, Vec<Value>>, // External inbox per actor
}

/// Result of a scheduler tick operation.
#[derive(Debug)]
pub enum TickResult {
    ActorYielded(u32),
    ActorFinished(u32, Value),
    ActorErrored(u32, VmError),
}

/// Error types that can occur during scheduler operations.
#[derive(Debug)]
pub enum PhysicsError {
    ActorNotFound(u32),
    SchedulerError(String),
}

impl std::fmt::Display for PhysicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhysicsError::ActorNotFound(id) => write!(f, "Actor not found: {}", id),
            PhysicsError::SchedulerError(msg) => write!(f, "Scheduler error: {}", msg),
        }
    }
}

impl PhysicsScheduler {
    /// Creates a new PhysicsScheduler with empty actor list.
    pub fn new() -> Self {
        Self {
            actors: Vec::new(),
            current_actor_index: 0,
            message_queues: HashMap::new(),
        }
    }

    /// Main execution tick. Runs the current actor until it yields, finishes, or hits a limit.
    pub fn tick(&mut self) -> Result<TickResult, PhysicsError> {
        // Check if there are any actors
        if self.actors.is_empty() {
            return Err(PhysicsError::SchedulerError(
                "No actors to schedule".to_string(),
            ));
        }

        // Get current actor
        let current_index = self.current_actor_index;
        let actor = &mut self.actors[current_index];

        // Process any messages in the actor's mailbox first
        if !actor.mailbox.is_empty() {
            // For now, just push messages onto the stack
            // In a real implementation, this would be more sophisticated
            for message in actor.mailbox.drain(..) {
                actor.vm.stack.push(message);
            }
        }

        // Execute the actor's VM until it yields, finishes, or errors
        loop {
            match actor.vm.step() {
                Ok(InstructionResult::Continue) => {
                    // Continue executing
                    continue;
                }
                Ok(InstructionResult::Yield) => {
                    // Actor yielded, move to next actor
                    let actor_id = actor.id;
                    self.advance_to_next_actor();
                    return Ok(TickResult::ActorYielded(actor_id));
                }
                Ok(InstructionResult::Finished(value)) => {
                    // Actor finished, move to next actor
                    let actor_id = actor.id;
                    self.advance_to_next_actor();
                    return Ok(TickResult::ActorFinished(actor_id, value));
                }
                Err(vm_error) => {
                    // Actor errored, move to next actor
                    let actor_id = actor.id;
                    self.advance_to_next_actor();
                    return Ok(TickResult::ActorErrored(actor_id, vm_error));
                }
            }
        }
    }

    /// Delivers a message to an actor's external queue.
    pub fn send_message(&mut self, target: u32, message: Value) {
        self.message_queues
            .entry(target)
            .or_insert_with(Vec::new)
            .push(message);
    }

    /// Advances to the next actor in round-robin fashion.
    fn advance_to_next_actor(&mut self) {
        if self.actors.is_empty() {
            self.current_actor_index = 0;
            return;
        }

        self.current_actor_index = (self.current_actor_index + 1) % self.actors.len();
    }

    /// Adds a new actor to the scheduler.
    pub fn add_actor(&mut self, actor: Actor) {
        self.actors.push(actor);
    }

    /// Gets the current actor ID.
    pub fn current_actor_id(&self) -> Option<u32> {
        self.actors.get(self.current_actor_index).map(|a| a.id)
    }

    /// Delivers external messages to actors' mailboxes.
    pub fn deliver_external_messages(&mut self) {
        for (actor_id, messages) in self.message_queues.drain() {
            if let Some(actor) = self.actors.iter_mut().find(|a| a.id == actor_id) {
                actor.mailbox.extend(messages);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::OpCode;

    #[test]
    fn test_new_scheduler() {
        let scheduler = PhysicsScheduler::new();
        assert_eq!(scheduler.actors.len(), 0);
        assert_eq!(scheduler.current_actor_index, 0);
        assert_eq!(scheduler.message_queues.len(), 0);
    }

    #[test]
    fn test_add_actor() {
        let mut scheduler = PhysicsScheduler::new();
        let actor = Actor {
            id: 1,
            vm: VmState::new(vec![], vec![], 100, 1024),
            mailbox: Vec::new(),
            is_waiting: false,
        };
        scheduler.add_actor(actor);

        assert_eq!(scheduler.actors.len(), 1);
        assert_eq!(scheduler.current_actor_id(), Some(1));
    }

    #[test]
    fn test_round_robin_scheduling() {
        let mut scheduler = PhysicsScheduler::new();

        // Add two actors
        let actor1 = Actor {
            id: 1,
            vm: VmState::new(vec![OpCode::Yield], vec![], 100, 1024),
            mailbox: Vec::new(),
            is_waiting: false,
        };

        let actor2 = Actor {
            id: 2,
            vm: VmState::new(vec![OpCode::Yield], vec![], 100, 1024),
            mailbox: Vec::new(),
            is_waiting: false,
        };

        scheduler.add_actor(actor1);
        scheduler.add_actor(actor2);

        // First tick should execute actor 1
        let result1 = scheduler.tick();
        assert!(matches!(result1, Ok(TickResult::ActorYielded(1))));

        // Second tick should execute actor 2
        let result2 = scheduler.tick();
        assert!(matches!(result2, Ok(TickResult::ActorYielded(2))));

        // Third tick should execute actor 1 again, but it has no more instructions so it finishes
        let result3 = scheduler.tick();
        assert!(matches!(result3, Ok(TickResult::ActorFinished(1, _))));
    }

    #[test]
    fn test_message_passing() {
        let mut scheduler = PhysicsScheduler::new();

        // Create actor 1 that sends a message
        let actor1 = Actor {
            id: 1,
            vm: VmState::new(
                vec![
                    OpCode::Int(42),
                    OpCode::Int(2), // Target actor ID
                    OpCode::Send, // This will pop the message and target actor ID and "send" it
                    OpCode::Yield,
                ],
                vec![],
                100,
                1024,
            ),
            mailbox: Vec::new(),
            is_waiting: false,
        };

        // Create actor 2 that will receive the message
        let actor2 = Actor {
            id: 2,
            vm: VmState::new(vec![OpCode::Yield], vec![], 100, 1024),
            mailbox: Vec::new(),
            is_waiting: false,
        };

        scheduler.add_actor(actor1);
        scheduler.add_actor(actor2);

        // Execute actor 1 (it will send a message but our Send implementation just pops it)
        let result1 = scheduler.tick();
        assert!(matches!(result1, Ok(TickResult::ActorYielded(1))));

        // Execute actor 2
        let result2 = scheduler.tick();
        assert!(matches!(result2, Ok(TickResult::ActorYielded(2))));
    }

    #[test]
    fn test_actor_finish() {
        let mut scheduler = PhysicsScheduler::new();

        // Create an actor that finishes immediately
        let actor = Actor {
            id: 1,
            vm: VmState::new(vec![], vec![], 100, 1024), // Empty program
            mailbox: Vec::new(),
            is_waiting: false,
        };

        scheduler.add_actor(actor);

        // Execute the actor
        let result = scheduler.tick();
        assert!(matches!(result, Ok(TickResult::ActorFinished(1, _))));
    }

    #[test]
    fn test_actor_error() {
        let mut scheduler = PhysicsScheduler::new();

        // Create an actor that will cause a stack underflow
        let actor = Actor {
            id: 1,
            vm: VmState::new(vec![OpCode::Pop], vec![], 100, 1024), // Pop from empty stack
            mailbox: Vec::new(),
            is_waiting: false,
        };

        scheduler.add_actor(actor);

        // Execute the actor
        let result = scheduler.tick();
        assert!(matches!(
            result,
            Ok(TickResult::ActorErrored(1, VmError::StackUnderflow))
        ));
    }

    #[test]
    fn test_send_message() {
        let mut scheduler = PhysicsScheduler::new();

        // Add an actor
        let actor = Actor {
            id: 1,
            vm: VmState::new(vec![], vec![], 100, 1024),
            mailbox: Vec::new(),
            is_waiting: false,
        };
        scheduler.add_actor(actor);

        // Send a message to the actor
        scheduler.send_message(1, Value::Int(42));

        // Deliver external messages
        scheduler.deliver_external_messages();

        // Check that the message was delivered to the actor's mailbox
        let actor = &scheduler.actors[0];
        assert_eq!(actor.mailbox.len(), 1);
        assert_eq!(actor.mailbox[0], Value::Int(42));
    }

    #[test]
    fn test_empty_scheduler_tick() {
        let mut scheduler = PhysicsScheduler::new();

        // Try to tick with no actors
        let result = scheduler.tick();
        assert!(matches!(result, Err(PhysicsError::SchedulerError(_))));
    }
}
