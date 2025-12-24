use crate::error::SourceLocation;
use physics_world::types::{Capability, HostFunction};

/// Standard FFI functions
pub fn create_standard_ffi_registry() -> super::global_ffi_registry::FfiRegistry {
    let mut registry = super::global_ffi_registry::FfiRegistry::new();

    // Register standard FFI functions
    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "read-sensor".to_string(),
        host_function: HostFunction::ReadSensor,
        required_capability: Capability::IoReadSensor,
        parameter_types: vec![],
        return_type: "Float".to_string(),
        documentation: "Read from virtual sensor".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "write-actuator".to_string(),
        host_function: HostFunction::WriteActuator,
        required_capability: Capability::IoWriteActuator,
        parameter_types: vec!["Float".to_string()],
        return_type: "Bool".to_string(),
        documentation: "Write to virtual actuator".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
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
