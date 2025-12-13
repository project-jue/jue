/// Specific Integration Cases Tests
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::beta_reduce;
use core_world::eval_relation::{eval_empty, EvalResult};

#[test]
fn test_specific_integration_case() {
    // Test the specific failing case: app(lam(app(var(1), var(0))), lam(var(0)))
    // This is: (λx.(1 0)) (λy.0)

    let expr = app(lam(app(var(1), var(0))), lam(var(0)));

    println!("Expression: {}", expr);

    // Beta reduction result
    let beta_result = beta_reduce(expr.clone());
    println!("Beta reduction result: {}", beta_result);

    // Evaluation result
    let eval_result = eval_empty(expr.clone());
    println!("Evaluation result: {:?}", eval_result);

    // Extract the value from evaluation result
    if let EvalResult::Value(eval_value) = eval_result {
        println!("Evaluation value: {}", eval_value);
        println!("Beta result: {}", beta_result);
        println!("Results match: {}", beta_result == eval_value);

        // They should be equal
        assert_eq!(
            beta_result, eval_value,
            "Beta reduction and evaluation should produce the same result"
        );
    } else {
        panic!("Expected value result from evaluation");
    }
}

#[test]
fn test_simple_case_that_works() {
    // Test a simple case that should work: app(lam(var(0)), var(1))
    // This is: (λx.x) y

    let expr = app(lam(var(0)), var(1));

    println!("Simple expression: {}", expr);

    // Beta reduction result
    let beta_result = beta_reduce(expr.clone());
    println!("Beta reduction result: {}", beta_result);

    // Evaluation result
    let eval_result = eval_empty(expr.clone());
    println!("Evaluation result: {:?}", eval_result);

    // Extract the value from evaluation result
    if let EvalResult::Value(eval_value) = eval_result {
        println!("Evaluation value: {}", eval_value);
        println!("Beta result: {}", beta_result);
        println!("Results match: {}", beta_result == eval_value);

        // They should be equal
        assert_eq!(
            beta_result, eval_value,
            "Beta reduction and evaluation should produce the same result"
        );
    } else {
        panic!("Expected value result from evaluation");
    }
}
