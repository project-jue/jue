/// Test for composite proof
use core_world::core_expr::{app, lam, var};
use core_world::proof_checker::{prove_beta_reduction, prove_evaluation, verify_proof, Proof};

#[test]
fn test_composite_proof() {
    // Test composite proof construction and verification
    let expr = app(lam(var(0)), var(1));

    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());

    let composite_proof = Proof::Composite {
        proofs: vec![beta_proof, eval_proof],
        conclusion: "β-reduction and evaluation".to_string(),
    };

    assert!(verify_proof(&composite_proof, &expr));
}
