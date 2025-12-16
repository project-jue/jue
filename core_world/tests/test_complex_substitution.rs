/// Test for complex substitution
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

#[test]
fn test_complex_substitution() {
    // Test complex substitution: (λx.λy.(x y)) a b → (a b)
    // In De Bruijn: (λ.λ.(1 0)) 0 1 → (0 1) (after two reductions)
    let complex_lam = lam(lam(app(var(1), var(0))));
    let a = var(0);
    let b = var(1);
    let expr = app(app(complex_lam, a), b);

    // Need to normalize to get the final result
    let reduced = normalize(expr);
    let expected = app(var(0), var(1)); // (a b) where a=0, b=1 after proper De Bruijn handling
    assert_eq!(reduced, expected);
}
