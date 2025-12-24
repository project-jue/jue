//! Error type definitions for the VM.
//!
//! This module contains the core error types used throughout the VM:
//! - [`SimpleVmError`]: Simple error types for internal use
//! - [`ErrorContext`]: Enhanced error context capturing VM state
//! - [`StackFrame`]: Represents a single stack frame in call traces
//! - [`VmError`]: Detailed VM errors with comprehensive context

use crate::types::{HeapPtr, OpCode, Value};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Simple error types that can occur during VM execution.
///
/// These are used internally by opcode handlers and converted to detailed errors.
#[derive(Debug)]
pub enum SimpleVmError {
    CpuLimitExceeded,       // Resource limit violation
    MemoryLimitExceeded,    // Resource limit violation
    StackUnderflow,         // Invalid operation
    InvalidHeapPtr,         // Memory safety violation
    UnknownOpCode,          // Invalid instruction
    TypeMismatch,           // Type system violation
    DivisionByZero,         // Arithmetic error
    ArithmeticOverflow,     // Arithmetic error
    CapabilityDenied,       // Capability system violation
    RecursionLimitExceeded, // Recursion depth exceeded
}

/// Enhanced error context that captures the VM state at the time of error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Instruction pointer at time of error
    pub instruction_pointer: usize,
    /// Current instruction being executed
    pub current_instruction: Option<OpCode>,
    /// Stack state at time of error
    pub stack_state: Vec<Value>,
    /// Call stack state at time of error
    pub call_stack_depth: usize,
    /// Steps remaining at time of error
    pub steps_remaining: u64,
    /// Actor ID where error occurred
    pub actor_id: u32,
    /// Memory usage at time of error
    pub memory_usage: usize,
    /// Stack trace showing the call chain
    pub stack_trace: Vec<StackFrame>,
    /// Execution history (last N instructions)
    pub execution_history: Vec<OpCode>,
    /// Error timestamp (global step count)
    pub timestamp: u64,
}

/// Represents a single stack frame in the call trace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StackFrame {
    /// Function name or description
    pub function_name: String,
    /// Instruction pointer at time of call
    pub call_ip: usize,
    /// Number of arguments passed
    pub arg_count: usize,
    /// Local variables captured
    pub locals: Vec<Value>,
}

/// Detailed VM error with comprehensive context information
#[derive(Debug)]
pub enum VmError {
    /// Resource limit violation - CPU execution steps exceeded
    CpuLimitExceeded { context: ErrorContext, limit: u64 },

    /// Resource limit violation - Memory allocation exceeded
    MemoryLimitExceeded {
        context: ErrorContext,
        limit: usize,
        requested: usize,
    },

    /// Stack operation error - insufficient values on stack
    StackUnderflow {
        context: ErrorContext,
        operation: String,
        required: usize,
        available: usize,
    },

    /// Memory access error - invalid heap pointer
    InvalidHeapPtr {
        context: ErrorContext,
        pointer: Option<HeapPtr>,
        operation: String,
    },

    /// Execution error - unknown or unsupported opcode
    UnknownOpCode {
        context: ErrorContext,
        opcode: Option<u8>,
    },

    /// Type system violation - operation on incompatible types
    TypeMismatch {
        context: ErrorContext,
        operation: String,
        expected: String,
        actual: String,
    },

    /// Arithmetic error - division by zero
    DivisionByZero {
        context: ErrorContext,
        operation: String,
    },

    /// Arithmetic error - overflow in arithmetic operation
    ArithmeticOverflow {
        context: ErrorContext,
        operation: String,
        operand1: Option<i64>,
        operand2: Option<i64>,
    },

    /// Capability system error - insufficient privileges
    CapabilityError {
        context: ErrorContext,
        capability: String,
        operation: String,
    },

    /// Serialization/deserialization error
    SerializationError {
        context: ErrorContext,
        message: String,
    },

    /// Heap corruption detected
    HeapCorruption {
        context: ErrorContext,
        message: String,
    },

    /// Recursion depth limit exceeded
    RecursionLimitExceeded {
        context: ErrorContext,
        limit: u32,
        current_depth: u32,
    },

    /// Stack overflow error
    StackOverflow {
        context: ErrorContext,
        max_depth: usize,
        attempted_depth: usize,
    },

    /// GC disabled error
    GcDisabled,

