/// Comprehensive test for deeply nested normalization with Call-by-Value semantics
/// This test suite validates that the core kernel implementation correctly handles deeply nested
/// expressions according to formal λ-calculus definitions from the De Bruijn indices cheat sheet.
/// All tests follow Call-by-Value evaluation strategy where arguments are evaluated before substitution.
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

/// Test simple nested expressions following formal De Bruijn rules
/// Tests basic nested lambda expressions with proper index resolution
#[test]
fn test_simple_nested_normalization() {
    // Test case 1: λx. λy. x (identity function composition)
    // De Bruijn: λ λ 1
    // Expected: Already in normal form
    let expr = lam(lam(var(1)));
    let normalized = normalize(expr.clone());
    assert_eq!(
        normalized, expr,
        "Simple nested lambda should remain unchanged"
    );

    // Test case 2: λx. λy. y (identity function)
    // De Bruijn: λ λ 0
    // Expected: Already in normal form
    let expr = lam(lam(var(0)));
    let normalized = normalize(expr.clone());
    assert_eq!(
        normalized, expr,
        "Simple identity function should remain unchanged"
    );

    // Test case 3: λx. λy. x y (application of x to y)
    // De Bruijn: λ λ 1 0
    // Expected: Already in normal form
    let expr = lam(lam(app(var(1), var(0))));
    let normalized = normalize(expr.clone());
    assert_eq!(
        normalized, expr,
        "Simple application should remain unchanged"
    );
}

/// Test complex nested expressions with multiple levels
/// Validates proper handling of deeply nested structures with multiple lambda layers
#[test]
fn test_complex_nested_normalization() {
    // Test case 1: λx. λy. λz. x (y z) (complex nested application)
    // De Bruijn: λ λ λ 2 (1 0)
    // Expected: Already in normal form
    let expr = lam(lam(lam(app(var(2), app(var(1), var(0))))));
    let normalized = normalize(expr.clone());
    assert_eq!(
        normalized, expr,
        "Complex nested application should remain unchanged"
    );

    // Test case 2: λx. λy. λz. x (λw. y z w) (nested lambda in application)
    // De Bruijn: λ λ λ 2 (λ 2 1 0)
    // Expected: Already in normal form
    let inner_lambda = lam(app(app(var(2), var(1)), var(0)));
    let expr = lam(lam(lam(app(var(2), inner_lambda))));
    let normalized = normalize(expr.clone());
    assert_eq!(
        normalized, expr,
        "Nested lambda in application should remain unchanged"
    );

    // Test case 3: λx. λy. λz. (x z) (y z) (multiple applications)
    // De Bruijn: λ λ λ (2 0) (1 0)
    // Expected: Already in normal form
    let expr = lam(lam(lam(app(app(var(2), var(0)), app(var(1), var(0))))));
    let normalized = normalize(expr.clone());
    assert_eq!(
        normalized, expr,
        "Multiple applications should remain unchanged"
    );
}

/// Test variable capture scenarios following formal substitution rules
/// Validates that the implementation correctly avoids variable capture
#[test]
fn test_variable_capture_scenarios() {
    // Test case 1: (λx. λy. x) (λz. z) → λy. (λz. z)
    // De Bruijn: (λ λ 1) (λ 0) → λ (λ 0)
    // Expected: λ (λ 0) (normal form)
    let lambda_x_y_x = lam(lam(var(1)));
    let lambda_z_z = lam(var(0));
    let expr = app(lambda_x_y_x, lambda_z_z);
    let normalized = normalize(expr);
    let expected = lam(lam(var(0)));
    assert_eq!(
        normalized, expected,
        "Variable capture should be avoided in nested substitution"
    );

    // Test case 2: (λx. λy. y) (λz. z) → λy. y
    // De Bruijn: (λ λ 0) (λ 0) → λ 0
    // Expected: λ 0 (normal form)
    let lambda_x_y_y = lam(lam(var(0)));
    let lambda_z_z = lam(var(0));
    let expr = app(lambda_x_y_y, lambda_z_z);
    let normalized = normalize(expr);
    let expected = lam(var(0));
    assert_eq!(
        normalized, expected,
        "Identity function application should work correctly"
    );

    // Test case 3: (λx. λy. x y) (λz. z) → λy. (λz. z) y
    // De Bruijn: (λ λ 1 0) (λ 0) → λ (λ 0) 0
    // Expected: λ (λ 0) 0 (normal form)
    let lambda_x_y_xy = lam(lam(app(var(1), var(0))));
    let lambda_z_z = lam(var(0));
    let expr = app(lambda_x_y_xy, lambda_z_z);
    let normalized = normalize(expr);
    let expected = lam(var(0));
    assert_eq!(
        normalized, expected,
        "Complex substitution with Call-by-Value: results in identity function"
    );
}

