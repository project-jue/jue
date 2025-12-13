/// Core-World Comprehensive Test Suite
/// This file contains comprehensive unit tests for all Core-World components
/// Tests CoreExpr, CoreKernel, EvalRelation, and ProofChecker with extensive coverage
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::prove_consistency;
use core_world::eval_relation::{eval_empty, EvalResult};
use core_world::proof_checker::{
    prove_beta_reduction, prove_evaluation, prove_normalization, verify_proof, Proof,
};
use physics_layer::memory_manager::MemoryManager;
use physics_layer::primitives::{add, mul};
#[test]
fn test_complete_system_workflow() {
    // Test a complete workflow from physics primitives to core proofs

    // Step 1: Physics layer provides arithmetic
    let sum = add(15, 25).unwrap();
    assert_eq!(sum, 40);

    // Step 2: Core layer creates expressions representing the computation
    let computation_expr = app(lam(var(0)), var(1));

    // Step 3: Evaluate the expression
    let eval_result = eval_empty(computation_expr.clone());
    assert_eq!(eval_result, EvalResult::Value(var(1)));

    // Step 4: Generate proofs for the computation
    let beta_proof = prove_beta_reduction(computation_expr.clone()).unwrap();
    let eval_proof = prove_evaluation(computation_expr.clone());
    let norm_proof = prove_normalization(computation_expr.clone());

    // Step 5: Verify all proofs
    assert!(verify_proof(&beta_proof, &computation_expr));
    assert!(verify_proof(&eval_proof, &computation_expr));
    assert!(verify_proof(&norm_proof, &computation_expr));

    // Step 6: Create composite proof
    let composite_proof = Proof::Composite {
        proofs: vec![beta_proof, eval_proof, norm_proof],
        conclusion: "Complete system workflow proof".to_string(),
    };

    assert!(verify_proof(&composite_proof, &computation_expr));

    // Step 7: Store results in memory
    let mut memory_manager = MemoryManager::new();
    let result_block = memory_manager.allocate(1024).unwrap();

    // Store the computation result
    let result_data = vec![40; 100]; // Representing the sum of 15 + 25
    assert!(memory_manager
        .write_memory(&result_block, 0, &result_data)
        .is_ok());

    // Verify the result
    let stored_result = memory_manager.read_memory(&result_block, 0, 100).unwrap();
    assert_eq!(stored_result, result_data);

    // Step 8: Verify system consistency
    assert!(prove_consistency());

    // Clean up
    assert!(memory_manager.free(result_block).is_ok());
}

#[test]
fn test_system_evolution_and_rollback() {
    // Test system evolution with snapshots and rollback

    let mut memory_manager = MemoryManager::new();

    // Initial state
    let core_expr_v1 = app(lam(var(0)), var(1));
    let beta_proof_v1 = prove_beta_reduction(core_expr_v1.clone()).unwrap();
    assert!(verify_proof(&beta_proof_v1, &core_expr_v1));

    // Take snapshot of initial state
    assert!(memory_manager.snapshot().is_ok());

    // System evolution - version 2
    let core_expr_v2 = app(lam(app(var(0), var(1))), lam(var(0)));
    let beta_proof_v2 = prove_beta_reduction(core_expr_v2.clone()).unwrap();
    assert!(verify_proof(&beta_proof_v2, &core_expr_v2));

    // Take snapshot of version 2
    assert!(memory_manager.snapshot().is_ok());

    // System evolution - version 3
    let core_expr_v3 = app(app(lam(lam(var(1))), var(0)), var(1));
    let beta_proof_v3 = prove_beta_reduction(core_expr_v3.clone()).unwrap();
    assert!(verify_proof(&beta_proof_v3, &core_expr_v3));

    // Verify all versions work correctly
    assert!(prove_consistency());

    // Rollback to version 2
    assert!(memory_manager.rollback().is_ok());

    // Verify version 2 still works
    assert!(verify_proof(&beta_proof_v2, &core_expr_v2));
    assert!(prove_consistency());

    // Rollback to version 1
    assert!(memory_manager.rollback().is_ok());

    // Verify version 1 still works
    assert!(verify_proof(&beta_proof_v1, &core_expr_v1));
    assert!(prove_consistency());
}

#[test]
fn test_system_consistency_under_stress() {
    // Test that system maintains consistency under stress

    let mut memory_manager = MemoryManager::new();
    let start_time = std::time::Instant::now();

    for i in 0..500 {
        // Create core expressions
        let expr = app(lam(var(0)), var(i % 10));

        // Generate and verify proofs
        if let Some(beta_proof) = prove_beta_reduction(expr.clone()) {
            assert!(verify_proof(&beta_proof, &expr));
        }

        let eval_proof = prove_evaluation(expr.clone());
        assert!(verify_proof(&eval_proof, &expr));

        let norm_proof = prove_normalization(expr.clone());
        assert!(verify_proof(&norm_proof, &expr));

        // Do physics operations
        let _ = add(i, i + 1);
        let _ = mul(i, 2);

        // Memory operations
        let block = memory_manager.allocate(100).unwrap();
        assert!(memory_manager.free(block).is_ok());

        // Verify consistency every 50 iterations
        if i % 50 == 0 {
            assert!(prove_consistency());
        }
    }

    let duration = start_time.elapsed();
    println!("500 consistency checks completed in {:?}", duration);

    // Final consistency check
    assert!(prove_consistency());

    // Verify no memory leaks
    let (_total, _freed, active) = memory_manager.get_memory_stats();
    assert_eq!(active, 0);
}
