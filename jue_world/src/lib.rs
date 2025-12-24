#!/ Jue-World V2.0 - The Capability-Aware Dual-Interpretation Language
//!
//! Jue-World is the capability-aware compiler bridge that transforms Dan-World's
//! cognitive operations into either Core-World proofs or Physics-World bytecode,
//! depending on the trust tier and capability requirements.
//#![allow(warnings)]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

/// Shared infrastructure including AST definitions and error handling
pub mod shared;

/// Token definitions
pub mod token;

/// FFI module
pub mod ffi;

/// Core compilation layer for Core-World integration
pub mod core_compilation;

/// Physics-World integration layer
/// Physics-World integration layer
pub mod physics_integration;

/// Foreign Function Interface system
pub mod ffi_system;

/// Macro system with hygienic expansion
pub mod macro_system;

/// Parsing infrastructure
pub mod parsing;

/// Integration with Core-World and Physics-World
pub mod integration;

/// Compiler infrastructure
pub mod compiler;

/// Compile-time execution with restricted capabilities
pub mod comptime;

/// Sandbox wrapper for experimental tier execution
pub mod sandbox;

/// Sandboxed compile-time execution with strict capability enforcement
pub mod sandboxed_comptime;

/// Test timeout and resource management utilities
pub mod test_timeout;

pub use crate::shared::ast;
pub use crate::shared::error;
pub use crate::shared::resource_limits;
pub use crate::shared::structured_error;
pub use crate::shared::trust_tier;
pub use crate::shared::type_system;

pub use crate::core_compilation::capability_analyzer;
pub use crate::core_compilation::core_compiler;
pub use crate::core_compilation::escape_analysis;

pub use crate::physics_integration::bytecode_generator;
pub use crate::physics_integration::physics_compiler;
pub use crate::physics_integration::runtime_checks;
pub use crate::physics_integration::sandbox_wrapper;

pub use crate::ffi_system::capability_mediator;
pub use crate::ffi_system::ffi_call_generator;
pub use crate::ffi_system::global_ffi_registry;
pub use crate::ffi_system::standard_functions;

pub use crate::macro_system::macro_expander;
pub use crate::macro_system::macro_ffi;

pub use crate::parsing::expression_parser;
pub use crate::parsing::parser;
pub use crate::parsing::tokenizer;

pub use crate::comptime::{
    ComptimeEnv, ComptimeExecutor, ComptimeResult,
};

// Note: recursion_analysis.rs contains test modules, not exportable types
// pub use crate::recursion_analysis::{...};

pub use crate::sandbox::{Sandbox, SandboxBuilder, SandboxConfig};

pub use crate::sandboxed_comptime::{
    execute_sandboxed_comptime, SandboxedComptimeBuilder, SandboxedComptimeEnv,
    SandboxedComptimeExecutor, SandboxedComptimeResult,
};

pub use crate::test_timeout::{run_test_with_guard, TestError, TestGuard};

pub use physics_world::types::{Capability, HostFunction};
