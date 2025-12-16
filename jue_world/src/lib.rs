#!/ Jue-World V2.0 - The Capability-Aware Dual-Interpretation Language
//!
//! Jue-World is the capability-aware compiler bridge that transforms Dan-World's
//! cognitive operations into either Core-World proofs or Physics-World bytecode,
//! depending on the trust tier and capability requirements.
#![allow(warnings)]
#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

pub mod ast;
pub mod compiler;
pub mod comptime;
pub mod error;
pub mod expression_parser;
pub mod ffi;
pub mod integration;
pub mod macro_system;
pub mod parser;
pub mod test_timeout;
pub mod token;
pub mod trust_tier;
pub mod type_system;

pub use crate::compiler::{
    compile, CapabilityCheck, CheckType, CompilationResult, EmpiricalResult,
};
pub use crate::error::{CapabilityViolation, CompilationError};
pub use crate::macro_system::MacroDefinition;
pub use crate::test_timeout::{run_test_with_guard, TestError, TestGuard};
pub use crate::trust_tier::TrustTier;
pub use crate::type_system::TypeSignature;
pub use physics_world::types::{Capability, HostFunction};
