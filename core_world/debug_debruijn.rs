use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize, substitute};

fn main() {
    println!("=== Debugging De Bruijn Issue ===");

    // Test the problematic case step by step
    let expr = app(app(lam(lam(var(2))), var(0)), var(1));
    println!("Original expression: {}", expr);
    println!("This represents: ((λx.λy.2) 0) 1");

    // Step 1: First beta reduction
    let step1 = beta_reduce(expr.clone());
    println!("\nAfter first beta reduction: {}", step1);
    println!("Expected: (λy.2) 1");

    // Step 2: Second beta reduction
    let step2 = beta_reduce(step1.clone());
    println!("\nAfter second beta reduction: {}", step2);
    println!("Expected: 1 (not 2)");

    // Full normalization
    let normalized = normalize(expr);
    println!("\nFully normalized: {}", normalized);
    println!("Expected: 1");

    // Let's also test the substitution directly
    println!("\n=== Testing substitution directly ===");
    let lambda_body = lam(var(2)); // λy.2
    let arg = var(1); // 1
    println!("Substituting {} for index 0 in {}", arg, lambda_body);

    let subst_result = substitute(lambda_body, 0, arg);
    println!("Substitution result: {}", subst_result);
    println!("Expected: var(1) because 2 > 0, so 2-1 = 1");
}
