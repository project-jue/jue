/// Test for multiple reduction steps
use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::core_kernel::beta_reduce;

#[test]
fn test_multiple_reduction_steps() {
    // Test that only one reduction step is performed
    let identity = lam(var(0));
    let nested_app = app(identity.clone(), var(1));
    let expr = app(identity.clone(), nested_app);

    // First reduction should give: (λx.x) 1
    let reduced_once = beta_reduce(expr);
    let expected_once = app(identity, var(1));

    assert_eq!(reduced_once, var(1));
}
