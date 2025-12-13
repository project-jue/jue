/// Core-Jue Integration Tests
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::beta_reduce;
use core_world::eval_relation::{eval_empty, EvalResult};
use core_world::proof_checker::{
    prove_beta_reduction, prove_evaluation, prove_normalization, verify_proof,
};

#[test]
fn test_core_jue_integration() {
    // This test would involve:
    // 1. Creating Jue expressions
    // 2. Compiling them to CoreExpr
    // 3. Verifying the core expressions work correctly
    // 4. Generating and verifying proofs

    // For now, we'll test the core components that Jue would use
    let identity = lam(var(0));
    let y = var(1);
    let expr = app(identity, y);

    // Test beta reduction
    let reduced = beta_reduce(expr.clone());
    assert_eq!(reduced, var(1));

    // Test evaluation
    let eval_result = eval_empty(expr.clone());
    assert_eq!(eval_result, EvalResult::Value(var(1)));

    // Test proof generation
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));
}
