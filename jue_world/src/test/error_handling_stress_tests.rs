#[cfg(test)]
mod error_handling_stress_tests {
    use crate::compiler::compile;
    use crate::error::CompilationError;
    use crate::trust_tier::TrustTier;

    #[test]
    fn test_error_handling_with_nested_try_catch() {
        // Test error handling with deeply nested try-catch blocks
        let source = r#"
            (let ((result
                    (try
                      (try
                        (try
                          (read-sensor 1)
                          (catch (e1)
                            (try
                              (read-sensor 2)
                              (catch (e2)
                                (network-send "error2" e2)
                                0)))
                          (catch (e3)
                            (network-send "error3" e3)
                            0)))))))
              result)
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have multiple error handling constructs
                let has_error_handling = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| {
                        matches!(
                            op,
                            physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                        )
                    })
                    .count();

                assert!(
                    has_error_handling >= 4,
                    "Expected multiple error handling constructs"
                );

                // Should have capability checks for FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 3,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_error_handling_with_ffi_in_catch_blocks() {
        // Test error handling with FFI calls in catch blocks
        let source = r#"
            (let ((result1
                    (try
                      (read-sensor 1)
                      (catch (e)
                        (network-send "sensor1-error" e)
                        0)))
                  (result2
                    (try
                      (read-sensor 2)
                      (catch (e)
                        (network-send "sensor2-error" e)
                        0)))
                  (result3
                    (try
                      (read-sensor 3)
                      (catch (e)
                        (network-send "sensor3-error" e)
                        0))))
              (+ result1 result2 result3))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have error handling constructs
                let has_error_handling = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| {
                        matches!(
                            op,
                            physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                        )
                    })
                    .count();

                assert!(
                    has_error_handling >= 3,
                    "Expected error handling constructs"
                );

                // Should have capability checks for all FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 6,
                    "Expected capability checks for all FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_error_handling_with_macro_generated_errors() {
        // Test error handling with macros that generate error-prone code
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro safe-ffi (ffi-call)
              `(try
                 ,ffi-call
                 (catch (e)
                   (println "FFI Error:" e)
                   0)))

            (defmacro retry-ffi (ffi-call max-attempts)
              `(let ((attempt 0)
                     (result 0))
                 (while (and (< attempt ,max-attempts) (= result 0))
                   (set! result (safe-ffi ,ffi-call))
                   (when (= result 0)
                     (set! attempt (+ attempt 1))))
                 result))

            (let ((sensor1 (retry-ffi (read-sensor 1) 3))
                  (sensor2 (retry-ffi (read-sensor 2) 3))
                  (sensor3 (retry-ffi (read-sensor 3) 3)))
              (+ sensor1 sensor2 sensor3))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have error handling constructs
                let has_error_handling = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| {
                        matches!(
                            op,
                            physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                        )
                    })
                    .count();

                assert!(
                    has_error_handling >= 6,
                    "Expected error handling constructs"
                );

                // Should have capability checks for FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 3,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_error_handling_with_ffi_in_finally_blocks() {
        // Test error handling with FFI calls in finally-like patterns
        let source = r#"
            (let ((result
                    (let ((cleanup-done false))
                      (try
                        (let ((val (read-sensor 1)))
                          (when (> val 100)
                            (write-actuator 1 val))
                          val)
                        (catch (e)
                          (network-send "sensor-error" e)
                          0)
                        (finally
                          (set! cleanup-done true)
                          (network-send "cleanup-complete" cleanup-done))))))
              result)
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have error handling constructs
                let has_error_handling = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| {
                        matches!(
                            op,
                            physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                        )
                    })
                    .count();

                assert!(
                    has_error_handling >= 1,
                    "Expected error handling constructs"
                );

                // Should have capability checks for all FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 3,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_error_handling_with_nested_macro_errors() {
        // Test error handling with nested macros that can fail
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro outer-safe (inner-macro)
              `(try
                 ,inner-macro
                 (catch (e)
                   (network-send "outer-error" e)
                   0)))

            (defmacro inner-safe (ffi-call)
              `(try
                 ,ffi-call
                 (catch (e)
                   (network-send "inner-error" e)
                   0)))

            (let ((result1 (outer-safe (inner-safe (read-sensor 1))))
                  (result2 (outer-safe (inner-safe (read-sensor 2))))
                  (result3 (outer-safe (inner-safe (read-sensor 3)))))
              (+ result1 result2 result3))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have nested error handling constructs
                let has_error_handling = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| {
                        matches!(
                            op,
                            physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                        )
                    })
                    .count();

                assert!(
                    has_error_handling >= 6,
                    "Expected nested error handling constructs"
                );

                // Should have capability checks for all FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 6,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_error_handling_with_ffi_in_lambda_errors() {
        // Test error handling with FFI calls in lambda functions that can fail
        let source = r#"
            (let ((safe-sensor-reader
                    (lambda (id)
                      (try
                        (read-sensor id)
                        (catch (e)
                          (network-send "sensor-read-error" (list id e))
                          0))))
                  (safe-actuator-writer
                    (lambda (id val)
                      (try
                        (write-actuator id val)
                        (catch (e)
                          (network-send "actuator-write-error" (list id val e))
                          false))))
                  (safe-network-sender
                    (lambda (msg data)
                      (try
                        (network-send msg data)
                        (catch (e)
                          (println "Network error:" e)
                          false)))))

              (let ((sensor1 (safe-sensor-reader 1))
                    (sensor2 (safe-sensor-reader 2))
                    (actuator1-result (safe-actuator-writer 1 sensor1))
                    (actuator2-result (safe-actuator-writer 2 sensor2))
                    (network-result (safe-network-sender "summary" (list sensor1 sensor2 actuator1-result actuator2-result))))
                (+ sensor1 sensor2)))
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

                assert!(has_closure_ops >= 3, "Expected closure operations");

                // Should have error handling constructs
                let has_error_handling = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| {
                        matches!(
                            op,
                            physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                        )
                    })
                    .count();

                assert!(
                    has_error_handling >= 3,
                    "Expected error handling constructs"
                );

                // Should have capability checks for all FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 9,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_error_handling_with_ffi_in_recursive_errors() {
        // Test error handling with FFI calls in recursive functions that can fail
        let source = r#"
            (let ((safe-recursive-sensor-sum
                    (lambda (id limit)
                      (try
                        (if (> id limit)
                            0
                            (let ((val (read-sensor id)))
                              (if (> val 1000)
                                  (throw "sensor-value-too-high")
                                  (+ val (safe-recursive-sensor-sum (+ id 1) limit)))))
                        (catch (e)
                          (network-send "recursive-error" (list id e))
                          0)))))

              (safe-recursive-sensor-sum 1 5))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have error handling constructs
                let has_error_handling = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| {
                        matches!(
                            op,
                            physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                        )
                    })
                    .count();

                assert!(
                    has_error_handling >= 1,
                    "Expected error handling constructs"
                );

                // Should have recursive calls
                let has_recursive_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::Call(_)))
                    .count();

                assert!(has_recursive_calls >= 1, "Expected recursive calls");

                // Should have capability checks for FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 2,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_error_handling_with_ffi_in_complex_conditional_errors() {
        // Test error handling with FFI calls in complex conditional error scenarios
        let source = r#"
            (let ((sensor1 (read-sensor 1))
                  (sensor2 (read-sensor 2))
                  (sensor3 (read-sensor 3)))

              (cond
                ((> sensor1 1000)
                  (try
                    (write-actuator 1 sensor1)
                    (catch (e)
                      (network-send "actuator1-error" e)
                      0)))
                ((> sensor2 1000)
                  (try
                    (write-actuator 2 sensor2)
                    (catch (e)
                      (network-send "actuator2-error" e)
                      0)))
                ((> sensor3 1000)
                  (try
                    (write-actuator 3 sensor3)
                    (catch (e)
                      (network-send "actuator3-error" e)
                      0)))
                (else
                  (try
                    (network-send "all-normal" (list sensor1 sensor2 sensor3))
                    (catch (e)
                      (println "Network error:" e)
                      (+ sensor1 sensor2 sensor3))))))
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
                    has_conditional_jumps >= 6,
                    "Expected complex conditional logic"
                );

                // Should have error handling constructs
                let has_error_handling = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| {
                        matches!(
                            op,
                            physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                        )
                    })
                    .count();

                assert!(
                    has_error_handling >= 4,
                    "Expected error handling constructs"
                );

                // Should have capability checks for all FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 6,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_error_handling_with_ffi_in_resource_constrained_errors() {
        // Test error handling with FFI calls under resource constraints
        let source = r#"
            (let ((result
                    (try
                      (let ((sensor1 (read-sensor 1))
                            (sensor2 (read-sensor 2))
                            (sensor3 (read-sensor 3))
                            (sensor4 (read-sensor 4))
                            (sensor5 (read-sensor 5)))
                        (let ((sum (+ sensor1 sensor2 sensor3 sensor4 sensor5))
                              (avg (/ sum 5)))
                          (if (> avg 50)
                              (network-send "high-average" avg)
                              (network-send "low-average" avg))
                          sum))
                      (catch (e)
                        (network-send "computation-error" e)
                        0))))
              result)
        "#;

        // Test with limited resources
        let result = compile(source, TrustTier::Empirical, 1000, 512);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 1000);
                assert_eq!(compilation_result.memory_limit, 512);

                // Should have error handling constructs
                let has_error_handling = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| {
                        matches!(
                            op,
                            physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                        )
                    })
                    .count();

                assert!(
                    has_error_handling >= 1,
                    "Expected error handling constructs"
                );

                // Should have capability checks for FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 6,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_error_handling_with_ffi_in_macro_expansion_errors() {
        // Test error handling with FFI calls in macro expansion under error conditions
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro error-handling-ffi (ffi-call error-handler)
              `(try
                 ,ffi-call
                 (catch (e)
                   ,error-handler)))

            (defmacro network-error-handler (e)
              `(network-send "ffi-error" e))

            (let ((sensor1 (error-handling-ffi (read-sensor 1) (network-error-handler e)))
                  (sensor2 (error-handling-ffi (read-sensor 2) (network-error-handler e)))
                  (sensor3 (error-handling-ffi (read-sensor 3) (network-error-handler e)))
                  (sensor4 (error-handling-ffi (read-sensor 4) (network-error-handler e)))
                  (sensor5 (error-handling-ffi (read-sensor 5) (network-error-handler e))))
              (+ sensor1 sensor2 sensor3 sensor4 sensor5))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have error handling constructs
                let has_error_handling = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| {
                        matches!(
                            op,
                            physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                        )
                    })
                    .count();

                assert!(
                    has_error_handling >= 5,
                    "Expected error handling constructs"
                );

                // Should have capability checks for all FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 10,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_error_handling_with_ffi_in_trust_tier_error_scenarios() {
        // Test error handling with FFI calls across trust tiers in error scenarios
        let source = r#"
            (let ((formal-result
                    (try
                      (:formal (+ 1 2 3))
                      (catch (e)
                        (network-send "formal-error" e)
                        0)))
                  (empirical-result
                    (try
                      (:empirical (read-sensor 1))
                      (catch (e)
                        (network-send "empirical-error" e)
                        0)))
                  (experimental-result
                    (try
                      (:experimental (get-wall-clock))
                      (catch (e)
                        (network-send "experimental-error" e)
                        0))))
              (+ formal-result empirical-result experimental-result))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have error handling constructs
                let has_error_handling = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| {
                        matches!(
                            op,
                            physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                        )
                    })
                    .count();

                assert!(
                    has_error_handling >= 3,
                    "Expected error handling constructs"
                );

                // Should have capability checks for different trust tiers
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 3,
                    "Expected capability checks for different trust tiers"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }
}