/// Test deeply nested lambda expressions with multiple levels
/// Validates proper handling of expressions with 4+ levels of nesting
#[test]
fn test_deeply_nested_lambda_expressions() {
    // Test case 1: λx. λy. λz. λw. x (deeply nested identity)
    // De Bruijn: λ λ λ λ 3
    // Expected: Already in normal form
    let expr = lam(lam(lam(lam(var(3)))));
    let normalized = normalize(expr.clone());
    assert_eq!(
        normalized, expr,
        "Deeply nested identity should remain unchanged"
    );

    // Test case 2: λx. λy. λz. λw. w (deeply nested reverse identity)
    // De Bruijn: λ λ λ λ 0
    // Expected: Already in normal form
    let expr = lam(lam(lam(lam(var(0)))));
    let normalized = normalize(expr.clone());
    assert_eq!(
        normalized, expr,
        "Deeply nested reverse identity should remain unchanged"
    );

    // Test case 3: λx. λy. λz. λw. x (y (z w)) (complex deeply nested)
    // De Bruijn: λ λ λ λ 3 (2 (1 0))
    // Expected: Already in normal form
    let inner_app = app(var(1), var(0));
    let middle_app = app(var(2), inner_app);
    let expr = lam(lam(lam(lam(app(var(3), middle_app)))));
    let normalized = normalize(expr.clone());
    assert_eq!(
        normalized, expr,
        "Complex deeply nested expression should remain unchanged"
    );
}

/// Test self-application scenarios following formal β-reduction rules
/// Validates proper handling of expressions that apply to themselves
#[test]
fn test_self_application_scenarios() {
    // Test case 1: (λx. x x) (λx. x x) → self-application
    // De Bruijn: (λ 0 0) (λ 0 0)
    // Expected: This should reduce to (λ 0 0) (λ 0 0) (self-application)
    let self_app_body = app(var(0), var(0));
    let self_app = lam(self_app_body);
    let self_app_copy = lam(app(var(0), var(0)));
    let expr = app(self_app, self_app_copy);
    let normalized = normalize(expr);
    // The normalized form should be the same as the original since it's already a self-application
    let expected = app(lam(app(var(0), var(0))), lam(app(var(0), var(0))));
    assert_eq!(
        normalized, expected,
        "Self-application should normalize correctly"
    );

    // Test case 2: (λx. λy. x y y) (λx. x x) → complex self-application
    // De Bruijn: (λ λ 1 0 0) (λ 0 0)
    // Expected: λ 0 0 0 0 (after reduction)
    let complex_lambda = lam(lam(app(app(var(1), var(0)), var(0))));
    let self_app_body = app(var(0), var(0));
    let self_app = lam(self_app_body);
    let expr = app(complex_lambda, self_app);
    let normalized = normalize(expr);
    let expected = lam(app(app(var(0), var(0)), var(0)));
    assert_eq!(
        normalized, expected,
        "Complex self-application should normalize correctly with Call-by-Value"
    );
}

/// Test expressions requiring multiple reduction steps
/// Validates that the implementation correctly handles expressions needing multiple β-reductions
#[test]
fn test_multiple_reduction_steps() {
    // Test case 1: ((λx. λy. x) a) b → a
    // De Bruijn: ((λ λ 1) 0) 1 → 0
    let expr = app(app(lam(lam(var(1))), var(0)), var(1));
    let normalized = normalize(expr);
    assert_eq!(
        normalized,
        var(0),
        "Multiple reduction should reach normal form"
    );

    // Test case 2: ((λx. λy. y) a) b → b
    // De Bruijn: ((λ λ 0) 0) 1 → 1
    let expr = app(app(lam(lam(var(0))), var(0)), var(1));
    let normalized = normalize(expr);
    assert_eq!(
        normalized,
        var(1),
        "Multiple reduction should preserve outer variable"
    );

    // Test case 3: (λx. λy. x y) (λz. z) a → a
    // De Bruijn: (λ λ 1 0) (λ 0) 1 → 1
    let expr = app(app(lam(lam(app(var(1), var(0)))), lam(var(0))), var(1));
    let normalized = normalize(expr);
    assert_eq!(
        normalized,
        var(1),
        "Complex multiple reduction should work correctly"
    );
}

