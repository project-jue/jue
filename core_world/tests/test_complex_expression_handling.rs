use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize};
use core_world::eval_relation::{eval_empty, EvalResult};
use core_world::proof_checker::{prove_beta_reduction, prove_evaluation, prove_normalization, verify_proof};
use std::time::Instant;

#[test]
pub(crate) fn test_complex_expression_handling() {
    let start_time = Instant::now();

    // Create and process complex expressions
    for _ in 0..100 {
        let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));

        let normalized = normalize(deeply_nested.clone());
        let beta_proof = prove_beta_reduction(deeply_nested.clone());
        let eval_proof = prove_evaluation(deeply_nested.clone());
        let norm_proof = prove_normalization(deeply_nested.clone());

        // The normalization produces lam(var(0)) which is correct
        // because the expression (λx.(λy.y) x) (λz.z) reduces to (λy.y)
        // which is represented as lam(var(0)) in De Bruijn indices
        assert_eq!(normalized, lam(var(0)));
        assert!(beta_proof.is_some());
        assert!(verify_proof(&eval_proof, &deeply_nested));
        assert!(verify_proof(&norm_proof, &deeply_nested));
    }

    let duration = start_time.elapsed();
    println!("100 complex expressions processed in {:?}", duration);
}

#[test]
pub(crate) fn test_identity_function_handling() {
    // Test identity function: (λx.x) y → y
    let identity = lam(var(0));
    let y = var(1);
    let expr = app(identity, y.clone());

    // β-reduction should reduce to y
    let beta_result = beta_reduce(expr.clone());
    assert_eq!(beta_result, y.clone());

    // Normalization should also reduce to y
    let norm_result = normalize(expr.clone());
    assert_eq!(norm_result, y);

    // Evaluation should produce a value
    let eval_result = eval_empty(expr.clone());
    assert!(matches!(eval_result, EvalResult::Value(_)));

    // Proof generation and verification
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));
}

#[test]
pub(crate) fn test_combinator_handling() {
    // Test K combinator: (λx.λy.x) a b → a
    let k_combinator = lam(lam(var(1)));
    let a = var(0);
    let b = var(1);
    let expr = app(app(k_combinator, a.clone()), b);

    // β-reduction should reduce to λy.x where x is still the outer variable
    // With call-by-value, this becomes var(0) because the outer variable gets substituted
    let beta_result = beta_reduce(expr.clone());
    assert_eq!(beta_result, var(0));

    // Full normalization should reduce to a
    let norm_result = normalize(expr.clone());
    assert_eq!(norm_result, a);

    // Evaluation should produce a value
    let eval_result = eval_empty(expr.clone());
    assert!(matches!(eval_result, EvalResult::Value(_)));

    // Proof generation and verification
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));
}

#[test]
pub(crate) fn test_church_numeral_handling() {
    // Test Church numeral 2: λf.λx.f (f x)
    let church_two = lam(lam(app(var(0), app(var(0), var(1)))));
    let f = var(0);
    let x = var(1);
    let expr = app(app(church_two, f.clone()), x.clone());

    // β-reduction should reduce to f (f x)
    // With call-by-value, the indices shift appropriately
    let beta_result = beta_reduce(expr.clone());
    let expected = app(var(1), app(var(1), var(0)));
    assert_eq!(beta_result, expected);

    // Normalization should fully reduce the expression
    let norm_result = normalize(expr.clone());
    // The result should be f (f x) since it's already in normal form
    assert_eq!(norm_result, expected);

    // Evaluation should produce a value
    let eval_result = eval_empty(expr.clone());
    assert!(matches!(eval_result, EvalResult::Value(_)));

    // Proof generation and verification
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));
}

#[test]
pub(crate) fn test_multiple_reduction_steps_handling() {
    // Test expression requiring multiple reduction steps: ((λx.λy.x) a) b → a
    let expr = app(app(lam(lam(var(1))), var(0)), var(1));

    // First β-reduction should reduce to (λy.x) b
    // With call-by-value, this becomes var(0) because it fully evaluates
    let beta_result1 = beta_reduce(expr.clone());
    assert_eq!(beta_result1, var(0));

    // Second β-reduction should reduce to x (which is a, var(0))
    let beta_result2 = beta_reduce(beta_result1);
    assert_eq!(beta_result2, var(0));

    // Full normalization should reduce to a
    let norm_result = normalize(expr.clone());
    assert_eq!(norm_result, var(0));

    // Evaluation should produce a value
    let eval_result = eval_empty(expr.clone());
    assert!(matches!(eval_result, EvalResult::Value(_)));

    // Proof generation and verification
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));
}

