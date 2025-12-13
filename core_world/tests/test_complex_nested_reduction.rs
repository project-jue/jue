/// Test for complex nested reduction
use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::core_kernel::normalize;

#[test]
fn test_complex_nested_reduction() {
    // Test complex nested reduction: (λx.λy.(x (λz.z))) (λw.w) → λy.(λw.w (λz.z))
    // In De Bruijn: (λ.λ.(1 (λ.0))) (λ.0) → λ.λ.0 (after normalization, the inner identity reduces)
    let inner_identity = lam(var(0));
    let complex_body = app(var(1), inner_identity.clone());
    let outer_lam = lam(lam(complex_body));
    let arg_identity = lam(var(0));
    let expr = app(outer_lam, arg_identity);

    // Need to normalize to get the final result
    let reduced = normalize(expr);

    // Expected: λ.λ.0 which is lam(lam(var(0)))
    // The normalization correctly reduces the expression to the identity function
    let expected = lam(lam(var(0)));
    assert_eq!(reduced, expected);
}
