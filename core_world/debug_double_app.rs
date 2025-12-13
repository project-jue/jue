use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

fn main() {
    println!("Testing double application normalization");
    println!("Expression: ((λx.λy.x) a) b → should become a");

    // Create the expression: ((λx.λy.x) a) b
    let nested_lam = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let expr = app(app(nested_lam.clone(), a.clone()), b.clone());

    println!("Original expression: {:?}", expr);
    println!("nested_lam: {:?}", nested_lam);
    println!("a: {:?}", a);
    println!("b: {:?}", b);

    let normalized = normalize(expr.clone());
    println!("Normalized result: {:?}", normalized);

    // Let's also test step by step
    let step1 = app(nested_lam, a);
    println!("Step 1 - (λx.λy.x) a: {:?}", step1);
    let step1_normalized = normalize(step1);
    println!("Step 1 normalized: {:?}", step1_normalized);

    let step2 = app(step1_normalized, b);
    println!("Step 2 - (result) b: {:?}", step2);
    let step2_normalized = normalize(step2);
    println!("Step 2 normalized: {:?}", step2_normalized);
}
