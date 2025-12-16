/// Test for variable capture avoidance
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::beta_reduce;

#[test]
fn test_variable_capture_avoidance() {
    // Test that variable capture is avoided: (λx.λy.x) y → λy.x (not λy.y)
    // In De Bruijn: (λ.λ.1) 0 → λ.1 (x shifts from 1 to 1, y is at 0)
    let nested_lam = lam(lam(var(1)));
    let y = var(0);
    let expr = app(nested_lam, y);

    let reduced = beta_reduce(expr);
    let expected = lam(var(1)); // x shifts to index 1, avoiding capture
    assert_eq!(reduced, expected);
}
