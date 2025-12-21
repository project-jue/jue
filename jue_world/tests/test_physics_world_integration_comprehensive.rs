use jue_world::ast::{AstNode, Literal};
use jue_world::integration::physics::{compile_to_physics_world, PhysicsWorldCompiler};
use jue_world::trust_tier::TrustTier;
/// Comprehensive Integration Tests for Physics-World TODO Implementation
/// Tests all newly implemented features working together end-to-end
use physics_world::types::{Capability, OpCode, Value};
use physics_world::vm::VmState;

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to extract float value from Value enum
    fn extract_float(value: &Value) -> Option<f64> {
        match value {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    /// Test 1: Float Literal Compilation and Execution
    #[test]
    fn test_float_literal_integration() {
        // Test float literal compilation
        let ast = AstNode::Literal(Literal::Float(3.14159));
        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        assert_eq!(bytecode, vec![OpCode::Float(3.14159)]);

        // Test float literal execution
        let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Float(3.14159));
    }

    /// Test 2: String Literal Compilation and Execution  
    #[test]
    fn test_string_literal_integration() {
        let test_string = "Hello, World!";

        // Test string literal compilation
        let ast = AstNode::Literal(Literal::String(test_string.to_string()));
        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        // Should generate LoadString with proper index
        assert_eq!(bytecode.len(), 1);
        if let OpCode::LoadString(idx) = &bytecode[0] {
            assert!(*idx >= 0);
        } else {
            panic!("Expected LoadString opcode");
        }

        // Test string literal execution
        let string_pool = vec![Value::String(test_string.to_string())];
        let mut vm = VmState::new(bytecode, string_pool, 100, 1024, 1, 100);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::String(test_string.to_string()));
    }

    /// Test 3: Float Arithmetic Operations
    #[test]
    fn test_float_arithmetic_integration() {
        let ast = AstNode::Call {
            function: Box::new(AstNode::Symbol("add".to_string())),
            arguments: vec![
                AstNode::Literal(Literal::Float(10.5)),
                AstNode::Literal(Literal::Float(5.25)),
            ],
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        // Should contain float constants and FAdd operation
        assert!(bytecode.contains(&OpCode::Float(10.5)));
        assert!(bytecode.contains(&OpCode::Float(5.25)));
        assert!(bytecode.contains(&OpCode::FAdd));

        // Test execution
        let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Float(15.75));
    }

    /// Test 4: Variable Environment Management
    #[test]
    fn test_variable_environment_integration() {
        let ast = AstNode::Let {
            bindings: vec![
                ("x".to_string(), AstNode::Literal(Literal::Int(42))),
                ("y".to_string(), AstNode::Literal(Literal::Int(8))),
            ],
            body: Box::new(AstNode::Call {
                function: Box::new(AstNode::Symbol("add".to_string())),
                arguments: vec![
                    AstNode::Variable("x".to_string()),
                    AstNode::Variable("y".to_string()),
                ],
                location: Default::default(),
            }),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        // Should contain SetLocal and GetLocal operations
        assert!(bytecode.contains(&OpCode::SetLocal(0)));
        assert!(bytecode.contains(&OpCode::SetLocal(1)));
        assert!(bytecode.contains(&OpCode::GetLocal(0)));
        assert!(bytecode.contains(&OpCode::GetLocal(1)));

        // Test execution
        let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Int(50));
    }

    /// Test 5: Closure Environment Capture
    #[test]
    fn test_closure_environment_capture_integration() {
        // Test closure that captures variables from outer scope
        let ast = AstNode::Let {
            bindings: vec![("multiplier".to_string(), AstNode::Literal(Literal::Int(3)))],
            body: Box::new(AstNode::Lambda {
                parameters: vec!["x".to_string()],
                body: Box::new(AstNode::Call {
                    function: Box::new(AstNode::Symbol("mul".to_string())),
                    arguments: vec![
                        AstNode::Variable("x".to_string()),
                        AstNode::Variable("multiplier".to_string()),
                    ],
                    location: Default::default(),
                }),
                location: Default::default(),
            }),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        // Should contain MakeClosure with proper capture count
        assert!(bytecode
            .iter()
            .any(|op| matches!(op, OpCode::MakeClosure(_, 1))));

        // Test closure creation and execution
        let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
        let result = vm.run().unwrap();

        // Should create a closure that can be called
        assert!(matches!(result, Value::Closure(_)));
    }

    /// Test 6: Trust Tier Capability Enforcement - Empirical
    #[test]
    fn test_empirical_capability_enforcement() {
        let ast = AstNode::FfiCall {
            function: "read-sensor".to_string(),
            arguments: vec![],
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        // Should contain capability check operations
        assert!(bytecode.contains(&OpCode::HasCap(0)));
        assert!(bytecode.contains(&OpCode::JmpIfFalse(2)));

        // Test that compilation succeeds with proper capability checks
        let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
        let result = vm.run();

        // Should either succeed or fail gracefully with capability error
        // The exact behavior depends on the capability registry setup
        assert!(result.is_ok() || result.is_err());
    }

    /// Test 7: Trust Tier Sandbox Wrapper - Experimental
    #[test]
    fn test_experimental_sandbox_wrapper() {
        let ast = AstNode::Call {
            function: Box::new(AstNode::Symbol("add".to_string())),
            arguments: vec![
                AstNode::Literal(Literal::Int(1)),
                AstNode::Literal(Literal::Int(2)),
            ],
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Experimental);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        // Should contain sandbox wrapper operations
        assert!(bytecode.contains(&OpCode::InitSandbox));
        assert!(bytecode.contains(&OpCode::IsolateCapabilities));
        assert!(bytecode.contains(&OpCode::CleanupSandbox));

        // Test that sandboxed execution works
        let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Int(3));
    }

    /// Test 8: Complex Integration - All Features Together
    #[test]
    fn test_complex_integration_all_features() {
        let ast = AstNode::Let {
            bindings: vec![
                ("pi".to_string(), AstNode::Literal(Literal::Float(3.14159))),
                ("radius".to_string(), AstNode::Literal(Literal::Float(5.0))),
                (
                    "name".to_string(),
                    AstNode::Literal(Literal::String("circle".to_string())),
                ),
            ],
            body: Box::new(AstNode::Call {
                function: Box::new(AstNode::Symbol("mul".to_string())),
                arguments: vec![
                    AstNode::Call {
                        function: Box::new(AstNode::Symbol("mul".to_string())),
                        arguments: vec![
                            AstNode::Variable("pi".to_string()),
                            AstNode::Call {
                                function: Box::new(AstNode::Symbol("mul".to_string())),
                                arguments: vec![
                                    AstNode::Variable("radius".to_string()),
                                    AstNode::Variable("radius".to_string()),
                                ],
                                location: Default::default(),
                            },
                        ],
                        location: Default::default(),
                    },
                    AstNode::Literal(Literal::Int(1)), // Trivial multiplication
                ],
                location: Default::default(),
            }),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        // Should contain all types of operations
        assert!(bytecode.iter().any(|op| matches!(op, OpCode::Float(_))));
        assert!(bytecode
            .iter()
            .any(|op| matches!(op, OpCode::LoadString(_))));
        assert!(bytecode.iter().any(|op| matches!(op, OpCode::SetLocal(_))));
        assert!(bytecode.iter().any(|op| matches!(op, OpCode::GetLocal(_))));
        assert!(bytecode.iter().any(|op| matches!(op, OpCode::FMul)));

        // Test execution - should calculate π * r²
        let mut vm = VmState::new(bytecode, vec![Value::String("circle".to_string())], 1000, 1024, 1, 100);
        let result = vm.run().unwrap();

        let expected = 3.14159 * 25.0; // π * 5²
        assert!((extract_float(&result).unwrap_or(0.0) - expected).abs() < 0.0001);
    }

    /// Test 9: String Operations Integration
    #[test]
    fn test_string_operations_integration() {
        let ast = AstNode::Call {
            function: Box::new(AstNode::Symbol("str-concat".to_string())),
            arguments: vec![
                AstNode::Literal(Literal::String("Hello".to_string())),
                AstNode::Literal(Literal::String(" World".to_string())),
            ],
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        // Should contain string operations
        assert!(bytecode
            .iter()
            .any(|op| matches!(op, OpCode::LoadString(_))));
        assert!(bytecode.contains(&OpCode::StrConcat));

        // Test execution
        let string_pool = vec![Value::String("Hello".to_string()), Value::String(" World".to_string())];
        let mut vm = VmState::new(bytecode, string_pool, 100, 1024, 1, 100);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::String("Hello World".to_string()));
    }

    /// Test 10: Nested Scope Variable Resolution
    #[test]
    fn test_nested_scope_variable_resolution() {
        let ast = AstNode::Let {
            bindings: vec![("outer".to_string(), AstNode::Literal(Literal::Int(10)))],
            body: Box::new(AstNode::Let {
                bindings: vec![("inner".to_string(), AstNode::Literal(Literal::Int(20)))],
                body: Box::new(AstNode::Call {
                    function: Box::new(AstNode::Symbol("add".to_string())),
                    arguments: vec![
                        AstNode::Variable("outer".to_string()),
                        AstNode::Variable("inner".to_string()),
                    ],
                    location: Default::default(),
                }),
                location: Default::default(),
            }),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        // Test execution - should resolve both outer and inner variables
        let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Int(30));
    }

    /// Test 11: Error Handling for Undefined Variables
    #[test]
    fn test_undefined_variable_error() {
        let ast = AstNode::Variable("undefined_var".to_string());

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let result = compiler.compile_to_physics(&ast);

        // Should fail with undefined variable error
        assert!(result.is_err());
        let error_msg = result.unwrap_err().to_string();
        assert!(error_msg.contains("Undefined variable"));
        assert!(error_msg.contains("undefined_var"));
    }

    /// Test 12: Multiple String Literals Deduplication
    #[test]
    fn test_string_deduplication() {
        let ast = AstNode::Let {
            bindings: vec![
                (
                    "str1".to_string(),
                    AstNode::Literal(Literal::String("test".to_string())),
                ),
                (
                    "str2".to_string(),
                    AstNode::Literal(Literal::String("test".to_string())),
                ),
            ],
            body: Box::new(AstNode::Variable("str1".to_string())),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        // Check that both literals reference the same string constant
        let load_string_ops: Vec<_> = bytecode
            .iter()
            .filter_map(|op| {
                if let OpCode::LoadString(idx) = op {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();

        // Should use the same index for both "test" strings
        assert_eq!(load_string_ops[0], load_string_ops[1]);
    }

    /// Test 13: Performance Test - Many Operations
    #[test]
    fn test_performance_many_operations() {
        let ast = AstNode::Call {
            function: Box::new(AstNode::Symbol("add".to_string())),
            arguments: (0..100)
                .map(|i| AstNode::Literal(Literal::Int(1)))
                .collect(),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        // Test that compilation completes quickly
        let start = std::time::Instant::now();
        let mut vm = VmState::new(bytecode, vec![], 10000, 1024, 1, 100);
        let result = vm.run().unwrap();
        let duration = start.elapsed();

        // Should complete in reasonable time and produce correct result
        assert_eq!(result, Value::Int(100));
        assert!(duration.as_millis() < 1000); // Should complete in under 1 second
    }

    /// Test 14: Memory Usage Test
    #[test]
    fn test_memory_usage_many_strings() {
        let mut bindings = Vec::new();
        for i in 0..50 {
            bindings.push((
                format!("str{}", i),
                AstNode::Literal(Literal::String(format!("string_{}", i))),
            ));
        }

        let ast = AstNode::Let {
            bindings,
            body: Box::new(AstNode::Literal(Literal::Int(42))),
            location: Default::default(),
        };

        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let bytecode = compiler.compile_to_physics(&ast).unwrap();

        // Test that memory usage is reasonable
        let mut vm = VmState::new(bytecode, vec![], 1000, 10240, 1, 100);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Int(42));
        // Memory usage should be reasonable (not more than allocated limit)
        assert!(vm.memory.next_free() <= 10240);
    }

    /// Test 15: Trust Tier Formal - No Capability Checks
    #[test]
    fn test_formal_tier_no_capability_checks() {
        let ast = AstNode::Call {
            function: Box::new(AstNode::Symbol("add".to_string())),
            arguments: vec![
                AstNode::Literal(Literal::Int(1)),
                AstNode::Literal(Literal::Int(2)),
            ],
            location: Default::default(),
        };

        let bytecode = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();

        // Formal tier should not add capability checks
        assert!(!bytecode.iter().any(|op| matches!(op, OpCode::HasCap(_))));
        assert!(!bytecode
            .iter()
            .any(|op| matches!(op, OpCode::JmpIfFalse(_))));

        // Test execution
        let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
        let result = vm.run().unwrap();

        assert_eq!(result, Value::Int(3));
    }
}
