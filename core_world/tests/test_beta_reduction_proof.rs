/// Beta Reduction Proof Tests
use core_world::core_expr::{app, lam, var};
use core_world::proof_checker::{
    prove_alpha_equivalence, prove_beta_reduction, prove_evaluation, prove_normalization,
    verify_proof,
};

#[test]
fn test_beta_reduction_proof() {
    // Test β-reduction proof generation and verification
    let identity = lam(var(0));
    let y = var(1);
    let expr = app(identity, y);

    let proof = prove_beta_reduction(expr.clone()).unwrap();
    assert!(verify_proof(&proof, &expr));
}

#[test]
fn test_alpha_equivalence_proof() {
    // Test α-equivalence proof generation and verification
    let expr1 = lam(var(0));
    let expr2 = lam(var(0)); // Should be α-equivalent

    let proof = prove_alpha_equivalence(expr1.clone(), expr2.clone()).unwrap();
    assert!(verify_proof(&proof, &expr1));
}

#[test]
fn test_normalization_proof() {
    // Test normalization proof generation and verification
    let expr = app(lam(var(0)), var(1));

    let proof = prove_normalization(expr.clone());
    assert!(verify_proof(&proof, &expr));
}

#[test]
fn test_evaluation_proof() {
    // Test evaluation proof generation and verification
    let expr = app(lam(var(0)), var(1));

    let proof = prove_evaluation(expr.clone());
    assert!(verify_proof(&proof, &expr));
}

#[test]
fn test_consistency_proof() {
    // Test consistency proof generation and verification
    //TODO: Implement consistency proof test when the function is available
}
