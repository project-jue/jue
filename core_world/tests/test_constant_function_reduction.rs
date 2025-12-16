/// Test for constant function reduction
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::beta_reduce;

#[test]
fn test_constant_function_reduction() {
    // Test constant function: (λx.y) z → y
    // In De Bruijn: (λ.1) 2 → 0 (after proper index adjustment)
    let constant_func = lam(var(1));
    let z = var(2);
    let expr = app(constant_func, z);

    let reduced = beta_reduce(expr);
    assert_eq!(reduced, var(0)); // After proper De Bruijn index adjustment
}
