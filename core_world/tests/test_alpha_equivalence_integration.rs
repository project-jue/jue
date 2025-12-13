/// Alpha Equivalence Integration Tests
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{alpha_equiv, beta_reduce, normalize};
use core_world::eval_relation::EvalResult;
use core_world::eval_relation::{eval_empty, is_normal_form};

use core_world::proof_checker::{
    prove_beta_reduction, prove_evaluation, prove_normalization, verify_proof,
};

#[test]
fn test_alpha_equivalence_integration() {
    // Test α-equivalence integration with other components
    let expr1 = lam(var(0));
    let expr2 = lam(var(0)); // Should be α-equivalent

    // Test α-equivalence
    assert!(alpha_equiv(expr1.clone(), expr2.clone()));

    // Test that α-equivalent expressions behave the same
    let app1 = app(expr1.clone(), var(1));
    let app2 = app(expr2.clone(), var(1));

    let result1 = normalize(app1);
    let result2 = normalize(app2);

    assert_eq!(result1, result2);
}

#[test]
fn test_performance_integration() {
    // Test performance with reasonably complex expressions
    let mut expr = var(0);

    // Build a complex expression
    for _i in 0..10 {
        expr = app(lam(var(0)), expr);
    }

    // All operations should complete without issues
    let _beta_result = beta_reduce(expr.clone());
    let _norm_result = normalize(expr.clone());
    let _eval_result = eval_empty(expr.clone());

    // Normalization should fully reduce the expression
    // Beta reduction does one step, normalization does multiple steps
    // So they may not be equal, but both should be valid
    assert!(verify_proof(&prove_normalization(expr.clone()), &expr));
    assert!(verify_proof(&prove_evaluation(expr.clone()), &expr));

    // For a simpler case, test consistency
    let simple_expr = app(lam(var(0)), var(1));
    let simple_beta = beta_reduce(simple_expr.clone());
    let simple_norm = normalize(simple_expr.clone());

    assert_eq!(simple_beta, simple_norm);
}

#[test]
fn test_edge_case_integration() {
    // Test edge cases in integration
    let identity = lam(var(0));

    // Test identity function properties
    let app_expr = app(identity.clone(), var(1));

    // All operations should handle this correctly
    let beta_result = beta_reduce(app_expr.clone());
    let norm_result = normalize(app_expr.clone());
    let eval_result = eval_empty(app_expr.clone());

    // All should produce var(1)
    assert_eq!(beta_result, var(1));
    assert_eq!(norm_result, var(1));

    if let EvalResult::Value(eval_value) = eval_result {
        assert_eq!(eval_value, var(1));
    } else {
        panic!("Expected value result");
    }

    // Proofs should work
    let proof = prove_beta_reduction(app_expr.clone()).unwrap();
    assert!(verify_proof(&proof, &app_expr));
}

#[test]
fn test_normal_form_consistency() {
    // Test consistency in normal form detection
    let expr = lam(var(0));

    // Test that all components agree on normal forms
    let eval_result = eval_empty(expr.clone());
    let is_normal = is_normal_form(&eval_result);

    // Identity function should be in normal form
    assert!(is_normal);

    // Test with a reducible expression
    let app_expr = app(lam(var(0)), var(1));
    let eval_result = eval_empty(app_expr.clone());
    let is_normal = is_normal_form(&eval_result);

    // After evaluation, should be in normal form
    assert!(is_normal);
}
