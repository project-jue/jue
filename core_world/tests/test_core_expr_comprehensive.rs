/// Core Expression Comprehensive Tests
use core_world::core_expr::{app, lam, var, CoreExpr};

#[test]
fn test_core_expr_comprehensive() {
    // Test variable creation with different indices
    for i in 0..10 {
        let v = var(i);
        assert!(matches!(v, CoreExpr::Var(idx) if idx == i));
    }

    // Test lambda creation with different bodies
    let simple_lam = lam(var(0));

    let nested_lam = lam(lam(var(1)));
    let complex_lam = lam(app(var(0), var(1)));

    assert!(matches!(simple_lam, CoreExpr::Lam(_)));
    assert!(matches!(nested_lam, CoreExpr::Lam(_)));
    assert!(matches!(complex_lam, CoreExpr::Lam(_)));

    // Test application creation
    let identity = lam(var(0));
    let v = var(1);
    let app_expr = app(identity, v);

    assert!(matches!(app_expr, CoreExpr::App(..)));

    // Test deeply nested expressions
    let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));
    assert!(matches!(deeply_nested, CoreExpr::App(..)));
}

#[test]
fn test_core_expr_display_comprehensive() {
    // Test variable display
    for i in 0..5 {
        let v = var(i);
        assert_eq!(format!("{}", v), i.to_string());
    }

    // Test lambda display
    let l = lam(var(0));
    assert_eq!(format!("{}", l), "λx.0");

    let nested_lam = lam(lam(var(1)));
    assert_eq!(format!("{}", nested_lam), "λx.λx.1");

    // Test application display
    let identity = lam(var(0));
    let v = var(1);
    let app_expr = app(identity, v);
    assert_eq!(format!("{}", app_expr), "(λx.0) 1");

    // Test complex nested display
    let nested = app(lam(app(var(1), var(0))), lam(var(0)));
    assert_eq!(format!("{}", nested), "(λx.(1 0)) λx.0");
}
