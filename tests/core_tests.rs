/// Core-World Tests
/// This file contains comprehensive unit tests for Core-World components
/// Tests for CoreExpr, CoreKernel, EvalRelation, and ProofChecker

// Note: This test file should be run from within the core_world crate
// or the core_world crate should be added as a dependency to the main project

#[cfg(feature = "core_world_tests")]
mod core_world_tests {
    use core_world::core_expr::{app, lam, var, CoreExpr};
    use core_world::core_kernel::{alpha_equiv, beta_reduce, normalize, prove_consistency};
    use core_world::eval_relation::{eval, eval_empty, is_normal_form, Env, EvalResult};
    use core_world::proof_checker::{
        attach_proof, prove_alpha_equivalence, prove_beta_reduction,
        prove_consistency as prove_consistency_proof, prove_evaluation, prove_normalization,
        verify_proof, Proof, ProvenExpr,
    };

    #[test]
    fn test_core_expr_creation() {
        // Test variable creation
        let v = var(0);
        assert!(matches!(v, CoreExpr::Var(0)));

        // Test lambda creation
        let l = lam(var(0));
        assert!(matches!(l, CoreExpr::Lam(_)));

        // Test application creation
        let identity = lam(var(0));
        let v = var(1);
        let app_expr = app(identity, v);
        assert!(matches!(app_expr, CoreExpr::App(..)));
    }

    #[test]
    fn test_core_expr_display() {
        // Test variable display
        let v = var(5);
        assert_eq!(format!("{}", v), "5");

        // Test lambda display
        let l = lam(var(0));
        assert_eq!(format!("{}", l), "λx.0");

        // Test application display
        let identity = lam(var(0));
        let v = var(1);
        let app_expr = app(identity, v);
        assert_eq!(format!("{}", app_expr), "(λx.0) 1");

        // Test nested display
        let nested = app(lam(app(var(1), var(0))), lam(var(0)));
        assert_eq!(format!("{}", nested), "(λx.(1 0)) λx.0");
    }

    #[test]
    fn test_beta_reduction() {
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
        let v = var(5);
        let reduced = beta_reduce(v);
        assert_eq!(reduced, var(5));
    }

    #[test]
    fn test_alpha_equivalence() {
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
    }

    #[test]
    fn test_normalization() {
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
        let normalized = normalize(identity);
        assert_eq!(normalized, identity);
    }

    #[test]
    fn test_kernel_consistency() {
        assert!(prove_consistency());
    }

    #[test]
    fn test_eval_relation_var_lookup() {
        let mut env = Env::new();
        env.insert(0, var(5));

        // Test bound variable
        let result = eval(&env, var(0));
        assert_eq!(result, EvalResult::Value(var(5)));

        // Test free variable
        let result = eval(&env, var(1));
        assert_eq!(result, EvalResult::Value(var(1)));
    }

    #[test]
    fn test_eval_relation_lam_intro() {
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

    #[test]
    fn test_eval_relation_app_elim() {
        // Test simple application: (λx.x) y → y
        let env = Env::new();
        let identity = lam(var(0));
        let y = var(1);
        let app_expr = app(identity, y);

        let result = eval(&env, app_expr);
        assert_eq!(result, EvalResult::Value(var(1)));

        // Test complex application: (λx.λy.x) a b → a
        let env = Env::new();
        let outer_lam = lam(lam(var(1)));
        let a = var(0);
        let b = var(1);
        let app_expr = app(app(outer_lam, a), b);

        let result = eval(&env, app_expr);
        assert_eq!(result, EvalResult::Value(var(0)));
    }

    #[test]
    fn test_eval_empty_environment() {
        let expr = lam(var(0));
        let result = eval_empty(expr);

        match result {
            EvalResult::Closure(closure) => {
                assert!(closure.env.is_empty());
                assert_eq!(closure.body, var(0));
            }
            _ => panic!("Expected closure"),
        }
    }

    #[test]
    fn test_normal_form_detection() {
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
    }

    #[test]
    fn test_proof_verification() {
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
    }

    #[test]
    fn test_proven_expr() {
        let expr = lam(var(0));
        let proof = prove_evaluation(expr.clone());
        let proven_expr = attach_proof(expr.clone(), proof);

        assert!(proven_expr.verify());
        assert_eq!(proven_expr.expr, expr);
    }

    #[test]
    fn test_composite_proof() {
        let expr = app(lam(var(0)), var(1));

        let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
        let eval_proof = prove_evaluation(expr.clone());

        let composite_proof = Proof::Composite {
            proofs: vec![beta_proof, eval_proof],
            conclusion: "β-reduction and evaluation".to_string(),
        };

        assert!(verify_proof(&composite_proof, &expr));
    }

    #[test]
    fn test_invalid_proof() {
        let expr1 = lam(var(0));
        let expr2 = lam(var(1));

        // This should fail because the expressions are not α-equivalent
        let proof = Proof::AlphaEquivalence {
            expr1: expr1.clone(),
            expr2: expr2.clone(),
        };

        assert!(!verify_proof(&proof, &expr1));
    }

    #[test]
    fn test_integration_core_kernel_eval() {
        // Test that β-reduction and evaluation produce consistent results
        let expr = app(lam(var(0)), var(1));

        // β-reduction result
        let beta_reduced = beta_reduce(expr.clone());

        // Evaluation result
        let eval_result = eval_empty(expr.clone());
        if let EvalResult::Value(eval_value) = eval_result {
            // Both should produce the same result (var(1))
            assert_eq!(beta_reduced, eval_value);
        } else {
            panic!("Expected value result from evaluation");
        }
    }

    #[test]
    fn test_integration_proof_system() {
        // Test that the proof system correctly verifies kernel operations
        let expr = app(lam(var(0)), var(1));

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

    #[test]
    fn test_edge_cases() {
        // Test deeply nested expressions
        let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));

        let normalized = normalize(deeply_nested);
        // Should normalize to var(0) after multiple reductions
        assert_eq!(normalized, var(0));

        // Test evaluation of deeply nested expression
        let eval_result = eval_empty(deeply_nested);
        assert!(is_normal_form(&eval_result));

        // Test proof generation for complex expression
        let proof = prove_normalization(deeply_nested);
        assert!(verify_proof(&proof, &deeply_nested));
    }
}
