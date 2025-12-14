/// Test for application display functionality
use core_world::core_expr::{app, lam, var};

#[test]
fn test_application_display() {
    // Test application display formatting
    let identity = lam(var(0));
    let v0 = var(0);
    let v1 = var(1);

    let simple_app = app(identity.clone(), v0.clone());
    let complex_app = app(app(identity.clone(), v0.clone()), v1.clone());

    assert_eq!(format!("{}", simple_app), "(λx.0) 0");
    assert_eq!(format!("{}", complex_app), "((λx.0) 0) 1");
}

#[test]
fn test_simple_applications_with_different_lambdas() {
    // Test various lambda expressions applied to variables
    let identity = lam(var(0));
    let constant = lam(var(1));
    let successor = lam(app(var(0), var(1)));

    let v0 = var(0);
    let v1 = var(1);

    // Identity function applied to variable 0
    let app1 = app(identity.clone(), v0.clone());
    assert_eq!(format!("{}", app1), "(λx.0) 0");

    // Constant function applied to variable 1
    let app2 = app(constant.clone(), v1.clone());
    assert_eq!(format!("{}", app2), "(λx.1) 1");

    // Successor function applied to variable 0
    let app3 = app(successor.clone(), v0.clone());
    assert_eq!(format!("{}", app3), "(λx.(0 1)) 0");
}

#[test]
fn test_complex_nested_applications() {
    // Test deeply nested application structures
    let identity = lam(var(0));
    let v0 = var(0);
    let v1 = var(1);
    let v2 = var(2);

    // ((identity v0) v1) v2
    let nested_app = app(
        app(app(identity.clone(), v0.clone()), v1.clone()),
        v2.clone(),
    );
    assert_eq!(format!("{}", nested_app), "(((λx.0) 0) 1) 2");

    // identity (identity v0)
    let nested_app2 = app(identity.clone(), app(identity.clone(), v0.clone()));
    assert_eq!(format!("{}", nested_app2), "(λx.0) ((λx.0) 0)");
}

#[test]
fn test_applications_with_multiple_variables() {
    // Test applications involving multiple different variables
    let identity = lam(var(0));
    let v0 = var(0);
    let v1 = var(1);
    let v2 = var(2);
    let v3 = var(3);

    // Create a complex expression: ((identity v0) v1) applied to v2, then applied to v3
    let complex_expr = app(
        app(app(identity.clone(), v0.clone()), v1.clone()),
        v2.clone(),
    );
    let final_expr = app(complex_expr, v3.clone());

    assert_eq!(format!("{}", final_expr), "((((λx.0) 0) 1) 2) 3");
}

#[test]
fn test_deeply_nested_structures() {
    // Test applications with deep nesting levels
    let identity = lam(var(0));
    let v0 = var(0);

    // Build a deeply nested application: identity(identity(identity(...identity(v0))))
    let mut deep_app = app(identity.clone(), v0.clone());
    for _ in 0..5 {
        deep_app = app(identity.clone(), deep_app);
    }

    // Should show proper nesting with parentheses
    let expected = "(λx.0) ((λx.0) ((λx.0) ((λx.0) ((λx.0) ((λx.0) 0)))))";
    assert_eq!(format!("{}", deep_app), expected);
}

#[test]
fn test_edge_cases_identity_and_constant_functions() {
    // Test identity function applications
    let identity = lam(var(0));
    let v0 = var(0);
    let v1 = var(1);

    // Identity function applied to itself
    let self_app = app(identity.clone(), identity.clone());
    assert_eq!(format!("{}", self_app), "(λx.0) λx.0");

    // Identity applied to variable
    let identity_app = app(identity.clone(), v0.clone());
    assert_eq!(format!("{}", identity_app), "(λx.0) 0");

    // Constant function (always returns first argument)
    let constant_fn = lam(var(1));
    let const_app = app(constant_fn.clone(), v0.clone());
    assert_eq!(format!("{}", const_app), "(λx.1) 0");

    // Nested constant functions
    let nested_const = app(app(constant_fn.clone(), v0.clone()), v1.clone());
    assert_eq!(format!("{}", nested_const), "((λx.1) 0) 1");
}

