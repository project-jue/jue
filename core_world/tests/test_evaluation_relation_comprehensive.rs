/// Evaluation Relation Comprehensive Tests
use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::eval_relation::{eval, eval_empty, is_normal_form, Env, EvalResult};

#[test]
fn test_eval_relation_comprehensive() {
    // Test variable lookup
    let mut env = Env::new();
    env.insert(0, var(5));
    env.insert(1, var(10));

    // Test bound variable
    let result = eval(&env, var(0));
    assert_eq!(result, EvalResult::Value(var(5)));

    // Test free variable
    let result = eval(&env, var(2));
    assert_eq!(result, EvalResult::Value(var(2)));

    // Test lambda introduction
    let env = Env::new();
    let lambda = lam(var(0));

    let result = eval(&env, lambda);
    match result {
        EvalResult::Closure(closure) => {
            assert_eq!(closure.body, var(0));
            assert!(closure.env.is_empty());
        }
        _ => panic!("Expected closure"),
    }

    // Test complex lambda
    let complex_lambda = lam(app(var(0), var(1)));
    let result = eval(&env, complex_lambda);
    match result {
        EvalResult::Closure(closure) => {
            assert!(matches!(closure.body, CoreExpr::App(..)));
            assert!(closure.env.is_empty());
        }
        _ => panic!("Expected closure"),
    }

    // Test application elimination
    let identity = lam(var(0));
    let y = var(1);
    let app_expr = app(identity, y);

    let result = eval(&env, app_expr);
    assert_eq!(result, EvalResult::Value(var(1)));

    // Test nested application
    let outer_lam = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let app_expr = app(app(outer_lam, a), b);

    let result = eval(&env, app_expr);
    assert_eq!(result, EvalResult::Value(var(1)));
}

#[test]
fn test_eval_empty_environment_comprehensive() {
    // Test simple lambda
    let expr = lam(var(0));
    let result = eval_empty(expr);

    match result {
        EvalResult::Closure(closure) => {
            assert!(closure.env.is_empty());
            assert_eq!(closure.body, var(0));
        }
        _ => panic!("Expected closure"),
    }

    // Test nested lambda
    let expr = lam(lam(var(1)));
    let result = eval_empty(expr);

    match result {
        EvalResult::Closure(closure) => {
            assert!(closure.env.is_empty());
            assert!(matches!(closure.body, CoreExpr::Lam(_)));
        }
        _ => panic!("Expected closure"),
    }

    // Test application
    let expr = app(lam(var(0)), var(1));
    let result = eval_empty(expr);
    assert_eq!(result, EvalResult::Value(var(1)));
}

#[test]
fn test_normal_form_detection_comprehensive() {
    // Identity function is in normal form
    let identity = lam(var(0));
    let result = eval_empty(identity);
    assert!(is_normal_form(&result));

    // Application of identity to variable is not in normal form (when unevaluated)
    let app_expr = app(lam(var(0)), var(1));
    let unevaluated = EvalResult::Value(app(lam(var(0)), var(1)));
    assert!(!is_normal_form(&unevaluated));

    // After evaluation, it should be in normal form
    let reduced_result = eval_empty(app_expr.clone());
    assert!(is_normal_form(&reduced_result));

    // Test complex expressions
    let complex_expr = app(app(lam(lam(var(1))), var(0)), var(1));
    let unevaluated_complex = EvalResult::Value(app(app(lam(lam(var(1))), var(0)), var(1)));
    assert!(!is_normal_form(&unevaluated_complex));

    let reduced_result = eval_empty(complex_expr);
    assert!(is_normal_form(&reduced_result));
}
