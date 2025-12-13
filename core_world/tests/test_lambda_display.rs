/// Test for lambda display functionality
use core_world::core_expr::{lam, var, CoreExpr};

#[test]
fn test_lambda_display() {
    // Test lambda display formatting
    let identity = lam(var(0));
    let constant = lam(var(1));
    let nested = lam(lam(var(0)));

    assert_eq!(format!("{}", identity), "λx.0");
    assert_eq!(format!("{}", constant), "λx.1");
    assert_eq!(format!("{}", nested), "λx.λx.0");
}
