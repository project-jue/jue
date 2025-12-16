/// Test for identity function reduction
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::beta_reduce;

#[test]
fn test_identity_function_reduction() {
    // Test basic identity function: (λx.x) y → y
    // In De Bruijn: (λ.0) 1 → 1
    let identity = lam(var(0));
    let y = var(1);
    let expr = app(identity, y);

    let reduced = beta_reduce(expr);
    assert_eq!(reduced, var(1));
}
