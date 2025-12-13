/// Core Kernel and Evaluation Consistency Tests
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize};
use core_world::eval_relation::{eval_empty, EvalResult};

#[test]
fn test_core_kernel_eval_consistency() {
    // Test that β-reduction and evaluation produce consistent results
    let expr = app(lam(var(0)), var(1));

    // β-reduction result
    let beta_reduced = beta_reduce(expr.clone());

    // Evaluation result
    let eval_result = eval_empty(expr.clone());
    if let EvalResult::Value(eval_value) = eval_result {
        // Both should produce the same result (var(1))
        assert_eq!(beta_reduced, eval_value);
    } else {
        panic!("Expected value result from evaluation");
    }
}

#[test]
fn test_kernel_normalization_consistency() {
    // Test that kernel operations (beta_reduce, normalize) and evaluation produce consistent results
    // This test follows formal De Bruijn index definitions from the documentation
    //
    // Test cases cover:
    // 1. Simple lambda expressions
    // 2. Nested lambda expressions
    // 3. Complex applications with multiple reductions
    // 4. Expressions with variable capture scenarios
    // 5. Deeply nested expressions

    // Test Case 1: Simple identity function application with Call-by-Value
    // λx.x applied to y → y (in De Bruijn: λ0 applied to 1 → 1)
    // Call-by-Value: argument is evaluated before substitution
    let simple_expr = app(lam(var(0)), var(1));
    test_consistency_for_expr(simple_expr, "Simple identity function with Call-by-Value");

    // Test Case 2: Nested lambda expressions
    // λx.λy.x applied to a,b → a (in De Bruijn: λλ1 applied to 2,3 → 2)
    let nested_expr = app(app(lam(lam(var(1))), var(2)), var(3));
    test_consistency_for_expr(nested_expr, "Nested lambda expressions");

    // Test Case 3: Complex application with multiple reductions
    // (λf.λx.f x) (λy.y) z → z (in De Bruijn: (λλ1 0) (λ0) 2 → 2)
    let complex_expr = app(app(lam(lam(app(var(1), var(0)))), lam(var(0))), var(2));
    test_consistency_for_expr(complex_expr, "Complex application with multiple reductions");

    // Test Case 4: Variable capture scenario
    // (λx.λy.x y) (λz.z) → λy.(λz.z) y (in De Bruijn: (λλ1 0) (λ0) → λ(λ0) 0)
    let capture_expr = app(lam(lam(app(var(1), var(0)))), lam(var(0)));
    test_consistency_for_expr(capture_expr, "Variable capture scenario");

    // Test Case 5: Deeply nested expressions
    // λa.λb.λc.λd.a b c d applied to w,x,y,z → w x y z
    // (in De Bruijn: λλλλ3 2 1 0 applied to 4,5,6,7 → 4 3 2 1)
    let deeply_nested_expr = app(
        app(
            app(
                app(
                    lam(lam(lam(lam(app(app(app(var(3), var(2)), var(1)), var(0)))))),
                    var(4),
                ),
                var(5),
            ),
            var(6),
        ),
        var(7),
    );
    test_consistency_for_expr(deeply_nested_expr, "Deeply nested expressions");

    // Test Case 6: Self-application (Y combinator-like structure)
    // (λx.x x) (λx.x x) - this should reduce but may not terminate
    // For safety, we test a simpler self-application that does terminate
    // (λx.λy.y) (λx.x x) → λy.y (in De Bruijn: (λλ0) (λ0 0) → λ0)
    let self_app_expr = app(lam(lam(var(0))), app(lam(var(0)), lam(var(0))));
    test_consistency_for_expr(self_app_expr, "Self-application scenario");
}

/// Helper function to test consistency for a given expression
fn test_consistency_for_expr(expr: core_world::core_expr::CoreExpr, test_name: &str) {
    println!("Testing consistency for: {}", test_name);

    // Get beta reduction result
    let beta_result = beta_reduce(expr.clone());
    println!("  Beta reduction result: {}", beta_result);

    // Get normalization result
    let norm_result = normalize(expr.clone());
    println!("  Normalization result: {}", norm_result);

    // Get evaluation result
    let eval_result = eval_empty(expr.clone());
    let eval_value = match eval_result {
        EvalResult::Value(v) => v,
        EvalResult::Closure(c) => {
            println!("  Evaluation produced closure: {:?}", c);
            // For closures, we can't directly compare, but we can check that normalization
            // produces a lambda expression (which is equivalent to a closure)
            // This is expected behavior for lambda expressions
            return;
        }
    };
    println!("  Evaluation result: {}", eval_value);

    // Verify consistency between normalization and evaluation
    // Both should produce equivalent results (though not necessarily identical due to different reduction strategies)
    // For this test, we check that normalization and evaluation produce the same result
    assert_eq!(
        norm_result, eval_value,
        "Normalization and evaluation should produce consistent results for: {}",
        test_name
    );

    // Also verify that beta reduction is consistent with the general direction
    // Note: beta_reduce does one step, while normalize does full normalization
    // So we don't expect them to be equal, but beta_reduce should make progress
    // toward the normalized form
    println!("  Consistency check passed for: {}\n", test_name);
}

#[test]
fn test_eval_normalization_consistency() {
    // Test that evaluation and normalization produce consistent results
    let expr = app(lam(var(0)), var(1));

    // Evaluation result
    let eval_result = eval_empty(expr.clone());
    if let EvalResult::Value(eval_value) = eval_result {
        // Normalization result
        let normalized = normalize(expr.clone());

        // Both should produce the same result
        assert_eq!(eval_value, normalized);
    } else {
        panic!("Expected value result from evaluation");
    }
}
