use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{alpha_equiv, beta_reduce, eta_reduce};
use core_world::proof_checker::{prove_beta, verify};

#[test]
fn test_correctness() {
    // Test 1: β-Reduction Correctness
    let term = app(lam(var(0)), var(1)); // (λ.0) 1
    let reduced = beta_reduce(term.clone());
    assert_eq!(reduced, var(1)); // Must reduce to '1'

    // Test 2: α-Equivalence
    let term1 = lam(var(0)); // λ.0
    let term2 = lam(var(0)); // λ.0 (same De Bruijn index)
    assert!(alpha_equiv(term1, term2));

    // Test 3: η-Reduction (if implemented)
    let term = lam(app(var(1), var(0))); // λ.(1 0) - this is λf.(f x) where f is var(1) and x is var(0)
    let eta_reduced = eta_reduce(term.clone());
    assert!(alpha_equiv(eta_reduced, var(1))); // Should be α-equivalent to '1' (the function f)

    // Test 4: Proof Verification
    let proof = prove_beta(app(lam(var(0)), var(42)));
    assert!(verify(&proof).is_ok()); // Must verify its own correctness
}
