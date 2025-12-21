#[cfg(test)]
mod tests {
    use crate::compiler::{
        compile, CapabilityCheck, CheckType, CompilationResult, EmpiricalResult,
    };
    use crate::error::SourceMap;
    use crate::error::{CompilationError, SourceLocation};
    use crate::test_timeout::{run_test_with_guard, TestError};
    use crate::trust_tier::TrustTier;
    use physics_world::types::{Capability, OpCode, Value};
    use std::time::Duration;

    /// Wrapper for compile function that includes timeout protection
    fn compile_with_timeout(
        source: String,
        tier: TrustTier,
        step_limit: u64,
        mem_limit: usize,
    ) -> Result<CompilationResult, CompilationError> {
        let source_for_errors = source.clone();
        let result = run_test_with_guard(
            move |guard| {
                // Check for cancellation periodically during compilation
                if guard.check_cancellation() {
                    panic!("Test timed out or exceeded memory limits");
                }
                compile(&source, tier, step_limit, mem_limit)
            },
            Duration::from_secs(10), // 10 second timeout for compilation
            200_000_000,             // 200MB memory limit
        );

        match result {
            Ok(result) => result,
            Err(TestError::Timeout) => {
                panic!("Test timed out while compiling: {}", source_for_errors);
            }
            Err(TestError::MemoryLimitExceeded) => {
                panic!(
                    "Test exceeded memory limits while compiling: {}",
                    source_for_errors
                );
            }
            Err(TestError::Panic) => {
                panic!("Test panicked while compiling: {}", source_for_errors);
            }
        }
    }

    #[test]
    fn test_compilation_result_structure() {
        let result = CompilationResult {
            bytecode: vec![OpCode::Nil],
            constants: vec![Value::Nil],
            step_limit: 1000,
            memory_limit: 1024,
            core_proof: None,
            core_expr: None,
            required_capabilities: vec![Capability::MacroHygienic],
            granted_capabilities: vec![Capability::MacroHygienic, Capability::ComptimeEval],
            empirical_check: EmpiricalResult::NotApplicable,
            sandboxed: false,
            source_map: SourceMap::new(),
            capability_audit: vec![CapabilityCheck {
                location: SourceLocation::default(),
                capability: Capability::MacroHygienic,
                check_type: CheckType::Static,
            }],
        };

        assert_eq!(result.bytecode.len(), 1);
        assert_eq!(result.constants.len(), 1);
        assert_eq!(result.step_limit, 1000);
        assert_eq!(result.memory_limit, 1024);
        assert_eq!(result.required_capabilities.len(), 1);
        assert_eq!(result.granted_capabilities.len(), 2);
        assert!(matches!(
            result.empirical_check,
            EmpiricalResult::NotApplicable
        ));
        assert!(!result.sandboxed);
    }

    #[test]
    fn test_empirical_result_variants() {
        let passed = EmpiricalResult::Passed {
            tests_run: 10,
            coverage: 0.95,
        };

        let failed = EmpiricalResult::Failed {
            reason: "Test case 5 failed".to_string(),
            failing_case: "test_case_5".to_string(),
        };

        let not_applicable = EmpiricalResult::NotApplicable;

        match passed {
            EmpiricalResult::Passed {
                tests_run,
                coverage,
            } => {
                assert_eq!(tests_run, 10);
                assert_eq!(coverage, 0.95);
            }
            _ => panic!("Expected Passed variant"),
        }

        match failed {
            EmpiricalResult::Failed {
                reason,
                failing_case,
            } => {
                assert_eq!(reason, "Test case 5 failed");
                assert_eq!(failing_case, "test_case_5");
            }
            _ => panic!("Expected Failed variant"),
        }

        match not_applicable {
            EmpiricalResult::NotApplicable => assert!(true),
            _ => panic!("Expected NotApplicable variant"),
        }
    }

    #[test]
    fn test_capability_check_types() {
        let static_check = CheckType::Static;
        let runtime_check = CheckType::Runtime;
        let proof_check = CheckType::Proof;

        assert_eq!(static_check, CheckType::Static);
        assert_eq!(runtime_check, CheckType::Runtime);
        assert_eq!(proof_check, CheckType::Proof);
    }