    /// Heap exhausted error
    HeapExhausted,

    /// Debugger error
    DebuggerError { message: String },
}

impl VmError {
    /// Create a new error context from the current VM state
    pub fn create_context(
        ip: usize,
        current_instruction: Option<OpCode>,
        stack: &[Value],
        call_stack_depth: usize,
        steps_remaining: u64,
        actor_id: u32,
        memory_usage: usize,
        stack_trace: Vec<StackFrame>,
        execution_history: Vec<OpCode>,
        timestamp: u64,
    ) -> ErrorContext {
        ErrorContext {
            instruction_pointer: ip,
            current_instruction,
            stack_state: stack.to_vec(),
            call_stack_depth,
            steps_remaining,
            actor_id,
            memory_usage,
            stack_trace,
            execution_history,
            timestamp,
        }
    }

    /// Create a simple error context with default values for stack trace and history
    pub fn create_simple_context(
        ip: usize,
        current_instruction: Option<OpCode>,
        stack: &[Value],
        call_stack_depth: usize,
        steps_remaining: u64,
        actor_id: u32,
        memory_usage: usize,
    ) -> ErrorContext {
        ErrorContext {
            instruction_pointer: ip,
            current_instruction,
            stack_state: stack.to_vec(),
            call_stack_depth,
            steps_remaining,
            actor_id,
            memory_usage,
            stack_trace: Vec::new(),
            execution_history: Vec::new(),
            timestamp: 0,
        }
    }

    /// Create a CPU limit exceeded error
    pub fn cpu_limit_exceeded(context: ErrorContext, limit: u64) -> Self {
        VmError::CpuLimitExceeded { context, limit }
    }

    /// Create a memory limit exceeded error
    pub fn memory_limit_exceeded(context: ErrorContext, limit: usize, requested: usize) -> Self {
        VmError::MemoryLimitExceeded {
            context,
            limit,
            requested,
        }
    }

    /// Create a stack underflow error
    pub fn stack_underflow(
        context: ErrorContext,
        operation: &str,
        required: usize,
        available: usize,
    ) -> Self {
        VmError::StackUnderflow {
            context,
            operation: operation.to_string(),
            required,
            available,
        }
    }

    /// Create an invalid heap pointer error
    pub fn invalid_heap_ptr(
        context: ErrorContext,
        pointer: Option<HeapPtr>,
        operation: &str,
    ) -> Self {
        VmError::InvalidHeapPtr {
            context,
            pointer,
            operation: operation.to_string(),
        }
    }

    /// Create an unknown opcode error
    pub fn unknown_opcode(context: ErrorContext, opcode: Option<u8>) -> Self {
        VmError::UnknownOpCode { context, opcode }
    }

    /// Create a type mismatch error
    pub fn type_mismatch(
        context: ErrorContext,
        operation: &str,
        expected: &str,
        actual: &str,
    ) -> Self {
        VmError::TypeMismatch {
            context,
            operation: operation.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        }
    }

    /// Create a division by zero error
    pub fn division_by_zero(context: ErrorContext, operation: &str) -> Self {
        VmError::DivisionByZero {
            context,
            operation: operation.to_string(),
        }
    }

    /// Create an arithmetic overflow error
    pub fn arithmetic_overflow(
        context: ErrorContext,
        operation: &str,
        operand1: Option<i64>,
        operand2: Option<i64>,
    ) -> Self {
        VmError::ArithmeticOverflow {
            context,
            operation: operation.to_string(),
            operand1,
            operand2,
        }
    }

    /// Create a capability error
    pub fn capability_error(context: ErrorContext, capability: &str, operation: &str) -> Self {
        VmError::CapabilityError {
            context,
            capability: capability.to_string(),
            operation: operation.to_string(),
        }
    }

    /// Create a serialization error
    pub fn serialization_error(context: ErrorContext, message: &str) -> Self {
        VmError::SerializationError {
            context,
            message: message.to_string(),
        }
    }

    /// Create a heap corruption error
    pub fn heap_corruption(context: ErrorContext, message: &str) -> Self {
        VmError::HeapCorruption {
            context,
            message: message.to_string(),
        }
    }

    /// Create a recursion limit exceeded error
    pub fn recursion_limit_exceeded(context: ErrorContext, limit: u32, current_depth: u32) -> Self {
        VmError::RecursionLimitExceeded {
            context,
            limit,
            current_depth,
        }
    }

