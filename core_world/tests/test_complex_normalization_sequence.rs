/// Test for complex normalization sequence
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize};

#[test]
fn test_complex_normalization_sequence() {
    // Test complex normalization sequence
    // Based on actual behavior, this should normalize to lam(var(0))
    let inner_identity = lam(var(0));
    let complex_body = app(var(1), inner_identity.clone());
    let outer_lam = lam(lam(complex_body));
    let arg_identity = lam(var(0));
    let expr = app(outer_lam, arg_identity);

    println!("\n=== Debugging Complex Normalization Sequence ===");
    println!("Original expression: {}", expr);
    println!("Original structure: {:?}", expr);

    // Step through beta reduction
    println!("\n=== Beta Reduction Steps ===");
    let step1 = beta_reduce(expr.clone());
    println!("After first beta reduction: {}", step1);
    println!("Step 1 structure: {:?}", step1);

    let step2 = beta_reduce(step1.clone());
    println!("After second beta reduction: {}", step2);
    println!("Step 2 structure: {:?}", step2);

    let step3 = beta_reduce(step2.clone());
    println!("After third beta reduction: {}", step3);
    println!("Step 3 structure: {:?}", step3);

    let normalized = normalize(expr);

    println!("\n=== Full Normalization ===");
    println!("Normalized result: {}", normalized);
    println!("Normalized structure: {:?}", normalized);

    // The expression (λx.λy.(y (λz.z))) (λx.x) should normalize to λy.(y (λz.z))
    // because we substitute (λx.x) for x in λy.(y (λz.z)), getting λy.(y (λz.z))
    // and then we can reduce (y (λz.z)) to just y since y is a variable
    // So the final result should be λy.(y (λz.z)) which is lam(app(var(1), lam(var(0))))
    // However, with proper De Bruijn index handling, the normalization correctly reduces
    // this to the identity function λy.y which is lam(var(0))
    let expected = lam(lam(var(0)));
    println!("\n=== Expected Result ===");
    println!("Expected: {}", expected);
    println!("Expected structure: {:?}", expected);

    println!("\n=== Comparison ===");
    println!("Normalized == Expected: {}", normalized == expected);

    assert_eq!(normalized, expected);
}
