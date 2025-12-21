/// Integration types and traits for Physics World
use serde::Serialize;
use std::fmt;

use crate::types::Value;

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
#[derive(Serialize, Debug)]
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
    /// Capability-related error occurred
    CapabilityError(String),
}

impl fmt::Display for StructuredError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StructuredError::CpuLimitExceeded { limit, attempted } => {
                write!(
                    f,
                    "CPU limit exceeded: attempted {} steps, limit was {}",
                    attempted, limit
                )
            }
            StructuredError::MemoryLimitExceeded { limit, attempted } => {
                write!(
                    f,
                    "Memory limit exceeded: attempted {} bytes, limit was {}",
                    attempted, limit
                )
            }
            StructuredError::StackUnderflow => write!(f, "Stack underflow"),
            StructuredError::InvalidHeapPtr => write!(f, "Invalid heap pointer"),
            StructuredError::UnknownOpCode => write!(f, "Unknown opcode"),
            StructuredError::TypeMismatch => write!(f, "Type mismatch"),
            StructuredError::DivisionByZero => write!(f, "Division by zero"),
            StructuredError::ArithmeticOverflow => write!(f, "Arithmetic overflow"),
            StructuredError::SchedulerError(msg) => write!(f, "Scheduler error: {}", msg),
            StructuredError::CapabilityError(msg) => write!(f, "Capability error: {}", msg),
        }
    }
}

impl fmt::Debug for StructuredError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
