/// Test for complex environment
use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::eval_relation::{eval, Env, EvalResult};

#[test]
fn test_complex_environment() {
    // Test evaluation with complex environment
    let mut env = Env::new();
    env.insert(0, var(10));
    env.insert(1, var(20));
    env.insert(2, app(var(0), var(1)));

    // Test variable lookup
    let result = eval(&env, var(2));
    match result {
        EvalResult::Value(expr) => {
            if let CoreExpr::App(..) = expr {
                // Should be the application
            } else {
                panic!("Expected application value");
            }
        }
        _ => panic!("Expected value result"),
    }
}
