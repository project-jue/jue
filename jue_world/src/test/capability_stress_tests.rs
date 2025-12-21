#[cfg(test)]
mod capability_stress_tests {
    use crate::compiler::compile;
    use crate::error::CompilationError;
    use crate::trust_tier::TrustTier;

    #[test]
    fn test_capability_checking_across_all_trust_tiers() {
        // Test capability checking behavior across all trust tiers
        let sources = vec![
            ("Formal", TrustTier::Formal),
            ("Verified", TrustTier::Verified),
            ("Empirical", TrustTier::Empirical),
            ("Experimental", TrustTier::Experimental),
        ];

        let capability_source = r#"
            (require-capability "macro-hygienic")
            (require-capability "io-read-sensor")
            (require-capability "io-write-actuator")
            (require-capability "io-network")
            (require-capability "sys-clock")
            (require-capability "io-persist")
            (require-capability "meta-grant")
            (require-capability "comptime-eval")

            (+ 1 2)
        "#;

        for (tier_name, tier) in sources {
            let result = compile(capability_source, tier, 5000, 2048);

            match tier {
                TrustTier::Formal => {
                    // Formal tier should only allow MacroHygienic and ComptimeEval
                    assert!(
                        result.is_ok(),
                        "Formal tier should compile with allowed capabilities"
                    );
                    let compilation_result = result.unwrap();
                    assert_eq!(compilation_result.sandboxed, false);
                }
                TrustTier::Verified => {
                    // Verified tier should allow MacroHygienic, ComptimeEval, and some system capabilities
                    assert!(
                        result.is_ok(),
                        "Verified tier should compile with allowed capabilities"
                    );
                    let compilation_result = result.unwrap();
                    assert_eq!(compilation_result.sandboxed, false);
                }
                TrustTier::Empirical => {
                    // Empirical tier should allow MacroHygienic, ComptimeEval, and I/O capabilities
                    assert!(
                        result.is_ok(),
                        "Empirical tier should compile with allowed capabilities"
                    );
                    let compilation_result = result.unwrap();
                    assert_eq!(compilation_result.sandboxed, false);
                }
                TrustTier::Experimental => {
                    // Experimental tier should allow all capabilities
                    assert!(
                        result.is_ok(),
                        "Experimental tier should compile with all capabilities"
                    );
                    let compilation_result = result.unwrap();
                    assert_eq!(compilation_result.sandboxed, true);
                }
            }
        }
    }

    #[test]
    fn test_capability_checking_with_complex_hierarchy() {
        // Test capability checking with complex capability hierarchies
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro capability-checker (cap-name)
              `(if (has-capability ,cap-name)
                    (println "Has capability:" ,cap-name)
                    (println "Missing capability:" ,cap-name)))

            (capability-checker "io-read-sensor")
            (capability-checker "io-write-actuator")
            (capability-checker "sys-clock")
            (capability-checker "meta-grant")
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have dynamic capability checks
                let has_dynamic_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_dynamic_cap_checks >= 4,
                    "Expected dynamic capability checks for each capability"
                );

                // Should have conditional jumps for each check
                let has_conditional_jumps = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::JmpIfFalse(_)))
                    .count();

                assert!(
                    has_conditional_jumps >= 4,
                    "Expected conditional jumps for each capability check"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_capability_checking_with_nested_macros() {
        // Test capability checking with nested macro expansions
        let source = r#"
            (require-capability "macro-hygienic")

            (defmacro outer-macro (x)
              `(let ((val ,x))
                 (inner-macro val)))

