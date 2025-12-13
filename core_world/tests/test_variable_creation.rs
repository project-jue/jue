/// Test for variable creation functionality
use core_world::core_expr::{var, CoreExpr};

#[test]
fn test_variable_creation() {
    // Test basic variable creation
    let v0 = var(0);
    let v1 = var(1);
    let v5 = var(5);

    assert!(matches!(v0, CoreExpr::Var(0)));
    assert!(matches!(v1, CoreExpr::Var(1)));
    assert!(matches!(v5, CoreExpr::Var(5)));
}
