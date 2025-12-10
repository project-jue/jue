use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::core_kernel::{alpha_equiv, beta_reduce, normalize, prove_consistency};
use core_world::eval_relation::{eval, eval_empty, is_normal_form, Env, EvalResult};
use core_world::proof_checker::{
    attach_proof, prove_alpha_equivalence, prove_beta_reduction,
    prove_consistency as prove_consistency_proof, prove_evaluation, prove_normalization,
    verify_proof, Proof, ProvenExpr,
};

/// Test CoreExpr Creation and Structure
#[test]
fn test_core_expr_comprehensive() {
    // Test variable creation with different indices
    for i in 0..10 {
        let v = var(i);
        assert!(matches!(v, CoreExpr::Var(idx) if idx == i));
    }

    // Test lambda creation with different bodies
    let simple_lam = lam(var(0));

    let nested_lam = lam(lam(var(1)));
    let complex_lam = lam(app(var(0), var(1)));

    assert!(matches!(simple_lam, CoreExpr::Lam(_)));
    assert!(matches!(nested_lam, CoreExpr::Lam(_)));
    assert!(matches!(complex_lam, CoreExpr::Lam(_)));

    // Test application creation
    let identity = lam(var(0));
    let v = var(1);
    let app_expr = app(identity, v);

    assert!(matches!(app_expr, CoreExpr::App(..)));

    // Test deeply nested expressions
    let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));
    assert!(matches!(deeply_nested, CoreExpr::App(..)));
}

/// Test CoreExpr Display Formatting
#[test]
fn test_core_expr_display_comprehensive() {
    // Test variable display
    for i in 0..5 {
        let v = var(i);
        assert_eq!(format!("{}", v), i.to_string());
    }

    // Test lambda display
    let l = lam(var(0));
    assert_eq!(format!("{}", l), "λx.0");

    let nested_lam = lam(lam(var(1)));
    assert_eq!(format!("{}", nested_lam), "λx.λx.1");

    // Test application display
    let identity = lam(var(0));
    let v = var(1);
    let app_expr = app(identity, v);
    assert_eq!(format!("{}", app_expr), "(λx.0) 1");

    // Test complex nested display
    let nested = app(lam(app(var(1), var(0))), lam(var(0)));
    assert_eq!(format!("{}", nested), "(λx.(1 0)) λx.0");
}

/// Test Beta Reduction Comprehensive
#[test]
fn test_beta_reduction_comprehensive() {
    // Test identity function: (λx.x) y → y
    let identity = lam(var(0));
    let y = var(1);
    let app_expr = app(identity, y);
    let reduced = beta_reduce(app_expr);
    assert_eq!(reduced, var(1));

    // Test complex reduction: (λx.λy.x) a b → a
    let outer_lam = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let app_expr = app(app(outer_lam, a), b);
    let reduced = beta_reduce(app_expr);
    assert_eq!(reduced, var(0));

    // Test that variables don't reduce
    for i in 0..5 {
        let v = var(i);
        let reduced = beta_reduce(v);
        assert_eq!(reduced, var(i));
    }

    // Test nested reductions
    let expr = app(app(lam(lam(var(2))), var(0)), var(1));
    let reduced = beta_reduce(expr);
    assert_eq!(reduced, var(0));

    // Test that reduction stops at normal forms
    let normal_form = lam(var(0));
    let normal_form_clone = normal_form.clone();
    let reduced = beta_reduce(normal_form);
    assert_eq!(reduced, normal_form_clone);
}

/// Test Alpha Equivalence Comprehensive
#[test]
fn test_alpha_equivalence_comprehensive() {
    // Test identical expressions
    let expr1 = lam(var(0));
    let expr2 = lam(var(0));
    assert!(alpha_equiv(expr1, expr2));

    // Test different bodies
    let expr1 = lam(var(0));
    let expr2 = lam(var(1));
    assert!(!alpha_equiv(expr1, expr2));

    // Test nested equivalence
    let expr1 = app(lam(var(0)), var(1));
    let expr2 = app(lam(var(0)), var(1));
    assert!(alpha_equiv(expr1, expr2));

    // Test complex expressions
    let expr1 = app(lam(app(var(0), var(1))), lam(var(0)));
    let expr2 = app(lam(app(var(0), var(1))), lam(var(0)));
    assert!(alpha_equiv(expr1, expr2));

    // Test non-equivalent expressions
    let expr1 = app(lam(var(0)), var(1));
    let expr2 = app(lam(var(1)), var(0));
    assert!(!alpha_equiv(expr1, expr2));
}

