//! Jue-World compiler module
//!
//! This module contains the core compilation pipeline for Jue-World V2.0,
//! including bytecode generation, capability analysis, and Core-World integration.

/// Bytecode generation module
pub mod bytecode_generation;
/// Capability analysis module
pub mod capability_analysis;
/// Capability checking module
pub mod capability_checking;
/// Main compiler module
pub mod compiler;
/// Core expression compilation module
pub mod core_compilation;
/// Environment management module
pub mod environment;
/// Test modules
pub mod test;

pub use capability_checking::{CapabilityCheck, CheckType};
pub use compiler::*;
