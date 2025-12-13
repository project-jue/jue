/// Test for variable display functionality
use core_world::core_expr::{var, CoreExpr};

#[test]
fn test_variable_display() {
    // Test variable display formatting
    let v0 = var(0);
    let v1 = var(1);
    let v10 = var(10);

    assert_eq!(format!("{}", v0), "0");
    assert_eq!(format!("{}", v1), "1");
    assert_eq!(format!("{}", v10), "10");
}
