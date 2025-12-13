/// Test for normalization preserves structure
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

#[test]
fn test_normalization_preserves_structure() {
    // Test that normalization preserves the structure of non-redex parts
    let identity = lam(var(0));
    let complex_arg = app(var(1), var(2));
    let expr = app(identity, complex_arg.clone());

    let normalized = normalize(expr);
    assert_eq!(normalized, complex_arg);
}
