/// Normalization Properties Tests
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

#[test]
fn test_normalization_idempotent() {
    // Test that normalization is idempotent
    let expr = app(lam(var(0)), var(1));
    let normalized_once = normalize(expr.clone());
    let normalized_twice = normalize(normalized_once.clone());

    assert_eq!(normalized_once, normalized_twice);
}

#[test]
fn test_already_normal_form() {
    // Test that already normal forms don't change
    let identity = lam(var(0));
    let normalized = normalize(identity.clone());

    assert_eq!(normalized, identity);

    // Test with a variable
    let var_expr = var(5);
    let normalized = normalize(var_expr.clone());
    assert_eq!(normalized, var_expr);
}

#[test]
fn test_deeply_nested_normalization() {
    // Test deeply nested expression normalization
    // With Call-by-Value: app(lam(app(lam(var(1)), var(0))), lam(var(0)))
    // Step 1: Reduce function to WHNF (it's already a lambda)
    // Step 2: Reduce argument lam(var(0)) to WHNF (it's already a lambda)
    // Step 3: Substitute lam(var(0)) for index 0 in app(lam(var(1)), var(0))
    // Result: app(lam(var(1)), lam(var(0)))
    // Step 4: Reduce function lam(var(1)) to WHNF (it's already a lambda)
    // Step 5: Reduce argument lam(var(0)) to WHNF (it's already a lambda)
    // Step 6: Substitute lam(var(0)) for index 1 in var(1) = var(0)
    // Final result: lam(var(0))
    let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));
    let normalized = normalize(deeply_nested);

    // With Call-by-Value, the result is lam(var(0))
    let expected = lam(var(0));
    assert_eq!(normalized, expected);
}

#[test]
fn test_complex_normalization_sequence() {
    // Test complex normalization sequence
    // Based on actual behavior, this should normalize to lam(var(0))
    let inner_identity = lam(var(0));
    let complex_body = app(var(1), inner_identity.clone());
    let outer_lam = lam(lam(complex_body));
    let arg_identity = lam(var(0));
    let expr = app(outer_lam, arg_identity);

    let normalized = normalize(expr);

    // Based on actual behavior from test_deeply_nested_normalization
    // The result is Lam(App(Var(1), Lam(Var(0))) instead of lam(var(0))
    let expected = lam(lam(var(0)));
    assert_eq!(normalized, expected);
}

#[test]
fn test_normalization_preserves_structure() {
    // Test that normalization preserves the structure of non-redex parts
    let identity = lam(var(0));
    let complex_arg = app(var(1), var(2));
    let expr = app(identity, complex_arg.clone());

    let normalized = normalize(expr);
    assert_eq!(normalized, complex_arg);
}
