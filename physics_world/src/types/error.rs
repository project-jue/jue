/// Error types for Physics World types
use std::fmt;

/// Type-safe operation result
#[derive(Debug, Clone)]
pub enum TypeSafeResult<T> {
    Success(T),
    TypeError(String),
    SafetyError(String),
}

impl<T> TypeSafeResult<T> {
    pub fn is_success(&self) -> bool {
        matches!(self, TypeSafeResult::Success(_))
    }

    pub fn unwrap(self) -> T {
        match self {
            TypeSafeResult::Success(value) => value,
            TypeSafeResult::TypeError(msg) => panic!("Type error: {}", msg),
            TypeSafeResult::SafetyError(msg) => panic!("Safety error: {}", msg),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            TypeSafeResult::Success(value) => value,
            _ => default,
        }
    }
}

// This module is intentionally left empty for now as the main error types
// are already defined in the scheduler error module. This provides a placeholder
// for future type-specific error handling.
