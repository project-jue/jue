/// Core-Physics Integration Tests
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, prove_consistency};
use core_world::eval_relation::eval_empty;
use core_world::proof_checker::{
    prove_beta_reduction, prove_evaluation, prove_normalization, verify_proof,
};
use physics_world::primitives::{add, div_i32, mul};

#[test]
fn test_core_physics_integration() {
    // Test that core expressions can use physics primitives
    // This would involve creating core expressions that represent arithmetic operations
    // and verifying they work correctly with physics world primitives

    // Test basic arithmetic that could be represented in core expressions
    assert_eq!(add(5, 3), Ok(8));
    assert_eq!(mul(2, 4), Ok(8));
    assert_eq!(div_i32(10, 2), Ok(5));

    // Test that core expressions maintain consistency
    let expr = app(lam(var(0)), var(1));
    let _beta_reduced = beta_reduce(expr.clone());
    let _eval_result = eval_empty(expr.clone());

    assert!(verify_proof(
        &prove_beta_reduction(expr.clone()).unwrap(),
        &expr
    ));
    assert!(verify_proof(&prove_evaluation(expr.clone()), &expr));
    assert!(verify_proof(&prove_normalization(expr.clone()), &expr));

    // Verify consistency
    assert!(prove_consistency());
}
