/// Debug test to trace the failing normalization step by step
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize, substitute};

#[test]
fn debug_failing_normalization() {
    println!("\n=== Debugging the failing normalization ===");

    // The failing test case: ((λ.λ.1) 0) 1
    // This should normalize to 1, but currently produces 0
    let expr = app(app(lam(lam(var(1))), var(0)), var(1));

    println!("Original expression: {}", expr);
    println!("Original structure: {:?}", expr);

    // Step 1: First beta reduction
    let step1 = beta_reduce(expr.clone());
    println!("\nStep 1 (first beta reduction): {}", step1);
    println!("Step 1 structure: {:?}", step1);

    // Step 2: Second beta reduction
    let step2 = beta_reduce(step1.clone());
    println!("\nStep 2 (second beta reduction): {}", step2);
    println!("Step 2 structure: {:?}", step2);

    // Step 3: Third beta reduction (if needed)
    let step3 = beta_reduce(step2.clone());
    println!("\nStep 3 (third beta reduction): {}", step3);
    println!("Step 3 structure: {:?}", step3);

    // Full normalization
    let normalized = normalize(expr);
    println!("\nFinal normalized result: {}", normalized);
    println!("Final normalized structure: {:?}", normalized);

    println!("\nExpected: var(1)");
    println!("Actual: {}", normalized);
    println!("Match: {}", normalized == var(1));

    // Let's also test the substitution directly
    println!("\n=== Testing substitution directly ===");
    let inner_lam = lam(var(1)); // λy.x
    let arg = var(0); // a
    println!("Inner lambda: {}", inner_lam);
    println!("Argument: {}", arg);

    let substitution_result = substitute(inner_lam, 0, arg);
    println!("Substitution result: {}", substitution_result);
    println!("Expected: lam(var(1)) (λy.a where a is now at index 1)");
    println!("Match: {}", substitution_result == lam(var(1)));
}
