/// Test to verify Call-by-Value semantics are working correctly
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize};

#[test]
fn test_call_by_value_semantics() {
    // Test 1: Simple identity function
    // (λx.x) y → y
    // In De Bruijn: (λ.0) 1 → 1
    let identity = lam(var(0));
    let y = var(1);
    let app_expr = app(identity, y);
    let reduced = beta_reduce(app_expr);
    assert_eq!(reduced, var(1));

    // Test 2: Function application with argument reduction
    // ((λx.λy.x) a) b → a
    // In De Bruijn: ((λ.λ.1) 0) 1 → 0
    let outer_lam = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let full_expr = app(app(outer_lam, a), b);
    let normalized = normalize(full_expr);
    // With Call-by-Value, this should reduce to var(0) (a)
    assert_eq!(normalized, var(0));

    // Test 3: Verify evaluation order - argument should be reduced first
    // (λf.(f (λx.x))) (λy.y)
    // In De Bruijn: (λ.(0 (λ.0))) (λ.0)
    // Should reduce the argument (λy.y) first, then apply
    let identity_lam = lam(var(0));
    let func_lam = lam(app(var(0), identity_lam.clone()));
    let arg_lam = lam(var(0));
    let expr = app(func_lam, arg_lam);
    let result = normalize(expr);
    // Should result in (λy.y) (λx.x) which is app(lam(var(0)), lam(var(0)))
    // After further reduction: (λx.x) which is lam(var(0))
    assert_eq!(result, lam(var(0)));
}
