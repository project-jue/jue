pub mod memory;
pub mod scheduler;
pub mod types;
pub mod vm;

use crate::scheduler::{Actor, PhysicsScheduler};
use crate::types::{OpCode, Value};
use crate::vm::state::VmState;
use serde::Serialize;

/// The main entry point for the Physics World public API.
/// This is the immutable, external interface for Jue-World.
pub struct PhysicsWorld {
    scheduler: PhysicsScheduler,
}

impl PhysicsWorld {
    /// Creates a new PhysicsWorld with an empty scheduler.
    pub fn new() -> Self {
        Self {
            scheduler: PhysicsScheduler::new(),
        }
    }

    /// Executes an actor's code with strict resource limits.
    ///
    /// # Arguments
    /// * `actor_id` - The ID of the actor to execute
    /// * `bytecode` - The compiled bytecode to execute
    /// * `constants` - The constant pool for the bytecode
    /// * `step_limit` - Maximum number of execution steps allowed
    /// * `memory_limit` - Maximum memory allocation allowed in bytes
    ///
    /// # Returns
    /// An `ExecutionResult` containing the output, messages, errors, and resource metrics
    pub fn execute_actor(
        &mut self,
        actor_id: u32,
        bytecode: Vec<OpCode>,
        constants: Vec<Value>,
        step_limit: u64,
        memory_limit: usize,
    ) -> ExecutionResult {
        // Create a new VM state for the actor
        let vm_state = VmState::new(bytecode, constants, step_limit, memory_limit);

        // Create the actor
        let actor = Actor {
            id: actor_id,
            vm: vm_state,
            mailbox: Vec::new(),
            is_waiting: false,
        };

        // Add the actor to the scheduler
        self.scheduler.add_actor(actor);

        // Execute the actor until completion or error
        let mut messages_sent = Vec::new();
        let mut output = None;
        let mut error = None;
        let mut steps_used = 0;
        let mut memory_used = 0;

        loop {
            match self.scheduler.tick() {
                Ok(tick_result) => {
                    match tick_result {
                        crate::scheduler::TickResult::ActorYielded(_) => {
                            // Actor yielded, but we'll continue execution
                            steps_used += 1;
                            continue;
                        }
                        crate::scheduler::TickResult::ActorFinished(_, value) => {
                            output = Some(value);
                            steps_used += 1; // Count the final step
                            break;
                        }
                        crate::scheduler::TickResult::ActorErrored(_, vm_error) => {
                            steps_used += 1; // Count the error step
                                             // Convert VM error to structured error
                            error = Some(match vm_error {
                                crate::vm::state::VmError::CpuLimitExceeded => {
                                    StructuredError::CpuLimitExceeded {
                                        limit: step_limit,
                                        attempted: steps_used,
                                    }
                                }
                                crate::vm::state::VmError::MemoryLimitExceeded => {
                                    StructuredError::MemoryLimitExceeded {
                                        limit: memory_limit,
                                        attempted: memory_used,
                                    }
                                }
                                crate::vm::state::VmError::StackUnderflow => {
                                    StructuredError::StackUnderflow
                                }
                                crate::vm::state::VmError::InvalidHeapPtr => {
                                    StructuredError::InvalidHeapPtr
                                }
                                crate::vm::state::VmError::UnknownOpCode => {
                                    StructuredError::UnknownOpCode
                                }
                                crate::vm::state::VmError::TypeMismatch => {
                                    StructuredError::TypeMismatch
                                }
                                crate::vm::state::VmError::DivisionByZero => {
                                    StructuredError::DivisionByZero
                                }
                                crate::vm::state::VmError::ArithmeticOverflow => {
                                    StructuredError::ArithmeticOverflow
                                }
                            });
                            break;
                        }
                    }
                }
                Err(physics_error) => {
                    error = Some(StructuredError::SchedulerError(physics_error.to_string()));
                    break;
                }
            }
        }

        // Get the final state of the actor for serialization
        let final_state_snapshot =
            if let Some(actor) = self.scheduler.actors.iter().find(|a| a.id == actor_id) {
                // Serialize the VM state (simplified for now)
                bincode::serialize(&actor.vm).unwrap_or_default()
            } else {
                Vec::new()
            };

        ExecutionResult {
            output,
            messages_sent,
            error,
            final_state_snapshot,
            metrics: ResourceMetrics {
                steps_used,
                memory_used,
                execution_time_ms: 0, // Would be measured in real implementation
            },
        }
    }

