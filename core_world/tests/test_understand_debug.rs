use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize, substitute};

#[test]
fn test_understand_debug_case() {
    println!("=== Understanding the debug case ===");

    // Let's break down ((λx.λy.2) 0) 1
    // In De Bruijn: ((λ.λ.2) 0) 1

    // First, let's understand what each part means:
    // λx.λy.2 = λ.λ.2 (x is index 0, y is index 1, 2 refers to x)
    // 0 = some variable (let's call it a)
    // 1 = some other variable (let's call it b)

    // So the expression is: ((λx.λy.x) a) b

    // Step 1: (λx.λy.x) a → λy.x where x is still the outer variable
    // In De Bruijn: (λ.λ.1) 0 → λ.1 (the outer x is now at index 1)

    // Step 2: (λy.x) b → x where x is the outer variable
    // In De Bruijn: (λ.1) 1 → 1 (but this doesn't seem right)

    // Let's test this step by step
    let outer_lam = lam(lam(var(2))); // λx.λy.2 (x is index 0, y is index 1, 2 refers to x)
    let a = var(0); // a
    let b = var(1); // b

    println!("Outer lambda: {}", outer_lam); // Should be λx.λy.2
    println!("a: {}", a); // Should be 0
    println!("b: {}", b); // Should be 1

    // First application: (λx.λy.2) 0
    let first_app = app(outer_lam.clone(), a.clone());
    println!("First application: {}", first_app); // Should be (λx.λy.2) 0

    let first_reduction = beta_reduce(first_app);
    println!("After first beta reduction: {}", first_reduction); // Should be λy.2

    // Second application: (result) 1
    let second_app = app(first_reduction, b.clone());
    println!("Second application: {}", second_app); // Should be (λy.2) 1

    let second_reduction = beta_reduce(second_app);
    println!("After second beta reduction: {}", second_reduction); // What should this be?

    // Full normalization
    let full_expr = app(app(outer_lam.clone(), a.clone()), b.clone());
    let normalized = normalize(full_expr);
    println!("Fully normalized: {}", normalized);

    // Let's also test the substitution directly
    println!("\n=== Testing substitution directly ===");
    let inner_lam = lam(var(2)); // λy.2
    let replacement = var(0); // 0
    let subst_result = substitute(inner_lam, 0, replacement);
    println!("substitute(λy.2, 0, 0) = {}", subst_result);

    // And another substitution
    let var_expr = var(2); // 2
    let subst_result2 = substitute(var_expr, 0, var(1)); // substitute 2 for 0 in 1
    println!("substitute(2, 0, 1) = {}", subst_result2);
}
