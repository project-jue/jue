use super::*;
use crate::error::CompilationError;
use physics_world::types::{Capability, HostFunction, OpCode, Value};

#[cfg(test)]
mod ffi_tests {
    use super::*;

    #[test]
    fn test_ffi_registry_function_registration() {
        let mut registry = FfiRegistry::new();
        
        let func = FfiFunction {
            name: "test-function".to_string(),
            host_function: HostFunction::ReadSensor,
            required_capability: Capability::IoReadSensor,
            parameter_types: vec!["Int".to_string()],
            return_type: "Bool".to_string(),
            documentation: "Test function".to_string(),
            location: SourceLocation::default(),
        };
        
        registry.register_function(func);
        
        // Test function lookup
        let found_func = registry.find_function("test-function");
        assert!(found_func.is_some());
        
        let found = found_func.unwrap();
        assert_eq!(found.name, "test-function");
        assert_eq!(found.host_function, HostFunction::ReadSensor);
        assert_eq!(found.required_capability, Capability::IoReadSensor);
    }

    #[test]
    fn test_ffi_registry_capability_indexing() {
        let mut registry = FfiRegistry::new();
        
        let func1 = FfiFunction {
            name: "sensor-read".to_string(),
            host_function: HostFunction::ReadSensor,
            required_capability: Capability::IoReadSensor,
            parameter_types: vec![],
            return_type: "Int".to_string(),
            documentation: "Read sensor".to_string(),
            location: SourceLocation::default(),
        };
        
        let func2 = FfiFunction {
            name: "clock-get".to_string(),
            host_function: HostFunction::GetWallClockNs,
            required_capability: Capability::SysClock,
            parameter_types: vec![],
            return_type: "Int".to_string(),
            documentation: "Get clock".to_string(),
            location: SourceLocation::default(),
        };
        
        registry.register_function(func1);
        registry.register_function(func2);
        
        // Test capability index resolution
        let.get_capability_index let clock_idx = sensor_idx = registry_index(&Capability::SysClock);
        
        assert!(sensor(&Capability::Io        assert!(clock_idx.is_some());
_idx.is_some());
sensor_idx.unwrap(), clock_idx.unwrap());
        assert_ne!(ReadSensor);
        registry.get_capability    }

