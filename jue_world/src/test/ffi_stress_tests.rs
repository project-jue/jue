#[cfg(test)]
mod ffi_stress_tests {
    use crate::ast::{AstNode, Literal};
    use crate::compiler::compile;
    use crate::error::CompilationError;
    use crate::error::SourceLocation;
    use crate::trust_tier::TrustTier;

    #[test]
    fn test_complex_nested_ffi_calls() {
        // Test deeply nested FFI calls with different capabilities
        let source = r#"
            (let ((sensor1 (read-sensor 1))
                  (sensor2 (read-sensor 2))
                  (actuator1 (write-actuator 1 42))
                  (actuator2 (write-actuator 2 43)))
              (if (> sensor1 sensor2)
                  (network-send "sensor1-higher" sensor1)
                  (network-send "sensor2-higher" sensor2)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded with capability checks
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Check that capability checks were inserted
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                // Should have capability checks for IoReadSensor, IoWriteActuator, and IoNetwork
                assert!(
                    has_cap_checks >= 3,
                    "Expected at least 3 capability checks, found {}",
                    has_cap_checks
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_ffi_capability_interaction_stress() {
        // Test complex interactions between different capability requirements
        let source = r#"
            (require-capability "macro-hygienic")
            (require-capability "io-read-sensor")
            (require-capability "io-write-actuator")

            (defmacro sensor-reader (id)
              `(let ((val (read-sensor ,id)))
                 (if (> val 100)
                     (write-actuator ,id val)
                     val)))

            (sensor-reader 1)
            (sensor-reader 2)
            (sensor-reader 3)
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have multiple capability checks for different operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 6,
                    "Expected at least 6 capability checks, found {}",
                    has_cap_checks
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_ffi_with_complex_conditional_logic() {
        // Test FFI calls within complex conditional structures
        let source = r#"
            (let ((sensor-val (read-sensor 1)))
              (cond
                ((> sensor-val 100) (write-actuator 1 sensor-val))
                ((> sensor-val 50) (network-send "medium" sensor-val))
                (else (network-send "low" sensor-val))))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Verify conditional jumps and capability checks are present
                let has_conditional_jumps = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::JmpIfFalse(_)))
                    .count();

                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_conditional_jumps >= 2,
                    "Expected at least 2 conditional jumps"
                );
                assert!(has_cap_checks >= 3, "Expected at least 3 capability checks");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_ffi_with_lambda_and_closures() {
        // Test FFI calls within lambda functions and closures
        let source = r#"
            (let ((sensor-reader (lambda (id)
                                   (let ((val (read-sensor id)))
                                     (if (> val 100)
                                         (write-actuator id val)
                                         val)))))
              ((sensor-reader 1))
              ((sensor-reader 2)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have closure creation and capability checks
                let has_closure_ops = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::MakeClosure(_, _)))
                    .count();

                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_closure_ops >= 1,
                    "Expected at least 1 closure operation"
                );
                assert!(has_cap_checks >= 4, "Expected at least 4 capability checks");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_ffi_error_handling_in_nested_calls() {
        // Test error handling in nested FFI calls
        let source = r#"
            (let ((result1 (read-sensor 1))
                  (result2 (read-sensor 2)))
              (try
                (write-actuator 1 (+ result1 result2))
                (catch (e)
                  (network-send "error" e))))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have error handling constructs and capability checks
                let has_error_handling = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                    )
                });

                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(has_error_handling, "Expected error handling constructs");
                assert!(has_cap_checks >= 3, "Expected at least 3 capability checks");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_ffi_with_recursive_functions() {
        // Test FFI calls within recursive functions
        let source = r#"
            (let ((sum-sensors (lambda (id limit)
                                 (if (> id limit)
                                     0
                                     (+ (read-sensor id)
                                        (sum-sensors (+ id 1) limit))))))
              (sum-sensors 1 5))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have recursive calls and capability checks
                let has_recursive_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::Call(_)))
                    .count();

                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_recursive_calls >= 5,
                    "Expected multiple recursive calls"
                );
                assert!(
                    has_cap_checks >= 5,
                    "Expected capability checks for each sensor read"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_ffi_capability_conflict_resolution() {
        // Test resolution of capability conflicts in complex scenarios
        let source = r#"
            (let ((sensor-val (read-sensor 1)))
              (if (has-capability "io-write-actuator")
                  (write-actuator 1 sensor-val)
                  (network-send "no-actuator-cap" sensor-val)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have dynamic capability checks and conditional logic
                let has_dynamic_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                let has_conditional_jumps = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::JmpIfFalse(_)))
                    .count();

                assert!(
                    has_dynamic_cap_checks >= 2,
                    "Expected dynamic capability checks"
                );
                assert!(has_conditional_jumps >= 1, "Expected conditional jumps");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_ffi_with_complex_data_structures() {
        // Test FFI calls with complex data structures
        let source = r#"
            (let ((sensor-data (list (read-sensor 1) (read-sensor 2) (read-sensor 3)))
                  (processed (map (lambda (val) (* val 2)) sensor-data)))
              (network-send "processed" processed))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have list operations and capability checks
                let has_list_ops = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::MakeList | physics_world::OpCode::ListGet(_)
                    )
                });

                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(has_list_ops, "Expected list operations");
                assert!(
                    has_cap_checks >= 4,
                    "Expected capability checks for sensor reads and network send"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_ffi_stress_with_multiple_trust_tiers() {
        // Test FFI behavior across different trust tiers
        let sources = vec![
            ("Formal", TrustTier::Formal),
            ("Verified", TrustTier::Verified),
            ("Empirical", TrustTier::Empirical),
            ("Experimental", TrustTier::Experimental),
        ];

        let ffi_source = r#"
            (read-sensor 1)
            (write-actuator 1 42)
            (network-send "test" 123)
        "#;

        for (tier_name, tier) in sources {
            let result = compile(ffi_source, tier, 5000, 2048);

            match tier {
                TrustTier::Formal => {
                    // Formal tier should reject all FFI calls
                    assert!(result.is_err(), "Formal tier should reject FFI calls");
                }
                TrustTier::Verified => {
                    // Verified tier should reject I/O FFI calls
                    assert!(result.is_err(), "Verified tier should reject I/O FFI calls");
                }
                TrustTier::Empirical => {
                    // Empirical tier should allow I/O but not system calls
                    assert!(
                        result.is_err(),
                        "Empirical tier should reject network calls"
                    );
                }
                TrustTier::Experimental => {
                    // Experimental tier should allow all FFI calls
                    assert!(
                        result.is_ok(),
                        "Experimental tier should allow all FFI calls"
                    );
                    let compilation_result = result.unwrap();
                    assert!(!compilation_result.bytecode.is_empty());

                    // Should have capability checks for all operations
                    let has_cap_checks = compilation_result
                        .bytecode
                        .iter()
                        .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                        .count();

                    assert!(
                        has_cap_checks >= 3,
                        "Expected capability checks for all FFI calls"
                    );
                }
            }
        }
    }

    #[test]
    fn test_ffi_with_resource_limits() {
        // Test FFI calls with tight resource constraints
        let source = r#"
            (let ((result1 (read-sensor 1))
                  (result2 (read-sensor 2))
                  (result3 (read-sensor 3))
                  (result4 (read-sensor 4))
                  (result5 (read-sensor 5)))
              (+ result1 result2 result3 result4 result5))
        "#;

        // Test with very limited resources
        let result = compile(source, TrustTier::Empirical, 100, 256);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 100);
                assert_eq!(compilation_result.memory_limit, 256);

                // Should still have capability checks even with limited resources
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 5,
                    "Expected capability checks for all sensor reads"
                );
            }
            Err(e) => {
                panic!("Compilation failed with resource limits: {:?}", e);
            }
        }
    }
}
