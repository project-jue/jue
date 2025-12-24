#[cfg(test)]
mod tests {
    use core_world::core_expr::CoreExpr;

    use crate::{
        ast::{AstNode, Literal},
        error::{CompilationError, SourceLocation},
        integration::CoreWorldCompiler,
    };

    #[test]
    fn test_core_world_compiler_creation() {
        let compiler = CoreWorldCompiler::new();
        assert!(matches!(compiler.location, SourceLocation { .. }));
    }

    #[test]
    fn test_literal_compilation() {
        let compiler = CoreWorldCompiler::new();

        let literal = AstNode::Literal(Literal::Int(42));
        let result = compiler.compile_to_core(&literal);

        // For now, this should return a placeholder Var expression
        assert!(result.is_ok());
        match result.unwrap() {
            CoreExpr::Var(_) => assert!(true),
            _ => panic!("Expected Var expression"),
        }
    }

    #[test]
    fn test_variable_compilation() {
        let compiler = CoreWorldCompiler::new();

        let variable = AstNode::Variable("x".to_string());
        let result = compiler.compile_to_core(&variable);

        assert!(result.is_ok());
        match result.unwrap() {
            CoreExpr::Var(_) => assert!(true),
            _ => panic!("Expected Var expression"),
        }
    }

    #[test]
    fn test_simple_call_compilation() {
        let compiler = CoreWorldCompiler::new();

        let call = AstNode::Call {
            function: Box::new(AstNode::Variable("f".to_string())),
            arguments: vec![AstNode::Variable("x".to_string())],
            location: SourceLocation::default(),
        };

        let result = compiler.compile_to_core(&call);

        assert!(result.is_ok());
        match result.unwrap() {
            CoreExpr::App(_, _) => assert!(true),
            _ => panic!("Expected App expression"),
        }
    }

    #[test]
    fn test_lambda_compilation() {
        let compiler = CoreWorldCompiler::new();

        let lambda = AstNode::Lambda {
            parameters: vec!["x".to_string()],
            body: Box::new(AstNode::Variable("x".to_string())),
            location: SourceLocation::default(),
        };

        let result = compiler.compile_to_core(&lambda);

        assert!(result.is_ok());
        match result.unwrap() {
            CoreExpr::Lam(_) => assert!(true),
            _ => panic!("Expected Lam expression"),
        }
    }

    #[test]
    fn test_nested_lambda_compilation() {
        let compiler = CoreWorldCompiler::new();

        let lambda = AstNode::Lambda {
            parameters: vec!["x".to_string(), "y".to_string()],
            body: Box::new(AstNode::Call {
                function: Box::new(AstNode::Variable("+".to_string())),
                arguments: vec![
                    AstNode::Variable("x".to_string()),
                    AstNode::Variable("y".to_string()),
                ],
                location: SourceLocation::default(),
            }),
            location: SourceLocation::default(),
        };

        let result = compiler.compile_to_core(&lambda);

        assert!(result.is_ok());
        // Should be a nested lambda structure
        match result.unwrap() {
            CoreExpr::Lam(inner) => match *inner {
                CoreExpr::Lam(_) => assert!(true),
                _ => panic!("Expected nested Lam expression"),
            },
            _ => panic!("Expected Lam expression"),
        }
    }

    #[test]
    fn test_unsupported_ast_node() {
        let compiler = CoreWorldCompiler::new();

        let trust_tier = AstNode::TrustTier {
            tier: ":formal".to_string(),
            expression: Box::new(AstNode::Literal(Literal::Int(42))),
            location: SourceLocation::default(),
        };

        let result = compiler.compile_to_core(&trust_tier);

        assert!(result.is_err());
        match result {
            Err(CompilationError::InternalError(_)) => assert!(true),
            _ => panic!("Expected InternalError"),
        }
    }

    #[test]
    fn test_proof_obligation_generation() {
        let compiler = CoreWorldCompiler::new();

        let core_expr = CoreExpr::Var(0);
        let result = compiler.generate_proof_obligations(&core_expr);

        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0); // No proofs generated yet
    }

    #[test]
    fn test_proof_verification() {
        let compiler = CoreWorldCompiler::new();

        let core_expr = CoreExpr::Var(0);
        let proofs = vec![];

        let result = compiler.verify_proofs(&core_expr, &proofs);

        assert!(result.is_ok());
    }
}
