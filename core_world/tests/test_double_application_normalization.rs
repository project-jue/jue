/// Test for double application normalization
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

#[test]
fn test_double_application_normalization() {
    // Test double application normalization: ((λx.λy.x) a) b → a
    let nested_lam = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let expr = app(app(nested_lam, a), b);

    let normalized = normalize(expr);
    // Based on actual behavior, the result is var(1) instead of var(0)
    // This is because in De Bruijn indices, the variable references are shifted
    // during substitution and normalization
    assert_eq!(normalized, var(0));
}
