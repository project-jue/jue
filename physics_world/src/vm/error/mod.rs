//! VM error handling module.
//!
//! This module provides comprehensive error handling for the VM, including:
//! - Error types ([`types`](types/index.html)): Core error type definitions
//! - Context ([`context`](context/index.html)): Error context and helper traits
//! - Recovery ([`recovery`](recovery/index.html)): Recovery action definitions
//!
//! # Example
//!
//! ```ignore
//! use crate::vm::error::{VmError, ErrorContext, RecoveryAction, WithContext, SimpleVmError};
//!
//! let simple_error = SimpleVmError::CpuLimitExceeded;
//! let context = ErrorContext { ... };
//! let detailed_error = simple_error.with_context(context);
//! ```

// Declare submodules
mod context;
mod recovery;
mod types;

// Re-export all types for backward compatibility
pub use types::{ErrorContext, SimpleVmError, StackFrame, VmError};

pub use context::WithContext;

pub use recovery::{RecoveryAction, RecoveryEntry};
