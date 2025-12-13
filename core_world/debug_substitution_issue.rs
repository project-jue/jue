#[cfg(test)]
mod debug_substitution_issue {
    use core_world::core_expr::{app, lam, var};
    use core_world::core_kernel::{beta_reduce, normalize, substitute};

    #[test]
    fn debug_substitution_issue() {
        println!("\n=== Debugging Substitution Issue ===");

        // Test the specific case that's failing
        // Expression: lam(lam(var(2))) - which is λx.λy.x
        // Substitute var(0) for index 0
        // Expected: lam(lam(var(1))) - because:
        // [var(0)/0](λ λ 2) = λ([↑(var(0))/1](λ 2)) = λ([var(1)/1](λ 2))
        // [var(1)/1](λ 2) = λ([↑(var(1))/2]2) = λ([var(2)/2]2) = λ var(2) but wait...

        // Let me trace this step by step
        println!("Original expression: lam(lam(var(2)))");
        let expr = lam(lam(var(2)));
        println!("Expression: {}", expr);

        let replacement = var(0);
        println!("Replacement: {}", replacement);

        let result = substitute(expr, 0, replacement);
        println!("Substitution result: {}", result);
        println!("Expected: lam(lam(var(1)))"); // This might be wrong based on formal rules

        // Let's manually trace the formal rules:
        // [var(0)/0](λ λ 2) = λ([↑(var(0))/1](λ 2))
        // ↑(var(0)) = var(1)
        // So: λ([var(1)/1](λ 2))
        // [var(1)/1](λ 2) = λ([↑(var(1))/2]2)
        // ↑(var(1)) = var(2)
        // So: λ([var(2)/2]2)
        // [var(2)/2]2 = 2 (since 2 == 2, replace with var(2))
        // So final result: λ λ var(2)

        // Wait, that doesn't seem right. Let me re-read the formal rules...

        // Actually, let me test a simpler case first
        println!("\n=== Simple case: substitute var(1) for index 0 in lam(var(1)) ===");
        let simple_expr = lam(var(1));
        let simple_replacement = var(1);
        let simple_result = substitute(simple_expr, 0, simple_replacement);
        println!("Expression: {}", simple_expr);
        println!("Replacement: {}", simple_replacement);
        println!("Result: {}", simple_result);
        println!("Expected: lam(var(2))");

        // And the complex case
        println!("\n=== Complex case: ((λx.λy.x) a) b ===");
        let complex_expr = app(app(lam(lam(var(1))), var(0)), var(1));
        println!("Original: {}", complex_expr);

        let step1 = beta_reduce(complex_expr.clone());
        println!("After first beta reduction: {}", step1);

        let step2 = beta_reduce(step1.clone());
        println!("After second beta reduction: {}", step2);

        let normalized = normalize(complex_expr);
        println!("Fully normalized: {}", normalized);
        println!("Expected: var(0) (which represents 'a')");
    }
}
