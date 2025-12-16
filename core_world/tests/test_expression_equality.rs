/// Test for expression equality functionality
use core_world::core_expr::{app, lam, var};

#[test]
fn test_expression_equality() {
    // Test expression equality
    let expr1 = lam(var(0));
    let expr2 = lam(var(0));
    let expr3 = lam(var(1));

    assert_eq!(expr1, expr2);
    assert_ne!(expr1, expr3);

    // Test with applications
    let app1 = app(lam(var(0)), var(1));
    let app2 = app(lam(var(0)), var(1));
    let app3 = app(lam(var(1)), var(0));

    assert_eq!(app1, app2);
    assert_ne!(app1, app3);
}
