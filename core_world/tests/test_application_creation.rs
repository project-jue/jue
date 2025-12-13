/// Test for application creation functionality
use core_world::core_expr::{app, lam, var, CoreExpr};

#[test]
fn test_application_creation() {
    // Test basic application creation
    let identity = lam(var(0));
    let v0 = var(0);
    let app_expr = app(identity, v0);

    assert!(matches!(app_expr, CoreExpr::App(..)));
}
