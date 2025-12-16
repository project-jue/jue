use crate::error::{CompilationError, SourceLocation};
use physics_world::types::{Capability, HostFunction};
/// Foreign Function Interface (FFI) with capability mediation
use serde::{Deserialize, Serialize};

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

/// FFI registry
#[derive(Debug, Clone, Default)]
pub struct FfiRegistry {
    /// Registered FFI functions
    pub functions: Vec<FfiFunction>,
}

impl FfiRegistry {
    /// Create a new FFI registry
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a new FFI function
    pub fn register_function(&mut self, func: FfiFunction) {
        self.functions.push(func);
    }

    /// Find FFI function by name
    pub fn find_function(&self, name: &str) -> Option<&FfiFunction> {
        self.functions.iter().find(|f| f.name == name)
    }

    /// Get all FFI functions
    pub fn get_functions(&self) -> &[FfiFunction] {
        &self.functions
    }
}

/// FFI call generator
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

    /// Generate bytecode for an FFI call
    pub fn generate_ffi_call(
        &self,
        name: &str,
        arguments: Vec<physics_world::Value>,
    ) -> Result<Vec<physics_world::OpCode>, CompilationError> {
        // Find the FFI function
        let _func = self.registry.find_function(name).ok_or_else(|| {
            CompilationError::FfiError(format!("FFI function {} not found", name))
        })?;

        // Generate the HostCall opcode
        let opcode = physics_world::OpCode::HostCall {
            cap_idx: 0, // TODO: Get actual capability index
            func_id: _func.host_function as u16,
            args: arguments.len() as u8,
        };

        // Push arguments to stack (in reverse order)
        let mut bytecode = Vec::new();
        for arg in arguments.into_iter().rev() {
            match arg {
                physics_world::Value::Nil => bytecode.push(physics_world::OpCode::Nil),
                physics_world::Value::Bool(b) => bytecode.push(physics_world::OpCode::Bool(b)),
                physics_world::Value::Int(i) => bytecode.push(physics_world::OpCode::Int(i)),
                physics_world::Value::Symbol(s) => bytecode.push(physics_world::OpCode::Symbol(s)),
                physics_world::Value::Pair(_ptr) => {
                    // TODO: Handle pair values
                    bytecode.push(physics_world::OpCode::Nil);
                }
                physics_world::Value::Closure(_ptr) => {
                    // TODO: Handle closure values
                    bytecode.push(physics_world::OpCode::Nil);
                }
                physics_world::Value::ActorId(_id) => {
                    // TODO: Handle actor ID values
                    bytecode.push(physics_world::OpCode::Nil);
                }
                physics_world::Value::Capability(_cap) => {
                    // TODO: Handle capability values
                    bytecode.push(physics_world::OpCode::Nil);
                }
            }
        }

        // Add the HostCall opcode
        bytecode.push(opcode);

        Ok(bytecode)
    }

    /// Generate capability check for FFI call
    pub fn generate_capability_check(
        &self,
        name: &str,
    ) -> Result<Vec<physics_world::OpCode>, CompilationError> {
        // Find the FFI function
        let _func = self.registry.find_function(name).ok_or_else(|| {
            CompilationError::FfiError(format!("FFI function {} not found", name))
        })?;

        // Generate HasCap opcode to check if capability is available
        let opcode = physics_world::OpCode::HasCap(0); // TODO: Get actual capability index

        Ok(vec![opcode])
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