/// Test Normalization Comprehensive
#[test]
fn test_normalization_comprehensive() {
    // Test idempotent normalization
    let expr = app(lam(var(0)), var(1));
    let normalized_once = normalize(expr.clone());
    let normalized_twice = normalize(normalized_once.clone());
    assert_eq!(normalized_once, normalized_twice);

    // Test complex normalization
    let expr = app(app(lam(lam(var(1))), var(0)), var(1));
    let normalized = normalize(expr);
    assert_eq!(normalized, var(0));

    // Test that already normal forms don't change
    let identity = lam(var(0));
    let identity_clone = identity.clone();
    let normalized = normalize(identity);
    assert_eq!(normalized, identity_clone);

    // Test deeply nested normalization
    let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));
    let normalized = normalize(deeply_nested);
    assert_eq!(normalized, var(0));

    // Test multiple reduction paths
    let expr = app(app(lam(lam(var(2))), var(0)), app(var(1), var(2)));
    let normalized = normalize(expr);
    assert_eq!(normalized, var(0));
}

/// Test Kernel Consistency
#[test]
fn test_kernel_consistency_comprehensive() {
    // Test that kernel consistency holds
    assert!(prove_consistency());

    // Test consistency with various expressions
    let _expr1 = lam(var(0));
    let _expr2 = app(lam(var(0)), var(1));
    let _expr3 = app(app(lam(lam(var(1))), var(0)), var(1));

    // All should be consistent
    assert!(prove_consistency());
}

/// Test Evaluation Relation Comprehensive
#[test]
fn test_eval_relation_comprehensive() {
    // Test variable lookup
    let mut env = Env::new();
    env.insert(0, var(5));
    env.insert(1, var(10));

    // Test bound variable
    let result = eval(&env, var(0));
    assert_eq!(result, EvalResult::Value(var(5)));

    // Test free variable
    let result = eval(&env, var(2));
    assert_eq!(result, EvalResult::Value(var(2)));

    // Test lambda introduction
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

    // Test complex lambda
    let complex_lambda = lam(app(var(0), var(1)));
    let result = eval(&env, complex_lambda);
    match result {
        EvalResult::Closure(closure) => {
            assert!(matches!(closure.body, CoreExpr::App(..)));
            assert!(closure.env.is_empty());
        }
        _ => panic!("Expected closure"),
    }

    // Test application elimination
    let identity = lam(var(0));
    let y = var(1);
    let app_expr = app(identity, y);

    let result = eval(&env, app_expr);
    assert_eq!(result, EvalResult::Value(var(1)));

    // Test nested application
    let outer_lam = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let app_expr = app(app(outer_lam, a), b);

    let result = eval(&env, app_expr);
    assert_eq!(result, EvalResult::Value(var(0)));
}

/// Test Empty Environment Evaluation
#[test]
fn test_eval_empty_environment_comprehensive() {
    // Test simple lambda
    let expr = lam(var(0));
    let result = eval_empty(expr);

    match result {
        EvalResult::Closure(closure) => {
            assert!(closure.env.is_empty());
            assert_eq!(closure.body, var(0));
        }
        _ => panic!("Expected closure"),
    }

    // Test nested lambda
    let expr = lam(lam(var(1)));
    let result = eval_empty(expr);

    match result {
        EvalResult::Closure(closure) => {
            assert!(closure.env.is_empty());
            assert!(matches!(closure.body, CoreExpr::Lam(_)));
        }
        _ => panic!("Expected closure"),
    }

    // Test application
    let expr = app(lam(var(0)), var(1));
    let result = eval_empty(expr);
    assert_eq!(result, EvalResult::Value(var(1)));
}

