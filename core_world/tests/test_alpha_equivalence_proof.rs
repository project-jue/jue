/// Test for alpha equivalence proof
use core_world::core_expr::{lam, var};
use core_world::proof_checker::{prove_alpha_equivalence, verify_proof};

#[test]
fn test_alpha_equivalence_proof() {
    // Test α-equivalence proof generation and verification
    let expr1 = lam(var(0));
    let expr2 = lam(var(0)); // Should be α-equivalent

    let proof = prove_alpha_equivalence(expr1.clone(), expr2.clone()).unwrap();
    assert!(verify_proof(&proof, &expr1));
}
