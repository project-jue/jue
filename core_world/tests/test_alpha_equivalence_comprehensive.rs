use core_world::core_expr::app;

use core_world::core_expr::var;

use core_world::core_expr::lam;
use core_world::core_kernel::alpha_equiv;

/// Test Alpha Equivalence Comprehensive
#[test]
pub(crate) fn test_alpha_equivalence_comprehensive() {
    // Test identical expressions
    let expr1 = lam(var(0));
    let expr2 = lam(var(0));
    assert!(alpha_equiv(expr1, expr2));

    // Test different bodies
    let expr1 = lam(var(0));
    let expr2 = lam(var(1));
    assert!(!alpha_equiv(expr1, expr2));

    // Test nested equivalence
    let expr1 = app(lam(var(0)), var(1));
    let expr2 = app(lam(var(0)), var(1));
    assert!(alpha_equiv(expr1, expr2));

    // Test complex expressions
    let expr1 = app(lam(app(var(0), var(1))), lam(var(0)));
    let expr2 = app(lam(app(var(0), var(1))), lam(var(0)));
    assert!(alpha_equiv(expr1, expr2));

    // Test non-equivalent expressions
    let expr1 = app(lam(var(0)), var(1));
    let expr2 = app(lam(var(1)), var(0));
    assert!(!alpha_equiv(expr1, expr2));
}