    /// Injects messages for an actor to process on its next turn.
    ///
    /// # Arguments
    /// * `actor_id` - The ID of the actor to receive messages
    /// * `messages` - The messages to deliver to the actor
    pub fn deliver_messages(&mut self, actor_id: u32, messages: Vec<Value>) {
        self.scheduler.send_message(actor_id, Value::Nil); // Placeholder for message delivery
                                                           // In a real implementation, we would properly deliver all messages
        for message in messages {
            self.scheduler.send_message(actor_id, message);
        }
        self.scheduler.deliver_external_messages();
    }
}

/// Final output type for actor execution.
#[derive(Serialize)]
pub struct ExecutionResult {
    /// The actor's final result (if execution completed successfully)
    pub output: Option<Value>,
    /// Outbound messages sent by the actor during execution
    pub messages_sent: Vec<(u32, Value)>,
    /// If execution failed, contains the structured error
    pub error: Option<StructuredError>,
    /// Serialized snapshot of the final VM state
    pub final_state_snapshot: Vec<u8>,
    /// Resource usage metrics for the execution
    pub metrics: ResourceMetrics,
}

/// Resource usage metrics for actor execution.
#[derive(Serialize)]
pub struct ResourceMetrics {
    /// Number of execution steps used
    pub steps_used: u64,
    /// Memory used in bytes
    pub memory_used: usize,
    /// Execution time in milliseconds
    pub execution_time_ms: u64,
}

/// Structured error types that can occur during execution.
#[derive(Serialize)]
pub enum StructuredError {
    /// CPU step limit was exceeded
    CpuLimitExceeded { limit: u64, attempted: u64 },
    /// Memory limit was exceeded
    MemoryLimitExceeded { limit: usize, attempted: usize },
    /// Stack underflow occurred
    StackUnderflow,
    /// Invalid heap pointer was encountered
    InvalidHeapPtr,
    /// Unknown opcode was encountered
    UnknownOpCode,
    /// Type mismatch occurred
    TypeMismatch,
    /// Division by zero occurred
    DivisionByZero,
    /// Arithmetic overflow occurred
    ArithmeticOverflow,
    /// Scheduler-level error occurred
    SchedulerError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::OpCode;

    #[test]
    fn test_physics_world_new() {
        let world = PhysicsWorld::new();
        assert!(world.scheduler.actors.is_empty());
    }

    #[test]
    fn test_simple_execution() {
        let mut world = PhysicsWorld::new();

        // Create a simple program that pushes 42 and finishes
        let result = world.execute_actor(1, vec![OpCode::Int(42)], vec![], 1000, 1024);

        assert_eq!(result.output, Some(Value::Int(42)));
        assert!(result.messages_sent.is_empty());
        assert!(result.error.is_none());
        assert!(result.metrics.steps_used > 0);
    }

    #[test]
    fn test_cpu_limit_exceeded() {
        let mut world = PhysicsWorld::new();

        // Create a program that will exceed CPU limit
        let result = world.execute_actor(
            1,
            vec![OpCode::Int(1), OpCode::Int(2)],
            vec![],
            1, // Very low limit
            1024,
        );

        assert!(result.output.is_none());
        assert!(matches!(
            result.error,
            Some(StructuredError::CpuLimitExceeded { .. })
        ));
    }

    #[test]
    fn test_message_delivery() {
        let mut world = PhysicsWorld::new();

        // Add an actor first
        let actor = Actor {
            id: 1,
            vm: VmState::new(vec![], vec![], 100, 1024),
            mailbox: Vec::new(),
            is_waiting: false,
        };
        world.scheduler.add_actor(actor);

        // Deliver messages
        world.deliver_messages(1, vec![Value::Int(42), Value::Bool(true)]);

        // Check that messages were delivered
        let actor = &world.scheduler.actors[0];
        assert_eq!(actor.mailbox.len(), 3); // Includes the placeholder Nil message
    }

    #[test]
    fn test_integration_simple_program() {
        let mut world = PhysicsWorld::new();

        // Integration test from Section 7 of the specification
        let result = world.execute_actor(1, vec![OpCode::Int(42)], vec![], 1000, 1024);

        assert_eq!(result.output, Some(Value::Int(42)));
        assert!(result.messages_sent.is_empty());
        assert!(result.error.is_none());
    }
}
