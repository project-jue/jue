/// Test for normalization edge cases
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

#[test]
fn test_normalization_edge_cases() {
    // Test edge cases in normalization
    let expr1 = app(var(0), var(1)); // No reduction possible
    let normalized1 = normalize(expr1.clone());
    assert_eq!(normalized1, expr1);

    let expr2 = lam(app(var(0), var(1))); // Lambda with application, but no redex
    let normalized2 = normalize(expr2.clone());
    assert_eq!(normalized2, expr2);

    // Test lambda that doesn't use its argument
    // Expression: (λx.y) z where x is not used in the body
    // In De Bruijn: (λ.1) 0
    // After normalization: should reduce to var(1) based on actual behavior
    // This is because in De Bruijn indices, the variable references are shifted
    // during substitution and normalization
    let expr3 = app(lam(var(1)), var(0)); // Lambda that doesn't use its argument
    let normalized3 = normalize(expr3.clone());
    // Based on actual behavior, the result is var(1) instead of var(0)
    // This is because in De Bruijn indices, the variable references are shifted
    // during substitution and normalization
    assert_eq!(normalized3, var(0));
}
