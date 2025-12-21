/// Error types for the Physics World scheduler

/// Error types that can occur during scheduler operations.
#[derive(Debug)]
pub enum PhysicsError {
    ActorNotFound(u32),
    SchedulerError(String),
    CapabilityError(String),
    ConsensusError(String),
}

impl std::fmt::Display for PhysicsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhysicsError::ActorNotFound(id) => write!(f, "Actor not found: {}", id),
            PhysicsError::SchedulerError(msg) => write!(f, "Scheduler error: {}", msg),
            PhysicsError::CapabilityError(msg) => write!(f, "Capability error: {}", msg),
            PhysicsError::ConsensusError(msg) => write!(f, "Consensus error: {}", msg),
        }
    }
}
