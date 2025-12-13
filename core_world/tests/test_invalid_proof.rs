/// Test for invalid proof
use core_world::core_expr::{lam, var};
use core_world::proof_checker::{verify_proof, Proof};

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
