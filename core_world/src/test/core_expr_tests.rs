use super::*;
use crate::core_expr::{app, lam, nat, pair, var};

#[test]
fn test_var_creation() {
    let v = var(0);
    assert!(matches!(v, CoreExpr::Var(0)));
}

#[test]
fn test_lam_creation() {
    let l = lam(var(0));
    assert!(matches!(l, CoreExpr::Lam(_)));
    if let CoreExpr::Lam(body) = l {
        assert!(matches!(*body, CoreExpr::Var(0)));
    }
}

#[test]
fn test_app_creation() {
    let identity = lam(var(0));
    let v = var(1);
    let app_expr = app(identity, v);
    assert!(matches!(app_expr, CoreExpr::App(..)));
}

#[test]
fn test_display_var() {
    let v = var(5);
    assert_eq!(format!("{}", v), "5");
}

#[test]
fn test_display_lam() {
    let l = lam(var(0));
    assert_eq!(format!("{}", l), "λx.0");
}

#[test]
fn test_display_app() {
    let identity = lam(var(0));
    let v = var(1);
    let app_expr = app(identity, v);
    assert_eq!(format!("{}", app_expr), "(λx.0) 1");
}

#[test]
fn test_nested_display() {
    let nested = app(lam(app(var(1), var(0))), lam(var(0)));
    // Updated expectation to match the new display logic
    assert_eq!(format!("{}", nested), "(λx.(1 0)) λx.0");
}

#[test]
fn test_nat_creation() {
    let n = nat(42);
    assert!(matches!(n, CoreExpr::Nat(42)));
}

#[test]
fn test_nat_display() {
    let n = nat(42);
    assert_eq!(format!("{}", n), "42");
}

#[test]
fn test_pair_creation() {
    let p = pair(var(0), var(1));
    assert!(matches!(p, CoreExpr::Pair(..)));
    if let CoreExpr::Pair(first, second) = p {
        assert!(matches!(*first, CoreExpr::Var(0)));
        assert!(matches!(*second, CoreExpr::Var(1)));
    }
}

#[test]
fn test_pair_display() {
    let p = pair(var(0), var(1));
    assert_eq!(format!("{}", p), "(0, 1)");
}

#[test]
fn test_nested_pair_display() {
    let inner = pair(var(0), var(1));
    let outer = pair(inner, var(2));
    assert_eq!(format!("{}", outer), "((0, 1), 2)");
}

#[test]
fn test_complex_expression_with_nat_and_pair() {
    let expr = app(lam(pair(var(0), nat(5))), nat(10));
    assert_eq!(format!("{}", expr), "(λx.(0, 5)) 10");
}
