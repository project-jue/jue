/// Test for eval empty environment
use core_world::core_expr::{lam, var};
use core_world::eval_relation::{eval_empty, EvalResult};

#[test]
fn test_eval_empty_environment() {
    // Test evaluation with empty environment
    let expr = lam(var(0));
    let result = eval_empty(expr);

    match result {
        EvalResult::Closure(closure) => {
            assert!(closure.env.is_empty());
            assert_eq!(closure.body, var(0));
        }
        _ => panic!("Expected closure"),
    }
}
