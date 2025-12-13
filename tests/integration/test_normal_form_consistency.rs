/// Test for normal form consistency
use core_world::core_expr::{app, lam, var};
use core_world::eval_relation::{eval_empty, is_normal_form};

#[test]
fn test_normal_form_consistency() {
    // Test consistency in normal form detection
    let expr = lam(var(0));

    // Test that all components agree on normal forms
    let eval_result = eval_empty(expr.clone());
    let is_normal = is_normal_form(&eval_result);

    // Identity function should be in normal form
    assert!(is_normal);

    // Test with a reducible expression
    let app_expr = app(lam(var(0)), var(1));
    let eval_result = eval_empty(app_expr.clone());
    let is_normal = is_normal_form(&eval_result);

    // After evaluation, should be in normal form
    assert!(is_normal);
}
