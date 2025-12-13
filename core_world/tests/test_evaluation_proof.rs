/// Test for evaluation proof
use core_world::core_expr::{app, lam, var};
use core_world::proof_checker::{prove_evaluation, verify_proof};

#[test]
fn test_evaluation_proof() {
    // Test evaluation proof generation and verification
    let expr = app(lam(var(0)), var(1));

    let proof = prove_evaluation(expr.clone());
    assert!(verify_proof(&proof, &expr));
}
