/// Test for performance with large expressions
use core_world::core_expr::CoreExpr;
/// Test for performance with large expressions
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize};
use core_world::eval_relation::{eval, is_normal_form, Env};
#[test]
fn test_performance_with_large_expressions() {
    // Test evaluation performance with large expressions
    let env = Env::new();
    let mut expr = var(0);

    // Build a reasonably large expression
    for _i in 0..10 {
        expr = app(lam(var(0)), expr);
    }

    // Should be able to evaluate without issues
    let result = eval(&env, expr);
    assert!(is_normal_form(&result));
}
#[test]
fn test_performance_with_large_expressions2() {
    // Test that normalization can handle reasonably large expressions
    let mut expr = var(0);

    // Build a large expression: (λx.x) ((λx.x) ((λx.x) ... (λx.x) 0 ...))
    for _i in 0..20 {
        expr = app(lam(var(0)), expr);
    }

    // Should be able to normalize without issues
    let normalized = normalize(expr);
    assert_eq!(normalized, var(0)); // Should reduce all the way to the innermost variable
}

#[test]
fn test_performance_with_large_expressions3() {
    // Test that reduction can handle reasonably large expressions
    let mut expr = var(0);

    // Build a large expression: (λx.x) ((λx.x) ((λx.x) ... (λx.x) 0 ...))
    // In De Bruijn, this creates a chain where each step should reduce properly
    for _i in 0..50 {
        expr = app(lam(var(0)), expr);
    }

    // Should be able to reduce without issues
    // The final result should be the innermost variable after all reductions
    let reduced = beta_reduce(expr);
    // Since we're doing one reduction step with Call-by-Value semantics,
    // it should reduce the outermost application to the innermost variable
    assert!(matches!(reduced, CoreExpr::Var(..)));
}
