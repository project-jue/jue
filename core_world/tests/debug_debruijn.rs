use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::core_kernel::beta_reduce;
use core_world::core_kernel::normalize;

#[test]
fn debug_simple_beta_reduction() {
    // Test: (λx.x) y → y
    // In De Bruijn: (λ.0) 1 → 1
    let identity = lam(var(0));
    let y = var(1);
    let expr = app(identity, y.clone());

    println!("Original expression: (λ.0) 1");
    println!("Expected β-reduction result: 1");

    let beta_result = beta_reduce(expr.clone());
    println!("Actual β-reduction result: {:?}", beta_result);

    let norm_result = normalize(expr.clone());
    println!("Normalization result: {:?}", norm_result);

    // This should be y (var(1))
    assert_eq!(beta_result, y);
    assert_eq!(norm_result, y);
}

#[test]
fn debug_k_combinator() {
    // Test K combinator: (λx.λy.x) a b → a
    // In De Bruijn: (λ.λ.1) 0 1 → 0
    let k_combinator = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let expr = app(app(k_combinator, a.clone()), b);

    println!("K combinator expression: ((λ.λ.1) 0) 1");
    println!("Expected final result: 0");

    let beta_result1 = beta_reduce(expr.clone());
    println!("First β-reduction: {:?}", beta_result1);

    let norm_result = normalize(expr.clone());
    println!("Normalization result: {:?}", norm_result);

    // After first β-reduction, should be λ.1 (but with shifted indices)
    // After full normalization, should be a (var(0))
    assert_eq!(norm_result, a);
}