
use physics_layer::memory_manager::MemoryManager;

use core_world::proof_checker::prove_normalization;

use core_world::proof_checker::prove_evaluation;

use core_world::proof_checker::prove_beta_reduction;

use core_world::eval_relation::EvalResult;

use core_world::eval_relation::eval_empty;

use core_world::core_kernel::beta_reduce;

use core_world::core_expr::var;

use core_world::core_expr::lam;

use core_world::core_expr::app;

/// Test System Consistency Across Layers
#[test]
pub(crate) fn test_cross_layer_system_consistency() {
    // Test that the entire system maintains consistency across all layers

    // Test core consistency
    assert!(prove_consistency());

    // Test that core operations are consistent
    let expr = app(lam(var(0)), var(1));
    let beta_result = beta_reduce(expr.clone());
    let eval_result = eval_empty(expr.clone());

    if let EvalResult::Value(value) = eval_result {
        assert_eq!(beta_result, value);
    }

    // Test that proofs verify consistently
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));

    // Test physics layer consistency
    assert_eq!(add(5, 3), Ok(8));
    assert_eq!(mul(2, 4), Ok(8));
    assert_eq!(div_i32(10, 2), Ok(5));

    // Test memory management consistency
    let mut memory_manager = MemoryManager::new();
    let block = memory_manager.allocate(100).unwrap();

    let (total, _freed, active) = memory_manager.get_memory_stats();
    assert_eq!(total, 100);
    assert_eq!(active, 1);

    assert!(memory_manager.free(block).is_ok());

    let (total, _freed, active) = memory_manager.get_memory_stats();
    assert_eq!(total, 100);
    assert_eq!(active, 0);

    // Test that the entire system maintains consistency
    assert!(prove_consistency());
}
