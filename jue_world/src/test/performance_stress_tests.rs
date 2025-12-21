#[cfg(test)]
mod performance_stress_tests {
    use crate::compiler::compile;
    use crate::error::CompilationError;
    use crate::trust_tier::TrustTier;
    use std::time::Instant;

    #[test]
    fn test_performance_with_large_expression() {
        // Test performance with a very large expression
        let mut large_expression = String::from("(let (");

        // Add many variable bindings
        for i in 1..=100 {
            large_expression.push_str(&format!("(x{} {}) ", i, i));
        }

        large_expression.push_str(") (+ ");

        // Add all variables to a sum
        for i in 1..=100 {
            large_expression.push_str(&format!("x{} ", i));
        }

        large_expression.push_str("))");

        let start_time = Instant::now();
        let result = compile(&large_expression, TrustTier::Empirical, 10000, 4096);
        let compilation_time = start_time.elapsed();

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should complete in reasonable time
                assert!(
                    compilation_time.as_millis() < 1000,
                    "Compilation took too long: {}ms",
                    compilation_time.as_millis()
                );

                // Should have reasonable bytecode size
                assert!(
                    compilation_result.bytecode.len() < 1000,
                    "Bytecode too large: {}",
                    compilation_result.bytecode.len()
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_performance_with_deeply_nested_expressions() {
        // Test performance with deeply nested expressions
        let mut nested_expression = String::from("(let ((x 1)) ");

        // Create deeply nested if expressions
        for _ in 0..50 {
            nested_expression.push_str("(if (> x 0) ");
        }

        nested_expression.push_str("x ");

        for _ in 0..50 {
            nested_expression.push_str("0)");
        }

        nested_expression.push_str(")");

        let start_time = Instant::now();
        let result = compile(&nested_expression, TrustTier::Empirical, 10000, 4096);
        let compilation_time = start_time.elapsed();

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should complete in reasonable time
                assert!(
                    compilation_time.as_millis() < 1000,
                    "Compilation took too long: {}ms",
                    compilation_time.as_millis()
                );

                // Should have many conditional jumps
                let has_conditional_jumps = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::JmpIfFalse(_)))
                    .count();

                assert!(
                    has_conditional_jumps >= 50,
                    "Expected many conditional jumps"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_performance_with_many_ffi_calls() {
        // Test performance with many FFI calls
        let mut ffi_expression = String::from("(let (");

        // Add many FFI calls
        for i in 1..=50 {
            ffi_expression.push_str(&format!("(sensor{} (read-sensor {})) ", i, i));
        }

        ffi_expression.push_str(") (+ ");

        // Sum all FFI results
        for i in 1..=50 {
            ffi_expression.push_str(&format!("sensor{} ", i));
        }

        ffi_expression.push_str("))");

        let start_time = Instant::now();
        let result = compile(&ffi_expression, TrustTier::Empirical, 10000, 4096);
        let compilation_time = start_time.elapsed();

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should complete in reasonable time
                assert!(
                    compilation_time.as_millis() < 1000,
                    "Compilation took too long: {}ms",
                    compilation_time.as_millis()
                );

                // Should have capability checks for all FFI calls
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 50,
                    "Expected capability checks for all FFI calls"
                );

                // Should have all FFI calls
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HostCall { .. }))
                    .count();

