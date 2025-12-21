use super::*;
use crate::core_expr::{app, lam, var};

#[test]
fn test_beta_step_serialization() {
    // Test BetaStep serialization roundtrip
    let redex = app(lam(var(0)), var(1));
    let proof = prove_beta(redex.clone());

    let serialized = serialize_proof(&proof);
    let deserialized = deserialize_proof(&serialized).unwrap();

    // Verify the deserialized proof is valid
    let result = verify(&deserialized);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert!(alpha_equiv(left, redex));
    assert_eq!(right, var(1));
}

#[test]
fn test_eta_step_serialization() {
    // Test EtaStep serialization roundtrip
    let redex = lam(app(var(1), var(0)));
    let proof = prove_eta(redex.clone()).unwrap();

    let serialized = serialize_proof(&proof);
    let deserialized = deserialize_proof(&serialized).unwrap();

    // Verify the deserialized proof is valid
    let result = verify(&deserialized);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert!(alpha_equiv(left, redex));
    assert_eq!(right, var(1));
}

#[test]
fn test_refl_serialization() {
    // Test Refl serialization roundtrip
    let expr = var(42);
    let proof = Proof::Refl(expr.clone());

    let serialized = serialize_proof(&proof);
    let deserialized = deserialize_proof(&serialized).unwrap();

    // Verify the deserialized proof is valid
    let result = verify(&deserialized);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert_eq!(left, expr);
    assert_eq!(right, expr);
}

#[test]
fn test_sym_serialization() {
    // Test Sym serialization roundtrip
    let redex = app(lam(var(0)), var(1));
    let beta_proof = prove_beta(redex.clone());
    let sym_proof = Proof::Sym(Box::new(beta_proof));

    let serialized = serialize_proof(&sym_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();

    // Verify the deserialized proof is valid
    let result = verify(&deserialized);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert_eq!(left, var(1));
    assert!(alpha_equiv(right, redex));
}

#[test]
fn test_trans_serialization() {
    // Test Trans serialization roundtrip
    let inner_redex = app(lam(var(0)), var(1));
    let outer_redex = app(lam(var(0)), inner_redex.clone());

    let proof1 = prove_beta(outer_redex.clone());
    let proof2 = prove_beta(inner_redex.clone());
    let trans_proof = Proof::Trans {
        proof_a: Box::new(proof1),
        proof_b: Box::new(proof2),
    };

    let serialized = serialize_proof(&trans_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();

    // Verify the deserialized proof is valid
    let result = verify(&deserialized);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert!(alpha_equiv(left, outer_redex));
    assert_eq!(right, var(1));
}

#[test]
fn test_cong_app_serialization() {
    // Test CongApp serialization roundtrip
    let f1 = lam(var(0));
    let a1 = var(1);

    let proof_f = Proof::Refl(f1.clone());
    let proof_a = Proof::Refl(a1.clone());
    let cong_app_proof = Proof::CongApp {
        proof_f: Box::new(proof_f),
        proof_a: Box::new(proof_a),
    };

    let serialized = serialize_proof(&cong_app_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();

    // Verify the deserialized proof is valid
    let result = verify(&deserialized);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();

    let expected_left = app(f1.clone(), a1.clone());
    let expected_right = app(f1.clone(), a1.clone());

    assert!(alpha_equiv(left, expected_left));
    assert!(alpha_equiv(right, expected_right));
}

#[test]
fn test_cong_lam_serialization() {
    // Test CongLam serialization roundtrip
    let m = var(0);
    let proof_b = Proof::Refl(m.clone());
    let cong_lam_proof = Proof::CongLam {
        proof_b: Box::new(proof_b),
    };

    let serialized = serialize_proof(&cong_lam_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();

    // Verify the deserialized proof is valid
    let result = verify(&deserialized);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();

    let expected_left = lam(m.clone());
    let expected_right = lam(m.clone());

    assert!(alpha_equiv(left, expected_left));
    assert!(alpha_equiv(right, expected_right));
}

#[test]
fn test_complex_proof_serialization() {
    // Test a complex proof with multiple nested constructs
    let redex = app(lam(var(0)), var(1));
    let beta_proof = prove_beta(redex.clone());
    let sym_proof = Proof::Sym(Box::new(beta_proof));

    let serialized = serialize_proof(&sym_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();

    // Verify the deserialized proof is valid
    let result = verify(&deserialized);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert_eq!(left, var(1));
    assert!(alpha_equiv(right, redex));
}

#[test]
fn test_empty_input_error() {
    let result = deserialize_proof(&[]);
    assert!(matches!(result, Err(ProofParseError::EmptyInput)));
}

#[test]
fn test_invalid_tag_error() {
    let invalid_tag = vec![0xFF];
    let result = deserialize_proof(&invalid_tag);
    assert!(matches!(result, Err(ProofParseError::InvalidTag(0xFF))));
}

#[test]
fn test_incomplete_data_error() {
    // Incomplete BetaStep - only tag, no data
    let incomplete_beta = vec![0x01];
    let result = deserialize_proof(&incomplete_beta);
    assert!(matches!(
        result,
        Err(ProofParseError::CoreExprParseError(_))
    ));
}
