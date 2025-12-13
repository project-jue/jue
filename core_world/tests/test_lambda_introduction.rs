/// Test for lambda introduction
use core_world::core_expr::{lam, var};
use core_world::eval_relation::{eval, Env, EvalResult};

#[test]
fn test_lambda_introduction() {
    // Test lambda introduction (closure creation)
    let env = Env::new();
    let lambda = lam(var(0));

    let result = eval(&env, lambda);
    match result {
        EvalResult::Closure(closure) => {
            assert_eq!(closure.body, var(0));
            assert!(closure.env.is_empty());
        }
        _ => panic!("Expected closure"),
    }
}
