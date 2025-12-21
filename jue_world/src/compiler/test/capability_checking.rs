
#[test]
fn test_capability_check_insertion() {
    // Create a simple AST node that requires capability checks
    let ast = crate::ast::AstNode::RequireCapability {
        capability: "io-read-sensor".to_string(),
        location: SourceLocation::default(),
    };

    // Test with Empirical tier (should insert checks)
    let (bytecode, audit) =
        insert_capability_checks(vec![OpCode::Int(42)], &ast, TrustTier::Empirical);

    // Should have inserted a HasCap check
    assert_eq!(bytecode.len(), 2);
    assert!(matches!(bytecode[0], OpCode::HasCap(0)));
    assert!(matches!(bytecode[1], OpCode::Int(42)));

    // Should have created an audit entry
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].capability, Capability::IoReadSensor);
    assert_eq!(audit[0].check_type, CheckType::Runtime);

    // Test with Formal tier (should not insert checks)
    let (bytecode_formal, audit_formal) =
        insert_capability_checks(vec![OpCode::Int(42)], &ast, TrustTier::Formal);

    // Should not have inserted any checks
    assert_eq!(bytecode_formal.len(), 1);
    assert!(matches!(bytecode_formal[0], OpCode::Int(42)));
    assert!(audit_formal.is_empty());
}

#[test]
fn test_ffi_capability_requirement() {
    // Create an FFI call AST node
    let ast = crate::ast::AstNode::FfiCall {
        function: "some_function".to_string(),
        arguments: vec![],
        location: SourceLocation::default(),
    };

    // Test with Experimental tier
    let (bytecode, audit) =
        insert_capability_checks(vec![OpCode::Int(42)], &ast, TrustTier::Experimental);

    // Should have inserted a HasCap check for MacroUnsafe
    assert_eq!(bytecode.len(), 2);
    assert!(matches!(bytecode[0], OpCode::HasCap(0)));
    assert!(matches!(bytecode[1], OpCode::Int(42)));

    // Should have created an audit entry
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].capability, Capability::MacroUnsafe);
    assert_eq!(audit[0].check_type, CheckType::Runtime);
}

#[test]
fn test_macro_capability_requirement() {
    // Create a macro definition AST node
    let ast = crate::ast::AstNode::MacroDefinition {
        name: "test-macro".to_string(),
        parameters: vec![],
        body: Box::new(crate::ast::AstNode::Literal(crate::ast::Literal::Int(42))),
        capabilities: vec!["macro-hygienic".to_string()],
        tier: "empirical".to_string(),
        location: SourceLocation::default(),
    };

    // Test with Empirical tier
    let (bytecode, audit) =
        insert_capability_checks(vec![OpCode::Int(42)], &ast, TrustTier::Empirical);

    // Should have inserted a HasCap check for MacroHygienic
    assert_eq!(bytecode.len(), 2);
    assert!(matches!(bytecode[0], OpCode::HasCap(0)));
    assert!(matches!(bytecode[1], OpCode::Int(42)));

    // Should have created an audit entry
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].capability, Capability::MacroHygienic);
    assert_eq!(audit[0].check_type, CheckType::Runtime);
}

#[test]
fn test_ffi_capability_analysis_integration() {
    use crate::compiler::capability_analysis::{analyze_capabilities, validate_ffi_call};

    // Test FFI capability analysis for known functions
    let ast = crate::ast::AstNode::FfiCall {
        function: "read-sensor".to_string(),
        arguments: vec![],
        location: SourceLocation::default(),
    };

    // Should detect IoReadSensor capability
    let capabilities = analyze_capabilities(&ast).unwrap();
    assert_eq!(capabilities.len(), 1);
    assert_eq!(capabilities[0], Capability::IoReadSensor);

    // Test validation with Empirical tier (should pass)
    let result = validate_ffi_call(TrustTier::Empirical, "read-sensor");
    assert!(result.is_ok());

    // Test validation with Formal tier (should fail)
    let result = validate_ffi_call(TrustTier::Formal, "read-sensor");
    assert!(result.is_err());
}

#[test]
fn test_ffi_capability_mapping() {
    use crate::compiler::capability_analysis::get_ffi_function_capability;

    // Test known FFI function mappings
    assert_eq!(
        get_ffi_function_capability("read-sensor"),
        Some(Capability::IoReadSensor)
    );
    assert_eq!(
        get_ffi_function_capability("write-actuator"),
        Some(Capability::IoWriteActuator)
    );
    assert_eq!(
        get_ffi_function_capability("get-wall-clock"),
        Some(Capability::SysClock)
    );
    assert_eq!(
        get_ffi_function_capability("network-send"),
        Some(Capability::IoNetwork)
    );
    assert_eq!(
        get_ffi_function_capability("persist-write"),
        Some(Capability::IoPersist)
    );

    // Test unknown FFI function (should return None)
    assert_eq!(get_ffi_function_capability("unknown-function"), None);
}

