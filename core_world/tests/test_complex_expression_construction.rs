/// Test for complex expression construction functionality
use core_world::core_expr::{app, lam, var, CoreExpr};

#[test]
fn test_complex_expression_construction() {
    // Test construction of complex nested expressions
    let identity = lam(var(0));
    let _v0 = var(0);
    let v1 = var(1);

    // Build: (λx.x) ((λy.y) z)
    let inner_app = app(identity.clone(), v1.clone());
    let outer_app = app(identity.clone(), inner_app);

    assert!(matches!(outer_app, CoreExpr::App(..)));

    // Verify structure
    if let CoreExpr::App(func, arg) = outer_app {
        assert!(matches!(*func, CoreExpr::Lam(_)));
        assert!(matches!(*arg, CoreExpr::App(..)));
    }
}
