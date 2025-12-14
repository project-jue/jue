/// Comprehensive tests for closure application edge cases
use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::eval_relation::{eval, Env, EvalResult};

#[test]
fn test_nested_closure_application() {
    // Test: ((λx.λy.x) a) b → a
    // In De Bruijn: ((λ.λ.1) 0) 1 → 0
    // Tests proper environment handling in nested applications

    let mut env = Env::new();
    env.insert(0, var(10)); // Bind a to 10
    env.insert(1, var(20)); // Bind b to 20

    // Create expression: ((λx.λy.x) a) b
    // λx.λy.x = lam(lam(var(1)))
    // a = var(0) (index 0 in env)
    // b = var(1) (index 1 in env)
    let outer_lam = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let expr = app(app(outer_lam, a), b);

    let result = eval(&env, expr);

    // Should evaluate to a (var(10))
    assert_eq!(result, EvalResult::Value(var(10)));
}

#[test]
fn test_closure_with_free_variables() {
    // Test: (λy.x) z where x is free
    // Should capture x from outer environment

    let mut env = Env::new();
    env.insert(0, var(5)); // Bind x to 5

    // Create lambda y.x where x is free (index 1)
    let lambda = lam(var(1));
    let z = var(2); // Free variable z

    let expr = app(lambda, z);
    let result = eval(&env, expr);

    // Should evaluate to x (var(5)) from environment
    assert_eq!(result, EvalResult::Value(var(5)));
}

#[test]
fn test_shadowing_variables() {
    // Test: (λx.(λx.x) x) y → y
    // Tests variable shadowing and proper index shifting

    let mut env = Env::new();
    env.insert(0, var(100)); // Bind y to 100

    // Create expression: (λx.(λx.x) x) y
    // Outer lambda: λx.(λx.x) x
    // Inner lambda: λx.x
    // x (outer) = var(0)
    // y = var(0) in env
    let inner_lam = lam(var(0));
    let outer_x = var(0);
    let outer_lam = lam(app(inner_lam, outer_x));
    let y = var(0);

    let expr = app(outer_lam, y);
    let result = eval(&env, expr);

    // Should evaluate to y (var(100))
    assert_eq!(result, EvalResult::Value(var(100)));
}

#[test]
fn test_closure_application_to_closure() {
    // Test: (λf.f f) (λx.x) → (λx.x) (λx.x)
    // Tests applying closure to another closure

    let env = Env::new();

    // Create identity function: λx.x
    let identity = lam(var(0));

    // Create function: λf.f f
    let f = var(0);
    let f_f = app(f.clone(), f);
    let func = lam(f_f);

    // Apply: (λf.f f) (λx.x)
    let expr = app(func, identity.clone());
    let result = eval(&env, expr);

    // Should evaluate to identity applied to identity
    match result {
        EvalResult::Value(CoreExpr::App(func_result, arg_result)) => {
            // func_result should be identity
            assert_eq!(*func_result, identity);
            // arg_result should be identity
            assert_eq!(*arg_result, identity);
        }
        _ => panic!("Expected application of identity to identity"),
    }
}

#[test]
fn test_complex_substitution() {
    // Test: (λx.λy.(x y)) (λz.z) → λy.( (λz.z) y )
    // Tests complex substitution in nested lambdas

    let env = Env::new();

    // Create identity: λz.z
    let identity = lam(var(0));

    // Create function: λx.λy.(x y)
    let x = var(0);
    let y = var(0);
    let x_y = app(x, y);
    let inner_lam = lam(x_y);
    let func = lam(inner_lam);

    // Apply: (λx.λy.(x y)) (λz.z)
    let expr = app(func, identity);
    let result = eval(&env, expr);

    // Should evaluate to λy.(identity y)
    match result {
        EvalResult::Value(CoreExpr::Lam(body)) => {
            // Body should be λy.(identity y)
            match *body {
                CoreExpr::Lam(inner_body) => {
                    // Inner body should be (identity y) = (var(1) var(0))
                    match *inner_body {
                        CoreExpr::App(func_expr, arg_expr) => {
                            // func_expr should be identity (var(1))
                            assert_eq!(*func_expr, var(1));
                            // arg_expr should be y (var(0))
                            assert_eq!(*arg_expr, var(0));
                        }
                        _ => panic!("Expected application in inner body"),
                    }
                }
                _ => panic!("Expected lambda in body"),
            }
        }
        _ => panic!("Expected lambda result"),
    }
}

#[test]
fn test_environment_lookup_edge_cases() {
    // Test free variable lookup (index >= env.len())
    // Should return the variable as-is

    let env = Env::new();

    // Create a free variable with high index
    let free_var = var(999);
    let result = eval(&env, free_var);

    // Should return the free variable as-is
    assert_eq!(result, EvalResult::Value(var(999)));
}

#[test]
fn test_closure_with_multiple_free_variables() {
    // Test: (λz.x y) w where x and y are free
    // Should capture both x and y from environment

    let mut env = Env::new();
    env.insert(0, var(10)); // Bind x to 10
    env.insert(1, var(20)); // Bind y to 20

    // Create lambda z.x y where x=var(1), y=var(2)
    let x = var(1);
    let y = var(2);
    let x_y = app(x, y);
    let lambda = lam(x_y);

    let w = var(3); // Free variable w
    let expr = app(lambda, w);
    let result = eval(&env, expr);

    // Should evaluate to x y = var(10) var(20)
    match result {
        EvalResult::Value(CoreExpr::App(func, arg)) => {
            assert_eq!(*func, var(10));
            assert_eq!(*arg, var(20));
        }
        _ => panic!("Expected application result"),
    }
}

#[test]
fn test_closure_application_preserves_environment() {
    // Test that closure application doesn't mutate the original environment

    let mut env = Env::new();
    env.insert(0, var(5));
    env.insert(1, var(10));

    // Create original environment copy
    let original_env = env.clone();

    // Create and apply a closure
    let lambda = lam(var(1)); // λy.x (captures x from env)
    let arg = var(20);
    let expr = app(lambda, arg);
    let _result = eval(&env, expr);

    // Environment should be unchanged
    assert_eq!(env, original_env);
}