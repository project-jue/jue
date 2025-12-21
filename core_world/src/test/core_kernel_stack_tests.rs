use super::*;
use crate::core_expr::{app, lam, var};

#[test]
fn test_stack_based_beta_reduction() {
    // Test simple β-reduction: (λx.x) y → y
    let expr = app(lam(var(0)), var(1));
    let reduced = beta_reduce_step_stack_based(expr);
    assert_eq!(reduced, var(1));
}

#[test]
fn test_stack_based_eta_reduction() {
    // Test η-reduction: λx.(f x) → f (when x is not free in f)
    let f = var(1);
    let eta_expr = lam(app(f.clone(), var(0)));
    let reduced = eta_reduce_stack_based(eta_expr);
    assert_eq!(reduced, f);
}

#[test]
fn test_stack_based_normalization_simple() {
    // Test simple normalization: (λx.x) y → y
    let expr = app(lam(var(0)), var(1));
    let result = normalize_stack_based(expr, 10).unwrap();
    assert_eq!(result, var(1));
}

#[test]
fn test_stack_based_normalization_complex() {
    // Test complex normalization: ((λx.λy.x) a) b → a
    let expr = app(app(lam(lam(var(1))), var(0)), var(1));
    let result = normalize_stack_based(expr, 100).unwrap();
    assert_eq!(result, var(0));
}

#[test]
fn test_stack_based_normalization_identity() {
    // Test identity function normalization
    let identity = lam(var(0));
    let expr = app(identity.clone(), var(42));
    let result = normalize_stack_based(expr, 10).unwrap();
    assert_eq!(result, var(42));
}

#[test]
fn test_stack_based_normalization_step_limit() {
    // Test that step limit is respected
    // Create an expression that will definitely exceed step limit
    // This creates a deeply nested application that will take many reduction steps
    let identity = lam(var(0));
    let mut expr = identity.clone();
    for _ in 0..10 {
        expr = app(expr, identity.clone());
    }

    let result = normalize_stack_based(expr, 3);
    assert!(matches!(
        result,
        Err(crate::NormalizationError::StepLimitExceeded(3))
    ));
}

#[test]
fn test_stack_based_vs_recursive_consistency() {
    // Test that stack-based and recursive normalization give same results
    let expr = app(app(lam(lam(var(1))), var(0)), var(1));

    let recursive_result = normalize(expr.clone());
    let stack_result = normalize_stack_based(expr, 100).unwrap();

    assert_eq!(recursive_result, stack_result);
}

#[test]
fn test_stack_based_normalization_idempotent() {
    // Test that normalization is idempotent
    let expr = app(lam(var(0)), var(1));
    let normalized_once = normalize_stack_based(expr.clone(), 10).unwrap();
    let normalized_twice = normalize_stack_based(normalized_once.clone(), 10).unwrap();
    assert_eq!(normalized_once, normalized_twice);
}

#[test]
fn test_stack_based_normalization_preserves_alpha_equivalence() {
    // Test that normalization preserves alpha equivalence
    let expr1 = app(lam(var(0)), var(1));
    let expr2 = app(lam(var(0)), var(1)); // Same structure

    let result1 = normalize_stack_based(expr1, 10).unwrap();
    let result2 = normalize_stack_based(expr2, 10).unwrap();

    assert!(alpha_equiv(result1, result2));
}
