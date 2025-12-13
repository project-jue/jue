/// Test for eval normalization consistency
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;
use core_world::eval_relation::{eval_empty, EvalResult};

#[test]
fn test_eval_normalization_consistency() {
    // Test that evaluation and normalization produce consistent results
    let expr = app(lam(var(0)), var(1));

    // Evaluation result
    let eval_result = eval_empty(expr.clone());
    if let EvalResult::Value(eval_value) = eval_result {
        // Normalization result
        let normalized = normalize(expr.clone());

        // Both should produce the same result
        assert_eq!(eval_value, normalized);
    } else {
        panic!("Expected value result from evaluation");
    }
}
