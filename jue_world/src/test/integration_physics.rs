use super::*;
use crate::ast::{AstNode, Literal};
use crate::trust_tier::TrustTier;

#[test]
fn test_physics_world_compiler_creation() {
    let compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);
    assert_eq!(compiler.tier, TrustTier::Empirical);
    assert_eq!(compiler.capability_indices.len(), 0);
}

#[test]
fn test_literal_compilation() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let literal = AstNode::Literal(Literal::Int(42));
    let result = compiler.compile_to_physics(&literal);

    assert!(result.is_ok());
    let bytecode = result.unwrap();
    assert_eq!(bytecode.len(), 1);
    match &bytecode[0] {
        OpCode::Int(i) => assert_eq!(*i, 42),
        _ => panic!("Expected Int opcode"),
    }
}

#[test]
fn test_variable_compilation() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let variable = AstNode::Variable("x".to_string());
    let result = compiler.compile_to_physics(&variable);

    assert!(result.is_ok());
    let bytecode = result.unwrap();
    assert_eq!(bytecode.len(), 1);
    match &bytecode[0] {
        OpCode::Nil => assert!(true), // Placeholder
        _ => panic!("Expected placeholder opcode"),
    }
}

#[test]
fn test_simple_call_compilation() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let call = AstNode::Call {
        function: Box::new(AstNode::Variable("f".to_string())),
        arguments: vec![AstNode::Literal(Literal::Int(5))],
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&call);

    assert!(result.is_ok());
    let bytecode = result.unwrap();

    // Should have: argument, function, call
    assert!(bytecode.len() >= 3);
    assert!(matches!(bytecode[bytecode.len() - 1], OpCode::Call(1)));
}

#[test]
fn test_lambda_compilation() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let lambda = AstNode::Lambda {
        parameters: vec!["x".to_string()],
        body: Box::new(AstNode::Variable("x".to_string())),
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&lambda);

    assert!(result.is_ok());
    let bytecode = result.unwrap();

    // Should have body code followed by MakeClosure
    assert!(bytecode.len() >= 2);
    assert!(matches!(
        bytecode[bytecode.len() - 1],
        OpCode::MakeClosure(_, 1)
    ));
}

#[test]
fn test_trust_tier_compilation() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let trust_tier = AstNode::TrustTier {
        tier: ":empirical".to_string(),
        expression: Box::new(AstNode::Literal(Literal::Int(42))),
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&trust_tier);

    assert!(result.is_ok());
    let bytecode = result.unwrap();
    assert_eq!(bytecode.len(), 1);
    match &bytecode[0] {
        OpCode::Int(i) => assert_eq!(*i, 42),
        _ => panic!("Expected Int opcode"),
    }
}

#[test]
fn test_require_capability_compilation() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let require_cap = AstNode::RequireCapability {
        capability: "IoReadSensor".to_string(),
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&require_cap);

    assert!(result.is_ok());
    let bytecode = result.unwrap();
    assert_eq!(bytecode.len(), 0); // No bytecode generated for now
}

#[test]
fn test_has_capability_compilation() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let has_cap = AstNode::HasCapability {
        capability: "IoReadSensor".to_string(),
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&has_cap);

    assert!(result.is_ok());
    let bytecode = result.unwrap();
    assert_eq!(bytecode.len(), 1);
    match &bytecode[0] {
        OpCode::HasCap(_) => assert!(true),
        _ => panic!("Expected HasCap opcode"),
    }
}

#[test]
fn test_capability_index_management() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let cap1 = Capability::IoReadSensor;
    let cap2 = Capability::IoWriteActuator;

    let index1 = compiler.get_capability_index(&cap1);
    let index2 = compiler.get_capability_index(&cap2);
    let index1_again = compiler.get_capability_index(&cap1);

    assert_eq!(index1, 0);
    assert_eq!(index2, 1);
    assert_eq!(index1_again, 0); // Same index for same capability
    assert_eq!(compiler.capability_indices.len(), 2);
}

#[test]
fn test_unsupported_ast_node() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let macro_expansion = AstNode::MacroExpansion {
        name: "test".to_string(),
        arguments: vec![],
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&macro_expansion);

    assert!(result.is_err());
    match result {
        Err(CompilationError::InternalError(_)) => assert!(true),
        _ => panic!("Expected InternalError"),
    }
}

#[test]
fn test_invalid_capability() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let require_cap = AstNode::RequireCapability {
        capability: "InvalidCapability".to_string(),
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&require_cap);

    assert!(result.is_err());
    match result {
        Err(CompilationError::InternalError(_)) => assert!(true),
        _ => panic!("Expected InternalError"),
    }
}

#[test]
fn test_capability_not_allowed_in_tier() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);

    let require_cap = AstNode::RequireCapability {
        capability: "MacroUnsafe".to_string(),
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&require_cap);

    assert!(result.is_err());
    match result {
        Err(CompilationError::CapabilityError(_)) => assert!(true),
        _ => panic!("Expected CapabilityError"),
    }
}
