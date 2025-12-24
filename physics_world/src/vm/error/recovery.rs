//! Recovery action definitions for error handling.
//!
//! This module defines the [`RecoveryAction`] enum that specifies possible
//! recovery strategies when VM errors occur.

use crate::types::OpCode;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Recovery actions that can be taken to handle errors
#[derive(Debug, Clone)]
pub enum RecoveryAction {
    /// Increase the CPU step limit
    IncreaseCpuLimit(u64),
    /// Increase the memory limit
    IncreaseMemoryLimit(usize),
    /// Request additional capability
    RequestCapability(String),
    /// Terminate execution gracefully
    TerminateGracefully,
    /// Continue with default values
    ContinueWithDefaults,
}

/// Priority queue entry for error recovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryEntry {
    /// Priority value (lower = higher priority)
    pub priority: u64,
    /// Error message or description
    pub message: String,
    /// Number of recovery attempts
    pub attempts: u32,
    /// Last operation attempted
    pub last_opcode: Option<OpCode>,
    /// Timestamp of when this entry was created
    pub timestamp: u64,
}

impl RecoveryEntry {
    /// Create a new recovery entry
    pub fn new(priority: u64, message: String, attempts: u32, last_opcode: Option<OpCode>) -> Self {
        Self {
            priority,
            message,
            attempts,
            last_opcode,
            timestamp: 0, // Will be set by the caller
        }
    }

    /// Increment the attempt counter
    pub fn increment_attempts(&mut self) {
        self.attempts += 1;
    }

    /// Update the timestamp
    pub fn set_timestamp(&mut self, timestamp: u64) {
        self.timestamp = timestamp;
    }
}

impl fmt::Display for RecoveryAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecoveryAction::IncreaseCpuLimit(limit) => write!(f, "Increase CPU limit to {}", limit),
            RecoveryAction::IncreaseMemoryLimit(limit) => {
                write!(f, "Increase memory limit to {} bytes", limit)
            }
            RecoveryAction::RequestCapability(cap) => {
                write!(f, "Request capability: {}", cap)
            }
            RecoveryAction::TerminateGracefully => write!(f, "Terminate execution gracefully"),
            RecoveryAction::ContinueWithDefaults => write!(f, "Continue with default values"),
        }
    }
}
