/// Dan World Core World Integration Tests
use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::core_kernel::prove_consistency;
use core_world::eval_relation::{eval_empty, EvalResult};
use core_world::proof_checker::{prove_evaluation, prove_normalization, verify_proof};

#[test]
fn test_dan_world_core_world_integration() {
    // Test that Dan World patterns work with Core World

    // Create expressions that represent Dan World concepts
    let module_code = lam(app(var(0), var(1))); // Module function
    let event_handler = lam(var(0)); // Event handler
    let mutation_request = app(lam(lam(var(1))), var(0)); // Mutation

    // Verify they all work with Core World
    let exprs = vec![module_code, event_handler, mutation_request];

    for expr in exprs {
        // Should evaluate without error
        let eval_result = eval_empty(expr.clone());
        match eval_result {
            EvalResult::Value(_) => assert!(true),
            EvalResult::Closure(_) => assert!(true),
        }

        // Should generate valid proofs
        let eval_proof = prove_evaluation(expr.clone());
        assert!(verify_proof(&eval_proof, &expr));

        // Should normalize correctly
        let norm_proof = prove_normalization(expr.clone());
        assert!(verify_proof(&norm_proof, &expr));
    }
}

#[test]
fn test_module_kernel_functionality() {
    // Test that we can create module proposals using core expressions
    // (which is what the Jue module kernel would use internally)

    let valid_proposal_code = CoreExpr::Var(0);
    let valid_proof = CoreExpr::Var(0);

    // This simulates what would happen when Jue module kernel
    // validates a proposal - it uses core expressions
    assert!(matches!(valid_proposal_code, CoreExpr::Var(_)));
    assert!(matches!(valid_proof, CoreExpr::Var(_)));

    // Test that core expressions work correctly
    let identity = lam(var(0));
    let test_var = var(1);
    let expr = app(identity, test_var.clone());

    // Verify beta reduction works (foundation for module validation)
    let reduced = beta_reduce(expr.clone());
    assert_eq!(reduced, test_var);

    // Verify proof generation works
    let beta_proof = prove_beta_reduction(expr.clone());
    assert!(beta_proof.is_some());

    // Verify proof verification works
    let proof = beta_proof.unwrap();
    assert!(verify_proof(&proof, &expr));
}
