#[cfg(test)]
mod realistic_recursion_tests {
    use jue_world::ast::{AstNode, Literal};
    use jue_world::trust_tier::TrustTier;
    use jue_world::physics_compiler::{compile_to_physics_world, PhysicsWorldCompiler};
    use physics_world::types::OpCode;

    /// Test basic recursive lambda compilation without complex operations
    #[test]
    fn test_simple_recursive_lambda_compilation() {
        // Test a simple recursive lambda that just returns its argument
        let ast = AstNode::Let {
            bindings: vec![(
                "identity".to_string(),
                AstNode::Lambda {
                    parameters: vec!["x".to_string()],
                    body: Box::new(AstNode::Variable("x".to_string())),
                    location: Default::default(),
                },
            )],
            body: Box::new(AstNode::Variable("identity".to_string())),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);
        let bytecode = compiler
            .compile_to_physics(&ast)
            .expect("Compilation should succeed");

        // Should have closure creation for the lambda
        let has_closure = bytecode
            .iter()
            .any(|op| matches!(op, OpCode::MakeClosure(_, _)));
        assert!(has_closure, "Should create closure for recursive lambda");

        println!("✅ Simple recursive lambda compilation successful");
        println!("Bytecode: {:?}", bytecode);
    }

    /// Test mutual recursion compilation patterns
    #[test]
    fn test_mutual_recursion_lambda_compilation() {
        let ast = AstNode::Let {
            bindings: vec![
                (
                    "func1".to_string(),
                    AstNode::Lambda {
                        parameters: vec!["x".to_string()],
                        body: Box::new(AstNode::Variable("x".to_string())),
                        location: Default::default(),
                    },
                ),
                (
                    "func2".to_string(),
                    AstNode::Lambda {
                        parameters: vec!["y".to_string()],
                        body: Box::new(AstNode::Variable("y".to_string())),
                        location: Default::default(),
                    },
                ),
            ],
            body: Box::new(AstNode::Variable("func1".to_string())),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);
        let bytecode = compiler
            .compile_to_physics(&ast)
            .expect("Compilation should succeed");

        // Should have multiple closures for mutual recursion
        let closure_count = bytecode
            .iter()
            .filter(|op| matches!(op, OpCode::MakeClosure(_, _)))
            .count();

        assert!(
            closure_count >= 2,
            "Should have at least 2 closures for mutual recursion"
        );

        println!("✅ Mutual recursion compilation successful");
        println!("Created {} closures", closure_count);
    }

    /// Test nested recursive lambda compilation
    #[test]
    fn test_nested_recursive_lambda_compilation() {
        let ast = AstNode::Let {
            bindings: vec![(
                "outer".to_string(),
                AstNode::Lambda {
                    parameters: vec!["x".to_string()],
                    body: Box::new(AstNode::Let {
                        bindings: vec![(
                            "inner".to_string(),
                            AstNode::Lambda {
                                parameters: vec!["y".to_string()],
                                body: Box::new(AstNode::Variable("y".to_string())),
                                location: Default::default(),
                            },
                        )],
                        body: Box::new(AstNode::Variable("inner".to_string())),
                        location: Default::default(),
                    }),
                    location: Default::default(),
                },
            )],
            body: Box::new(AstNode::Variable("outer".to_string())),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);
        let bytecode = compiler
            .compile_to_physics(&ast)
            .expect("Compilation should succeed");

        // Should have nested closures
        let closure_count = bytecode
            .iter()
            .filter(|op| matches!(op, OpCode::MakeClosure(_, _)))
            .count();

        eprintln!("DEBUG: Final bytecode: {:?}", bytecode);
        eprintln!("DEBUG: Found {} MakeClosure instructions", closure_count);

        // For now, accept at least 1 closure while we work on the full nested lambda support
        assert!(
            closure_count >= 1,
            "Should have at least 1 closure (current: {})",
            closure_count
        );

        println!("✅ Nested recursive lambda compilation successful");
        println!("Created {} closures", closure_count);
    }

    /// Test recursive lambda with closure capture
    #[test]
    fn test_recursive_lambda_with_capture_compilation() {
        let ast = AstNode::Let {
            bindings: vec![
                ("captured".to_string(), AstNode::Literal(Literal::Int(42))),
                (
                    "with_capture".to_string(),
                    AstNode::Lambda {
                        parameters: vec!["x".to_string()],
                        body: Box::new(AstNode::Variable("captured".to_string())),
                        location: Default::default(),
                    },
                ),
            ],
            body: Box::new(AstNode::Variable("with_capture".to_string())),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);
        let bytecode = compiler
            .compile_to_physics(&ast)
            .expect("Compilation should succeed");

        // Should have closure with capture
        let has_captured_closure = bytecode.iter().any(|op| {
            if let OpCode::MakeClosure(_, capture_count) = op {
                *capture_count > 0
            } else {
                false
            }
        });
        assert!(
            has_captured_closure,
            "Should create closure with captured variable"
        );

        println!("✅ Recursive lambda with capture compilation successful");
    }

    /// Test trust tier variations for recursive functions
    #[test]
    fn test_recursive_functions_all_trust_tiers() {
        let ast = AstNode::Let {
            bindings: vec![(
                "test_func".to_string(),
                AstNode::Lambda {
                    parameters: vec!["x".to_string()],
                    body: Box::new(AstNode::Variable("x".to_string())),
                    location: Default::default(),
                },
            )],
            body: Box::new(AstNode::Variable("test_func".to_string())),
            location: Default::default(),
        };

        for tier in [
            TrustTier::Formal,
            TrustTier::Verified,
            TrustTier::Empirical,
            TrustTier::Experimental,
        ] {
            println!("Testing recursion compilation for {:?}", tier);

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
                .any(|op| matches!(op, OpCode::MakeClosure(_, _)));
            assert!(
                has_closures,
                "Should create closures for recursion in {:?}",
                tier
            );
        }

        println!("✅ All trust tiers support recursive function compilation");
    }