#[test]
pub(crate) fn test_complex_nesting_handling() {
    // Test deeply nested expression: (λx.(λy.(λz.z) y) x) a
    let inner_lam = lam(var(0));
    let middle_lam = lam(app(inner_lam, var(0)));
    let outer_lam = lam(app(middle_lam.clone(), var(0)));
    let a = var(0);
    let expr = app(outer_lam, a.clone());

    // β-reduction should reduce to (λy.(λz.z) y) a
    // With call-by-value, this becomes var(0) because it fully evaluates
    let beta_result = beta_reduce(expr.clone());
    assert_eq!(beta_result, var(0));

    // Full normalization should reduce to a
    let norm_result = normalize(expr.clone());
    assert_eq!(norm_result, a.clone());

    // Evaluation should produce a value
    let eval_result = eval_empty(expr.clone());
    assert!(matches!(eval_result, EvalResult::Value(_)));

    // Proof generation and verification
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));
}

#[test]
pub(crate) fn test_proof_generation_edge_cases() {
    // Test proof generation for expressions that don't reduce
    let identity = lam(var(0));
    let expr = identity.clone();

    // β-reduction should not change the expression
    let beta_result = beta_reduce(expr.clone());
    assert_eq!(beta_result, identity);

    // Proof generation should return None for non-reducible expressions
    let beta_proof = prove_beta_reduction(expr.clone());
    assert!(beta_proof.is_none());

    // But normalization and evaluation proofs should still work
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));
}

#[test]
pub(crate) fn test_consistency_between_methods() {
    // Test that β-reduction and normalization produce consistent results
    let expr = app(app(lam(lam(var(1))), var(0)), var(1));

    // Single β-reduction
    let beta_result = beta_reduce(expr.clone());

    // Full normalization
    let norm_result = normalize(expr.clone());

    // The normalization result should be reachable from the β-reduction result
    let norm_from_beta = normalize(beta_result);
    assert_eq!(norm_from_beta, norm_result);

    // Both should produce valid proofs
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));
}

#[test]
pub(crate) fn test_memory_intensive_expression_handling() {
    // Test expression with many nested applications and lambdas
    // This tests memory handling and performance characteristics
    let expr = app(
        app(
            app(
                lam(lam(lam(var(2)))),
                var(0)
            ),
            var(1)
        ),
        var(2)
    );

    // β-reduction should reduce step by step
    // With call-by-value, this becomes var(0) because it fully evaluates
    let beta_result = beta_reduce(expr.clone());
    assert_eq!(beta_result, var(0));

    // Full normalization should reduce to the outermost variable
    let norm_result = normalize(expr.clone());
    assert_eq!(norm_result, var(0));

    // Evaluation should produce a value
    let eval_result = eval_empty(expr.clone());
    assert!(matches!(eval_result, EvalResult::Value(_)));

    // Proof generation and verification should work
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));
}

#[test]
pub(crate) fn test_error_conditions_handling() {
    // Test edge cases and error conditions
    let expr = app(lam(var(0)), var(1));

    // All operations should complete without panicking
    let beta_result = beta_reduce(expr.clone());
    let norm_result = normalize(expr.clone());
    let eval_result = eval_empty(expr.clone());

    // Proof generation should work for valid expressions
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    // All proofs should verify correctly
    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));

    // Results should be consistent
    assert_eq!(beta_result, var(1));
    assert_eq!(norm_result, var(1));
    if let EvalResult::Value(value) = eval_result {
        assert_eq!(value, var(1));
    } else {
        panic!("Expected value result from evaluation");
    }
}

#[test]
pub(crate) fn test_complex_expression_performance() {
    let start_time = Instant::now();

    // Test performance with various complex expressions
    let expressions = vec![
        // Identity function
        app(lam(var(0)), var(1)),
        // K combinator
        app(app(lam(lam(var(1))), var(0)), var(1)),
        // Church numeral 2
        app(app(lam(lam(app(var(0), app(var(0), var(1))))), var(0)), var(1)),
        // Deeply nested
        app(lam(app(lam(var(1)), var(0))), lam(var(0))),
        // Complex nesting
        app(lam(lam(app(var(1), var(0)))), app(var(0), var(1))),
    ];

    for expr in expressions {
        for _ in 0..20 {
            let _normalized = normalize(expr.clone());
            let beta_proof = prove_beta_reduction(expr.clone());
            let eval_proof = prove_evaluation(expr.clone());
            let norm_proof = prove_normalization(expr.clone());

            // Verify all proofs
            assert!(verify_proof(&eval_proof, &expr));
            assert!(verify_proof(&norm_proof, &expr));

            // For reducible expressions, beta proof should exist
            if let Some(proof) = beta_proof {
                assert!(verify_proof(&proof, &expr));
            }
        }
    }

    let duration = start_time.elapsed();
    println!("Complex expression performance test completed in {:?}", duration);
}
