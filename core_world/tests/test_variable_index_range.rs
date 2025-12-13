/// Test for variable index range functionality
use core_world::core_expr::{var, CoreExpr};

#[test]
fn test_variable_index_range() {
    // Test that variable indices work correctly across reasonable ranges
    for i in 0..1000 {
        let v = var(i);
        assert!(matches!(v, CoreExpr::Var(idx) if idx == i));
        assert_eq!(format!("{}", v), i.to_string());
    }
}
