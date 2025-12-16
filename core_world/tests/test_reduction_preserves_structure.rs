/// Test for reduction preserves structure
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::beta_reduce;

#[test]
fn test_reduction_preserves_structure() {
    // Test that reduction preserves the structure of non-redex parts
    let identity = lam(var(0));
    let complex_arg = app(var(1), var(2));
    let expr = app(identity, complex_arg.clone());

    let reduced = beta_reduce(expr);
    assert_eq!(reduced, complex_arg);
}
