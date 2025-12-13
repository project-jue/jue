/// Debug file to trace substitution step by step
use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::core_kernel::{beta_reduce, normalize, shift_indices, substitute};

fn main() {
    println!("=== Detailed Substitution Debug ===");

    // Test case: ((λ.λ.1) 0) 1
    // Step 1: (λ.λ.1) 0 → λ.1
    let inner_lam = lam(lam(var(1))); // λ.λ.1
    let arg1 = var(0); // 0
    let step1_expr = app(inner_lam, arg1);

    println!("Step 1 expression: {}", step1_expr);
    let step1_result = beta_reduce(step1_expr);
    println!("Step 1 result: {}", step1_result);

    // Now let's manually trace what should happen in step 1
    println!("\n=== Manual trace of step 1 ===");
    println!("Expression: (λ.λ.1) 0");
    println!("This is: lam(lam(var(1))) applied to var(0)");
    println!("β-reduction: [var(0)/0](lam(lam(var(1))))");

    // Manual substitution
    let body = lam(lam(var(1)));
    let replacement = var(0);
    println!("Body: {}", body);
    println!("Replacement: {}", replacement);

    let manual_result = substitute(body, 0, replacement);
    println!("Manual substitution result: {}", manual_result);

    // Step 2: (λ.1) 1 → ?
    if let CoreExpr::App(func, arg) = step1_result {
        if let CoreExpr::Lam(body) = *func {
            println!("\n=== Step 2 ===");
            println!("Expression: (λ.1) 1");
            println!("This is: lam(var(1)) applied to var(1)");
            println!("β-reduction: [var(1)/0](lam(var(1)))");

            let body2 = *body;
            let replacement2 = *arg;
            println!("Body: {}", body2);
            println!("Replacement: {}", replacement2);

            let manual_result2 = substitute(body2, 0, replacement2);
            println!("Manual substitution result: {}", manual_result2);

            // Let's also test the shift function directly
            println!("\n=== Testing shift function ===");
            let test_expr = var(1);
            println!("Original: {}", test_expr);
            let shifted = shift_indices(test_expr, 1, 0);
            println!("Shifted by 1, cutoff 0: {}", shifted);
        }
    }

    // Full normalization
    let expr = app(app(lam(lam(var(1))), var(0)), var(1));
    let normalized = normalize(expr);
    println!("\n=== Full normalization ===");
    println!("Original: {}", expr);
    println!("Normalized: {}", normalized);
    println!("Expected: var(1) (which is 'b')");
    println!("Actual: {}", normalized);
}
