/// Test for edge case integration
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize};
use core_world::eval_relation::{eval_empty, EvalResult};
use core_world::proof_checker::{prove_beta_reduction, verify_proof};

#[test]
fn test_edge_case_integration() {
    // Test edge cases in integration
    let identity = lam(var(0));

    // Test identity function properties
    let app_expr = app(identity.clone(), var(1));

    // All operations should handle this correctly
    let beta_result = beta_reduce(app_expr.clone());
    let norm_result = normalize(app_expr.clone());
    let eval_result = eval_empty(app_expr.clone());

    // All should produce var(1)
    assert_eq!(beta_result, var(1));
    assert_eq!(norm_result, var(1));

    if let EvalResult::Value(eval_value) = eval_result {
        assert_eq!(eval_value, var(1));
    } else {
        panic!("Expected value result");
    }

    // Proofs should work
    let proof = prove_beta_reduction(app_expr.clone()).unwrap();
    assert!(verify_proof(&proof, &app_expr));
}
