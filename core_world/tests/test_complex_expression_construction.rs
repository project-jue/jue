/// Test for complex expression construction functionality
use core_world::core_expr::{app, lam, var, CoreExpr};

#[test]
fn test_complex_expression_construction() {
    // Test construction of complex nested expressions
    let identity = lam(var(0));
    let _v0 = var(0);
    let v1 = var(1);

    // Build: (λx.x) ((λy.y) z)
    let inner_app = app(identity.clone(), v1.clone());
    let outer_app = app(identity.clone(), inner_app);

    assert!(matches!(outer_app, CoreExpr::App(..)));

    // Verify structure
    if let CoreExpr::App(func, arg) = outer_app {
        assert!(matches!(*func, CoreExpr::Lam(_)));
        assert!(matches!(*arg, CoreExpr::App(..)));
    }
}

#[test]
fn test_deeply_nested_expressions() {
    // Test construction of deeply nested expressions
    let identity = lam(var(0));

    // Build: (λx.x) ((λx.x) ((λx.x) ((λx.x) z)))
    let v3 = var(3);
    let innermost = app(identity.clone(), v3);
    let level3 = app(identity.clone(), innermost);
    let level2 = app(identity.clone(), level3);
    let deeply_nested = app(identity.clone(), level2);

    // Verify the structure is correct
    assert!(matches!(deeply_nested, CoreExpr::App(..)));

    // Verify we can traverse the entire structure
    if let CoreExpr::App(_, arg) = deeply_nested {
        if let CoreExpr::App(_, arg) = *arg {
            if let CoreExpr::App(_, arg) = *arg {
                if let CoreExpr::App(_, arg) = *arg {
                    assert!(matches!(*arg, CoreExpr::Var(3)));
                }
            }
        }
    }
}

#[test]
fn test_large_expression_with_many_levels() {
    // Test construction of large expressions with many levels of nesting
    let identity = lam(var(0));

    // Build a chain of 10 nested applications: (λx.x) ((λx.x) ((λx.x) ... (λx.x) z)...)
    let mut current_expr = app(identity.clone(), var(10));
    for i in 1..10 {
        current_expr = app(identity.clone(), current_expr);
    }

    // Verify we can traverse all levels
    let mut expr = current_expr;
    for _ in 0..10 {
        if let CoreExpr::App(_, arg) = expr {
            expr = *arg;
        } else {
            panic!("Expected nested application structure");
        }
    }

    // Should reach the base variable
    assert!(matches!(expr, CoreExpr::Var(10)));
}

#[test]
fn test_de_bruijn_indices_in_complex_constructions() {
    // Test De Bruijn indices in complex constructions
    // Build: (λx.λy.x y) (λz.z)  - This should demonstrate proper index handling

    // λx.λy.x y
    let inner_lam = lam(app(var(1), var(0))); // x is index 1, y is index 0 in inner lambda
    let outer_lam = lam(inner_lam);

    // λz.z
    let identity = lam(var(0));

    // Application: (λx.λy.x y) (λz.z)
    let complex_app = app(outer_lam, identity);

    // Verify structure
    assert!(matches!(complex_app, CoreExpr::App(..)));

    if let CoreExpr::App(func, arg) = complex_app {
        assert!(matches!(*func, CoreExpr::Lam(_)));
        assert!(matches!(*arg, CoreExpr::Lam(_)));

        // Verify the function part has correct structure
        if let CoreExpr::Lam(inner_func) = *func {
            if let CoreExpr::Lam(innermost_func) = *inner_func {
                if let CoreExpr::App(app_func, app_arg) = *innermost_func {
                    assert!(matches!(*app_func, CoreExpr::Var(1))); // x (index 1)
                    assert!(matches!(*app_arg, CoreExpr::Var(0)));  // y (index 0)
                }
            }
        }
    }
}

#[test]
fn test_church_numerals_construction() {
    // Test construction of Church numerals
    // Church numeral for 2: λf.λx.f (f x)
    let f = var(1);
    let x = var(0);
    let fx = app(f.clone(), x.clone());
    let f_fx = app(f.clone(), fx);
    let inner_lam = lam(f_fx);
    let church_two = lam(inner_lam);

    // Verify structure
    assert!(matches!(church_two, CoreExpr::Lam(_)));

    if let CoreExpr::Lam(outer_body) = church_two {
        if let CoreExpr::Lam(inner_body) = *outer_body {
            if let CoreExpr::App(func, arg) = *inner_body {
                assert!(matches!(*func, CoreExpr::Var(1))); // f

                if let CoreExpr::App(inner_func, inner_arg) = *arg {
                    assert!(matches!(*inner_func, CoreExpr::Var(1))); // f
                    assert!(matches!(*inner_arg, CoreExpr::Var(0)));  // x
                }
            }
        }
    }
}

