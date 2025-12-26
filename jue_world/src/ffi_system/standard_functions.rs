use crate::error::SourceLocation;
use physics_world::types::{Capability, HostFunction};

/// Standard FFI functions
pub fn create_standard_ffi_registry() -> super::global_ffi_registry::FfiRegistry {
    let mut registry = super::global_ffi_registry::FfiRegistry::new();

    // Register standard FFI functions
    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "read-sensor".to_string(),
        host_function: HostFunction::ReadSensor,
        required_capability: Some(Capability::IoReadSensor),
        parameter_types: vec![],
        return_type: "Float".to_string(),
        documentation: "Read from virtual sensor".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "write-actuator".to_string(),
        host_function: HostFunction::WriteActuator,
        required_capability: Some(Capability::IoWriteActuator),
        parameter_types: vec!["Float".to_string()],
        return_type: "Bool".to_string(),
        documentation: "Write to virtual actuator".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "get-wall-clock".to_string(),
        host_function: HostFunction::GetWallClockNs,
        required_capability: Some(Capability::SysClock),
        parameter_types: vec![],
        return_type: "Int".to_string(),
        documentation: "Get current wall clock time in nanoseconds".to_string(),
        location: SourceLocation::default(),
    });

    // ========== INTEGER ARITHMETIC (no capability required) ==========

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "add".to_string(),
        host_function: HostFunction::IntAdd,
        required_capability: None,
        parameter_types: vec!["Int".to_string(), "Int".to_string()],
        return_type: "Int".to_string(),
        documentation: "Add two integers".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "sub".to_string(),
        host_function: HostFunction::IntSub,
        required_capability: None,
        parameter_types: vec!["Int".to_string(), "Int".to_string()],
        return_type: "Int".to_string(),
        documentation: "Subtract second integer from first".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "mul".to_string(),
        host_function: HostFunction::IntMul,
        required_capability: None,
        parameter_types: vec!["Int".to_string(), "Int".to_string()],
        return_type: "Int".to_string(),
        documentation: "Multiply two integers".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "div".to_string(),
        host_function: HostFunction::IntDiv,
        required_capability: None,
        parameter_types: vec!["Int".to_string(), "Int".to_string()],
        return_type: "Int".to_string(),
        documentation: "Divide first integer by second (integer division)".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "mod".to_string(),
        host_function: HostFunction::IntMod,
        required_capability: None,
        parameter_types: vec!["Int".to_string(), "Int".to_string()],
        return_type: "Int".to_string(),
        documentation: "Modulo: remainder of first integer divided by second".to_string(),
        location: SourceLocation::default(),
    });

    // ========== FLOAT ARITHMETIC (no capability required) ==========

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "fadd".to_string(),
        host_function: HostFunction::FloatAdd,
        required_capability: None,
        parameter_types: vec!["Float".to_string(), "Float".to_string()],
        return_type: "Float".to_string(),
        documentation: "Add two floats".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "fsub".to_string(),
        host_function: HostFunction::FloatSub,
        required_capability: None,
        parameter_types: vec!["Float".to_string(), "Float".to_string()],
        return_type: "Float".to_string(),
        documentation: "Subtract second float from first".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "fmul".to_string(),
        host_function: HostFunction::FloatMul,
        required_capability: None,
        parameter_types: vec!["Float".to_string(), "Float".to_string()],
        return_type: "Float".to_string(),
        documentation: "Multiply two floats".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "fdiv".to_string(),
        host_function: HostFunction::FloatDiv,
        required_capability: None,
        parameter_types: vec!["Float".to_string(), "Float".to_string()],
        return_type: "Float".to_string(),
        documentation: "Divide first float by second".to_string(),
        location: SourceLocation::default(),
    });

    // ========== TYPE CONVERSIONS (no capability required) ==========

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "int-to-float".to_string(),
        host_function: HostFunction::IntToFloat,
        required_capability: None,
        parameter_types: vec!["Int".to_string()],
        return_type: "Float".to_string(),
        documentation: "Convert integer to float".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "float-to-int".to_string(),
        host_function: HostFunction::FloatToInt,
        required_capability: None,
        parameter_types: vec!["Float".to_string()],
        return_type: "Int".to_string(),
        documentation: "Convert float to integer (truncates decimal)".to_string(),
        location: SourceLocation::default(),
    });

    // ========== INTEGER COMPARISONS (no capability required) ==========

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "eq".to_string(),
        host_function: HostFunction::IntEq,
        required_capability: None,
        parameter_types: vec!["Int".to_string(), "Int".to_string()],
        return_type: "Int".to_string(),
        documentation: "Check if two integers are equal (returns 1 if true, 0 if false)"
            .to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "lt".to_string(),
        host_function: HostFunction::IntLt,
        required_capability: None,
        parameter_types: vec!["Int".to_string(), "Int".to_string()],
        return_type: "Int".to_string(),
        documentation: "Check if first integer is less than second (returns 1 if true, 0 if false)"
            .to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "gt".to_string(),
        host_function: HostFunction::IntGt,
        required_capability: None,
        parameter_types: vec!["Int".to_string(), "Int".to_string()],
        return_type: "Int".to_string(),
        documentation:
            "Check if first integer is greater than second (returns 1 if true, 0 if false)"
                .to_string(),
        location: SourceLocation::default(),
    });

    // ========== FLOAT COMPARISONS (no capability required) ==========

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "feq".to_string(),
        host_function: HostFunction::FloatEq,
        required_capability: None,
        parameter_types: vec!["Float".to_string(), "Float".to_string()],
        return_type: "Int".to_string(),
        documentation: "Check if two floats are equal (returns 1 if true, 0 if false)".to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "flt".to_string(),
        host_function: HostFunction::FloatLt,
        required_capability: None,
        parameter_types: vec!["Float".to_string(), "Float".to_string()],
        return_type: "Int".to_string(),
        documentation: "Check if first float is less than second (returns 1 if true, 0 if false)"
            .to_string(),
        location: SourceLocation::default(),
    });

    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "fgt".to_string(),
        host_function: HostFunction::FloatGt,
        required_capability: None,
        parameter_types: vec!["Float".to_string(), "Float".to_string()],
        return_type: "Int".to_string(),
        documentation:
            "Check if first float is greater than second (returns 1 if true, 0 if false)"
                .to_string(),
        location: SourceLocation::default(),
    });

    registry
}
