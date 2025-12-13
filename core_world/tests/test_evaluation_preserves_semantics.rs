/// Test for evaluation preserves semantics
use core_world::core_expr::{app, lam, var};
use core_world::eval_relation::{eval, Env, EvalResult};

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
