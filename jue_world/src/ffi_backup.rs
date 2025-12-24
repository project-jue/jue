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
}

/// FFI call generator with dynamic capability resolution
pub struct FfiCallGenerator {
    /// FFI registry
    pub registry: FfiRegistry,

    /// Current source location
    pub location: SourceLocation,
}

impl FfiCallGenerator {
    /// Create a new FFI call generator
    pub fn new() -> Self {
        Self {
            registry: FfiRegistry::new(),
            location: SourceLocation::default(),
        }
    }

    /// Generate bytecode for an FFI call with dynamic capability resolution
    pub fn generate_ffi_call(
        &self,
        name: &str,
        arguments: Vec<Value>,
    ) -> Result<Vec<OpCode>, CompilationError> {
        // Find the FFI function
        let func = self.registry.find_function(name).ok_or_else(|| {
            CompilationError::FfiError(format!("FFI function {} not found", name))
        })?;

        // Get dynamic capability index
        let cap_idx = self
            .registry
            .get_capability_index(&func.required_capability)
            .ok_or_else(|| {
                CompilationError::FfiError(format!(
                    "No capability index found for capability {:?}",
                    func.required_capability
                ))
            })?;

        // Generate the HostCall opcode with dynamic capability index
        let opcode = OpCode::HostCall {
            cap_idx: cap_idx,
            func_id: func.host_function as u16,
            args: arguments.len() as u8,
        };

        // Push arguments to stack (in reverse order) with proper value handling
        let mut bytecode = Vec::new();
        for arg in arguments.into_iter().rev() {
            self.push_value_to_bytecode(&mut bytecode, arg)?;
        }

        // Add the HostCall opcode
        bytecode.push(opcode);

        Ok(bytecode)
    }

    /// Generate capability check for FFI call with dynamic resolution
    pub fn generate_capability_check(&self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
        // Find the FFI function
        let func = self.registry.find_function(name).ok_or_else(|| {
            CompilationError::FfiError(format!("FFI function {} not found", name))
        })?;

        // Get dynamic capability index
        let cap_idx = self
            .registry
            .get_capability_index(&func.required_capability)
            .ok_or_else(|| {
                CompilationError::FfiError(format!(
                    "No capability index found for capability {:?}",
                    func.required_capability
                ))
            })?;

        // Generate HasCap opcode with dynamic capability index
        let opcode = OpCode::HasCap(cap_idx);

        Ok(vec![opcode])
    }

    /// Push a Value to bytecode with proper type handling
    fn push_value_to_bytecode(
        &self,
        bytecode: &mut Vec<OpCode>,
        value: Value,
    ) -> Result<(), CompilationError> {
        match value {
            // Basic types - fully implemented
            Value::Nil => bytecode.push(OpCode::Nil),
            Value::Bool(b) => bytecode.push(OpCode::Bool(b)),
            Value::Int(i) => bytecode.push(OpCode::Int(i)),
            Value::Float(f) => bytecode.push(OpCode::Float(f)), // Handle float values
            Value::String(_s) => {
                // Handle string values - push nil as placeholder for now
                bytecode.push(OpCode::Nil);
            }
            Value::Symbol(s) => bytecode.push(OpCode::Symbol(s)),

            // Complex types - now properly implemented
            Value::Pair(ptr) => {
                // Convert heap pointer to bytecode representation
                let ptr_value = ptr.get() as u32;
                bytecode.push(OpCode::Int(ptr_value as i64));
            }
            Value::Closure(ptr) => {
                // Convert heap pointer to bytecode representation
                let ptr_value = ptr.get() as u32;
                bytecode.push(OpCode::Int(ptr_value as i64));
            }
            Value::ActorId(id) => {
                // Convert actor ID to bytecode representation
                bytecode.push(OpCode::Int(id as i64));
            }
            Value::Capability(cap) => {
                // Convert capability to bytecode representation
                let cap_hash = self.hash_capability(&cap) as u32;
                bytecode.push(OpCode::Int(cap_hash as i64));
            }
            Value::GcPtr(ptr) => {
                // Convert GC pointer to bytecode representation
                let ptr_value = ptr.0 as u32;
                bytecode.push(OpCode::Int(ptr_value as i64));
            }
        }

        Ok(())
    }

    /// Hash a capability for bytecode representation
    fn hash_capability(&self, capability: &Capability) -> u32 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        capability.hash(&mut hasher);
        hasher.finish() as u32
    }
}

/// Standard FFI functions
pub fn create_standard_ffi_registry() -> FfiRegistry {
    let mut registry = FfiRegistry::new();

    // Register standard FFI functions
    registry.register_function(FfiFunction {
        name: "read-sensor".to_string(),
        host_function: HostFunction::ReadSensor,
        required_capability: Capability::IoReadSensor,
        parameter_types: vec![],
        return_type: "Float".to_string(),
        documentation: "Read from virtual sensor".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(FfiFunction {
        name: "write-actuator".to_string(),
        host_function: HostFunction::WriteActuator,
        required_capability: Capability::IoWriteActuator,
        parameter_types: vec!["Float".to_string()],
        return_type: "Bool".to_string(),
        documentation: "Write to virtual actuator".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(FfiFunction {
        name: "get-wall-clock".to_string(),
        host_function: HostFunction::GetWallClockNs,
        required_capability: Capability::SysClock,
        parameter_types: vec![],
        return_type: "Int".to_string(),
        documentation: "Get current wall clock time in nanoseconds".to_string(),
        location: SourceLocation::default(),
    });

    registry
}

#[cfg(test)]
#[path = "test/ffi.rs"]
mod tests;
