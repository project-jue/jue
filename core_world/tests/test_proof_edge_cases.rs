/// Test for proof edge cases
use core_world::core_expr::{app, lam, var};
use core_world::proof_checker::{
    attach_proof, prove_beta_reduction, prove_consistency as prove_consistency_proof,
    prove_evaluation, verify_proof,
};

#[test]
fn test_proof_edge_cases() {
    // Test proof system edge cases
    let expr = lam(var(0));

    // Test consistency proof
    let consistency_proof = prove_consistency_proof();
    assert!(verify_proof(&consistency_proof, &expr));

    // Test that proven expressions work correctly
    let proof = prove_evaluation(expr.clone());
    let proven_expr = attach_proof(expr.clone(), proof);
    assert!(proven_expr.verify());

    // Test that different proof types work
    let beta_expr = app(lam(var(0)), var(1));
    let beta_proof = prove_beta_reduction(beta_expr.clone()).unwrap();
    assert!(verify_proof(&beta_proof, &beta_expr));
}