    #[test]
    fn test_compile_function_signature() {
        // Test that the compile function has the correct signature
        let source = "(+ 1 1)".to_string();
        let tier = TrustTier::Empirical;
        let step_limit = 1000;
        let mem_limit = 1024;

        // This should compile without panicking
        let result = compile_with_timeout(source, tier, step_limit, mem_limit);

        // Now that we have proper parsing, this should succeed
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod integration_tests {
    use crate::compiler::{compile, CompilationResult, TrustTier};
    use crate::error::CompilationError;

    #[test]
    fn test_compile_hello_world() {
        let source = r#"
            (println "Hello, World!")
        "#;

        let result = compile(source, TrustTier::Empirical, 1000, 1024);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded
                assert!(
                    !compilation_result.bytecode.is_empty()
                        || compilation_result.constants.is_empty()
                );
                assert_eq!(compilation_result.sandboxed, false); // Empirical tier should not be sandboxed
                assert_eq!(compilation_result.step_limit, 1000);
                assert_eq!(compilation_result.memory_limit, 1024);
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_compile_arithmetic() {
        let source = r#"
            (+ 1 2 3)
            (- 10 5)
            (* 2 3)
            (/ 10 2)
        "#;

        let result = compile(source, TrustTier::Empirical, 1000, 1024);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded
                assert!(
                    !compilation_result.bytecode.is_empty()
                        || compilation_result.constants.is_empty()
                );
                assert_eq!(compilation_result.sandboxed, false);
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_compile_lambda() {
        let source = r#"
            (lambda (x) (+ x 1))
        "#;

        let result = compile(source, TrustTier::Empirical, 1000, 1024);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded
                assert!(
                    !compilation_result.bytecode.is_empty()
                        || compilation_result.constants.is_empty()
                );
                assert_eq!(compilation_result.sandboxed, false);
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_compile_lambda_application() {
        let source = r#"
            ((lambda (x) (+ x 1)) 5)
        "#;

        let result = compile(source, TrustTier::Empirical, 1000, 1024);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded
                assert!(
                    !compilation_result.bytecode.is_empty()
                        || compilation_result.constants.is_empty()
                );
                assert_eq!(compilation_result.sandboxed, false);
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_compile_let_binding() {
        let source = r#"
            (let ((x 5)
                  (y 10))
              (+ x y))
        "#;

        let result = compile(source, TrustTier::Empirical, 1000, 1024);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded
                assert!(
                    !compilation_result.bytecode.is_empty()
                        || compilation_result.constants.is_empty()
                );
                assert_eq!(compilation_result.sandboxed, false);
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_compile_conditional() {
        let source = r#"
            (if (> 5 3)
                "greater"
                "less")
        "#;

        let result = compile(source, TrustTier::Empirical, 1000, 1024);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded
                assert!(
                    !compilation_result.bytecode.is_empty()
                        || compilation_result.constants.is_empty()
                );
                assert_eq!(compilation_result.sandboxed, false);
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_compile_list_operations() {
        let source = r#"
            (cons 1 2)
            (list 1 2 3 4 5)
        "#;

        let result = compile(source, TrustTier::Empirical, 1000, 1024);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded
                assert!(
                    !compilation_result.bytecode.is_empty()
                        || compilation_result.constants.is_empty()
                );
                assert_eq!(compilation_result.sandboxed, false);
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_compile_with_different_trust_tiers() {
        let source = r#"
            (+ 1 2)
        "#;

        // Test with Formal tier
        let formal_result = compile(source, TrustTier::Formal, 1000, 1024);
        assert!(formal_result.is_ok());
        let formal_compilation = formal_result.unwrap();
        assert_eq!(formal_compilation.sandboxed, false);

        // Test with Verified tier
        let verified_result = compile(source, TrustTier::Verified, 1000, 1024);
        assert!(verified_result.is_ok());
        let verified_compilation = verified_result.unwrap();
        assert_eq!(verified_compilation.sandboxed, false);

        // Test with Empirical tier
        let empirical_result = compile(source, TrustTier::Empirical, 1000, 1024);
        assert!(empirical_result.is_ok());
        let empirical_compilation = empirical_result.unwrap();
        assert_eq!(empirical_compilation.sandboxed, false);

        // Test with Experimental tier
        let experimental_result = compile(source, TrustTier::Experimental, 1000, 1024);
        assert!(experimental_result.is_ok());
        let experimental_compilation = experimental_result.unwrap();
        assert_eq!(experimental_compilation.sandboxed, true); // Experimental should be sandboxed
    }

    #[test]
    fn test_compile_complex_expression() {
        let source = r#"
            (let ((identity (lambda (x) x))
                  (add-one (lambda (x) (+ x 1))))
              (let ((result ((compose add-one identity) 5)))
                (if (> result 5)
                    "success"
                    "failure")))
        "#;

        let result = compile(source, TrustTier::Empirical, 1000, 1024);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded
                assert!(
                    !compilation_result.bytecode.is_empty()
                        || compilation_result.constants.is_empty()
                );
                assert_eq!(compilation_result.sandboxed, false);
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_compile_resource_limits() {
        let source = r#"
            (+ 1 2)
        "#;

        let result = compile(source, TrustTier::Empirical, 5000, 2048);

        match result {
            Ok(compilation_result) => {
                // Verify resource limits are set correctly
                assert_eq!(compilation_result.step_limit, 5000);
                assert_eq!(compilation_result.memory_limit, 2048);
            }
            Err(e) => {
                panic!("Compilation failed: {:?}", e);
            }
        }
    }
}
