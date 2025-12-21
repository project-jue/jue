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

/// FFI Call Tests
#[test]
fn test_ffi_call_compilation_success() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let ffi_call = AstNode::FfiCall {
        function: "read-sensor".to_string(),
        arguments: vec![],
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&ffi_call);

    assert!(result.is_ok());
    let bytecode = result.unwrap();

    // Should have: capability check, conditional jump, host call, error handling
    assert!(bytecode.len() >= 5);

    // Check that we have the expected opcodes
    let has_cap_found = bytecode.iter().any(|op| matches!(op, OpCode::HasCap(_)));
    let jmp_if_false_found = bytecode
        .iter()
        .any(|op| matches!(op, OpCode::JmpIfFalse(_)));
    let host_call_found = bytecode
        .iter()
        .any(|op| matches!(op, OpCode::HostCall { .. }));

    assert!(has_cap_found, "Expected HasCap opcode in FFI call bytecode");
    assert!(
        jmp_if_false_found,
        "Expected JmpIfFalse opcode in FFI call bytecode"
    );
    assert!(
        host_call_found,
        "Expected HostCall opcode in FFI call bytecode"
    );
}

#[test]
fn test_ffi_call_with_arguments() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let ffi_call = AstNode::FfiCall {
        function: "write-actuator".to_string(),
        arguments: vec![AstNode::Literal(Literal::Int(42))],
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&ffi_call);

    assert!(result.is_ok());
    let bytecode = result.unwrap();

    // Should have: argument, capability check, conditional jump, host call, error handling
    assert!(bytecode.len() >= 6);

    // Check that the argument is compiled first
    assert!(matches!(bytecode[0], OpCode::Int(42)));
}

#[test]
fn test_ffi_call_capability_validation() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);

    let ffi_call = AstNode::FfiCall {
        function: "read-sensor".to_string(),
        arguments: vec![],
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&ffi_call);

    // Formal tier should not allow FFI calls
    assert!(result.is_err());
    match result {
        Err(CompilationError::CapabilityError(_)) => assert!(true),
        _ => panic!("Expected CapabilityError for FFI call in Formal tier"),
    }
}

#[test]
fn test_ffi_call_unknown_function() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let ffi_call = AstNode::FfiCall {
        function: "unknown-ffi-function".to_string(),
        arguments: vec![],
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&ffi_call);

    // Unknown FFI function should fail
    assert!(result.is_err());
    match result {
        Err(CompilationError::FfiError(_)) => assert!(true),
        _ => panic!("Expected FfiError for unknown FFI function"),
    }
}

#[test]
fn test_ffi_call_system_function_not_allowed() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let ffi_call = AstNode::FfiCall {
        function: "get-wall-clock".to_string(), // Requires SysClock capability
        arguments: vec![],
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&ffi_call);

    // Empirical tier doesn't grant SysClock capability
    assert!(result.is_err());
    match result {
        Err(CompilationError::CapabilityError(_)) => assert!(true),
        _ => panic!("Expected CapabilityError for system FFI call in Empirical tier"),
    }
}

#[test]
fn test_ffi_call_experimental_tier_allowed() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Experimental);

    let ffi_call = AstNode::FfiCall {
        function: "get-wall-clock".to_string(), // Requires SysClock capability
        arguments: vec![],
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&ffi_call);

    // Experimental tier should allow this
    assert!(result.is_ok());
    let bytecode = result.unwrap();
    assert!(bytecode.len() >= 5);
}

#[test]
fn test_ffi_call_capability_index_management() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    // First FFI call should create capability index
    let ffi_call1 = AstNode::FfiCall {
        function: "read-sensor".to_string(),
        arguments: vec![],
        location: SourceLocation::default(),
    };

    let result1 = compiler.compile_to_physics(&ffi_call1);
    assert!(result1.is_ok());

    // Second FFI call with same function should reuse capability index
    let ffi_call2 = AstNode::FfiCall {
        function: "read-sensor".to_string(), // Same function, same capability
        arguments: vec![],
        location: SourceLocation::default(),
    };

    let result2 = compiler.compile_to_physics(&ffi_call2);
    assert!(result2.is_ok());

    // Check that capability indices are managed correctly
    // Both calls use IoReadSensor, so only 1 capability index should be created
    assert_eq!(compiler.capability_indices.len(), 1);
}

#[test]
fn test_ffi_call_host_call_opcode_details() {
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let ffi_call = AstNode::FfiCall {
        function: "read-sensor".to_string(),
        arguments: vec![],
        location: SourceLocation::default(),
    };

    let result = compiler.compile_to_physics(&ffi_call);
    assert!(result.is_ok());
    let bytecode = result.unwrap();

    // Find the HostCall opcode
    let host_call_op = bytecode
        .iter()
        .find(|op| matches!(op, OpCode::HostCall { .. }));
    assert!(host_call_op.is_some());

    if let OpCode::HostCall {
        cap_idx,
        func_id,
        args,
    } = host_call_op.unwrap()
    {
        assert_eq!(*cap_idx, 0); // First capability index
        assert_eq!(*func_id, 0); // ReadSensor function ID
        assert_eq!(*args, 0); // No arguments
    } else {
        panic!("Expected HostCall opcode");
    }
}
