/// Test for simple application elimination
use core_world::core_expr::{app, lam, var};
use core_world::eval_relation::{eval, Env, EvalResult};

#[test]
fn test_application_elimination_simple() {
    // Test simple application: (λx.x) y → y
    let env = Env::new();
    let identity = lam(var(0));
    let y = var(1);
    let app_expr = app(identity, y);

    let result = eval(&env, app_expr);
    assert_eq!(result, EvalResult::Value(var(1)));
}
