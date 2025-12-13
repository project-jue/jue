/// Test for nested lambda reduction
use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::core_kernel::beta_reduce;

#[test]
fn test_nested_lambda_reduction() {
    // Test nested lambda: (λx.λy.x) a → λy.x
    // In De Bruijn: (λ.λ.1) 0 → λ.1 (x is still the outer variable, now at index 1)
    let nested_lam = lam(lam(var(1)));
    let a = var(0);
    let expr = app(nested_lam, a);

    let reduced = beta_reduce(expr);
    let expected = lam(var(1)); // x shifts to index 1 after substitution
    assert_eq!(reduced, expected);
}
