use core_world::core_expr::{app, lam, var, CoreExpr};
use serde_json;

#[test]
fn test_core_expr_serialization() {
    // Test simple variable serialization
    let var_expr = var(0);
    let serialized = serde_json::to_string(&var_expr).unwrap();
    let deserialized: CoreExpr = serde_json::from_str(&serialized).unwrap();
    assert_eq!(var_expr, deserialized);

    // Test lambda serialization
    let lam_expr = lam(var(0));
    let serialized = serde_json::to_string(&lam_expr).unwrap();
    let deserialized: CoreExpr = serde_json::from_str(&serialized).unwrap();
    assert_eq!(lam_expr, deserialized);

    // Test application serialization
    let app_expr = app(lam(var(0)), var(1));
    let serialized = serde_json::to_string(&app_expr).unwrap();
    let deserialized: CoreExpr = serde_json::from_str(&serialized).unwrap();
    assert_eq!(app_expr, deserialized);

    // Test complex nested expression serialization
    let complex_expr = app(
        lam(app(var(1), var(0))),
        lam(var(0))
    );
    let serialized = serde_json::to_string(&complex_expr).unwrap();
    let deserialized: CoreExpr = serde_json::from_str(&serialized).unwrap();
    assert_eq!(complex_expr, deserialized);
}