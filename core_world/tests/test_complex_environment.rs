/// Comprehensive tests for complex environment scenarios
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

#[test]
fn test_variable_shadowing() {
    // Test variable shadowing in nested environments
    let mut env = Env::new();
    env.insert(0, var(100)); // Outer x = 100
    env.insert(1, var(200)); // Outer y = 200

    // Create lambda that shadows x: λx.λy.x
    // In De Bruijn: λ.λ.0 (inner x shadows outer x)
    let shadowing_lambda = lam(lam(var(0)));

    let result = eval(&env, shadowing_lambda);
    match result {
        EvalResult::Closure(_closure) => {
            // Apply to argument 5 (should become new x)
            let arg = var(5);
            let app_result = eval(&env, app(lam(lam(var(0))), arg));

            // Should resolve to the argument (5), not the outer x (100)
            assert_eq!(app_result, EvalResult::Value(var(5)));
        }
        _ => panic!("Expected closure"),
    }
}

#[test]
fn test_nested_lambda_with_complex_environment() {
    // Test nested lambda expressions with complex environment bindings
    let mut env = Env::new();
    env.insert(0, var(10));  // a = 10
    env.insert(1, var(20));  // b = 20
    env.insert(2, var(30));  // c = 30

    // Create nested lambda: λx.λy.λz.x (a) (b) (c)
    // In De Bruijn: λ.λ.λ.2
    let nested_lambda = lam(lam(lam(var(2))));

    let result = eval(&env, nested_lambda.clone());
    match result {
        EvalResult::Closure(_) => {
            // Apply step by step
            let app1 = app(nested_lambda.clone(), var(100)); // Apply to 100
            let result1 = eval(&env, app1);

            match result1 {
                EvalResult::Closure(_closure1) => {
                    let app2 = app(nested_lambda.clone(), var(200)); // Apply to 200
                    let result2 = eval(&env, app2);
        
                    match result2 {
                        EvalResult::Closure(_closure2) => {
                            let app3 = app(nested_lambda.clone(), var(300)); // Apply to 300
                            let result3 = eval(&env, app3);

                            // Should resolve to the original a (10) from environment
                            assert_eq!(result3, EvalResult::Value(var(10)));
                        }
                        _ => panic!("Expected closure after second application"),
                    }
                }
                _ => panic!("Expected closure after first application"),
            }
        }
        _ => panic!("Expected closure"),
    }
}

#[test]
fn test_large_environment() {
    // Test with large environment (many bindings)
    let mut env = Env::new();

    // Create environment with 50 bindings
    for i in 0..50 {
        env.insert(i, var(i + 1000));
    }

    // Test accessing variables at different positions
    let result_beginning = eval(&env, var(0));
    let result_middle = eval(&env, var(25));
    let result_end = eval(&env, var(49));

    assert_eq!(result_beginning, EvalResult::Value(var(1000)));
    assert_eq!(result_middle, EvalResult::Value(var(1025)));
    assert_eq!(result_end, EvalResult::Value(var(1049)));

    // Test accessing beyond environment (free variable)
    let result_free = eval(&env, var(50));
    assert_eq!(result_free, EvalResult::Value(var(50)));
}

#[test]
fn test_de_bruijn_indices_edge_cases() {
    // Test edge cases with De Bruijn indices
    let mut env = Env::new();
    env.insert(0, var(0)); // Self-referential variable

    // Test lambda that references itself through environment
    let self_ref_lambda = lam(var(1)); // References index 1 (the self-referential var)

    let result = eval(&env, self_ref_lambda);
    match result {
        EvalResult::Closure(_closure) => {
            // Apply to some argument
            let arg = var(42);
            let app_expr = app(lam(var(1)), arg);
            let app_result = eval(&env, app_expr);

            // Should resolve to the self-referential variable from environment
            assert_eq!(app_result, EvalResult::Value(var(0)));
        }
        _ => panic!("Expected closure"),
    }
}

#[test]
fn test_error_conditions() {
    // Test error conditions and boundary cases
    let env = Env::new();

    // Test evaluation of free variable (no binding in environment)
    let free_var_result = eval(&env, var(999));
    assert_eq!(free_var_result, EvalResult::Value(var(999)));

    // Test deeply nested application
    let deeply_nested = app(
        app(lam(var(0)), var(1)),
        app(lam(var(0)), var(2))
    );

    let nested_result = eval(&env, deeply_nested);
    // Should reduce to var(2) after full evaluation
    assert_eq!(nested_result, EvalResult::Value(var(2)));
}

#[test]
fn test_mixed_expression_types_in_environment() {
    // Test interaction between different expression types in environments
    let mut env = Env::new();

    // Add different types to environment
    env.insert(0, var(10));                    // Variable
    env.insert(1, lam(var(0)));               // Lambda (identity)
    env.insert(2, app(var(0), var(1)));       // Application
    env.insert(3, lam(app(var(1), var(0))));  // Lambda with application body

    // Test accessing each type
    let var_result = eval(&env, var(0));
    let lam_result = eval(&env, var(1));
    let app_result = eval(&env, var(2));
    let complex_lam_result = eval(&env, var(3));

    assert_eq!(var_result, EvalResult::Value(var(10)));

    match lam_result {
        EvalResult::Closure(closure) => {
            assert_eq!(closure.body, var(0));
        }
        _ => panic!("Expected closure for lambda"),
    }

    match app_result {
        EvalResult::Value(CoreExpr::App(..)) => {}
        _ => panic!("Expected application value"),
    }

    match complex_lam_result {
        EvalResult::Closure(closure) => {
            match closure.body {
                CoreExpr::App(..) => {}
                _ => panic!("Expected application in lambda body"),
            }
        }
        _ => panic!("Expected closure for complex lambda"),
    }
}

#[test]
fn test_environment_with_lambda_applications() {
    // Test environment containing lambda applications
    let mut env = Env::new();

    // Create identity function: λx.x
    let identity = lam(var(0));
    env.insert(0, identity);

    // Create constant function: λx.λy.x
    let constant = lam(lam(var(1)));
    env.insert(1, constant);

    // Test applying functions from environment
    let app_identity = app(var(0), var(100)); // Apply identity to 100
    let app_result = eval(&env, app_identity);
    assert_eq!(app_result, EvalResult::Value(var(100)));

    // Test applying constant function
    let app_constant = app(app(var(1), var(200)), var(300)); // Apply constant 200 to 300
    let constant_result = eval(&env, app_constant);
    assert_eq!(constant_result, EvalResult::Value(var(200)));
}

#[test]
fn test_recursion_limit_behavior() {
    // Test behavior with deeply nested expressions that might hit recursion limit
    let env = Env::new();

    // Create a deeply nested expression: (((...((x y) z) w) ...) v)
    let mut nested_expr = app(var(0), var(1));
    for i in 2..20 {
        nested_expr = app(nested_expr, var(i));
    }

    // This should not panic, but return a value (possibly unevaluated due to limit)
    let result = eval(&env, nested_expr);
    match result {
        EvalResult::Value(_) => {} // Expected behavior
        _ => panic!("Expected value result even with recursion limit"),
    }
}
