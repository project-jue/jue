use super::*;
use crate::core_expr::{app, lam, var};

#[test]
fn test_beta_reduce_identity() {
    // Test Call-by-Name: (λx.x) y → y
    // In De Bruijn: (λ.0) 1 → 1
    // Call-by-Name: function is reduced to WHNF first, then argument is substituted without evaluation
    let identity = lam(var(0));
    let y = var(1);
    let app_expr = app(identity, y);
    let reduced = beta_reduce(app_expr);

    // With Call-by-Name, the identity function is reduced to WHNF first
    // Then the argument y (var(1)) is substituted without evaluation
    assert_eq!(reduced, var(1));
}

#[test]
fn test_beta_reduce_complex() {
    // Test Call-by-Name: (λx.λy.x) a → λy.x
    // In De Bruijn: (λ.λ.1) 0 → λ.1 (x is still the outer variable, now at index 1)
    // Call-by-Name: function is reduced to WHNF first, then argument is substituted without evaluation
    let outer_lam = lam(lam(var(1)));
    let a = var(0);
    let app_expr = app(outer_lam.clone(), a.clone());

    // Expected: λy.x where x is still the outer variable (index 1)
    // With Call-by-Name, the outer lambda is reduced to WHNF first
    let expected_first_step = lam(var(1));
    assert_eq!(beta_reduce(app_expr), expected_first_step);

    // Test full reduction with Call-by-Name: ((λx.λy.x) a) b → a
    // In De Bruijn: ((λ.λ.1) 0) 1 → 0 (after proper substitution)
    // Call-by-Name evaluation order:
    // 1. Reduce ((λ.λ.1) 0) to WHNF: (λ.1)
    // 2. Substitute argument 0 without evaluation
    // 3. Apply: (λ.1) 1 → 0 (after substitution)
    // The outer variable 'a' is at index 0, and after substitution it becomes the result
    let outer_lam2 = lam(lam(var(1)));
    let a2 = var(0);
    let b = var(1);
    let full_expr = app(app(outer_lam2, a2), b);
    let normalized = normalize(full_expr);
    // This demonstrates Call-by-Name: arguments are substituted without evaluation
    assert_eq!(normalized, var(0));
}

#[test]
fn test_alpha_equiv_identical() {
    let expr1 = lam(var(0));
    let expr2 = lam(var(0));
    assert!(alpha_equiv(expr1, expr2));
}

#[test]
fn test_alpha_equiv_different_vars() {
    // λx.x and λy.y should be α-equivalent
    let expr1 = lam(var(0));
    let expr2 = lam(var(0)); // Same structure, so α-equivalent
    assert!(alpha_equiv(expr1, expr2));
}

#[test]
fn test_alpha_equiv_different_bodies() {
    let expr1 = lam(var(0));
    let expr2 = lam(var(1)); // Different body
    assert!(!alpha_equiv(expr1, expr2));
}

#[test]
fn test_normalize_idempotent() {
    let expr = app(lam(var(0)), var(1));
    let normalized_once = normalize(expr.clone());
    let normalized_twice = normalize(normalized_once.clone());
    assert_eq!(normalized_once, normalized_twice);
}

#[test]
fn test_normalize_complex() {
    // Test Call-by-Name normalization: ((λx.λy.x) a) b → a
    // In De Bruijn: ((λ.λ.1) 0) 1 → 0 (not 1)
    // Call-by-Name evaluation order:
    // 1. Reduce ((λ.λ.1) 0) to WHNF: (λ.1)
    // 2. Substitute argument 0 without evaluation
    // 3. Apply: (λ.1) 1 → 0 (after substitution)
    // The outer variable 'a' is at index 0, and after substitution it becomes the result
    let expr = app(app(lam(lam(var(1))), var(0)), var(1));
    let normalized = normalize(expr);
    // This demonstrates Call-by-Name: arguments are substituted without evaluation
    assert_eq!(normalized, var(0));
}

#[test]
fn test_prove_consistency() {
    assert!(prove_kernel_consistency());
}

#[test]
fn test_call_by_name_semantics() {
    // Test that demonstrates Call-by-Name behavior
    // Expression: (λf.(f (λx.x))) (λy.y)
    // In De Bruijn: (λ.0 (λ.0)) (λ.0)
    // Call-by-Name evaluation:
    // 1. Reduce function to WHNF: (λ.0 (λ.0)) is already in WHNF
    // 2. Substitute argument without evaluation
    // 3. Apply: substitute (λ.0) for index 0 in (0 (λ.0))
    // 4. Result: (λ.0) (λ.0) → λ.0 (after beta reduction)

    let identity = lam(var(0));
    let func = lam(app(var(0), identity.clone()));
    let arg = identity.clone();
    let expr = app(func, arg);

    let result = normalize(expr);

    // The result should be the identity function
    // This demonstrates Call-by-Name: function is evaluated to WHNF, argument is substituted without evaluation
    assert_eq!(result, identity);
}

#[test]
fn test_call_by_name_nested_application() {
    // Test nested application with Call-by-Name
    // Expression: ((λx.λy.x) (λz.z)) (λw.w)
    // In De Bruijn: ((λ.λ.1) (λ.0)) (λ.0)
    // Call-by-Name evaluation:
    // 1. Reduce ((λ.λ.1) (λ.0)) to WHNF: (λ.1)
    // 2. Substitute argument (λ.0) without evaluation
    // 3. Apply: (λ.1) (λ.0) → λ.0 (after substitution)

    let outer_identity = lam(var(0));
    let inner_identity = lam(var(0));
    let outer_lam = lam(lam(var(1)));
    let outer_identity = lam(var(0));
    let expr = app(app(outer_lam, outer_identity), inner_identity.clone());

    let result = normalize(expr);

    // The result should be the inner identity function
    // This demonstrates Call-by-Name: arguments are substituted without evaluation
    assert_eq!(result, inner_identity);
}