    /// Get the error context
    pub fn context(&self) -> &ErrorContext {
        match self {
            VmError::CpuLimitExceeded { context, .. } => context,
            VmError::MemoryLimitExceeded { context, .. } => context,
            VmError::StackUnderflow { context, .. } => context,
            VmError::InvalidHeapPtr { context, .. } => context,
            VmError::UnknownOpCode { context, .. } => context,
            VmError::TypeMismatch { context, .. } => context,
            VmError::DivisionByZero { context, .. } => context,
            VmError::ArithmeticOverflow { context, .. } => context,
            VmError::CapabilityError { context, .. } => context,
            VmError::SerializationError { context, .. } => context,
            VmError::HeapCorruption { context, .. } => context,
            VmError::RecursionLimitExceeded { context, .. } => context,
            VmError::StackOverflow { context, .. } => context,
            VmError::GcDisabled => panic!("GcDisabled error has no context"),
            VmError::HeapExhausted => panic!("HeapExhausted error has no context"),
            VmError::DebuggerError { .. } => panic!("DebuggerError has no context"),
        }
    }

    /// Get a detailed error message with context
    pub fn detailed_message(&self) -> String {
        match self {
            VmError::CpuLimitExceeded { context, limit } => {
                format!(
                    "CPU Limit Exceeded: {} steps limit reached at IP {} (actor {}). Steps remaining: {}, Stack depth: {}, Memory usage: {} bytes",
                    limit,
                    context.instruction_pointer,
                    context.actor_id,
                    context.steps_remaining,
                    context.stack_state.len(),
                    context.memory_usage
                )
            }
            VmError::MemoryLimitExceeded {
                context,
                limit,
                requested,
            } => {
                format!(
                    "Memory Limit Exceeded: Requested {} bytes, limit is {} bytes at IP {} (actor {}). Memory usage: {} bytes, Stack depth: {}",
                    requested, limit, context.instruction_pointer, context.actor_id, context.memory_usage, context.stack_state.len()
                )
            }
            VmError::StackUnderflow {
                context,
                operation,
                required,
                available,
            } => {
                format!(
                    "Stack Underflow: {} requires {} values, only {} available at IP {} (actor {}). Stack: {:?}",
                    operation, required, available, context.instruction_pointer, context.actor_id, context.stack_state
                )
            }
            VmError::InvalidHeapPtr {
                context,
                pointer,
                operation,
            } => {
                format!(
                    "Invalid Heap Pointer: {} failed with pointer {:?} at IP {} (actor {}). Stack: {:?}",
                    operation,
                    pointer.map(|p| p.get()).unwrap_or(0),
                    context.instruction_pointer,
                    context.actor_id,
                    context.stack_state
                )
            }
            VmError::UnknownOpCode { context, opcode } => {
                format!(
                    "Unknown OpCode: {:?} at IP {} (actor {}). Current instruction: {:?}, Stack: {:?}",
                    opcode,
                    context.instruction_pointer,
                    context.actor_id,
                    context.current_instruction,
                    context.stack_state
                )
            }
            VmError::TypeMismatch {
                context,
                operation,
                expected,
                actual,
            } => {
                format!(
                    "Type Mismatch: {} expected {}, got {} at IP {} (actor {}). Stack: {:?}",
                    operation,
                    expected,
                    actual,
                    context.instruction_pointer,
                    context.actor_id,
                    context.stack_state
                )
            }
            VmError::DivisionByZero { context, operation } => {
                format!(
                    "Division By Zero: {} at IP {} (actor {}). Stack: {:?}",
                    operation, context.instruction_pointer, context.actor_id, context.stack_state
                )
            }
            VmError::ArithmeticOverflow {
                context,
                operation,
                operand1,
                operand2,
            } => {
                format!(
                    "Arithmetic Overflow: {} with operands {:?}, {:?} at IP {} (actor {}). Stack: {:?}",
                    operation,
                    operand1,
                    operand2,
                    context.instruction_pointer,
                    context.actor_id,
                    context.stack_state
                )
            }
            VmError::CapabilityError {
                context,
                capability,
                operation,
            } => {
                format!(
                    "Capability Error: {} requires capability '{}' at IP {} (actor {}). Stack: {:?}",
                    operation, capability, context.instruction_pointer, context.actor_id, context.stack_state
                )
            }
            VmError::SerializationError { context, message } => {
                format!(
                    "Serialization Error: {} at IP {} (actor {}). Stack: {:?}",
                    message, context.instruction_pointer, context.actor_id, context.stack_state
                )
            }
            VmError::HeapCorruption { context, message } => {
                format!(
                    "Heap Corruption: {} at IP {} (actor {}). Stack: {:?}",
                    message, context.instruction_pointer, context.actor_id, context.stack_state
                )
            }
            VmError::RecursionLimitExceeded {
                context,
                limit,
                current_depth,
            } => {
                format!(
                    "Recursion Limit Exceeded: Depth {} exceeds limit {} at IP {} (actor {}). Stack: {:?}",
                    current_depth, limit, context.instruction_pointer, context.actor_id, context.stack_state
                )
            }
            VmError::StackOverflow {
                context,
                max_depth,
                attempted_depth,
            } => {
                format!(
                    "Stack Overflow: Attempted depth {} exceeds max {} at IP {} (actor {}). Stack: {:?}",
                    attempted_depth, max_depth, context.instruction_pointer, context.actor_id, context.stack_state
                )
            }
            VmError::GcDisabled => "GC Disabled: Garbage collection is disabled".to_string(),
            VmError::HeapExhausted => "Heap Exhausted: Heap memory exhausted".to_string(),
            VmError::DebuggerError { message } => {
                format!("Debugger Error: {}", message)
            }
        }
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            VmError::StackUnderflow { .. } => false,
            VmError::InvalidHeapPtr { .. } => false,
            VmError::UnknownOpCode { .. } => false,
            VmError::TypeMismatch { .. } => false,
            VmError::DivisionByZero { .. } => false,
            VmError::ArithmeticOverflow { .. } => false,
            VmError::HeapCorruption { .. } => false,
            VmError::SerializationError { .. } => false,
            VmError::StackOverflow { .. } => false,
            VmError::RecursionLimitExceeded { .. } => true, // Can be recovered with higher limit
            VmError::CpuLimitExceeded { .. } => true,       // Can be recovered with more steps
            VmError::MemoryLimitExceeded { .. } => true,    // Can be recovered with more memory
            VmError::CapabilityError { .. } => true, // Can be recovered with proper capabilities
            VmError::GcDisabled => true,             // Can be recovered by enabling GC
            VmError::HeapExhausted => true,          // Can be recovered with more heap
            VmError::DebuggerError { .. } => true,   // Debugger errors are recoverable
        }
    }

    /// Attempt to recover from the error if possible
    pub fn attempt_recovery(&self) -> Option<super::RecoveryAction> {
        match self {
            VmError::CpuLimitExceeded { context, limit } => {
                Some(super::RecoveryAction::IncreaseCpuLimit(*limit * 2))
            }
            VmError::MemoryLimitExceeded {
                context,
                limit,
                requested,
            } => Some(super::RecoveryAction::IncreaseMemoryLimit(*limit * 2)),
            VmError::CapabilityError { capability, .. } => {
                Some(super::RecoveryAction::RequestCapability(capability.clone()))
            }
            VmError::StackOverflow { max_depth, .. } => Some(
                super::RecoveryAction::IncreaseCpuLimit(*max_depth as u64 * 2),
            ),
            VmError::GcDisabled => Some(super::RecoveryAction::ContinueWithDefaults),
            VmError::HeapExhausted => {
                Some(super::RecoveryAction::IncreaseMemoryLimit(1024 * 1024)) // Increase by 1MB
            }
            VmError::DebuggerError { .. } => Some(super::RecoveryAction::ContinueWithDefaults),
            _ => None,
        }
    }
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.detailed_message())
    }
}

impl std::error::Error for VmError {}

/// Conversion trait to convert simple errors to detailed errors
impl From<SimpleVmError> for VmError {
    fn from(simple_error: SimpleVmError) -> Self {
        // Create a minimal error context for conversion
        let context = ErrorContext {
            instruction_pointer: 0,
            current_instruction: None,
            stack_state: Vec::new(),
            call_stack_depth: 0,
            steps_remaining: 0,
            actor_id: 0,
            memory_usage: 0,
            stack_trace: Vec::new(),
            execution_history: Vec::new(),
            timestamp: 0,
        };

        match simple_error {
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
