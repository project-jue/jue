#[cfg(test)]
mod macro_ffi_integration_tests {
    use crate::compiler::compile;
    use crate::error::CompilationError;
    use crate::trust_tier::TrustTier;

    #[test]
    fn test_macro_with_ffi_integration() {
        // Test macro that generates FFI calls
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro sensor-reader (id)
              `(read-sensor ,id))

            (let ((val1 (sensor-reader 1))
                  (val2 (sensor-reader 2)))
              (+ val1 val2))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have capability checks for the sensor reads
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 2,
                    "Expected capability checks for sensor reads"
                );

                // Should have the actual FFI calls
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .any(|op| matches!(op, physics_world::OpCode::HostCall { .. }));

                assert!(has_ffi_calls, "Expected FFI calls in bytecode");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_macro_with_conditional_ffi() {
        // Test macro that generates conditional FFI calls
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro conditional-sensor-read (id threshold)
              `(let ((val (read-sensor ,id)))
                 (if (> val ,threshold)
                     (write-actuator ,id val)
                     val)))

            (let ((result1 (conditional-sensor-read 1 100))
                  (result2 (conditional-sensor-read 2 50)))
              (+ result1 result2))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have capability checks for both sensor reads and potential actuator writes
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 4,
                    "Expected capability checks for sensor reads and actuator writes"
                );

                // Should have conditional logic
                let has_conditional_jumps = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::JmpIfFalse(_)))
                    .count();

                assert!(has_conditional_jumps >= 2, "Expected conditional jumps");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_macro_with_nested_ffi_calls() {
        // Test macro that generates nested FFI calls
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro sensor-and-network (id)
              `(let ((sensor-val (read-sensor ,id)))
                 (network-send "sensor-data" sensor-val)
                 sensor-val))

            (let ((result1 (sensor-and-network 1))
                  (result2 (sensor-and-network 2)))
              (network-send "summary" (+ result1 result2)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have capability checks for sensor reads and network sends
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 5,
                    "Expected capability checks for sensor reads and network sends"
                );

                // Should have multiple FFI calls
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HostCall { .. }))
                    .count();

                assert!(has_ffi_calls >= 5, "Expected multiple FFI calls");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_macro_with_ffi_and_error_handling() {
        // Test macro that generates FFI calls with error handling
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro safe-sensor-read (id)
              `(try
                 (read-sensor ,id)
                 (catch (e)
                   (println "Sensor error:" e)
                   0)))

            (let ((result1 (safe-sensor-read 1))
                  (result2 (safe-sensor-read 2)))
              (+ result1 result2))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have error handling constructs
                let has_error_handling = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                    )
                });

                assert!(has_error_handling, "Expected error handling constructs");

                // Should have capability checks for sensor reads
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 2,
                    "Expected capability checks for sensor reads"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_macro_with_ffi_in_lambda() {
        // Test macro that generates FFI calls within lambda functions
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro sensor-reader-lambda (id)
              `(lambda () (read-sensor ,id)))

            (let ((reader1 (sensor-reader-lambda 1))
                  (reader2 (sensor-reader-lambda 2)))
              (+ (reader1) (reader2)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have closure creation
                let has_closure_ops = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::MakeClosure(_, _)))
                    .count();

                assert!(has_closure_ops >= 2, "Expected closure operations");

                // Should have capability checks for sensor reads
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 2,
                    "Expected capability checks for sensor reads"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_macro_with_ffi_and_recursion() {
        // Test macro that generates recursive functions with FFI calls
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro recursive-sensor-sum (start end)
              `(let ((sum-sensors (lambda (id limit)
                                    (if (> id limit)
                                        0
                                        (+ (read-sensor id)
                                           (sum-sensors (+ id 1) limit))))))
                 (sum-sensors ,start ,end)))

            (recursive-sensor-sum 1 3)
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have recursive calls
                let has_recursive_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::Call(_)))
                    .count();

                assert!(has_recursive_calls >= 3, "Expected recursive calls");

                // Should have capability checks for sensor reads
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 3,
                    "Expected capability checks for sensor reads"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_macro_with_ffi_and_complex_data_structures() {
        // Test macro that generates FFI calls with complex data structures
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro sensor-list-reader (ids)
              `(let ((values (map (lambda (id) (read-sensor id)) ,ids)))
                 (network-send "sensor-values" values)
                 values))

            (sensor-list-reader (list 1 2 3 4 5))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have list operations
                let has_list_ops = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::MakeList | physics_world::OpCode::ListGet(_)
                    )
                });

                assert!(has_list_ops, "Expected list operations");

                // Should have capability checks for sensor reads and network send
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 6,
                    "Expected capability checks for sensor reads and network send"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_macro_with_ffi_across_trust_tiers() {
        // Test macro with FFI integration across different trust tiers
        let sources = vec![
            ("Formal", TrustTier::Formal),
            ("Verified", TrustTier::Verified),
            ("Empirical", TrustTier::Empirical),
            ("Experimental", TrustTier::Experimental),
        ];

        let macro_ffi_source = r#"
            (require-capability "macro-hygienic")

            (defmacro safe-sensor-read (id)
              `(read-sensor ,id))

            (safe-sensor-read 1)
        "#;

        for (tier_name, tier) in sources {
            let result = compile(macro_ffi_source, tier, 5000, 2048);

            match tier {
                TrustTier::Formal => {
                    // Formal tier should reject FFI calls even in macros
                    assert!(
                        result.is_err(),
                        "Formal tier should reject FFI calls in macros"
                    );
                }
                TrustTier::Verified => {
                    // Verified tier should reject I/O FFI calls in macros
                    assert!(
                        result.is_err(),
                        "Verified tier should reject I/O FFI calls in macros"
                    );
                }
                TrustTier::Empirical => {
                    // Empirical tier should allow I/O FFI calls in macros
                    assert!(
                        result.is_ok(),
                        "Empirical tier should allow I/O FFI calls in macros"
                    );
                    let compilation_result = result.unwrap();
                    assert!(!compilation_result.bytecode.is_empty());
                    assert_eq!(compilation_result.sandboxed, false);

                    // Should have capability check for sensor read
                    let has_cap_checks = compilation_result
                        .bytecode
                        .iter()
                        .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                        .count();

                    assert!(
                        has_cap_checks >= 1,
                        "Expected capability check for sensor read"
                    );
                }
                TrustTier::Experimental => {
                    // Experimental tier should allow all FFI calls in macros
                    assert!(
                        result.is_ok(),
                        "Experimental tier should allow all FFI calls in macros"
                    );
                    let compilation_result = result.unwrap();
                    assert!(!compilation_result.bytecode.is_empty());
                    assert_eq!(compilation_result.sandboxed, true);

                    // Should have capability check for sensor read
                    let has_cap_checks = compilation_result
                        .bytecode
                        .iter()
                        .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                        .count();

                    assert!(
                        has_cap_checks >= 1,
                        "Expected capability check for sensor read"
                    );
                }
            }
        }
    }

    #[test]
    fn test_macro_with_ffi_and_resource_limits() {
        // Test macro with FFI integration under tight resource constraints
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro multi-sensor-read (count)
              `(let ,(loop (i 1) (acc '()) (if (> i count) (reverse acc) (recur (+ i 1) (cons (read-sensor i) acc)))))
                 (apply + ,(loop (i 1) (acc '()) (if (> i count) (reverse acc) (recur (+ i 1) (cons (read-sensor i) acc)))))))

            (multi-sensor-read 5)
        "#;

        // Test with very limited resources
        let result = compile(source, TrustTier::Empirical, 200, 512);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 200);
                assert_eq!(compilation_result.memory_limit, 512);

                // Should still have capability checks even with limited resources
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 5,
                    "Expected capability checks for sensor reads"
                );
            }
            Err(e) => {
                panic!("Compilation failed with resource limits: {:?}", e);
            }
        }
    }

    #[test]
    fn test_macro_with_ffi_and_complex_control_flow() {
        // Test macro with FFI integration and complex control flow
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro sensor-monitor (id threshold callback)
              `(let ((val (read-sensor ,id)))
                 (cond
                   ((> val ,threshold) (,callback "high" val))
                   ((> val (* ,threshold 0.5)) (,callback "medium" val))
                   (else (,callback "low" val)))))

            (defmacro network-callback (level val)
              `(network-send "sensor-alert" (list ,level ,val)))

            (sensor-monitor 1 100 network-callback)
            (sensor-monitor 2 50 network-callback)
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have complex conditional logic
                let has_conditional_jumps = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::JmpIfFalse(_)))
                    .count();

                assert!(
                    has_conditional_jumps >= 4,
                    "Expected conditional jumps for complex logic"
                );

                // Should have capability checks for sensor reads and network sends
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 4,
                    "Expected capability checks for sensor reads and network sends"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_macro_with_ffi_and_error_recovery() {
        // Test macro with FFI integration and error recovery
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro safe-sensor-operation (id)
              `(try
                 (let ((val (read-sensor ,id)))
                   (if (> val 100)
                       (write-actuator ,id val)
                       val))
                 (catch (e)
                   (network-send "sensor-error" (list ,id e))
                   0)))

            (let ((result1 (safe-sensor-operation 1))
                  (result2 (safe-sensor-operation 2)))
              (+ result1 result2))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have error handling constructs
                let has_error_handling = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                    )
                });

                assert!(has_error_handling, "Expected error handling constructs");

                // Should have capability checks for sensor reads, actuator writes, and network sends
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 4,
                    "Expected capability checks for all FFI operations"
                );

                // Should have conditional logic
                let has_conditional_jumps = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::JmpIfFalse(_)))
                    .count();

                assert!(has_conditional_jumps >= 2, "Expected conditional jumps");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }
}