                assert!(has_ffi_calls >= 50, "Expected all FFI calls");
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_performance_with_complex_macro_expansion() {
        // Test performance with complex macro expansion
        let mut macro_expression = String::from("(require-capability \"macro-hygienic\")\n");

        // Define many macros
        for i in 1..=20 {
            macro_expression.push_str(&format!("(defmacro macro-{} (x) `(+ x {}))\n", i, i));
        }

        // Call all macros
        for i in 1..=20 {
            macro_expression.push_str(&format!("(macro-{} {})\n", i, i));
        }

        let start_time = Instant::now();
        let result = compile(&macro_expression, TrustTier::Empirical, 10000, 4096);
        let compilation_time = start_time.elapsed();

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should complete in reasonable time
                assert!(
                    compilation_time.as_millis() < 1000,
                    "Compilation took too long: {}ms",
                    compilation_time.as_millis()
                );

                // Should have arithmetic operations from macro expansion
                let has_arithmetic_ops = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::Add))
                    .count();

                assert!(
                    has_arithmetic_ops >= 20,
                    "Expected arithmetic operations from macro expansion"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_resource_limits_with_large_constants() {
        // Test resource limits with large constant data
        let mut constants_expression = String::from("(let (");

        // Add many large constants
        for i in 1..=100 {
            constants_expression.push_str(&format!("(large-const{} (list ", i));
            for j in 1..=10 {
                constants_expression.push_str(&format!("{} ", j * i));
            }
            constants_expression.push_str(")) ");
        }

        constants_expression.push_str(") (+ ");

        // Sum some constants
        for i in 1..=10 {
            constants_expression.push_str(&format!("(apply + large-const{}) ", i));
        }

        constants_expression.push_str("))");

        let result = compile(&constants_expression, TrustTier::Empirical, 5000, 1024);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 5000);
                assert_eq!(compilation_result.memory_limit, 1024);

                // Should have list operations
                let has_list_ops = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::MakeList | physics_world::OpCode::ListGet(_)
                    )
                });

                assert!(has_list_ops, "Expected list operations");
            }
            Err(e) => {
                panic!("Compilation failed with large constants: {:?}", e);
            }
        }
    }

    #[test]
    fn test_resource_limits_with_deep_recursion() {
        // Test resource limits with deep recursion
        let source = r#"
            (let ((deep-recursion (lambda (n)
                                    (if (= n 0)
                                        0
                                        (+ n (deep-recursion (- n 1)))))))
              (deep-recursion 100))
        "#;

        // Test with limited resources
        let result = compile(source, TrustTier::Empirical, 1000, 512);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 1000);
                assert_eq!(compilation_result.memory_limit, 512);

                // Should have recursive calls
                let has_recursive_calls = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::Call(_)))
                    .count();

                assert!(has_recursive_calls >= 1, "Expected recursive calls");
            }
            Err(e) => {
                panic!("Compilation failed with deep recursion: {:?}", e);
            }
        }
    }

    #[test]
    fn test_resource_limits_with_complex_ffi_operations() {
        // Test resource limits with complex FFI operations
        let source = r#"
            (let ((sensor1 (read-sensor 1))
                  (sensor2 (read-sensor 2))
                  (sensor3 (read-sensor 3))
                  (sensor4 (read-sensor 4))
                  (sensor5 (read-sensor 5))
                  (sensor6 (read-sensor 6))
                  (sensor7 (read-sensor 7))
                  (sensor8 (read-sensor 8))
                  (sensor9 (read-sensor 9))
                  (sensor10 (read-sensor 10)))
              (let ((sum (+ sensor1 sensor2 sensor3 sensor4 sensor5 sensor6 sensor7 sensor8 sensor9 sensor10))
                    (avg (/ sum 10)))
                (if (> avg 50)
                    (network-send "high-average" avg)
                    (network-send "low-average" avg))))
        "#;

        // Test with very limited resources
        let result = compile(source, TrustTier::Empirical, 500, 256);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 500);
                assert_eq!(compilation_result.memory_limit, 256);

                // Should have capability checks even with limited resources
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 11,
                    "Expected capability checks for sensor reads and network send"
                );

                // Should have conditional logic
                let has_conditional_jumps = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::JmpIfFalse(_)))
                    .count();

                assert!(has_conditional_jumps >= 1, "Expected conditional jump");
            }
            Err(e) => {
                panic!("Compilation failed with complex FFI operations: {:?}", e);
            }
        }
    }

    #[test]
    fn test_resource_limits_with_macro_expansion_stress() {
        // Test resource limits with extensive macro expansion
        let mut macro_stress_expression = String::from("(require-capability \"macro-hygienic\")\n");

        // Define macros that expand to more macros
        for i in 1..=5 {
            macro_stress_expression.push_str(&format!(
                "(defmacro outer-macro-{} (x) `(let ((val{} ,x)) (inner-macro-{} val{})))\n",
                i, i, i, i
            ));
            macro_stress_expression
                .push_str(&format!("(defmacro inner-macro-{} (y) `(+ y {}))\n", i, i));
        }

        // Call all macros multiple times
        for i in 1..=5 {
            for j in 1..=10 {
                macro_stress_expression.push_str(&format!("(outer-macro-{} {})\n", i, j));
            }
        }

        // Test with limited resources
        let result = compile(&macro_stress_expression, TrustTier::Empirical, 2000, 1024);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 2000);
                assert_eq!(compilation_result.memory_limit, 1024);

                // Should have arithmetic operations from macro expansion
                let has_arithmetic_ops = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::Add))
                    .count();

                assert!(
                    has_arithmetic_ops >= 50,
                    "Expected arithmetic operations from macro expansion"
                );
            }
            Err(e) => {
                panic!("Compilation failed with macro expansion stress: {:?}", e);
            }
        }
    }

    #[test]
    fn test_resource_limits_with_complex_error_handling() {
        // Test resource limits with complex error handling
        let source = r#"
            (let ((result1 (try (read-sensor 1) (catch (e) 0)))
                  (result2 (try (read-sensor 2) (catch (e) 0)))
                  (result3 (try (read-sensor 3) (catch (e) 0)))
                  (result4 (try (read-sensor 4) (catch (e) 0)))
                  (result5 (try (read-sensor 5) (catch (e) 0))))
              (let ((sum (+ result1 result2 result3 result4 result5))
                    (avg (/ sum 5)))
                (try
                  (network-send "sensor-average" avg)
                  (catch (e)
                    (println "Network error:" e)
                    avg))))
        "#;

        // Test with limited resources
        let result = compile(source, TrustTier::Empirical, 1000, 512);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 1000);
                assert_eq!(compilation_result.memory_limit, 512);

                // Should have error handling constructs
                let has_error_handling = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                    )
                });

                assert!(has_error_handling, "Expected error handling constructs");

                // Should have capability checks
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
                panic!("Compilation failed with complex error handling: {:?}", e);
            }
        }
    }

    #[test]
    fn test_resource_limits_with_mixed_operations() {
        // Test resource limits with mixed operations
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro sensor-processor (id)
              `(let ((val (read-sensor ,id)))
                 (if (> val 100)
                     (write-actuator ,id val)
                     val)))

            (let ((result1 (sensor-processor 1))
                  (result2 (sensor-processor 2))
                  (result3 (sensor-processor 3))
                  (result4 (sensor-processor 4))
                  (result5 (sensor-processor 5)))
              (let ((sum (+ result1 result2 result3 result4 result5))
                    (processed (map (lambda (x) (* x 2)) (list result1 result2 result3 result4 result5))))
                (try
                  (network-send "processed" processed)
                  (catch (e)
                    (println "Error:" e)
                    sum))))
        "#;

        // Test with very limited resources
        let result = compile(source, TrustTier::Empirical, 1500, 768);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 1500);
                assert_eq!(compilation_result.memory_limit, 768);

                // Should have mixed operations: FFI, macros, lambdas, error handling
                let has_ffi_calls = compilation_result
                    .bytecode
                    .iter()
                    .any(|op| matches!(op, physics_world::OpCode::HostCall { .. }));

                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                let has_error_handling = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::TryStart | physics_world::OpCode::TryEnd(_)
                    )
                });

                let has_list_ops = compilation_result.bytecode.iter().any(|op| {
                    matches!(
                        op,
                        physics_world::OpCode::MakeList | physics_world::OpCode::ListGet(_)
                    )
                });

                assert!(has_ffi_calls, "Expected FFI calls");
                assert!(has_cap_checks >= 10, "Expected capability checks");
                assert!(has_error_handling, "Expected error handling");
                assert!(has_list_ops, "Expected list operations");
            }
            Err(e) => {
                panic!("Compilation failed with mixed operations: {:?}", e);
            }
        }
    }

    #[test]
    fn test_resource_limits_with_trust_tier_transitions() {
        // Test resource limits with trust tier transitions
        let source = r#"
            (let ((formal-result
                    (:formal
                      (require-capability "macro-hygienic")
                      (+ 1 2 3 4 5)))
                  (empirical-result
                    (:empirical
                      (require-capability "io-read-sensor")
                      (let ((sensor1 (read-sensor 1))
                            (sensor2 (read-sensor 2)))
                        (+ sensor1 sensor2))))
                  (experimental-result
                    (:experimental
                      (require-capability "sys-clock")
                      (get-wall-clock))))
              (+ formal-result empirical-result experimental-result))
        "#;

        // Test with limited resources
        let result = compile(source, TrustTier::Empirical, 2000, 1024);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.step_limit, 2000);
                assert_eq!(compilation_result.memory_limit, 1024);

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
                    has_ffi_calls >= 3,
                    "Expected FFI calls for different trust tiers"
                );
            }
            Err(e) => {
                panic!("Compilation failed with trust tier transitions: {:?}", e);
            }
        }
    }
}
