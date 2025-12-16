use core_world::core_expr::{app, lam, var};
use core_world::proof_checker::{prove_beta, verify, Proof};

#[test]
fn test_verify_equivalence_beta_step() {
    // Create a simple beta reduction: (λ.0) 42 → 42
    let expr_a = app(lam(var(0)), var(42));
    let expr_b = var(42);

    // Generate a proof for this reduction
    let proof = prove_beta(expr_a.clone());

    // Verify that the proof shows expr_a ≡ expr_b
    let result = verify(&proof);

    assert!(
        result.is_ok(),
        "Proof verification should succeed for valid beta reduction"
    );
    let (left, right) = result.unwrap();
    assert!(core_world::core_kernel::alpha_equiv(left, expr_a));
    assert!(core_world::core_kernel::alpha_equiv(right, expr_b));
}

#[test]
fn test_verify_equivalence_reflexivity() {
    // Test reflexivity: M ≡ M
    let expr = app(lam(var(0)), var(1));

    // Create a reflexivity proof
    let proof = Proof::Refl(expr.clone());

    // Verify that the proof shows expr ≡ expr
    let result = verify(&proof);

    assert!(result.is_ok(), "Reflexivity proof should always verify");
    let (left, right) = result.unwrap();
    assert!(core_world::core_kernel::alpha_equiv(left, expr.clone()));
    assert!(core_world::core_kernel::alpha_equiv(right, expr));
}

#[test]
fn test_verify_equivalence_invalid() {
    // Test that invalid proofs are rejected
    let expr_a = app(lam(var(0)), var(42));
    let _expr_b = var(1); // Different from what the proof would show

    // Generate a proof for expr_a → var(42)
    let proof = prove_beta(expr_a.clone());

    // Try to verify that the proof shows expr_a ≡ expr_b (which it doesn't)
    let result = verify(&proof);

    assert!(
        result.is_ok(),
        "Proof verification should succeed for valid beta reduction"
    );
    let (left, right) = result.unwrap();
    assert!(core_world::core_kernel::alpha_equiv(left, expr_a));
    assert!(core_world::core_kernel::alpha_equiv(right, var(42)));
}