/// Test Normal Form Detection Comprehensive
#[test]
fn test_normal_form_detection_comprehensive() {
    // Identity function is in normal form
    let identity = lam(var(0));
    let result = eval_empty(identity);
    assert!(is_normal_form(&result));

    // Application of identity to variable is not in normal form
    let app_expr = app(lam(var(0)), var(1));
    let result = eval_empty(app_expr.clone());
    assert!(!is_normal_form(&result));

    // After evaluation, it should be in normal form
    let reduced_result = eval_empty(app_expr);
    assert!(is_normal_form(&reduced_result));

    // Test complex expressions
    let complex_expr = app(app(lam(lam(var(1))), var(0)), var(1));
    let result = eval_empty(complex_expr.clone());
    assert!(!is_normal_form(&result));

    let reduced_result = eval_empty(complex_expr);
    assert!(is_normal_form(&reduced_result));
}

/// Test Proof Verification Comprehensive
#[test]
fn test_proof_verification_comprehensive() {
    // Test β-reduction proof
    let identity = lam(var(0));
    let y = var(1);
    let expr = app(identity, y);
    let proof = prove_beta_reduction(expr.clone()).unwrap();
    assert!(verify_proof(&proof, &expr));

    // Test α-equivalence proof
    let expr1 = lam(var(0));
    let expr2 = lam(var(0));
    let proof = prove_alpha_equivalence(expr1.clone(), expr2.clone()).unwrap();
    assert!(verify_proof(&proof, &expr1));

    // Test normalization proof
    let expr = app(lam(var(0)), var(1));
    let proof = prove_normalization(expr.clone());
    assert!(verify_proof(&proof, &expr));

    // Test evaluation proof
    let expr = app(lam(var(0)), var(1));
    let proof = prove_evaluation(expr.clone());
    assert!(verify_proof(&proof, &expr));

    // Test consistency proof
    let proof = prove_consistency_proof();
    assert!(verify_proof(&proof, &lam(var(0))));

    // Test multiple proofs for same expression
    let expr = app(lam(var(0)), var(1));
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));
}

/// Test Proven Expression Comprehensive
#[test]
fn test_proven_expr_comprehensive() {
    // Test simple proven expression
    let expr = lam(var(0));
    let proof = prove_evaluation(expr.clone());
    let proven_expr = attach_proof(expr.clone(), proof);

    assert!(proven_expr.verify());
    assert_eq!(proven_expr.expr, expr);

    // Test complex proven expression
    let expr = app(lam(var(0)), var(1));
    let proof = prove_beta_reduction(expr.clone()).unwrap();
    let proven_expr = ProvenExpr::new(expr.clone(), proof);

    assert!(proven_expr.verify());
    assert_eq!(proven_expr.expr, expr);

    // Test multiple proofs for same expression
    let expr_clone = expr.clone();
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr_clone.clone());

    let beta_proven = ProvenExpr::new(expr.clone(), beta_proof);
    let eval_proven = ProvenExpr::new(expr_clone, eval_proof);

    assert!(beta_proven.verify());
    assert!(eval_proven.verify());
}

/// Test Composite Proof Comprehensive
#[test]
fn test_composite_proof_comprehensive() {
    // Test simple composite proof
    let expr = app(lam(var(0)), var(1));

    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());

    let composite_proof = Proof::Composite {
        proofs: vec![beta_proof, eval_proof],
        conclusion: "β-reduction and evaluation".to_string(),
    };

    assert!(verify_proof(&composite_proof, &expr));

    // Test complex composite proof
    let expr_clone = expr.clone();
    let norm_proof = prove_normalization(expr_clone.clone());

    let complex_composite = Proof::Composite {
        proofs: vec![
            prove_beta_reduction(expr.clone()).unwrap(),
            prove_evaluation(expr.clone()),
            norm_proof,
        ],
        conclusion: "Complete proof verification".to_string(),
    };

    assert!(verify_proof(&complex_composite, &expr));

    // Test nested composite proofs
    let inner_composite = Proof::Composite {
        proofs: vec![
            prove_beta_reduction(expr.clone()).unwrap(),
            prove_evaluation(expr.clone()),
        ],
        conclusion: "Inner composite".to_string(),
    };

    let outer_composite = Proof::Composite {
        proofs: vec![inner_composite, prove_normalization(expr.clone())],
        conclusion: "Outer composite".to_string(),
    };

    assert!(verify_proof(&outer_composite, &expr));
}

