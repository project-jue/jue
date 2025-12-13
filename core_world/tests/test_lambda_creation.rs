/// Test for lambda creation functionality
use core_world::core_expr::{lam, var, CoreExpr};

#[test]
fn test_lambda_creation() {
    // Test basic lambda creation
    let lam0 = lam(var(0));
    let lam1 = lam(var(1));
    let nested_lam = lam(lam(var(0)));

    assert!(matches!(lam0, CoreExpr::Lam(_)));
    assert!(matches!(lam1, CoreExpr::Lam(_)));
    assert!(matches!(nested_lam, CoreExpr::Lam(_)));
}
