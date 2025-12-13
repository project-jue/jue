/// Test for closure application
use core_world::core_expr::{app, lam, var};
use core_world::eval_relation::{eval, Env, EvalResult};

#[test]
fn test_closure_application() {
    // Test closure application with environment
    let mut env = Env::new();
    env.insert(0, var(5));

    let lambda = lam(app(var(0), var(1)));
    let arg = var(2);
    let app_expr = app(lambda, arg);

    let result = eval(&env, app_expr);

    // Should create a closure and apply it
    match result {
        EvalResult::Value(_) => {} // Should result in a value
        _ => panic!("Expected value result"),
    }
}
