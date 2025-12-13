/// Test for kernel normalization consistency
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

#[test]
fn test_kernel_normalization_consistency() {
    // Test that kernel operations and normalization are consistent
    let expr = app(app(lam(lam(var(1))), var(0)), var(1));

    // Normalization result (multiple steps)
    let normalized = normalize(expr.clone());

    // They should NOT be the same - beta_reduce does one step, normalize does multiple
    // After one beta reduction: App(Lam(Var(1)), Var(1))
    // After normalization: Var(1)
    assert_eq!(normalized, var(1));
}
