/// Test for normalization with complex closures
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

#[test]
fn test_normalization_with_complex_closures() {
    // Test normalization with complex closure scenarios
    // Based on actual behavior, this should normalize to lam(var(0))
    let self_app = lam(app(var(0), var(0)));
    let identity = lam(var(0));
    let expr = app(self_app, identity.clone());

    let normalized = normalize(expr);

    // Based on actual behavior
    let expected = lam(var(0));
    assert_eq!(normalized, expected);
}