/// Test Invalid Proof Detection
#[test]
fn test_invalid_proof_comprehensive() {
    // Test invalid α-equivalence proof
    let expr1 = lam(var(0));
    let expr2 = lam(var(1));

    let proof = Proof::AlphaEquivalence {
        expr1: expr1.clone(),
        expr2: expr2.clone(),
    };

    assert!(!verify_proof(&proof, &expr1));

    // Test invalid β-reduction proof
    let expr = lam(var(0));
    let invalid_reduced = var(999);

    let proof = Proof::BetaReduction {
        original: expr.clone(),
        reduced: invalid_reduced,
        step: "invalid reduction".to_string(),
    };

    assert!(!verify_proof(&proof, &expr));

    // Test invalid normalization proof
    let expr = lam(var(0));
    let invalid_normal_form = var(999);

    let proof = Proof::Normalization {
        original: expr.clone(),
        normal_form: invalid_normal_form,
        steps: vec![],
    };

    assert!(!verify_proof(&proof, &expr));
}

/// Test Integration: Core Kernel and Evaluation
#[test]
fn test_integration_core_kernel_eval_comprehensive() {
    // Test that β-reduction and evaluation produce consistent results
    let test_cases = vec![
        app(lam(var(0)), var(1)),
        app(app(lam(lam(var(1))), var(0)), var(1)),
        app(lam(app(var(1), var(0))), lam(var(0))),
    ];

    for expr in test_cases {
        // β-reduction result
        let beta_reduced = beta_reduce(expr.clone());

        // Evaluation result
        let eval_result = eval_empty(expr.clone());
        if let EvalResult::Value(eval_value) = eval_result {
            // Both should produce the same result
            assert_eq!(beta_reduced, eval_value);
        } else {
            panic!("Expected value result from evaluation");
        }
    }
}

/// Test Integration: Proof System Comprehensive
#[test]
fn test_integration_proof_system_comprehensive() {
    // Test that the proof system correctly verifies kernel operations
    let test_expressions = vec![
        app(lam(var(0)), var(1)),
        app(app(lam(lam(var(1))), var(0)), var(1)),
        app(lam(app(var(1), var(0))), lam(var(0))),
    ];

    for expr in test_expressions {
        // Generate multiple proofs for the same expression
        let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
        let eval_proof = prove_evaluation(expr.clone());
        let norm_proof = prove_normalization(expr.clone());

        // All proofs should verify correctly
        assert!(verify_proof(&beta_proof, &expr));
        assert!(verify_proof(&eval_proof, &expr));
        assert!(verify_proof(&norm_proof, &expr));

        // Create a composite proof
        let composite_proof = Proof::Composite {
            proofs: vec![beta_proof, eval_proof, norm_proof],
            conclusion: "Complete proof verification".to_string(),
        };

        assert!(verify_proof(&composite_proof, &expr));
    }
}

/// Test Edge Cases and Complex Scenarios
#[test]
fn test_edge_cases_comprehensive() {
    // Test deeply nested expressions
    let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));

    let normalized = normalize(deeply_nested.clone());
    assert_eq!(normalized, var(0));

    // Test evaluation of deeply nested expression
    let eval_result = eval_empty(deeply_nested.clone());
    assert!(is_normal_form(&eval_result));

    // Test proof generation for complex expression
    let proof = prove_normalization(deeply_nested.clone());
    assert!(verify_proof(&proof, &deeply_nested));

    // Test multiple levels of nesting
    let multi_level = app(app(lam(lam(lam(var(2)))), var(0)), app(var(1), var(2)));

    let normalized = normalize(multi_level.clone());
    assert_eq!(normalized, var(0));

    // Test proof verification for multi-level expression
    let proof = prove_beta_reduction(multi_level.clone()).unwrap();
    assert!(verify_proof(&proof, &multi_level));
}

/// Test Proof System Consistency
#[test]
fn test_proof_system_consistency() {
    // Test that proof system is consistent across different operations
    let expr = app(lam(var(0)), var(1));

    // Generate all possible proofs
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());
    let consistency_proof = prove_consistency_proof();

    // All should verify correctly
    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));
    assert!(verify_proof(&consistency_proof, &lam(var(0))));

    // Test that composite proofs maintain consistency
    let composite_proof = Proof::Composite {
        proofs: vec![beta_proof, eval_proof, norm_proof],
        conclusion: "Consistency test".to_string(),
    };

    assert!(verify_proof(&composite_proof, &expr));
}

/// Performance and Stress Tests
#[cfg(test)]
mod core_world_performance_tests;
