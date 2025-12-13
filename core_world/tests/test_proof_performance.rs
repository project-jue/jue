/// Test for proof performance
use core_world::core_expr::{app, lam, var};
use core_world::proof_checker::{prove_evaluation, prove_normalization, verify_proof};

#[test]
fn test_proof_performance() {
    // Test proof system performance with reasonably complex expressions
    let mut expr = var(0);

    // Build a reasonably complex expression
    for _i in 0..10 {
        expr = app(lam(var(0)), expr);
    }

    // Should be able to generate proofs without issues
    let proof = prove_normalization(expr.clone());
    assert!(verify_proof(&proof, &expr));

    let eval_proof = prove_evaluation(expr.clone());
    assert!(verify_proof(&eval_proof, &expr));
}
