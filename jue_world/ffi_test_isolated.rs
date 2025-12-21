use crate::error::SourceLocation;
use crate::ffi::{create_standard_ffi_registry, FfiCallGenerator, FfiFunction, FfiRegistry};
use physics_world::types::{Capability, HostFunction, OpCode, Value};

#[test]
fn test_ffi_function_creation_isolated() {
    let location = SourceLocation::default();

    let ffi_func = FfiFunction {
        name: "test-ffi".to_string(),
        host_function: HostFunction::ReadSensor,
        required_capability: Capability::IoReadSensor,
        parameter_types: vec!["Int".to_string()],
        return_type: "Bool".to_string(),
        documentation: "Test FFI function".to_string(),
        location,
    };

    assert_eq!(ffi_func.name, "test-ffi");
    assert_eq!(ffi_func.host_function, HostFunction::ReadSensor);
    assert_eq!(ffi_func.required_capability, Capability::IoReadSensor);
    assert_eq!(ffi_func.parameter_types.len(), 1);
    assert_eq!(ffi_func.return_type, "Bool");
}

#[test]
fn test_ffi_registry_isolated() {
    let mut registry = FfiRegistry::new();

    assert_eq!(registry.get_functions().len(), 0);

    let ffi_func = FfiFunction {
        name: "test-ffi".to_string(),
        host_function: HostFunction::ReadSensor,
        required_capability: Capability::IoReadSensor,
        parameter_types: vec![],
        return_type: "Int".to_string(),
        documentation: "Test".to_string(),
        location: SourceLocation::default(),
    };

    registry.register_function(ffi_func.clone());

    assert_eq!(registry.get_functions().len(), 1);
    assert_eq!(registry.find_function("test-ffi"), Some(&ffi_func));
    assert_eq!(registry.find_function("nonexistent"), None);
}

#[test]
fn test_ffi_call_generator_isolated() {
    let generator = FfiCallGenerator::new();

    assert_eq!(generator.registry.get_functions().len(), 0);

    // Test with empty registry - should fail
    let result = generator.generate_ffi_call("nonexistent", vec![]);
    assert!(result.is_err());
}

#[test]
fn test_ffi_call_generation_isolated() {
    let mut registry = FfiRegistry::new();

    // Register a test function
    registry.register_function(FfiFunction {
        name: "test-call".to_string(),
        host_function: HostFunction::ReadSensor,
        required_capability: Capability::IoReadSensor,
        parameter_types: vec![],
        return_type: "Int".to_string(),
        documentation: "Test".to_string(),
        location: SourceLocation::default(),
    });

    let generator = FfiCallGenerator {
        registry,
        location: SourceLocation::default(),
    };

    // Generate FFI call
    let result = generator.generate_ffi_call("test-call", vec![]);

    assert!(result.is_ok());
    let bytecode = result.unwrap();

    // Should have the HostCall opcode
    assert_eq!(bytecode.len(), 1);
    match &bytecode[0] {
        OpCode::HostCall { func_id, args, .. } => {
            assert_eq!(*func_id, HostFunction::ReadSensor as u16);
            assert_eq!(*args, 0);
        }
        _ => panic!("Expected HostCall opcode"),
    }
}

#[test]
fn test_ffi_call_with_arguments_isolated() {
    let mut registry = FfiRegistry::new();

    // Register a function that takes arguments
    registry.register_function(FfiFunction {
        name: "test-with-args".to_string(),
        host_function: HostFunction::WriteActuator,
        required_capability: Capability::IoWriteActuator,
        parameter_types: vec!["Float".to_string()],
        return_type: "Bool".to_string(),
        documentation: "Test".to_string(),
        location: SourceLocation::default(),
    });

    let generator = FfiCallGenerator {
        registry,
        location: SourceLocation::default(),
    };

    // Generate FFI call with arguments
    let result = generator.generate_ffi_call("test-with-args", vec![Value::Int(314)]);

    assert!(result.is_ok());
    let bytecode = result.unwrap();

    // Should have argument push followed by HostCall
    assert_eq!(bytecode.len(), 2);
    match &bytecode[0] {
        OpCode::Int(314) => assert!(true), // Changed from Float to Int
        _ => panic!("Expected argument push"),
    }
    match &bytecode[1] {
        OpCode::HostCall { func_id, args, .. } => {
            assert_eq!(*func_id, HostFunction::WriteActuator as u16);
            assert_eq!(*args, 1);
        }
        _ => panic!("Expected HostCall opcode"),
    }
}
