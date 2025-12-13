/// Test for complex substitution normalization
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize};

#[test]
fn test_complex_substitution_normalization() {
    // Test complex substitution normalization: (λx.λy.(x y)) a b → (a b)
    let complex_lam = lam(lam(app(var(1), var(0))));
    let a = var(0);
    let b = var(1);
    let expr = app(app(complex_lam.clone(), a.clone()), b.clone());

    println!("Original expression: {:?}", expr);
    println!("Complex lambda: {:?}", complex_lam);
    println!("a: {:?}", a);
    println!("b: {:?}", b);

    // Step-by-step debugging
    let step1 = beta_reduce(expr.clone());
    println!("After first beta reduction: {:?}", step1);

    let normalized = normalize(expr);
    println!("Normalized result: {:?}", normalized);

    let expected = app(var(0), var(1)); // (a b) where a=0, b=1 after proper De Bruijn handling
    println!("Expected result: {:?}", expected);

    assert_eq!(normalized, expected);
}
