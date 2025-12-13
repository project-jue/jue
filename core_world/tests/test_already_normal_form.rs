/// Test for already normal form
use core_world::core_expr::{lam, var};
use core_world::core_kernel::normalize;

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
