/// Test for consistency proof
use core_world::core_expr::{lam, var};
use core_world::proof_checker::{prove_consistency as prove_consistency_proof, verify_proof};

#[test]
fn test_consistency_proof() {
    // Test consistency proof generation and verification
    let proof = prove_consistency_proof();
    let dummy_expr = lam(var(0)); // Any expression for verification
    assert!(verify_proof(&proof, &dummy_expr));
}