#[test]
fn test_complex_combinator_patterns() {
    // Test construction of complex combinator patterns
    // Build S combinator: λx.λy.λz.x z (y z)
    let x = var(2);
    let y = var(1);
    let z = var(0);

    // x z (y z)
    let yz = app(y.clone(), z.clone());
    let xz = app(x.clone(), z.clone());
    let xz_yz = app(xz, yz);

    // λz.x z (y z)
    let inner_lam = lam(xz_yz);

    // λy.λz.x z (y z)
    let middle_lam = lam(inner_lam);

    // λx.λy.λz.x z (y z)
    let s_combinator = lam(middle_lam);

    // Verify structure
    assert!(matches!(s_combinator, CoreExpr::Lam(_)));

    // Verify we can traverse the entire S combinator structure
    if let CoreExpr::Lam(outer) = s_combinator {
        if let CoreExpr::Lam(middle) = *outer {
            if let CoreExpr::Lam(inner) = *middle {
                if let CoreExpr::App(left_app, right_app) = *inner {
                    // left_app should be (x z)
                    if let CoreExpr::App(x_app, z_var) = *left_app {
                        assert!(matches!(*x_app, CoreExpr::Var(2))); // x
                        assert!(matches!(*z_var, CoreExpr::Var(0)));  // z
                    }

                    // right_app should be (y z)
                    if let CoreExpr::App(y_app, z_var) = *right_app {
                        assert!(matches!(*y_app, CoreExpr::Var(1))); // y
                        assert!(matches!(*z_var, CoreExpr::Var(0)));  // z
                    }
                }
            }
        }
    }
}

#[test]
fn test_memory_intensive_expression_construction() {
    // Test construction of memory-intensive expressions
    // Build a large expression tree with many branches

    let identity = lam(var(0));
    let v0 = var(0);
    let v1 = var(1);
    let v2 = var(2);

    // Build: ((λx.x) (λx.x)) ((λx.x) (λx.x))
    let left_branch = app(identity.clone(), identity.clone());
    let right_branch = app(identity.clone(), identity.clone());
    let balanced_tree = app(left_branch, right_branch);

    // Verify structure
    assert!(matches!(balanced_tree, CoreExpr::App(..)));

    if let CoreExpr::App(left, right) = balanced_tree {
        assert!(matches!(*left, CoreExpr::App(..)));
        assert!(matches!(*right, CoreExpr::App(..)));
    }
}

#[test]
fn test_boundary_conditions_in_expression_construction() {
    // Test boundary conditions and edge cases
    let identity = lam(var(0));

    // Test with maximum reasonable De Bruijn index
    let large_index_var = var(1000);
    let large_index_expr = app(identity.clone(), large_index_var);

    // Verify it handles large indices correctly
    assert!(matches!(large_index_expr, CoreExpr::App(..)));

    if let CoreExpr::App(func, arg) = large_index_expr {
        assert!(matches!(*func, CoreExpr::Lam(_)));
        assert!(matches!(*arg, CoreExpr::Var(1000)));
    }
}

#[test]
fn test_mixed_expression_types_complexity() {
    // Test complex mixing of all expression types (Var, Lam, App)
    // Build: (λf.(λx.f (λy.x)) (λz.z)) (λa.a)

    // Innermost: λy.x (where x is index 1 in this context)
    let inner_lam = lam(var(1));

    // Middle: λx.f (λy.x)
    let f_var = var(1);
    let f_inner = app(f_var.clone(), inner_lam);
    let middle_lam = lam(f_inner);

    // Left side: λf.(λx.f (λy.x))
    let outer_lam = lam(middle_lam);

    // Right side: λz.z
    let right_identity = lam(var(0));

    // Application: (λf.(λx.f (λy.x))) (λz.z)
    let left_app = app(outer_lam, right_identity);

    // Final application: ((λf.(λx.f (λy.x)) (λz.z)) (λa.a))
    let final_identity = lam(var(0));
    let complex_mixed = app(left_app, final_identity);

    // Verify the complex structure
    assert!(matches!(complex_mixed, CoreExpr::App(..)));

    // Verify we can traverse the entire structure
    if let CoreExpr::App(outer_func, outer_arg) = complex_mixed {
        assert!(matches!(*outer_arg, CoreExpr::Lam(_)));

        if let CoreExpr::App(inner_func, inner_arg) = *outer_func {
            assert!(matches!(*inner_func, CoreExpr::Lam(_)));
            assert!(matches!(*inner_arg, CoreExpr::Lam(_)));
        }
    }
}
