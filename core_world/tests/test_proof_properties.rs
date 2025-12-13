/// Proof Properties Tests
use core_world::core_expr::{app, lam, var};
use core_world::proof_checker::{
    attach_proof, prove_beta_reduction, prove_evaluation, prove_normalization, verify_proof, Proof,
};

#[test]
fn test_proven_expr() {
    // Test proven expression creation and verification
    let expr = lam(var(0));
    let proof = prove_evaluation(expr.clone());
    let proven_expr = attach_proof(expr.clone(), proof);

    assert!(proven_expr.verify());
    assert_eq!(proven_expr.expr, expr);
}

#[test]
fn test_composite_proof() {
    // Test composite proof construction and verification
    let expr = app(lam(var(0)), var(1));

    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());

    let composite_proof = Proof::Composite {
        proofs: vec![beta_proof, eval_proof],
        conclusion: "β-reduction and evaluation".to_string(),
    };

    assert!(verify_proof(&composite_proof, &expr));
}

#[test]
fn test_invalid_proof() {
    // Test invalid proof detection
    let expr1 = lam(var(0));
    let expr2 = lam(var(1)); // Different expressions

    // This should fail because the expressions are not α-equivalent
    let proof = Proof::AlphaEquivalence {
        expr1: expr1.clone(),
        expr2: expr2.clone(),
    };

    assert!(!verify_proof(&proof, &expr1));
}

#[test]
fn test_proof_system_integration() {
    // Test integration between proof system and core operations
    let expr = app(lam(var(0)), var(1));

    // Generate multiple proofs for the same expression
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    // All proofs should verify correctly
    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));

    // Create a composite proof
    let composite_proof = Proof::Composite {
        proofs: vec![beta_proof, eval_proof, norm_proof],
        conclusion: "Complete proof verification".to_string(),
    };

    assert!(verify_proof(&composite_proof, &expr));
}
