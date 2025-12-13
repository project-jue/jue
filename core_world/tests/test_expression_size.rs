/// Test for expression size functionality
use core_world::core_expr::{app, lam, var, CoreExpr};

#[test]
fn test_expression_size() {
    // Test that expressions can handle reasonable sizes
    let mut expr = var(0);

    // Build a chain of 100 applications
    for _ in 0..100 {
        expr = app(lam(var(0)), expr);
    }

    // Should be able to display without issues
    let display = format!("{}", expr);
    assert!(display.len() > 100); // Should be reasonably long
}