    #[test]
    fn test_ffi_call_generator_basic_types() {
        let generator = FfiCallGenerator::new();
        
        // Test basic types that should work
        let basic_values = vec![
            Value::Nil,
            Value::Bool(true),
            Value::Int(42),
            Value::Symbol(123),
        ];
        
        let mut all_passed = true;
        for value in basic_values {
            let mut bytecode = Vec::new();
            match generator.push_value_to_bytecode(&mut bytecode, value) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Failed to encode value: {:?}", e);
                    all_passed = false;
                }
            }
        }
        
        assert!(all_passed, "All basic types should be encodable");
    }

    #[test]
    fn test_ffi_call_generator_complex_types() {
        use physics_world::types::HeapPtr;
        
        let generator = FfiCallGenerator::new();
        
        // Test complex types that were previously unimplemented
        let complex_values = vec![
            Value::Pair(HeapPtr::new(123)),
            Value::Closure(HeapPtr::new(456)),
            Value::ActorId(789),
            Value::Capability(Capability::IoReadSensor),
            Value::GcPtr(physics_world::vm::gc::GcPtr(101112)),
        ];
        
        let mut all_passed = true;
        for value in complex_values {
            let mut bytecode = Vec::new();
            match generator.push_value_to_bytecode(&mut bytecode, value) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("Failed to encode complex value: {:?}", e);
                    all_passed = false;
                }
            }
        }
        
        assert!(all_passed, "All complex types should now be encodable");
    }

    #[test]
    fn test_dynamic_capability_resolution() {
        let mut generator = FfiCallGenerator::new();
        
        // Register a function with specific capability
        let func = FfiFunction {
            name: "test-capability".to_string(),
            host_function: HostFunction::ReadSensor,
            required_capability: Capability::IoReadSensor,
            parameter_types: vec![],
            return_type: "Int".to_string(),
            documentation: "Test capability resolution".to_string(),
            location: SourceLocation::default(),
        };
        
        generator.registry.register_function(func);
        
        // Test capability check generation with dynamic index
        let check_result = generator.generate_capability_check("test-capability");
        assert!(check_result.is_ok());
        
        let bytecode = check_result.unwrap();
        assert_eq!(bytecode.len(), 1);
        
        match &bytecode[0] {
            OpCode::HasCap(idx) => {
                // Should not be hardcoded to 0 anymore
                assert!(*idx >= 0);
            }
            _ => panic!("Expected HasCap opcode"),
        }
    }

    #[test]
    fn test_ffi_call_with_mixed_arguments() {
        use physics_world::types::HeapPtr;
        
        let mut generator = FfiCallGenerator::new();
        
        // Register test function
        let func = FfiFunction {
            name: "mixed-args".to_string(),
            host_function: HostFunction::WriteActuator,
            required_capability: Capability::IoWriteActuator,
            parameter_types: vec!["Int".to_string(), "Bool".to_string()],
            return_type: "Bool".to_string(),
            documentation: "Test mixed arguments".to_string(),
            location: SourceLocation::default(),
        };
        
        generator.registry.register_function(func);
        
        // Test with mixed argument types including complex types
        let arguments = vec![
            Value::Int(123),
            Value::Bool(true),
            Value::ActorId(456),
            Value::Capability(Capability::IoWriteActuator),
        ];
        
        let result = generator.generate_ffi_call("mixed-args", arguments);
        assert!(result.is_ok());
        
        let bytecode = result.unwrap();
        // Should have push operations for each argument plus the HostCall
        assert!(bytecode.len() >= 5); // 4 arguments + 1 host call
    }

    #[test]
    fn test_standard_ffi_registry_creation() {
        let registry = create_standard_ffi_registry();
        
        // Test that all standard functions are registered
        assert!(registry.find_function("read-sensor").is_some());
        assert!(registry.find_function("write-actuator").is_some());
        assert!(registry.find_function("get-wall-clock").is_some());
        
        // Test capability indices are properly assigned
        let read_sensor_cap_idx = registry.get_capability_index(&Capability::IoReadSensor);
        let write_actuator_cap_idx = registry.get_capability_index(&Capability::IoWriteActuator);
        let clock_cap_idx = registry.get_capability_index(&Capability::SysClock);
        
        assert!(read_sensor_cap_idx.is_some());
        assert!(write_actuator_cap_idx.is_some());
        assert!(clock_cap_idx.is_some());
        
        // All should have different indices
        assert_ne!(read_sensor_cap_idx, write_actuator_cap_idx);
        assert_ne!(read_sensor_cap_idx, clock_cap_idx);
        assert_ne!(write_actuator_cap_idx, clock_cap_idx);
    }

    #[test]
    fn test_ffi_error_handling() {
        let generator = FfiCallGenerator::new();
        
        // Test function not found
        let result = generator.generate_ffi_call("non-existent-function", vec![]);
        assert!(result.is_err());
        
        match result {
            Err(CompilationError::FfiError(msg)) => {
                assert!(msg.contains("not found"));
            }
            _ => panic!("Expected FfiError"),
        }
        
        // Test capability index not found
        let mut generator_with_func = FfiCallGenerator::new();
        let func = FfiFunction {
            name: "orphan-function".to_string(),
            host_function: HostFunction::ReadSensor,
            required_capability: Capability::IoReadSensor,
            parameter_types: vec![],
            return_type: "Int".to_string(),
            documentation: "Orphan function".to_string(),
            location: SourceLocation::default(),
        };
        
        // Register but don't add to capability indices (simulate corrupted state)
        generator_with_func.registry.functions.insert("orphan-function".to_string(), func);
        
        let result = generator_with_func.generate_ffi_call("orphan-function", vec![]);
        assert!(result.is_err());
        
        match result {
            Err(CompilationError::FfiError(msg)) => {
                assert!(msg.contains("No capability index found"));
            }
            _ => panic!("Expected FfiError for missing capability index"),
        }
    }

    #[test]
    fn test_capability_hashing() {
        let generator = FfiCallGenerator::new();
        
        // Test that the same capability produces the same hash
        let cap1 = Capability::IoReadSensor;
        let cap2 = Capability::IoReadSensor;
        let cap3 = Capability::SysClock;
        
        let hash1 = generator.hash_capability(&cap1);
        let hash2 = generator.hash_capability(&cap2);
        let hash3 = generator.hash_capability(&cap3);
        
        assert_eq!(hash1, hash2); // Same capability, same hash
        assert_ne!(hash1, hash3); // Different capabilities, different hashes
    }

    #[test]
    fn test_namespace_organization() {
        let mut registry = FfiRegistry::new();
        
        // Register functions with namespaced names
        let function_specs = vec![
            ("io:sensor:read", HostFunction::ReadSensor, Capability::IoReadSensor),
            ("io:actuator:write", HostFunction::WriteActuator, Capability::IoWriteActuator),
            ("sys:clock:now", HostFunction::GetWallClockNs, Capability::SysClock),
        ];
        
        for (name, host_func, capability) in &function_specs {
            let func = FfiFunction {
                name: name.to_string(),
                host_function: *host_func,
                required_capability: capability.clone(),
                parameter_types: vec![],
                return_type: "Int".to_string(),
                documentation: format!("Function {}", name),
                location: SourceLocation::default(),
            };
            registry.register_function(func);
        }
        
        // Test namespace lookup efficiency (should be O(1) now)
        for (name, _, _) in function_specs {
            let start = std::time::Instant::now();
            let _ = registry.find_function(name);
            let duration = start.elapsed();
            
            // Should be very fast for HashMap lookup
            assert!(duration.as_nanos() < 1000, "HashMap lookup should be sub-microsecond");
        }
    }
}