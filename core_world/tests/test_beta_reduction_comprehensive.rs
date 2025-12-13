/// Beta Reduction Comprehensive Tests
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize};

#[test]
fn test_beta_reduction_comprehensive() {
    // Test identity function: (λx.x) y → y
    let identity = lam(var(0));
    let y = var(1);
    let app_expr = app(identity, y);
    let reduced = beta_reduce(app_expr);
    assert_eq!(reduced, var(1));

    // Test complex reduction: (λx.λy.x) a b → var(0) after one step with Call-by-Value
    // In Call-by-Value, the argument 'a' (var(0)) is evaluated first, then substituted
    let outer_lam = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let app_expr = app(app(outer_lam, a), b);
    let reduced = beta_reduce(app_expr);
    // With Call-by-Value: ((λ.λ.1) 0) 1 → (λ.1) 1 → 0 (after substitution)
    // This demonstrates Call-by-Value: arguments are evaluated before substitution
    assert_eq!(reduced, var(0));

    // Test that variables don't reduce
    for i in 0..5 {
        let v = var(i);
        let reduced = beta_reduce(v);
        assert_eq!(reduced, var(i));
    }

    // Test nested reductions - the result should be var(0) because:
    // ((λx.λy.2) 0) 1 → (λy.2) 1 → 2 (but at depth 1, so it becomes 0)
    let expr = app(app(lam(lam(var(2))), var(0)), var(1));
    println!("Debug: Original expression: {:?}", expr);

    let step1 = beta_reduce(expr.clone());
    println!("Debug: After first beta reduction: {:?}", step1);

    let normalized = normalize(expr);
    println!("Debug: Fully normalized: {:?}", normalized);

    // The normalization produces var(0) which is correct
    // because the final evaluation resolves to the original 'a' which becomes index 0
    // after the environment is properly handled. This is the expected behavior.
    assert_eq!(normalized, var(0));

    // Test that reduction stops at normal forms
    let normal_form = lam(var(0));
    let normal_form_clone = normal_form.clone();
    let reduced = beta_reduce(normal_form);
    assert_eq!(reduced, normal_form_clone);
}
