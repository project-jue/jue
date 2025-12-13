/// Normalization Comprehensive Tests
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

#[test]
fn test_normalization_comprehensive() {
    // Test idempotent normalization
    let expr = app(lam(var(0)), var(1));
    let normalized_once = normalize(expr.clone());
    let normalized_twice = normalize(normalized_once.clone());
    assert_eq!(normalized_once, normalized_twice);

    // Test complex normalization with Call-by-Value
    let expr = app(app(lam(lam(var(1))), var(0)), var(1));
    let normalized = normalize(expr);
    // With Call-by-Value: ((λ.λ.1) 0) 1 → (λ.1) 1 → 0 (after substitution)
    // This demonstrates Call-by-Value: arguments are evaluated before substitution
    assert_eq!(normalized, var(0));

    // Test that already normal forms don't change
    let identity = lam(var(0));
    let identity_clone = identity.clone();
    let normalized = normalize(identity);
    assert_eq!(normalized, identity_clone);

    // Test deeply nested normalization with Call-by-Value
    // Expression: (λx.(λy.x) x) (λz.z)
    // In De Bruijn: (λ.(λ.1) 0) (λ.0)
    // After normalization: should reduce to lam(var(0)) with Call-by-Value
    let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));
    let normalized = normalize(deeply_nested);
    // With Call-by-Value: (λ.(λ.1) 0) (λ.0) → (λ.1) 0 → λ.0 (after substitution)
    // This demonstrates Call-by-Value: arguments are evaluated before substitution
    assert_eq!(normalized, lam(var(0)));

    // Test multiple reduction paths with Call-by-Value
    // ((λ.λ.2) 0) (1 2) represents ((λx.λy.x) a) (b c)
    // Step 1: (λx.λy.x) a → λy.a (which is lam(var(2)))
    // Step 2: (λy.a) (b c) → a (which is var(2))
    // With Call-by-Value: ((λ.λ.2) 0) (1 2) → (λ.2) (1 2) → 0
    // This demonstrates Call-by-Value: arguments are evaluated before substitution
    let expr = app(app(lam(lam(var(2))), var(0)), app(var(1), var(2)));
    let normalized = normalize(expr);
    assert_eq!(normalized, var(0));
}
