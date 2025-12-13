/// Test for performance integration
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize};
use core_world::eval_relation::eval_empty;
use core_world::proof_checker::{prove_evaluation, prove_normalization, verify_proof};

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
