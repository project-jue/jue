use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::beta_reduce;
use core_world::core_kernel::normalize;

#[test]
fn debug_combinator() {
    // Test K combinator: (λx.λy.x) a b → a
    let k_combinator = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let expr = app(app(k_combinator, a.clone()), b);

    println!("Original expression: {}", expr);
    println!("Expression structure: {:?}", expr);

    // β-reduction should reduce to λy.x where x is still the outer variable
    let beta_result = beta_reduce(expr.clone());
    println!("After first beta reduction: {}", beta_result);
    println!("First beta result structure: {:?}", beta_result);

    // Full normalization should reduce to a
    let norm_result = normalize(expr.clone());
    println!("Fully normalized: {}", norm_result);
    println!("Normalized structure: {:?}", norm_result);

    println!("Expected a: {}", a);
    println!("Match with a: {}", norm_result == a);
}