/// Normal Form Detection Tests
use core_world::core_expr::{app, lam, var};
use core_world::eval_relation::{eval, eval_empty, is_normal_form, Env, EvalResult};

#[test]
fn test_normal_form_detection() {
    // Test normal form detection
    let env = Env::new();

    // Identity function is in normal form
    let identity = lam(var(0));
    let result = eval(&env, identity.clone());
    assert!(is_normal_form(&result));

    // Application of identity to variable is not in normal form initially
    // But after evaluation, it becomes a value which is in normal form
    let app_expr = app(lam(var(0)), var(1));
    let result = eval(&env, app_expr.clone());
    // After evaluation, it should be in normal form (it's a value)
    assert!(is_normal_form(&result));

    // Test with an unevaluated application
    let unevaluated = EvalResult::Value(app(lam(var(0)), var(1)));
    assert!(!is_normal_form(&unevaluated));
}

#[test]
fn test_evaluation_preserves_semantics() {
    // Test that evaluation preserves semantic equivalence
    let env = Env::new();

    // Test that (λx.x) y evaluates to y
    let identity_app = app(lam(var(0)), var(1));
    let result1 = eval(&env, identity_app);
    let result2 = eval(&env, var(1));

    assert_eq!(result1, result2);
}

#[test]
fn test_edge_cases() {
    // Test edge cases in evaluation
    let env = Env::new();

    // Test deeply nested lambda - should be in normal form
    let deeply_nested = lam(lam(lam(var(2))));
    let result = eval(&env, deeply_nested);
    assert!(is_normal_form(&result));

    // Test simple application that should evaluate to normal form
    let simple_app = app(lam(var(0)), var(1));
    let result = eval(&env, simple_app);
    assert!(is_normal_form(&result));
}
