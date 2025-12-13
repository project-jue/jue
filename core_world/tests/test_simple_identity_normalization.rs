/// Test for simple identity normalization
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
