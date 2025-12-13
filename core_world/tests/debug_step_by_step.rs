use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize, substitute};

#[test]
fn debug_step_by_step() {
    println!("=== Debugging step by step ===");

    // Test case: (λx. λy. x) (λz. z)
    // De Bruijn: (λ λ 1) (λ 0)
    // Expected: λ λ 0 (which is λy. λz. z)

    let lambda_x_y_x = lam(lam(var(1))); // λx. λy. x
    let lambda_z_z = lam(var(0)); // λz. z
    let expr = app(lambda_x_y_x, lambda_z_z);

    println!("Original expression: {}", expr);
    println!("Structure: {:?}", expr);

    // Step 1: First beta reduction
    let step1 = beta_reduce(expr.clone());
    println!("After first beta reduction: {}", step1);
    println!("Structure: {:?}", step1);

    // Step 2: Full normalization
    let normalized = normalize(expr);
    println!("Fully normalized: {}", normalized);
    println!("Structure: {:?}", normalized);

    // Let's also test the substitution directly
    println!("\n=== Testing substitution directly ===");
    let body = lam(var(1)); // λy. x (where x is index 1)
    let replacement = lam(var(0)); // λz. z
    println!("Body: {}", body);
    println!("Replacement: {}", replacement);

    let subst_result = substitute(body, 0, replacement);
    println!("Substitution result: {}", subst_result);
    println!("Expected: lam(lam(var(0)))"); // λy. λz. z
    println!("Match: {}", subst_result == lam(lam(var(0))));
}
