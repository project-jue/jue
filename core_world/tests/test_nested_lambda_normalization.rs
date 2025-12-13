/// Test for nested lambda normalization
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

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