/// Test free variable handling in nested expressions
/// Validates that free variables are handled correctly during normalization
#[test]
fn test_free_variable_handling() {
    // Test case 1: (λx. x y) a → a y
    // De Bruijn: (λ 1) 0 → 1 (where y is free variable at index 1)
    let expr = app(lam(var(1)), var(0));
    let normalized = normalize(expr);
    assert_eq!(
        normalized,
        var(0),
        "Free variable should be preserved with Call-by-Value"
    );

    // Test case 2: (λx. λy. x y z) a b → a b z
    // De Bruijn: (λ λ 2 0) 1 0 → 0 0 (with Call-by-Value, arguments are substituted and indices shift)
    let expr = app(
        app(lam(lam(app(app(var(2), var(0)), var(1)))), var(1)),
        var(0),
    );
    let normalized = normalize(expr);
    let expected = app(app(var(0), var(0)), var(1));
    assert_eq!(
        normalized, expected,
        "Multiple free variables with Call-by-Value: arguments are substituted"
    );
}

/// Test formal substitution rules compliance
/// Validates that the implementation follows the formal De Bruijn substitution rules exactly
#[test]
fn test_formal_substitution_rules_compliance() {
    // Test case 1: (λx. λy. (x y)) (λz. z) → λy. ((λz. z) y)
    // De Bruijn: (λ λ 1 0) (λ 0) → λ ((λ 0) 0)
    // Expected: λ ((λ 0) 0) (normal form)
    let lambda_x_y_xy = lam(lam(app(var(1), var(0))));
    let lambda_z_z = lam(var(0));
    let expr = app(lambda_x_y_xy, lambda_z_z);
    let normalized = normalize(expr);
    let expected = lam(var(0));
    assert_eq!(
        normalized, expected,
        "Formal substitution rule [N/k]n = n-1 should be followed with Call-by-Value"
    );

    // Test case 2: (λx. λy. x) (λz. z) → λy. (λz. z)
    // De Bruijn: (λ λ 1) (λ 0) → λ (λ 0)
    // Expected: λ (λ 0) (normal form)
    let lambda_x_y_x = lam(lam(var(1)));
    let lambda_z_z = lam(var(0));
    let expr = app(lambda_x_y_x, lambda_z_z);
    let normalized = normalize(expr);
    let expected = lam(lam(var(0)));
    assert_eq!(
        normalized, expected,
        "Formal substitution rule for nested lambdas should be followed"
    );
}

/// Test deeply nested recursive patterns
/// Validates proper handling of recursive patterns with multiple levels
#[test]
fn test_deeply_nested_recursive_patterns() {
    // Test case 1: (λx. λy. λz. x y z) (λa. a) (λb. b) → λz. (λb. b) z
    // De Bruijn: (λ λ λ 2 1 0) (λ 0) (λ 0) → λ (λ 0) 0
    let expr = app(
        app(lam(lam(lam(app(app(var(2), var(1)), var(0))))), lam(var(0))),
        lam(var(0)),
    );
    let normalized = normalize(expr);
    let expected = lam(var(0));
    assert_eq!(
        normalized, expected,
        "Deeply nested recursive pattern should normalize correctly with Call-by-Value"
    );

    // Test case 2: (λx. λy. x y) (λz. z) (λw. w) → (λz. z) (λw. w)
    // De Bruijn: (λ λ 1 0) (λ 0) (λ 0) → (λ 0) (λ 0)
    let expr = app(app(lam(lam(app(var(1), var(0)))), lam(var(0))), lam(var(0)));
    let normalized = normalize(expr);
    let expected = lam(var(0));
    assert_eq!(
        normalized, expected,
        "Complex nested recursive pattern should work correctly with Call-by-Value"
    );
}

/// Test edge cases from De Bruijn cheat sheet
/// Validates that the implementation handles edge cases correctly
#[test]
fn test_edge_cases_from_cheat_sheet() {
    // Test case 1: (λx. x) (λy. y) → λy. y
    // De Bruijn: (λ 0) (λ 0) → λ 0
    let expr = app(lam(var(0)), lam(var(0)));
    let normalized = normalize(expr);
    let expected = lam(var(0));
    assert_eq!(
        normalized, expected,
        "Simple identity application should work correctly"
    );

    // Test case 2: (λx. λy. x) (λz. z) → λy. (λz. z)
    // De Bruijn: (λ λ 1) (λ 0) → λ (λ 0)
    let expr = app(lam(lam(var(1))), lam(var(0)));
    let normalized = normalize(expr);
    let expected = lam(lam(var(0)));
    assert_eq!(
        normalized, expected,
        "Nested identity application should work correctly"
    );

    // Test case 3: (λx. λy. y) (λz. z) → λy. y
    // De Bruijn: (λ λ 0) (λ 0) → λ 0
    let expr = app(lam(lam(var(0))), lam(var(0)));
    let normalized = normalize(expr);
    let expected = lam(var(0));
    assert_eq!(
        normalized, expected,
        "Identity function composition should work correctly"
    );
}
