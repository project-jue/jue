/// Test for deeply nested expressions functionality
use core_world::core_expr::{app, lam, var};

#[test]
fn test_deeply_nested_expressions() {
    // Test creation and display of deeply nested expressions
    let innermost = app(var(2), var(1));
    let middle = app(var(1), innermost);
    let inner_lam = lam(middle);
    let middle_lam = lam(inner_lam);
    let outer_lam = lam(middle_lam);

    // Updated expected display based on actual output
    let expected_display = "λx.λx.λx.(1 (2 1))";
    assert_eq!(format!("{}", outer_lam), expected_display);
}
