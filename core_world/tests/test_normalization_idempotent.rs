/// Test for normalization idempotent
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
