#[cfg(test)]
mod type_system_ffi_stress_tests {
    use crate::compiler::compile;
    use crate::error::CompilationError;
    use crate::trust_tier::TrustTier;

    #[test]
    fn test_type_system_with_ffi_and_complex_types() {
        // Test type system interactions with FFI and complex types
        let source = r#"
            (let ((sensor-data (list (read-sensor 1) (read-sensor 2) (read-sensor 3)))
                  (processed (map (lambda (val) (* val 2.0)) sensor-data))
                  (filtered (filter (lambda (val) (> val 100.0)) processed))
                  (reduced (reduce (lambda (acc val) (+ acc val)) 0 filtered)))
              (network-send "processed-result" (list sensor-data processed filtered reduced)))
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
                    has_cap_checks >= 4,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_type_system_with_ffi_and_type_coercion() {
        // Test type system with FFI and type coercion scenarios
        let source = r#"
            (let ((int-sensor (read-sensor 1))
                  (float-sensor (* int-sensor 1.5))
                  (string-result (str "Sensor value: " float-sensor))
                  (bool-result (> float-sensor 50.0)))
              (network-send "typed-result" (list int-sensor float-sensor string-result bool-result)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have type conversion operations
                let has_type_ops = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::Float(_) | physics_world::OpCode::Str(_)
                    )
                });

                assert!(has_type_ops, "Expected type conversion operations");

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
    fn test_type_system_with_ffi_and_generic_functions() {
        // Test type system with FFI and generic function applications
        let source = r#"
            (let ((sensor-values (list (read-sensor 1) (read-sensor 2) (read-sensor 3)))
                  (doubled (map (lambda (x) (* x 2)) sensor-values))
                  (tripled (map (lambda (x) (* x 3)) sensor-values))
                  (combined (map (lambda (x y) (+ x y)) doubled tripled)))
              (network-send "generic-result" (list sensor-values doubled tripled combined)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have generic function operations
                let has_generic_ops = compilation_result
                    .bytecode
                    .iter()
                    .any(|op| matches!(op, physics_world::OpCode::Call(_)));

                assert!(has_generic_ops, "Expected generic function operations");

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
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_type_system_with_ffi_and_type_inference() {
        // Test type system with FFI and complex type inference
        let source = r#"
            (let ((sensor1 (read-sensor 1))
                  (sensor2 (read-sensor 2))
                  (sensor3 (read-sensor 3))
                  (sum (+ sensor1 sensor2 sensor3))
                  (average (/ sum 3))
                  (result (if (> average 50)
                              (str "High: " average)
                              (str "Low: " average))))
              (network-send "inference-result" result))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have type inference operations
                let has_type_inference = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::Add
                            | physics_world::OpCode::Div
                            | physics_world::OpCode::Str(_)
                    )
                });

                assert!(has_type_inference, "Expected type inference operations");

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
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_type_system_with_ffi_and_macro_type_expansion() {
        // Test type system with FFI and macro-generated type expansions
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro typed-ffi-wrapper (ffi-call result-type)
              `(let ((raw-result ,ffi-call)
                     (typed-result (case ,result-type
                                      ("int" raw-result)
                                      ("float" (* raw-result 1.0))
                                      ("string" (str "Result: " raw-result))
                                      ("bool" (> raw-result 0))
                                      (else raw-result))))
                 typed-result))

            (let ((int-result (typed-ffi-wrapper (read-sensor 1) "int"))
                  (float-result (typed-ffi-wrapper (read-sensor 2) "float"))
                  (string-result (typed-ffi-wrapper (read-sensor 3) "string"))
                  (bool-result (typed-ffi-wrapper (read-sensor 4) "bool")))
              (network-send "typed-results" (list int-result float-result string-result bool-result)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have type conversion operations from macro expansion
                let has_type_conversions = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::Float(_) | physics_world::OpCode::Str(_)
                    )
                });

                assert!(has_type_conversions, "Expected type conversion operations");

                // Should have capability checks for FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 5,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_type_system_with_ffi_and_complex_type_hierarchies() {
        // Test type system with FFI and complex type hierarchies
        let source = r#"
            (let ((sensor-data
                    (list
                      (list (read-sensor 1) (read-sensor 2))
                      (list (read-sensor 3) (read-sensor 4))
                      (list (read-sensor 5) (read-sensor 6))))
                  (processed
                    (map (lambda (pair)
                           (let ((sum (apply + pair))
                                 (avg (/ sum (length pair)))
                                 (scaled (* avg 100.0)))
                             (list sum avg scaled)))
                         sensor-data))
                  (flattened (apply concat processed))
                  (final-sum (apply + (map first flattened))))
              (network-send "hierarchy-result" (list sensor-data processed flattened final-sum)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have complex type hierarchy operations
                let has_type_hierarchy_ops = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::MakeList
                            | physics_world::OpCode::ListGet(_)
                            | physics_world::OpCode::Call(_)
                    )
                });

                assert!(has_type_hierarchy_ops, "Expected type hierarchy operations");

                // Should have capability checks for FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 7,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_type_system_with_ffi_and_type_constraints() {
        // Test type system with FFI and type constraint scenarios
        let source = r#"
            (let ((sensor1 (read-sensor 1))
                  (sensor2 (read-sensor 2))
                  (sensor3 (read-sensor 3))
                  (constrained-sum
                    (let ((sum (+ sensor1 sensor2 sensor3)))
                      (if (number? sum)
                          (if (integer? sum)
                              sum
                              (int sum))
                          0)))
                  (constrained-avg
                    (let ((avg (/ constrained-sum 3)))
                      (if (number? avg)
                          (if (float? avg)
                              avg
                              (float avg))
                          0.0)))
                  (constrained-result
                    (if (> constrained-avg 50.0)
                        (str "High: " constrained-avg)
                        (str "Low: " constrained-avg))))
              (network-send "constrained-result" (list constrained-sum constrained-avg constrained-result)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have type constraint operations
                let has_type_constraints = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::Int(_)
                            | physics_world::OpCode::Float(_)
                            | physics_world::OpCode::Str(_)
                    )
                });

                assert!(has_type_constraints, "Expected type constraint operations");

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
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_type_system_with_ffi_and_type_polymorphism() {
        // Test type system with FFI and polymorphic type scenarios
        let source = r#"
            (let ((sensor-values (list (read-sensor 1) (read-sensor 2) (read-sensor 3)))
                  (processors
                    (list
                      (lambda (x) (* x 2))
                      (lambda (x) (+ x 100))
                      (lambda (x) (if (> x 50) x 0))))
                  (processed
                    (map (lambda (processor)
                           (map processor sensor-values))
                         processors))
                  (flattened (apply concat processed))
                  (summed (apply + flattened))
                  (averaged (/ summed (length flattened)))
                  (result (if (> averaged 100)
                              (str "High average: " averaged)
                              (str "Low average: " averaged))))
              (network-send "polymorphic-result" (list sensor-values processed flattened summed averaged result)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have polymorphic operations
                let has_polymorphic_ops = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::MakeClosure(_, _) | physics_world::OpCode::Call(_)
                    )
                });

                assert!(has_polymorphic_ops, "Expected polymorphic operations");

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
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_type_system_with_ffi_and_type_inference_across_trust_tiers() {
        // Test type system with FFI and type inference across trust tiers
        let source = r#"
            (let ((formal-result
                    (:formal
                      (let ((x 42)
                            (y 3.14)
                            (z (str "Result: " (+ x y))))
                        z)))
                  (empirical-result
                    (:empirical
                      (let ((sensor1 (read-sensor 1))
                            (sensor2 (read-sensor 2))
                            (sum (+ sensor1 sensor2))
                            (result (str "Sensors: " sum)))
                        result)))
                  (experimental-result
                    (:experimental
                      (let ((time (get-wall-clock))
                            (time-str (str "Time: " time)))
                        time-str))))
              (network-send "cross-tier-types" (list formal-result empirical-result experimental-result)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have type inference across tiers
                let has_type_inference = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::Add | physics_world::OpCode::Str(_)
                    )
                });

                assert!(has_type_inference, "Expected type inference operations");

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

    #[test]
    fn test_type_system_with_ffi_and_type_coercion_in_macros() {
        // Test type system with FFI and type coercion in macros
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro coerce-ffi-result (ffi-call target-type)
              `(let ((raw-result ,ffi-call)
                     (coerced-result
                       (cond
                         ((= ,target-type "int") (int raw-result))
                         ((= ,target-type "float") (float raw-result))
                         ((= ,target-type "string") (str "Value: " raw-result))
                         ((= ,target-type "bool") (> raw-result 0))
                         (else raw-result))))
                 coerced-result))

            (let ((int-sensor (coerce-ffi-result (read-sensor 1) "int"))
                  (float-sensor (coerce-ffi-result (read-sensor 2) "float"))
                  (string-sensor (coerce-ffi-result (read-sensor 3) "string"))
                  (bool-sensor (coerce-ffi-result (read-sensor 4) "bool")))
              (network-send "coerced-results" (list int-sensor float-sensor string-sensor bool-sensor)))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have type coercion operations from macro expansion
                let has_type_coercion = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::Int(_)
                            | physics_world::OpCode::Float(_)
                            | physics_world::OpCode::Str(_)
                    )
                });

                assert!(has_type_coercion, "Expected type coercion operations");

                // Should have capability checks for FFI operations
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 5,
                    "Expected capability checks for FFI operations"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_type_system_with_ffi_and_complex_type_operations() {
        // Test type system with FFI and complex type operations under resource constraints
        let source = r#"
            (let ((sensor-data
                    (list
                      (list (read-sensor 1) (read-sensor 2) (read-sensor 3))
                      (list (read-sensor 4) (read-sensor 5) (read-sensor 6))
                      (list (read-sensor 7) (read-sensor 8) (read-sensor 9))))
                  (transposed
                    (apply map list sensor-data))
                  (processed
                    (map (lambda (row)
                           (let ((sum (apply + row))
                                 (avg (/ sum (length row)))
                                 (scaled (* avg 100.0))
                                 (categorized (cond
                                                ((> scaled 5000.0) "high")
                                                ((> scaled 1000.0) "medium")
                                                (else "low"))))
                             (list sum avg scaled categorized)))
                         transposed))
                  (result (apply str (map (lambda (item) (str (first item) ":" (last item) " ")) processed))))
              (network-send "complex-type-result" (list sensor-data transposed processed result)))
        "#;

        // Test with limited resources
        let result = compile(source, TrustTier::Empirical, 3000, 1536);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 3000);
                assert_eq!(compilation_result.memory_limit, 1536);

                // Should have complex type operations
                let has_complex_type_ops = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::MakeList
                            | physics_world::OpCode::ListGet(_)
                            | physics_world::OpCode::Call(_)
                            | physics_world::OpCode::Str(_)
                    )
                });

                assert!(has_complex_type_ops, "Expected complex type operations");

                // Should have capability checks for FFI operations
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
}
