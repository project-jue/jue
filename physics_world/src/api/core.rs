/// Core Physics World implementation
use std::collections::HashSet;

use crate::scheduler::{Actor, CapDecision, PhysicsScheduler, TickResult};
use crate::types::{OpCode, Value};
use crate::vm::state::VmState;

use super::integration::{ExecutionResult, ResourceMetrics, StructuredError};

/// The main entry point for the Physics World public API.
/// This is the immutable, external interface for Jue-World.
pub struct PhysicsWorld {
    scheduler: PhysicsScheduler,
}

impl PhysicsWorld {
    /// Get a mutable reference to the scheduler for internal use
    pub fn scheduler_mut(&mut self) -> &mut PhysicsScheduler {
        &mut self.scheduler
    }
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
        let vm_state = VmState::new(bytecode, constants, step_limit, memory_limit, actor_id, 100);

        // Create the actor with V2 capability fields and priority fields
        let actor = Actor {
            id: actor_id,
            vm: vm_state,
            mailbox: Vec::new(),
            is_waiting: false,
            capabilities: HashSet::new(),
            capability_requests: Vec::new(),
            parent_id: None,
            priority: 128, // Default priority
            priority_boost: None,
        };

        // Add the actor to the scheduler
        self.scheduler.add_actor(actor);

        // Execute the actor until completion or error
        let messages_sent = Vec::new();
        let mut output = None;
        let mut error = None;
        let mut steps_used = 0;
        let memory_used = 0;

        loop {
            match self.scheduler.tick() {
                Ok(tick_result) => {
                    match tick_result {
                        TickResult::ActorYielded(_) => {
                            // Actor yielded, but we'll continue execution
                            steps_used += 1;
                            continue;
                        }
                        TickResult::ActorFinished(_, value) => {
                            output = Some(value);
                            steps_used += 1; // Count the final step
                            break;
                        }
                        TickResult::ActorErrored(_, vm_error) => {
                            steps_used += 1; // Count the error step
                                             // Convert VM error to structured error
                            error = Some(match vm_error {
                                crate::vm::error::VmError::CpuLimitExceeded { limit, .. } => {
                                    StructuredError::CpuLimitExceeded {
                                        limit,
                                        attempted: steps_used,
                                    }
                                }
                                crate::vm::error::VmError::MemoryLimitExceeded {
                                    limit, ..
                                } => StructuredError::MemoryLimitExceeded {
                                    limit,
                                    attempted: memory_used,
                                },
                                crate::vm::error::VmError::StackUnderflow { .. } => {
                                    StructuredError::StackUnderflow
                                }
                                crate::vm::error::VmError::InvalidHeapPtr { .. } => {
                                    StructuredError::InvalidHeapPtr
                                }
                                crate::vm::error::VmError::UnknownOpCode { .. } => {
                                    StructuredError::UnknownOpCode
                                }
                                crate::vm::error::VmError::TypeMismatch { .. } => {
                                    StructuredError::TypeMismatch
                                }
                                crate::vm::error::VmError::DivisionByZero { .. } => {
                                    StructuredError::DivisionByZero
                                }
                                crate::vm::error::VmError::ArithmeticOverflow { .. } => {
                                    StructuredError::ArithmeticOverflow
                                }
                                crate::vm::error::VmError::CapabilityError {
                                    capability, ..
                                } => StructuredError::CapabilityError(format!(
                                    "Capability error: {}",
                                    capability
                                )),
                                crate::vm::error::VmError::SerializationError {
                                    message, ..
                                } => StructuredError::SchedulerError(format!(
                                    "Serialization error: {}",
                                    message
                                )),
                                crate::vm::error::VmError::HeapCorruption { message, .. } => {
                                    StructuredError::SchedulerError(format!(
                                        "Heap corruption: {}",
                                        message
                                    ))
                                }
                                crate::vm::error::VmError::RecursionLimitExceeded { .. } => {
                                    StructuredError::SchedulerError(
                                        "Recursion limit exceeded".to_string(),
                                    )
                                }
                                crate::vm::error::VmError::StackOverflow { .. } => {
                                    StructuredError::SchedulerError(
                                        "Stack overflow".to_string(),
                                    )
                                }
                                crate::vm::error::VmError::GcDisabled => {
                                    StructuredError::SchedulerError(
                                        "GC disabled".to_string(),
                                    )
                                }
                                crate::vm::error::VmError::HeapExhausted => {
                                    StructuredError::SchedulerError(
                                        "Heap exhausted".to_string(),
                                    )
                                }
                                crate::vm::error::VmError::DebuggerError { message } => {
                                    StructuredError::SchedulerError(format!(
                                        "Debugger error: {}",
                                        message
                                    ))
                                }
                            });
                            break;
                        }
                        TickResult::ActorWaitingForCapability(actor_id, capability) => {
                            // Handle capability request
                            let decision = self.scheduler.handle_capability_request(
                                actor_id,
                                capability,
                                "Requested during execution",
                            );

                            match decision {
                                CapDecision::Granted => {
                                    // Capability granted, continue execution
                                    continue;
                                }
                                CapDecision::Denied => {
                                    // Capability denied, return error
                                    error = Some(StructuredError::CapabilityError(
                                        "Capability request denied".to_string(),
                                    ));
                                    break;
                                }
                                CapDecision::PendingConsensus => {
                                    // Capability pending consensus, yield and wait
                                    steps_used += 1;
                                    return ExecutionResult {
                                        output: None,
                                        messages_sent,
                                        error: Some(StructuredError::CapabilityError(
                                            "Capability request pending consensus".to_string(),
                                        )),
                                        final_state_snapshot: Vec::new(),
                                        metrics: ResourceMetrics {
                                            steps_used,
                                            memory_used,
                                            execution_time_ms: 0,
                                        },
                                    };
                                }
                            }
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