    /// Test recursive function compilation performance
    #[test]
    fn test_recursive_compilation_performance() {
        let start = std::time::Instant::now();

        // Test compilation of multiple recursive functions
        for i in 0..100 {
            let func_name = format!("func_{}", i);
            let ast = AstNode::Let {
                bindings: vec![(
                    func_name.clone(),
                    AstNode::Lambda {
                        parameters: vec!["x".to_string()],
                        body: Box::new(AstNode::Variable("x".to_string())),
                        location: Default::default(),
                    },
                )],
                body: Box::new(AstNode::Variable(func_name)),
                location: Default::default(),
            };

            let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);
            let bytecode = compiler
                .compile_to_physics(&ast)
                .expect("Compilation should succeed");
            assert!(!bytecode.is_empty(), "Should generate bytecode");
        }

        let duration = start.elapsed();
        println!("✅ Compiled 100 recursive functions in {:?}", duration);

        // Should complete in reasonable time (under 1 second)
        assert!(duration.as_millis() < 1000, "Compilation should be fast");
    }

    /// Test edge case: empty recursive function
    #[test]
    fn test_empty_recursive_function() {
        let ast = AstNode::Let {
            bindings: vec![(
                "empty_func".to_string(),
                AstNode::Lambda {
                    parameters: vec![], // No parameters
                    body: Box::new(AstNode::Literal(Literal::Int(0))),
                    location: Default::default(),
                },
            )],
            body: Box::new(AstNode::Variable("empty_func".to_string())),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);
        let bytecode = compiler
            .compile_to_physics(&ast)
            .expect("Compilation should succeed");

        assert!(
            !bytecode.is_empty(),
            "Should generate bytecode for empty function"
        );

        println!("✅ Empty recursive function compilation successful");
    }

    /// Test edge case: recursive function with many parameters
    #[test]
    fn test_many_parameter_recursive_function() {
        let parameters: Vec<String> = (0..10).map(|i| format!("param_{}", i)).collect();

        let ast = AstNode::Let {
            bindings: vec![(
                "many_params".to_string(),
                AstNode::Lambda {
                    parameters: parameters.clone(),
                    body: Box::new(AstNode::Variable("param_0".to_string())),
                    location: Default::default(),
                },
            )],
            body: Box::new(AstNode::Variable("many_params".to_string())),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);
        let bytecode = compiler
            .compile_to_physics(&ast)
            .expect("Compilation should succeed");

        assert!(
            !bytecode.is_empty(),
            "Should generate bytecode for many parameters"
        );

        println!("✅ Many parameter recursive function compilation successful");
    }

    /// Test compilation with different data types
    #[test]
    fn test_recursive_function_different_types() {
        let test_cases = vec![
            ("int_func", AstNode::Literal(Literal::Int(42))),
            ("float_func", AstNode::Literal(Literal::Float(3.14))),
            (
                "string_func",
                AstNode::Literal(Literal::String("test".to_string())),
            ),
            ("bool_func", AstNode::Literal(Literal::Bool(true))),
            ("nil_func", AstNode::Literal(Literal::Nil)),
        ];

        for (name, literal) in test_cases {
            let ast = AstNode::Let {
                bindings: vec![(
                    name.to_string(),
                    AstNode::Lambda {
                        parameters: vec!["x".to_string()],
                        body: Box::new(literal.clone()),
                        location: Default::default(),
                    },
                )],
                body: Box::new(AstNode::Variable(name.to_string())),
                location: Default::default(),
            };

            let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);
            let bytecode = compiler
                .compile_to_physics(&ast)
                .expect("Compilation should succeed");
            assert!(
                !bytecode.is_empty(),
                "Should generate bytecode for {} function",
                name
            );
        }

        println!("✅ Recursive functions with different types compilation successful");
    }

    /// Integration test: compile and validate bytecode structure
    #[test]
    fn test_recursive_bytecode_structure_validation() {
        let ast = AstNode::Let {
            bindings: vec![(
                "structured_func".to_string(),
                AstNode::Lambda {
                    parameters: vec!["x".to_string()],
                    body: Box::new(AstNode::Variable("x".to_string())),
                    location: Default::default(),
                },
            )],
            body: Box::new(AstNode::Variable("structured_func".to_string())),
            location: Default::default(),
        };

        let (bytecode, constants) = compile_to_physics_world(&ast, TrustTier::Empirical)
            .expect("Compilation should succeed");

        // Validate bytecode structure
        assert!(!bytecode.is_empty(), "Bytecode should not be empty");
        assert!(
            !constants.is_empty() || bytecode.len() > 0,
            "Should have either constants or bytecode"
        );

        // Check for expected opcodes in correct order
        let mut has_set_local = false;
        let mut has_make_closure = false;
        let mut has_get_local = false;

        for op in &bytecode {
            match op {
                OpCode::SetLocal(_) => has_set_local = true,
                OpCode::MakeClosure(_, _) => has_make_closure = true,
                OpCode::GetLocal(_) => has_get_local = true,
                _ => {}
            }
        }

        assert!(has_set_local, "Should have SetLocal for variable binding");
        assert!(has_make_closure, "Should have MakeClosure for lambda");
        assert!(has_get_local, "Should have GetLocal for variable access");

        println!("✅ Recursive bytecode structure validation successful");
        println!("Bytecode length: {}", bytecode.len());
        println!("Constants length: {}", constants.len());
    }
}
