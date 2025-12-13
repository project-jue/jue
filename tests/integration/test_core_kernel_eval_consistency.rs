/// Test for core kernel eval consistency
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::beta_reduce;
use core_world::eval_relation::{eval_empty, EvalResult};

#[test]
fn test_core_kernel_eval_consistency() {
    // Test that β-reduction and evaluation produce consistent results
    let expr = app(lam(var(0)), var(1));

    // β-reduction result
    let beta_reduced = beta_reduce(expr.clone());

    // Evaluation result
    let eval_result = eval_empty(expr.clone());
    if let EvalResult::Value(eval_value) = eval_result {
        // Both should produce the same result (var(1))
        assert_eq!(beta_reduced, eval_value);
    } else {
        panic!("Expected value result from evaluation");
    }
}
