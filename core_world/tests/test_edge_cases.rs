/// Test for edge cases
use core_world::core_expr::{app, lam, var};
use core_world::eval_relation::{eval, is_normal_form, Env};

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
