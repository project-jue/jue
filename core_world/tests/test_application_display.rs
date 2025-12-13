/// Test for application display functionality
use core_world::core_expr::{app, lam, var, CoreExpr};

#[test]
fn test_application_display() {
    // Test application display formatting
    let identity = lam(var(0));
    let v0 = var(0);
    let v1 = var(1);

    let simple_app = app(identity.clone(), v0.clone());
    let complex_app = app(app(identity.clone(), v0.clone()), v1.clone());

    assert_eq!(format!("{}", simple_app), "(λx.0) 0");
    assert_eq!(format!("{}", complex_app), "((λx.0) 0) 1");
}
