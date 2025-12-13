/// Test for normalization proof
use core_world::core_expr::{app, lam, var};
use core_world::proof_checker::{prove_normalization, verify_proof};

#[test]
fn test_normalization_proof() {
    // Test normalization proof generation and verification
    let expr = app(lam(var(0)), var(1));

    let proof = prove_normalization(expr.clone());
    assert!(verify_proof(&proof, &expr));
}
