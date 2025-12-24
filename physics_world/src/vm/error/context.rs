//! Error context and context helper traits.
//!
//! This module provides the [`WithContext`] trait for adding error context
//! to simple errors, converting them into detailed [`VmError`] types.

use super::types::{ErrorContext, SimpleVmError, VmError};

/// Helper trait to convert simple errors to detailed errors with context
pub trait WithContext {
    /// Convert this error to a detailed VmError with the provided context
    fn with_context(self, context: ErrorContext) -> VmError;
}

impl WithContext for SimpleVmError {
    fn with_context(self, context: ErrorContext) -> VmError {
        match self {
            SimpleVmError::CpuLimitExceeded => VmError::cpu_limit_exceeded(context, 0),
            SimpleVmError::MemoryLimitExceeded => VmError::memory_limit_exceeded(context, 0, 0),
            SimpleVmError::StackUnderflow => VmError::stack_underflow(context, "operation", 1, 0),
            SimpleVmError::InvalidHeapPtr => VmError::invalid_heap_ptr(context, None, "operation"),
            SimpleVmError::UnknownOpCode => VmError::unknown_opcode(context, None),
            SimpleVmError::TypeMismatch => {
                VmError::type_mismatch(context, "operation", "expected", "actual")
            }
            SimpleVmError::DivisionByZero => VmError::division_by_zero(context, "division"),
            SimpleVmError::ArithmeticOverflow => {
                VmError::arithmetic_overflow(context, "operation", None, None)
            }
            SimpleVmError::CapabilityDenied => {
                VmError::capability_error(context, "unknown", "operation")
            }
            SimpleVmError::RecursionLimitExceeded => {
                VmError::recursion_limit_exceeded(context, 0, 0)
            }
        }
    }
}
