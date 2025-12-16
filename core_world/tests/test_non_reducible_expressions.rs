/// Test for non-reducible expressions
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::beta_reduce;

#[test]
fn test_non_reducible_expressions() {
    // Test that non-redex expressions remain unchanged
    let var_expr = var(5);
    let reduced = beta_reduce(var_expr);
    assert_eq!(reduced, var(5));

    let lam_expr = lam(var(0));
    let reduced = beta_reduce(lam_expr);
    assert_eq!(reduced, lam(var(0)));

    // Application where function is not a lambda
    let app_expr = app(var(0), var(1));
    let reduced = beta_reduce(app_expr);
    assert_eq!(reduced, app(var(0), var(1)));
}