            (defmacro inner-macro (y)
              `(if (> ,y 10)
                    (read-sensor ,y)
                    ,y))

            (outer-macro 5)
            (outer-macro 15)
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have capability checks for the sensor read
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 1,
                    "Expected capability check for sensor read"
                );

                // Should have conditional logic from the macro
                let has_conditional_jumps = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::JmpIfFalse(_)))
                    .count();

                assert!(
                    has_conditional_jumps >= 1,
                    "Expected conditional jump from macro"
                );
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_capability_checking_stress_with_many_requirements() {
        // Test stress scenario with many capability requirements
        let mut capability_source = String::from("(require-capability \"macro-hygienic\")\n");

        // Add many capability requirements
        for i in 1..=20 {
            capability_source.push_str(&format!("(require-capability \"cap-{}\")\n", i));
        }

        capability_source.push_str("(+ 1 2)");

        let result = compile(&capability_source, TrustTier::Experimental, 10000, 4096);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, true);

                // Should have capability checks for all the unknown capabilities
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                // Should have at least 20 capability checks (one for each unknown capability)
                assert!(
                    has_cap_checks >= 20,
                    "Expected capability checks for all requirements"
                );
            }
            Err(e) => {
                panic!(
                    "Compilation failed with many capability requirements: {:?}",
                    e
                );
            }
        }
    }

    #[test]
    fn test_capability_checking_with_trust_tier_transitions() {
        // Test capability checking when transitioning between trust tiers
        let source = r#"
            (let ((formal-result
                    (:formal
                      (require-capability "macro-hygienic")
                      (+ 1 2)))
                  (empirical-result
                    (:empirical
                      (require-capability "io-read-sensor")
                      (read-sensor 1))))
              (+ formal-result empirical-result))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have capability checks for both tiers
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 2,
                    "Expected capability checks for both trust tiers"
                );
            }
            Err(e) => {
                panic!("Compilation failed with trust tier transitions: {:?}", e);
            }
        }
    }

    #[test]
    fn test_capability_checking_with_error_recovery() {
        // Test capability checking with error recovery scenarios
        let source = r#"
            (try
              (require-capability "nonexistent-capability")
              (read-sensor 1)
              (catch (e)
                (println "Error:" e)
                0))
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

                // Should still have capability checks for the sensor read
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
            Err(e) => {
                panic!("Compilation failed with error recovery: {:?}", e);
            }
        }
    }

    #[test]
    fn test_capability_checking_with_complex_conditional_structures() {
        // Test capability checking with complex conditional structures
        let source = r#"
            (let ((has-sensor-cap (has-capability "io-read-sensor"))
                  (has-actuator-cap (has-capability "io-write-actuator"))
                  (has-network-cap (has-capability "io-network")))

              (cond
                ((and has-sensor-cap has-actuator-cap)
                  (let ((val (read-sensor 1)))
                    (write-actuator 1 val)))
                (has-network-cap
                  (network-send "status" "ok"))
                (else
                  (println "No capabilities available"))))
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, false);

                // Should have dynamic capability checks
                let has_dynamic_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_dynamic_cap_checks >= 3,
                    "Expected dynamic capability checks"
                );

                // Should have complex conditional logic
                let has_conditional_jumps = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::JmpIfFalse(_)))
                    .count();

                assert!(
                    has_conditional_jumps >= 3,
                    "Expected conditional jumps for complex logic"
                );
            }
            Err(e) => {
                panic!("Compilation failed with complex conditionals: {:?}", e);
            }
        }
    }

    #[test]
    fn test_capability_checking_with_lambda_and_closures() {
        // Test capability checking with lambda functions and closures
        let source = r#"
            (let ((capability-checker
                    (lambda (cap-name action)
                      (if (has-capability cap-name)
                          (action)
                          (println "Missing capability:" cap-name)))))

              (capability-checker "io-read-sensor" (lambda () (read-sensor 1)))
              (capability-checker "io-write-actuator" (lambda () (write-actuator 1 42))))
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

                // Should have dynamic capability checks
                let has_dynamic_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_dynamic_cap_checks >= 2,
                    "Expected dynamic capability checks"
                );
            }
            Err(e) => {
                panic!("Compilation failed with lambda and closures: {:?}", e);
            }
        }
    }

    #[test]
    fn test_capability_checking_with_resource_constraints() {
        // Test capability checking under tight resource constraints
        let source = r#"
            (require-capability "macro-hygienic")
            (require-capability "io-read-sensor")
            (require-capability "io-write-actuator")
            (require-capability "io-network")
            (require-capability "sys-clock")

            (let ((sensor1 (read-sensor 1))
                  (sensor2 (read-sensor 2))
                  (sensor3 (read-sensor 3))
                  (sensor4 (read-sensor 4))
                  (sensor5 (read-sensor 5)))
              (network-send "sensors" (list sensor1 sensor2 sensor3 sensor4 sensor5)))
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
                    "Expected capability checks for sensor reads and network send"
                );
            }
            Err(e) => {
                panic!("Compilation failed with resource constraints: {:?}", e);
            }
        }
    }

    #[test]
    fn test_capability_checking_with_macro_expansion_stress() {
        // Test capability checking with extensive macro expansion
        let mut source = String::from("(require-capability \"macro-hygienic\")\n");

        // Create many macros that each require different capabilities
        for i in 1..=10 {
            source.push_str(&format!(
                "(defmacro macro-{} () (require-capability \"cap-{}\") (read-sensor {}))\n",
                i, i, i
            ));
        }

        // Call all the macros
        for i in 1..=10 {
            source.push_str(&format!("(macro-{})\n", i));
        }

        let result = compile(&source, TrustTier::Experimental, 10000, 4096);

        match result {
            Ok(compilation_result) => {
                assert!(!compilation_result.bytecode.is_empty());
                assert_eq!(compilation_result.sandboxed, true);

                // Should have capability checks for all the sensor reads
                let has_cap_checks = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, physics_world::OpCode::HasCap(_)))
                    .count();

                assert!(
                    has_cap_checks >= 10,
                    "Expected capability checks for all sensor reads"
                );
            }
            Err(e) => {
                panic!("Compilation failed with macro expansion stress: {:?}", e);
            }
        }
    }
}
