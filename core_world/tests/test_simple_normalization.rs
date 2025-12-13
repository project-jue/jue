/// Simple Normalization Tests
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

#[test]
fn test_simple_identity_normalization() {
    // Test simple identity function normalization: (λx.x) y → y
    let identity = lam(var(0));
    let y = var(1);
    let expr = app(identity, y);

    let normalized = normalize(expr);
    assert_eq!(normalized, var(1));
}

#[test]
fn test_nested_lambda_normalization() {
    // Test nested lambda normalization: (λx.λy.x) a → λy.x
    let nested_lam = lam(lam(var(1)));
    let a = var(0);
    let expr = app(nested_lam, a);

    let normalized = normalize(expr);
    let expected = lam(var(1));
    assert_eq!(normalized, expected);
}

#[test]
fn test_double_application_normalization() {
    // Test double application normalization: ((λx.λy.x) a) b → a
    let nested_lam = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let expr = app(app(nested_lam, a), b);

    let normalized = normalize(expr);
    assert_eq!(normalized, var(0));
}

#[test]
fn test_complex_substitution_normalization() {
    // Test complex substitution normalization: (λx.λy.(x y)) a b → (a b)
    let complex_lam = lam(lam(app(var(1), var(0))));
    let a = var(0);
    let b = var(1);
    let expr = app(app(complex_lam, a), b);

    let normalized = normalize(expr);
    // The result should be App(Var(0), Var(1)) because:
    // 1. ((λx.λy.(x y)) a) b
    // 2. (λy.(a y)) b   (after first beta reduction)
    // 3. (a b)          (after second beta reduction)
    // In De Bruijn indices: App(Var(0), Var(1))
    let expected = app(var(0), var(1));
    assert_eq!(normalized, expected);
}