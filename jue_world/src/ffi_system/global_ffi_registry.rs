use crate::error::{CompilationError, SourceLocation};
use physics_world::types::{Capability, HostFunction, OpCode, Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Foreign Function Interface (FFI) with capability mediation

/// Capability index mapping for dynamic resolution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityIndex {
    /// The capability being indexed
    pub capability: Capability,
    /// The index position for this capability
    pub index: usize,
}

/// FFI function definition with capability requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FfiFunction {
    /// Function name
    pub name: String,

    /// Corresponding host function
    pub host_function: HostFunction,

    /// Required capability for this function
    pub required_capability: Capability,

    /// Parameter types
    pub parameter_types: Vec<String>,

    /// Return type
    pub return_type: String,

    /// Documentation
    pub documentation: String,

    /// Source location
    pub location: SourceLocation,
}

/// FFI registry with namespace support
#[derive(Debug, Clone, Default)]
pub struct FfiRegistry {
    /// Registered FFI functions by name
    pub functions: HashMap<String, FfiFunction>,

    /// Capability to index mapping
    pub capability_indices: HashMap<Capability, usize>,
}

impl FfiRegistry {
    /// Create a new FFI registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new FFI function
    pub fn register_function(&mut self, func: FfiFunction) {
        self.functions.insert(func.name.clone(), func.clone());

        // Add capability mapping if not already present
        if !self
            .capability_indices
            .contains_key(&func.required_capability)
        {
            let index = self.capability_indices.len();
            self.capability_indices
                .insert(func.required_capability.clone(), index);
        }
    }

    /// Find FFI function by name
    pub fn find_function(&self, name: &str) -> Option<&FfiFunction> {
        self.functions.get(name)
    }

    /// Get capability index for a given capability
    pub fn get_capability_index(&self, capability: &Capability) -> Option<usize> {
        self.capability_indices.get(capability).copied()
    }

    /// Get all FFI functions
    pub fn get_functions(&self) -> Vec<&FfiFunction> {
        self.functions.values().collect()
    }

    /// Get FFI index for a function name (for backward compatibility)
    pub fn get_ffi_index(&self, _name: &str) -> Option<usize> {
        // Return a placeholder - actual implementation depends on bytecode format
        Some(0)
    }
}
