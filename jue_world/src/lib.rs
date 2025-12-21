#!/ Jue-World V2.0 - The Capability-Aware Dual-Interpretation Language
//!
//! Jue-World is the capability-aware compiler bridge that transforms Dan-World's
//! cognitive operations into either Core-World proofs or Physics-World bytecode,
//! depending on the trust tier and capability requirements.
//#![allow(warnings)]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

/// Abstract Syntax Tree definitions for Jue language
pub mod ast;

/// Capability-mediated Foreign Function Interface
pub mod capability_ffi;

/// Main compilation pipeline and trust-tier based compilation
pub mod compiler;

/// Compile-time execution with restricted capabilities
pub mod comptime;

/// Error handling and reporting
pub mod error;

/// Expression parsing from tokens
pub mod expression_parser;

/// Foreign Function Interface with capability mediation
pub mod ffi;

/// Integration with Core-World and Physics-World
pub mod integration;

/// Macro system with hygienic expansion
pub mod macro_system;

/// Parser for Jue language source code
pub mod parser;

/// Resource limit enforcement and monitoring
pub mod resource_limits;

/// Sandbox wrapper for experimental tier execution
pub mod sandbox;

/// Sandboxed compile-time execution with strict capability enforcement
pub mod sandboxed_comptime;

/// Structured error handling with detailed context
pub mod structured_error;

/// Test timeout and resource management utilities
pub mod test_timeout;

/// Tokenization of Jue source code
pub mod token;

/// Trust tier system and capability management
pub mod trust_tier;

/// Type system and type checking
pub mod type_system;

pub use crate::capability_ffi::{
    CapabilityMediatedFfiBuilder, CapabilityMediatedFfiCall, CapabilityMediatedFfiGenerator,
    CapabilityMediatedFfiValidator,
};
pub use crate::compiler::{
    compile, CapabilityCheck, CheckType, CompilationResult, EmpiricalResult,
};
pub use crate::error::{CapabilityViolation, CompilationError};
pub use crate::macro_system::MacroDefinition;
pub use crate::resource_limits::{
    ResourceLimitBuilder, ResourceLimitEnforcer, ResourceLimits, ResourceMonitor,
    ResourceUsageReport,
};
pub use crate::sandbox::{Sandbox, SandboxBuilder, SandboxConfig};
pub use crate::sandboxed_comptime::{
    execute_sandboxed_comptime, SandboxedComptimeBuilder, SandboxedComptimeEnv,
    SandboxedComptimeExecutor, SandboxedComptimeResult,
};
pub use crate::structured_error::{
    ErrorContext, ErrorFormat, ErrorReporter, ErrorSeverity, ErrorType, StructuredError,
    StructuredErrorBuilder, StructuredErrorHandler,
};
pub use crate::test_timeout::{run_test_with_guard, TestError, TestGuard};
pub use crate::trust_tier::TrustTier;
pub use crate::type_system::TypeSignature;
pub use physics_world::types::{Capability, HostFunction};
