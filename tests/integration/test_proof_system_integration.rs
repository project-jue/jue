/// Test for proof system integration
use core_world::core_expr::{app, lam, var};
use core_world::proof_checker::{
    prove_beta_reduction, prove_evaluation, prove_normalization, verify_proof,
};

#[test]
fn test_proof_system_integration() {
    // Test that the proof system correctly verifies kernel operations
    let expr = app(lam(var(0)), var(1));

    // Generate multiple proofs for the same expression
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    // All proofs should verify correctly
    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));

    // Test that proofs are consistent with actual operations
    let beta_reduced = core_world::core_kernel::beta_reduce(expr.clone());
    let normalized = core_world::core_kernel::normalize(expr.clone());

    // For simple expressions, β-reduction and normalization should produce the same result
    assert_eq!(beta_reduced, normalized);
}
