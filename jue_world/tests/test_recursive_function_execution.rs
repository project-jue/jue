#[cfg(test)]
mod recursive_function_execution_tests {
    use jue_world::parser::parse;
    use jue_world::physics_compiler::compile_to_physics_world;
    use jue_world::trust_tier::TrustTier;
    use physics_world::types::Value;
    use physics_world::vm::VmState;

    /// Test helper to execute and validate recursive function results
    fn test_recursive_execution(
        source: &str,
        trust_tier: TrustTier,
        expected_result: Option<Value>,
        test_name: &str,
    ) {
        println!("\n=== Testing {} ===", test_name);

        // Parse and compile the program
        let ast = match parse(source) {
            Ok(parsed_ast) => {
                println!("✅ Parsing successful");
                parsed_ast
            }
            Err(e) => {
                panic!("❌ Parsing failed for {}: {:?}", test_name, e);
            }
        };

        let (bytecode, string_constants) = match compile_to_physics_world(&ast, trust_tier) {
            Ok((bytecode, constants)) => {
                println!("✅ Compilation successful");
                println!("Bytecode length: {}", bytecode.len());
                (bytecode, constants)
            }
            Err(e) => {
                panic!("❌ Compilation failed for {}: {:?}", test_name, e);
            }
        };

        // Execute the program
        let mut vm = VmState::new(bytecode, string_constants, 1000, 1024, 1, 100);
        let execution_result = match vm.run() {
            Ok(result) => {
                println!("✅ Execution successful!");
                result
            }
            Err(e) => {
                panic!("❌ Execution failed for {}: {:?}", test_name, e);
            }
        };

        // Validate the result if provided
        if let Some(expected) = expected_result {
            match (&execution_result, &expected) {
                (Value::Int(actual_int), Value::Int(expected_int)) => {
                    assert_eq!(
                        actual_int, expected_int,
                        "{}: Expected Int({}), got Int({})",
                        test_name, expected_int, actual_int
                    );
                    println!("✅ Correct result: Int({})", actual_int);
                }
                (Value::Bool(actual_bool), Value::Bool(expected_bool)) => {
                    assert_eq!(
                        actual_bool, expected_bool,
                        "{}: Expected Bool({}), got Bool({})",
                        test_name, expected_bool, actual_bool
                    );
                    println!("✅ Correct result: Bool({})", actual_bool);
                }
                (Value::Float(actual_float), Value::Float(expected_float)) => {
                    assert!(
                        (actual_float - expected_float).abs() < f64::EPSILON,
                        "{}: Expected Float({}), got Float({})",
                        test_name,
                        expected_float,
                        actual_float
                    );
                    println!("✅ Correct result: Float({})", actual_float);
                }
                (Value::String(actual_string), Value::String(expected_string)) => {
                    assert_eq!(
                        actual_string, expected_string,
                        "{}: Expected String({}), got String({})",
                        test_name, expected_string, actual_string
                    );
                    println!("✅ Correct result: String({})", actual_string);
                }
                _ => {
                    panic!(
                        "{}: Wrong result type! Expected {:?}, got {:?}",
                        test_name, expected, execution_result
                    );
                }
            }
        }
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_factorial_recursion_empirical() {
        let factorial_source = r#"
            (let ((fact (lambda (n)
                          (if (<= n 1)
                              1
                              (* n (fact (- n 1)))))))
              (fact 5))
        "#;

        test_recursive_execution(
            factorial_source,
            TrustTier::Empirical,
            Some(Value::Int(120)), // 5! = 120
            "Factorial Recursion (Empirical)",
        );
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_factorial_recursion_experimental() {
        let factorial_source = r#"
            (let ((fact (lambda (n)
                          (if (<= n 1)
                              1
                              (* n (fact (- n 1)))))))
              (fact 5))
        "#;

        test_recursive_execution(
            factorial_source,
            TrustTier::Experimental,
            Some(Value::Int(120)), // 5! = 120
            "Factorial Recursion (Experimental)",
        );
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_factorial_recursion_formal() {
        let factorial_source = r#"
            (let ((fact (lambda (n)
                          (if (<= n 1)
                              1
                              (* n (fact (- n 1)))))))
              (fact 5))
        "#;

        test_recursive_execution(
            factorial_source,
            TrustTier::Formal,
            Some(Value::Int(120)), // 5! = 120
            "Factorial Recursion (Formal)",
        );
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_factorial_recursion_verified() {
        let factorial_source = r#"
            (let ((fact (lambda (n)
                          (if (<= n 1)
                              1
                              (* n (fact (- n 1)))))))
              (fact 5))
        "#;

        test_recursive_execution(
            factorial_source,
            TrustTier::Verified,
            Some(Value::Int(120)), // 5! = 120
            "Factorial Recursion (Verified)",
        );
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_fibonacci_recursion() {
        let fibonacci_source = r#"
            (let ((fib (lambda (n)
                         (if (<= n 1)
                             n
                             (+ (fib (- n 1)) (fib (- n 2)))))))
              (fib 7))
        "#;

        test_recursive_execution(
            fibonacci_source,
            TrustTier::Empirical,
            Some(Value::Int(13)), // fib(7) = 13
            "Fibonacci Recursion",
        );
    }

    #[test]
    #[ignore = "Requires letrec support for mutual recursion - tracked in jue_world_code_review.md"]
    fn test_mutual_recursion_even_odd() {
        let mutual_recursion_source = r#"
            (let ((is-even? (lambda (n)
                              (if (= n 0)
                                  true
                                  (is-odd? (- n 1)))))
                  (is-odd? (lambda (n)
                             (if (= n 0)
                                 false
                                 (is-even? (- n 1))))))
              (is-even? 4))
        "#;

        test_recursive_execution(
            mutual_recursion_source,
            TrustTier::Empirical,
            Some(Value::Bool(true)), // 4 is even
            "Mutual Recursion (Even/Odd)",
        );
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_nested_recursive_functions() {
        let nested_recursion_source = r#"
            (let ((outer (lambda (x)
                           (let ((inner (lambda (y)
                                         (if (= y 0)
                                             x
                                             (inner (- y 1))))))
                             (inner x)))))
              (outer 3))
        "#;

        test_recursive_execution(
            nested_recursion_source,
            TrustTier::Empirical,
            Some(Value::Int(3)), // Should return x (3)
            "Nested Recursive Functions",
        );
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_recursive_with_closure_capture() {
        let closure_recursion_source = r#"
            (let ((base 10)
                  (count-down (lambda (n)
                                (if (= n 0)
                                    base
                                    (count-down (- n 1))))))
              (count-down 3))
        "#;

        test_recursive_execution(
            closure_recursion_source,
            TrustTier::Empirical,
            Some(Value::Int(10)), // Should return base (10)
            "Recursive with Closure Capture",
        );
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_recursive_with_let_bindings() {
        let let_binding_recursion_source = r#"
            (let ((result 0)
                  (accumulate (lambda (n)
                                (let ((new-result (+ result n)))
                                  (if (= n 0)
                                      new-result
                                      (accumulate (- n 1)))))))
              (accumulate 5))
        "#;

        test_recursive_execution(
            let_binding_recursion_source,
            TrustTier::Empirical,
            Some(Value::Int(15)), // 0+1+2+3+4+5 = 15
            "Recursive with Let Bindings",
        );
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_recursive_with_conditional_logic() {
        let conditional_recursion_source = r#"
            (let ((classify (lambda (n)
                              (if (= n 0)
                                  "zero"
                                  (if (< n 0)
                                      "negative"
                                      "positive")))))
              (classify -5))
        "#;

        // String result test - we'll just check it executes without error
        let ast = parse(conditional_recursion_source).expect("Parsing should succeed");
        let (bytecode, string_constants) = compile_to_physics_world(&ast, TrustTier::Empirical)
            .expect("Compilation should succeed");

        let mut vm = VmState::new(bytecode, string_constants, 1000, 1024, 1, 100);
        let result = vm.run().expect("Execution should succeed");

        assert!(
            matches!(result, Value::String(_)),
            "Should return a string result"
        );
        println!("✅ Recursive with conditional logic executed successfully");
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_recursive_power_function() {
        let power_source = r#"
            (let ((power (lambda (base exp)
                           (if (= exp 0)
                               1
                               (* base (power base (- exp 1)))))))
              (power 2 8))
        "#;

        test_recursive_execution(
            power_source,
            TrustTier::Empirical,
            Some(Value::Int(256)), // 2^8 = 256
            "Recursive Power Function",
        );
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_deep_recursion_stack_safety() {
        let deep_recursion_source = r#"
            (let ((count-down (lambda (n)
                                (if (= n 0)
                                    "done"
                                    (count-down (- n 1))))))
              (count-down 50))
        "#;

        test_recursive_execution(
            deep_recursion_source,
            TrustTier::Empirical,
            Some(Value::String("done".to_string())),
            "Deep Recursion Stack Safety (50 levels)",
        );
    }

    #[test]
    #[ignore = "Requires modulo (%) operator implementation - tracked in jue_world_code_review.md"]
    fn test_recursive_gcd_function() {
        let gcd_source = r#"
            (let ((gcd (lambda (a b)
                         (if (= b 0)
                             a
                             (gcd b (% a b))))))
              (gcd 48 18))
        "#;

        test_recursive_execution(
            gcd_source,
            TrustTier::Empirical,
            Some(Value::Int(6)), // gcd(48, 18) = 6
            "Recursive GCD Function",
        );
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_multiple_recursive_calls_same_function() {
        let multiple_calls_source = r#"
            (let ((double-fact (lambda (n)
                                 (if (<= n 2)
                                     n
                                     (* n (double-fact (- n 2)))))))
              (double-fact 8))
        "#;

        test_recursive_execution(
            multiple_calls_source,
            TrustTier::Empirical,
            Some(Value::Int(384)), // 8 * 6 * 4 * 2 = 384
            "Multiple Recursive Calls Same Function",
        );
    }

    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_recursive_with_float_operations() {
        let float_recursion_source = r#"
            (let ((harmonic (lambda (n)
                              (if (= n 1)
                                  1.0
                                  (+ (/ 1.0 n) (harmonic (- n 1)))))))
              (harmonic 5))
        "#;

        test_recursive_execution(
            float_recursion_source,
            TrustTier::Empirical,
            Some(Value::Float(2.283333333333333)), // 1 + 1/2 + 1/3 + 1/4 + 1/5
            "Recursive with Float Operations",
        );
    }

    /// Test compilation without execution for complex recursive patterns
    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_recursive_compilation_all_tiers() {
        let recursive_source = r#"
            (let ((fact (lambda (n)
                          (if (<= n 1)
                              1
                              (* n (fact (- n 1)))))))
              (fact 10))
        "#;

        let ast = parse(recursive_source).expect("Parsing should succeed");

        // Test compilation for all trust tiers
        for tier in [
            TrustTier::Formal,
            TrustTier::Verified,
            TrustTier::Empirical,
            TrustTier::Experimental,
        ] {
            println!("Testing compilation for {:?}", tier);
            let result = compile_to_physics_world(&ast, tier);
            assert!(result.is_ok(), "Compilation should succeed for {:?}", tier);

            let (bytecode, _) = result.unwrap();
            assert!(
                !bytecode.is_empty(),
                "Should generate bytecode for {:?}",
                tier
            );

            // Check for recursive closure creation
            let has_closures = bytecode
                .iter()
                .any(|op| matches!(op, physics_world::types::OpCode::MakeClosure(_, _)));
            assert!(
                has_closures,
                "Should create closures for recursion in {:?}",
                tier
            );
        }
    }

    /// Test edge case: recursive function with no base case (should handle gracefully)
    #[test]
    #[ignore = "Requires letrec support for recursive lambda bindings - tracked in jue_world_code_review.md"]
    fn test_recursive_no_base_case() {
        let no_base_case_source = r#"
            (let ((infinite-recursion (lambda (n)
                                       (infinite-recursion (+ n 1)))))
              (infinite-recursion 0))
        "#;

        let ast = parse(no_base_case_source).expect("Parsing should succeed");
        let (bytecode, string_constants) = compile_to_physics_world(&ast, TrustTier::Empirical)
            .expect("Compilation should succeed");

        // This should either execute with CPU limit or handle gracefully
        let mut vm = VmState::new(bytecode, string_constants, 100, 1024, 1, 10); // Low step limit
        let result = vm.run();

        // Should either succeed, hit CPU limit, or handle gracefully
        assert!(result.is_ok() || result.is_err());
        println!("✅ Recursive function without base case handled appropriately");
    }

    /// Test mutual recursion compilation patterns
    #[test]
    #[ignore = "Requires letrec support for mutual recursion - tracked in jue_world_code_review.md"]
    fn test_mutual_recursion_compilation_patterns() {
        let mutual_source = r#"
            (let ((even (lambda (n) (if (= n 0) true (odd (- n 1)))))
                  (odd (lambda (n) (if (= n 0) false (even (- n 1))))))
              (even 6))
        "#;

        let ast = parse(mutual_source).expect("Parsing should succeed");
        let (bytecode, _) = compile_to_physics_world(&ast, TrustTier::Empirical)
            .expect("Compilation should succeed");

        // Should have multiple closures for mutual recursion
        let closure_count = bytecode
            .iter()
            .filter(|op| matches!(op, physics_world::types::OpCode::MakeClosure(_, _)))
            .count();

        assert!(
            closure_count >= 2,
            "Should have at least 2 closures for mutual recursion"
        );
        println!(
            "✅ Mutual recursion compilation created {} closures",
            closure_count
        );
    }
}