#[test]
fn test_ffi_trust_tier_validation() {
    use crate::compiler::capability_analysis::validate_ffi_call;

    // Test FFI calls with different trust tiers

    // Empirical tier should allow I/O capabilities
    assert!(validate_ffi_call(TrustTier::Empirical, "read-sensor").is_ok());
    assert!(validate_ffi_call(TrustTier::Empirical, "write-actuator").is_ok());

    // Empirical tier should NOT allow system capabilities
    assert!(validate_ffi_call(TrustTier::Empirical, "get-wall-clock").is_err());
    assert!(validate_ffi_call(TrustTier::Empirical, "spawn-actor").is_err());

    // Experimental tier should allow most capabilities
    assert!(validate_ffi_call(TrustTier::Experimental, "read-sensor").is_ok());
    assert!(validate_ffi_call(TrustTier::Experimental, "write-actuator").is_ok());
    assert!(validate_ffi_call(TrustTier::Experimental, "get-wall-clock").is_ok());
    assert!(validate_ffi_call(TrustTier::Experimental, "spawn-actor").is_ok());

    // Formal tier should not allow any FFI capabilities
    assert!(validate_ffi_call(TrustTier::Formal, "read-sensor").is_err());
    assert!(validate_ffi_call(TrustTier::Formal, "write-actuator").is_err());
}

#[test]
fn test_ffi_capability_check_generation() {
    // Test capability check generation for different FFI functions

    // Test read-sensor (IoReadSensor)
    let ast = crate::ast::AstNode::FfiCall {
        function: "read-sensor".to_string(),
        arguments: vec![],
        location: SourceLocation::default(),
    };

    let (bytecode, audit) =
        insert_capability_checks(vec![OpCode::Int(42)], &ast, TrustTier::Empirical);

    assert_eq!(bytecode.len(), 2);
    assert!(matches!(bytecode[0], OpCode::HasCap(0)));
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].capability, Capability::IoReadSensor);

    // Test get-wall-clock (SysClock) - should fail for Empirical tier
    let ast = crate::ast::AstNode::FfiCall {
        function: "get-wall-clock".to_string(),
        arguments: vec![],
        location: SourceLocation::default(),
    };

    let (bytecode, audit) =
        insert_capability_checks(vec![OpCode::Int(42)], &ast, TrustTier::Empirical);

    // Should still generate capability check even though it will fail at runtime
    assert_eq!(bytecode.len(), 2);
    assert!(matches!(bytecode[0], OpCode::HasCap(0)));
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].capability, Capability::SysClock);
}

#[test]
fn test_ffi_capability_unknown_function() {
    // Test unknown FFI function (should default to MacroUnsafe)

    let ast = crate::ast::AstNode::FfiCall {
        function: "unknown-ffi-function".to_string(),
        arguments: vec![],
        location: SourceLocation::default(),
    };

    let (bytecode, audit) =
        insert_capability_checks(vec![OpCode::Int(42)], &ast, TrustTier::Experimental);

    assert_eq!(bytecode.len(), 2);
    assert!(matches!(bytecode[0], OpCode::HasCap(0)));
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].capability, Capability::MacroUnsafe);
}

#[test]
fn test_ffi_capability_complex_expression() {
    // Test FFI call in a complex expression

    let ast = crate::ast::AstNode::Let {
        bindings: vec![(
            "sensor-val".to_string(),
            crate::ast::AstNode::FfiCall {
                function: "read-sensor".to_string(),
                arguments: vec![],
                location: SourceLocation::default(),
            },
        )],
        body: Box::new(crate::ast::AstNode::Symbol("sensor-val".to_string())),
        location: SourceLocation::default(),
    };

    let (bytecode, audit) =
        insert_capability_checks(vec![OpCode::Int(42)], &ast, TrustTier::Empirical);

    // Should detect the FFI call in the let binding
    assert_eq!(bytecode.len(), 2);
    assert!(matches!(bytecode[0], OpCode::HasCap(0)));
    assert_eq!(audit.len(), 1);
    assert_eq!(audit[0].capability, Capability::IoReadSensor);
}
