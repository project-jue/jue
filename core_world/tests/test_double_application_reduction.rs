/// Test for double application reduction
use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::core_kernel::normalize;

#[test]
fn test_double_application_reduction() {
    // Test double application: ((λx.λy.x) a) b → a
    // In De Bruijn: ((λ.λ.1) 0) 1 → 1 (after two reductions, a ends up at index 1)
    let nested_lam = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let expr = app(app(nested_lam, a), b);

    // Need to normalize to get the final result
    let reduced = normalize(expr);
    assert_eq!(reduced, var(0)); // a ends up at index 0 after two reductions in Call-by-Value
}
