use crate::compiler::compiler::compile;
use crate::parser::parse;
use crate::trust_tier::TrustTier;
use physics_world::types::{Capability, OpCode};

#[test]
fn test_simple_ffi_call() {
    // Test simple FFI call
    let jue_code = "
        (:empirical
            (ffi-call 'read-sensor))
    ";

    // Compile with Empirical tier
    let result = compile(jue_code, TrustTier::Empirical, 1000, 1024);

    // Debug output
    if let Err(e) = &result {
        println!("Simple FFI compilation error: {:?}", e);
    }

    // Should succeed and include capability checks
    assert!(result.is_ok());
    let compilation_result = result.unwrap();

    // Verify capability requirements were analyzed
    assert!(compilation_result
        .required_capabilities
        .contains(&Capability::IoReadSensor));
    assert!(compilation_result
        .granted_capabilities
        .contains(&Capability::IoReadSensor));

    // Verify capability audit trail was created
    assert!(!compilation_result.capability_audit.is_empty());

    // Verify bytecode contains capability checks
    assert!(compilation_result
        .bytecode
        .iter()
        .any(|op| matches!(op, OpCode::HasCap(_))));
}

#[test]
fn test_capability_violation() {
    // Test Jue code that requires a capability not available in the tier
    let jue_code = "
        (:formal
            (require-capability 'io-read-sensor)
            (ffi-call 'read-sensor))
    ";

    // Compile with Formal tier (which doesn't grant IoReadSensor)
    let result = compile(jue_code, TrustTier::Formal, 1000, 1024);

    // Should fail with capability violation
    assert!(result.is_err());
}

#[test]
fn test_formal_tier_no_runtime_checks() {
    // Test Jue code in Formal tier
    let jue_code = "
        (:formal
            (+ 1 2))
    ";

    // Compile with Formal tier
    let result = compile(jue_code, TrustTier::Formal, 1000, 1024);

    assert!(result.is_ok());
    let compilation_result = result.unwrap();

    // Formal tier should not insert runtime capability checks
    assert!(compilation_result.capability_audit.is_empty());
    assert!(compilation_result
        .bytecode
        .iter()
        .all(|op| !matches!(op, OpCode::HasCap(_))));
}

#[test]
fn test_experimental_tier_sandboxed() {
    // Test Jue code in Experimental tier - simplified to just FFI call
    let jue_code = "
        (:experimental
            (ffi-call 'read-sensor))
    ";

    // First, test parsing directly
    let parse_result = parse(jue_code);
    if let Err(e) = &parse_result {
        println!("Parse error: {:?}", e);
    } else {
        println!("Parse successful: {:?}", parse_result);
    }

    // Compile with Experimental tier
    let result = compile(jue_code, TrustTier::Experimental, 1000, 1024);

    // Debug output
    if let Err(e) = &result {
        println!("Experimental tier compilation error: {:?}", e);
    }

    assert!(result.is_ok());
    let compilation_result = result.unwrap();

    // Experimental tier should be sandboxed
    assert!(compilation_result.sandboxed);

    // Should have capability checks
    assert!(!compilation_result.capability_audit.is_empty());
    assert!(compilation_result
        .bytecode
        .iter()
        .any(|op| matches!(op, OpCode::HasCap(_))));
}

#[test]
fn test_ffi_capability_integration() {
    // Test FFI call capability analysis integration
    let jue_code = "
        (:empirical
            (ffi-call 'read-sensor))
    ";

    // Compile with Empirical tier
    let result = compile(jue_code, TrustTier::Empirical, 1000, 1024);

    // Debug output
    if let Err(e) = &result {
        println!("FFI compilation error: {:?}", e);
    }

    // Should succeed
    assert!(result.is_ok());
    let compilation_result = result.unwrap();

    // Verify FFI capability was detected
    assert!(compilation_result
        .required_capabilities
        .contains(&Capability::IoReadSensor));

    // Verify capability audit trail was created
    assert!(!compilation_result.capability_audit.is_empty());
    assert_eq!(
        compilation_result.capability_audit[0].capability,
        Capability::IoReadSensor
    );
}

#[test]
fn test_ffi_capability_violation() {
    // Test FFI call that violates trust tier capabilities
    let jue_code = "
        (:formal
            (ffi-call 'read-sensor))
    ";

    // Compile with Formal tier (which doesn't grant IoReadSensor)
    let result = compile(jue_code, TrustTier::Formal, 1000, 1024);

    // Debug output
    if let Err(e) = &result {
        println!("FFI violation error: {:?}", e);
    }

    // Should fail with capability violation
    assert!(result.is_err());
}
