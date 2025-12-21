#[cfg(test)]
mod integration_stress_tests {
    use crate::compiler::compile;
    use crate::error::CompilationError;
    use crate::trust_tier::TrustTier;

    #[test]
    fn test_integration_ffi_macros_trust_tiers() {
        // Test integration of FFI, macros, and trust tiers
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro safe-ffi-wrapper (ffi-call)
              `(try
                 ,ffi-call
                 (catch (e)
                   (println "FFI Error:" e)
                   0)))

            (let ((formal-result
                    (:formal
                      (safe-ffi-wrapper (+ 1 2))))
                  (empirical-result
                    (:empirical
                      (safe-ffi-wrapper (read-sensor 1))))
                  (experimental-result
                    (:experimental
                      (safe-ffi-wrapper (get-wall-clock)))))
              (+ formal-result empirical-result experimental-result))
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

                // Should have FFI calls for different tiers
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HostCall { .. }))
                    .count();

                assert!(
                    has_ffi_calls >= 2,
                    "Expected FFI calls for empirical and experimental tiers"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_integration_macros_lambdas_ffi() {
        // Test integration of macros, lambdas, and FFI
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro lambda-wrapper (body)
              `(lambda () ,body))

            (defmacro ffi-lambda (ffi-call)
              `(let ((ffi-func (lambda-wrapper ,ffi-call)))
                 (ffi-func)))

            (let ((sensor-reader (ffi-lambda (read-sensor 1)))
                  (actuator-writer (ffi-lambda (write-actuator 1 42)))
                  (network-sender (ffi-lambda (network-send "test" 123))))
              (+ (sensor-reader) (actuator-writer) (network-sender)))
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

                // Should have FFI calls
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HostCall { .. }))
                    .count();

                assert!(has_ffi_calls >= 3, "Expected FFI calls");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_integration_ffi_error_handling_macros() {
        // Test integration of FFI, error handling, and macros
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro safe-ffi-chain (ffi-calls)
              `(try
                 (let ,(loop (calls ,ffi-calls) (bindings '()) (acc 0)
                           (if (empty? calls)
                               (reverse bindings)
                               (let ((call (first calls)))
                                 (recur (rest calls)
                                        (cons `(,(first call) ,(second call)) bindings)
                                        (+ acc 1)))))
                   ,(loop (i 0) (sum-exprs '())
                         (if (>= i (length ffi-calls))
                             (if (empty? sum-exprs)
                                 0
                                 `(+ ,@sum-exprs))
                             (recur (+ i 1)
                                    (cons `(nth ,i ,(quote ~(loop (j 0) (names '())
                                                                 (if (>= j (length ffi-calls))
                                                                     names
                                                                     (recur (+ j 1)
                                                                            (cons (symbol (str "result" j)) names)))))
                                          sum-exprs)))))
                 (catch (e)
                   (network-send "chain-error" e)
                   0)))

            (safe-ffi-chain ((read-sensor 1) (read-sensor 2) (read-sensor 3)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have complex error handling
                let has_error_handling = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                    )
                });

                assert!(has_error_handling, "Expected error handling constructs");

                // Should have capability checks for FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 4,
                    "Expected capability checks for FFI operations and error handling"
                );

                // Should have FFI calls
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HostCall { .. }))
                    .count();

                assert!(has_ffi_calls >= 3, "Expected FFI calls");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_integration_trust_tiers_macros_ffi() {
        // Test integration of trust tiers, macros, and FFI
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro tiered-ffi (tier ffi-call)
              `(,tier ,ffi-call))

            (let ((formal-result
                    (tiered-ffi :formal (+ 1 2 3)))
                  (empirical-result
                    (tiered-ffi :empirical (read-sensor 1)))
                  (verified-result
                    (tiered-ffi :verified (if (has-capability "sys-clock")
                                                     (get-wall-clock)
                                                     0)))
                  (experimental-result
                    (tiered-ffi :experimental (network-send "test" 42))))
              (+ formal-result empirical-result verified-result experimental-result))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have capability checks for different tiers
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 4,
                    "Expected capability checks for different tiers"
                );

                // Should have conditional logic for capability checks
                let has_conditional_jumps = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::JmpIfFalse(_)))
                    .count();

                assert!(has_conditional_jumps >= 1, "Expected conditional jumps");

                // Should have FFI calls for appropriate tiers
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HostCall { .. }))
                    .count();

                assert!(
                    has_ffi_calls >= 2,
                    "Expected FFI calls for empirical and experimental tiers"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_integration_macros_ffi_resource_limits() {
        // Test integration of macros, FFI, and resource limits
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro resource-aware-ffi (ffi-call max-retries)
              `(let ((attempt 0)
                     (result 0))
                 (while (and (< attempt ,max-retries) (= result 0))
                   (try
                     (set! result ,ffi-call)
                     (catch (e)
                       (println "Attempt" attempt "failed:" e)
                       (set! attempt (+ attempt 1))
                       (when (= attempt ,max-retries)
                         (network-send "final-error" e)))))
                 result))

            (let ((sensor1 (resource-aware-ffi (read-sensor 1) 3))
                  (sensor2 (resource-aware-ffi (read-sensor 2) 3))
                  (sensor3 (resource-aware-ffi (read-sensor 3) 3)))
              (+ sensor1 sensor2 sensor3))
        "#;

        // Test with limited resources
        let result = compile(source, TrustTier::Empirical, 3000, 1536);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 3000);
                assert_eq!(compilation_result.memory_limit, 1536);

                // Should have error handling constructs
                let has_error_handling = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                    )
                });

                assert!(has_error_handling, "Expected error handling constructs");

                // Should have capability checks for FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 4,
                    "Expected capability checks for FFI operations"
                );

                // Should have FFI calls
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HostCall { .. }))
                    .count();

                assert!(has_ffi_calls >= 3, "Expected FFI calls");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_integration_ffi_macros_complex_data() {
        // Test integration of FFI, macros, and complex data structures
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro sensor-data-processor (ids)
              `(let ((raw-data (map (lambda (id) (read-sensor id)) ,ids))
                     (processed (map (lambda (val) (* val 2)) raw-data))
                     (filtered (filter (lambda (val) (> val 100)) processed)))
                 (network-send "sensor-data" (list raw-data processed filtered))
                 (apply + filtered)))

            (sensor-data-processor (list 1 2 3 4 5 6 7 8 9 10))
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

                // Should have capability checks for FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 11,
                    "Expected capability checks for sensor reads and network send"
                );

                // Should have FFI calls
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HostCall { .. }))
                    .count();

                assert!(has_ffi_calls >= 11, "Expected FFI calls");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_integration_macros_ffi_error_recovery() {
        // Test integration of macros, FFI, and error recovery
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro robust-ffi-operation (ffi-call fallback)
              `(try
                 ,ffi-call
                 (catch (e)
                   (network-send "operation-failed" (list (quote ,ffi-call) e))
                   ,fallback)))

            (defmacro sensor-with-fallback (id)
              `(robust-ffi-operation (read-sensor ,id) 0))

            (defmacro actuator-with-fallback (id val)
              `(robust-ffi-operation (write-actuator ,id val) false))

            (let ((sensor-sum (+ (sensor-with-fallback 1)
                                (sensor-with-fallback 2)
                                (sensor-with-fallback 3)))
                  (actuator-results (list (actuator-with-fallback 1 42)
                                         (actuator-with-fallback 2 43)
                                         (actuator-with-fallback 3 44))))
              (network-send "results" (list sensor-sum actuator-results))
              sensor-sum)
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

                // Should have capability checks for FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 8,
                    "Expected capability checks for FFI operations"
                );

                // Should have FFI calls
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HostCall { .. }))
                    .count();

                assert!(has_ffi_calls >= 6, "Expected FFI calls");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_integration_ffi_macros_trust_tier_transitions() {
        // Test integration of FFI, macros, and trust tier transitions
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro tiered-ffi-wrapper (tier ffi-call)
              `(,tier
                 (try
                   ,ffi-call
                   (catch (e)
                     (println "FFI Error in" (quote ,tier) ":" e)
                     0))))

            (let ((formal-result
                    (tiered-ffi-wrapper :formal (+ 1 2 3 4 5)))
                  (verified-result
                    (tiered-ffi-wrapper :verified (if (has-capability "sys-clock")
                                                      (get-wall-clock)
                                                      0)))
                  (empirical-result
                    (tiered-ffi-wrapper :empirical (read-sensor 1)))
                  (experimental-result
                    (tiered-ffi-wrapper :experimental (network-send "test" 123))))
              (+ formal-result verified-result empirical-result experimental-result))
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

                // Should have capability checks for different tiers
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 4,
                    "Expected capability checks for different tiers"
                );

                // Should have conditional logic
                let has_conditional_jumps = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::JmpIfFalse(_)))
                    .count();

                assert!(has_conditional_jumps >= 1, "Expected conditional jumps");

                // Should have FFI calls for appropriate tiers
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HostCall { .. }))
                    .count();

                assert!(
                    has_ffi_calls >= 2,
                    "Expected FFI calls for empirical and experimental tiers"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_integration_macros_ffi_resource_constraints() {
        // Test integration of macros, FFI, and resource constraints
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro resource-bounded-ffi (ffi-call max-attempts)
              `(let ((attempt 0)
                     (result 0))
                 (while (and (< attempt ,max-attempts) (= result 0))
                   (try
                     (set! result ,ffi-call)
                     (catch (e)
                       (when (< attempt ,max-attempts)
                         (println "Attempt" attempt "failed:" e)
                         (set! attempt (+ attempt 1)))))
                   (when (and (= attempt ,max-attempts) (= result 0))
                     (network-send "max-attempts-reached" (list (quote ,ffi-call) ,max-attempts))))
                 result))

            (let ((sensor1 (resource-bounded-ffi (read-sensor 1) 2))
                  (sensor2 (resource-bounded-ffi (read-sensor 2) 2))
                  (sensor3 (resource-bounded-ffi (read-sensor 3) 2))
                  (sensor4 (resource-bounded-ffi (read-sensor 4) 2))
                  (sensor5 (resource-bounded-ffi (read-sensor 5) 2)))
              (+ sensor1 sensor2 sensor3 sensor4 sensor5))
        "#;

        // Test with very limited resources
        let result = compile(source, TrustTier::Empirical, 2000, 1024);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 2000);
                assert_eq!(compilation_result.memory_limit, 1024);

                // Should have error handling constructs
                let has_error_handling = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                    )
                });

                assert!(has_error_handling, "Expected error handling constructs");

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

                // Should have FFI calls
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HostCall { .. }))
                    .count();

                assert!(has_ffi_calls >= 5, "Expected FFI calls");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_integration_ffi_macros_complex_control_flow() {
        // Test integration of FFI, macros, and complex control flow
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro conditional-ffi-chain (conditions ffi-calls)
              `(let ,(loop (i 0) (bindings '())
                          (if (>= i (length conditions))
                              (reverse bindings)
                              (recur (+ i 1)
                                     (cons `(result, ,i (if ,(nth i conditions)
                                                           ,(nth i ffi-calls)
                                                           0))
                                           bindings))))
                 (+ ,@(loop (i 0) (sum-exprs '())
                           (if (>= i (length conditions))
                               sum-exprs
                               (recur (+ i 1)
                                      (cons `(result, ,i) sum-exprs)))))))

            (let ((sensor1 (read-sensor 1))
                  (sensor2 (read-sensor 2))
                  (sensor3 (read-sensor 3)))
              (conditional-ffi-chain
                ((> sensor1 50) (> sensor2 50) (> sensor3 50))
                ((write-actuator 1 sensor1)
                 (write-actuator 2 sensor2)
                 (write-actuator 3 sensor3))))
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

                // Should have FFI calls
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HostCall { .. }))
                    .count();

                assert!(has_ffi_calls >= 6, "Expected FFI calls");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }
}
