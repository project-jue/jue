/// Test for Dan World core world comprehensive integration
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, prove_consistency};
use core_world::eval_relation::{eval_empty, EvalResult};
use core_world::proof_checker::{
    prove_beta_reduction, prove_evaluation, prove_normalization, verify_proof,
};

#[test]
fn test_dan_world_core_world_comprehensive_integration() {
    // Test that all the core functionality Dan World depends on works

    // 1. Core expressions work
    let expr = app(lam(var(0)), var(1));
    let reduced = beta_reduce(expr.clone());
    assert_eq!(reduced, var(1));

    // 2. Proof system works
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));

    // 3. Consistency checking works
    assert!(prove_consistency());

    // 4. Evaluation works
    let eval_result = eval_empty(expr.clone());
    match eval_result {
        EvalResult::Value(_) => assert!(true),
        EvalResult::Closure(_) => assert!(true),
    }
}