#[test]
fn test_applications_with_different_lambda_combinations() {
    // Test various combinations of lambdas and variables
    let identity = lam(var(0));
    let self_apply = lam(app(var(0), var(0)));
    let compose = lam(lam(app(app(var(1), var(0)), var(2))));

    let v0 = var(0);
    let _v1 = var(1);

    // Identity applied to self-application lambda
    let app1 = app(identity.clone(), self_apply.clone());
    assert_eq!(format!("{}", app1), "(λx.0) λx.(0 0)");

    // Self-application lambda applied to identity
    let app2 = app(self_apply.clone(), identity.clone());
    assert_eq!(format!("{}", app2), "(λx.(0 0)) λx.0");

    // Composition function applied to two arguments
    let app3 = app(app(compose.clone(), identity.clone()), self_apply.clone());
    assert_eq!(format!("{}", app3), "((λx.λx.((1 0) 2)) λx.0) λx.(0 0)");

    // Complex nested application with different lambdas
    let complex = app(
        app(identity.clone(), compose.clone()),
        app(self_apply.clone(), v0.clone()),
    );
    assert_eq!(
        format!("{}", complex),
        "((λx.0) λx.λx.((1 0) 2)) ((λx.(0 0)) 0)"
    );
}

#[test]
fn test_application_display_with_complex_lambda_bodies() {
    // Test applications where lambdas have complex bodies
    let complex_lambda1 = lam(app(var(0), app(var(1), var(2))));
    let complex_lambda2 = lam(lam(app(app(var(2), var(1)), var(0))));

    let v0 = var(0);
    let v1 = var(1);

    // Complex lambda applied to variables
    let app1 = app(complex_lambda1.clone(), v0.clone());
    assert_eq!(format!("{}", app1), "(λx.(0 (1 2))) 0");

    // Nested complex lambdas
    let app2 = app(app(complex_lambda1.clone(), v0.clone()), v1.clone());
    assert_eq!(format!("{}", app2), "((λx.(0 (1 2))) 0) 1");

    // Very complex nested structure
    let app3 = app(
        app(complex_lambda2.clone(), v0.clone()),
        app(complex_lambda1.clone(), v1.clone()),
    );
    assert_eq!(
        format!("{}", app3),
        "((λx.λx.((2 1) 0)) 0) ((λx.(0 (1 2))) 1)"
    );
}

#[test]
fn test_application_chains() {
    // Test chains of applications (left-associative)
    let identity = lam(var(0));
    let v0 = var(0);
    let v1 = var(1);
    let v2 = var(2);

    // Create a chain: (((identity v0) v1) v2)
    let chain = app(
        app(app(identity.clone(), v0.clone()), v1.clone()),
        v2.clone(),
    );
    assert_eq!(format!("{}", chain), "(((λx.0) 0) 1) 2");

    // Another chain with different structure
    let chain2 = app(
        identity.clone(),
        app(identity.clone(), app(v0.clone(), v1.clone())),
    );
    assert_eq!(format!("{}", chain2), "(λx.0) ((λx.0) (0 1))");

    // Complex chain with mixed lambdas and variables
    let complex_chain = app(
        app(identity.clone(), v0.clone()),
        app(v1.clone(), v2.clone()),
    );
    assert_eq!(format!("{}", complex_chain), "((λx.0) 0) (1 2)");
}

#[test]
fn test_application_with_higher_order_functions() {
    // Test applications involving higher-order functions
    let apply_twice = lam(lam(app(app(var(1), var(0)), var(0))));
    let identity = lam(var(0));
    let v0 = var(0);

    // Apply-twice function
    let app1 = app(apply_twice.clone(), identity.clone());
    assert_eq!(format!("{}", app1), "(λx.λx.((1 0) 0)) λx.0");

    // Full application: apply_twice identity v0
    let app2 = app(app(apply_twice.clone(), identity.clone()), v0.clone());
    assert_eq!(format!("{}", app2), "((λx.λx.((1 0) 0)) λx.0) 0");

    // Nested higher-order application
    let nested = app(
        app(apply_twice.clone(), apply_twice.clone()),
        identity.clone(),
    );
    assert_eq!(
        format!("{}", nested),
        "((λx.λx.((1 0) 0)) λx.λx.((1 0) 0)) λx.0"
    );
}
