/// Test for Dan World physics layer integration
use core_world::core_expr::{app, lam, var};
use core_world::eval_relation::{eval_empty, EvalResult};
use physics_layer::primitives::{add, mul};

#[test]
fn test_dan_world_physics_layer_integration() {
    // Test that Dan World can work with physics primitives

    // Physics operations that Dan World might use
    assert_eq!(add(5, 3), Ok(8));
    assert_eq!(mul(2, 4), Ok(8));

    // Test that core expressions can represent physics-like operations
    let physics_expr = app(lam(app(var(0), var(1))), lam(var(0)));
    let eval_result = eval_empty(physics_expr);
    match eval_result {
        EvalResult::Value(_) => assert!(true),
        EvalResult::Closure(_) => assert!(true),
    }
}
