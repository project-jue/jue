/// Core-World Comprehensive Test Suite
/// This file contains comprehensive unit tests for all Core-World components
/// Tests CoreExpr, CoreKernel, EvalRelation, and ProofChecker with extensive coverage
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::beta_reduce;
use core_world::proof_checker::{prove_beta_reduction, prove_evaluation, prove_normalization};
use physics_world::memory_manager::MemoryManager;
use physics_world::primitives::{add, mul};
#[test]
fn test_high_volume_cross_layer_operations() {
    // Test high volume operations across all layers
    let start_time = std::time::Instant::now();

    for i in 0..1000 {
        // Core operations
        let expr = app(lam(var(0)), var(i % 10));
        let _reduced = beta_reduce(expr);

        // Physics operations
        let _ = add(i, i + 1);
        let _ = mul(i, 2);

        // Proof operations
        let proof_expr = app(lam(var(0)), var(1));
        let _proof = prove_beta_reduction(proof_expr);
    }

    let duration = start_time.elapsed();
    println!("1,000 cross-layer operations completed in {:?}", duration);
}

#[test]
fn test_memory_intensive_cross_layer_operations() {
    // Test memory-intensive operations across layers
    let mut memory_manager = MemoryManager::new();
    let start_time = std::time::Instant::now();

    // Allocate and free many blocks while doing core operations
    for i in 0..500 {
        // Allocate memory
        let block = memory_manager.allocate((i % 100) + 1).unwrap();

        // Do core operations
        let expr = app(lam(var(0)), var(i % 5));
        let _reduced = beta_reduce(expr);

        // Free memory
        assert!(memory_manager.free(block).is_ok());
    }

    let duration = start_time.elapsed();
    println!(
        "500 memory-intensive cross-layer operations completed in {:?}",
        duration
    );

    // Verify no memory leaks
    let (_total, _freedd, active) = memory_manager.get_memory_stats();
    assert_eq!(active, 0);
}

#[test]
fn test_proof_intensive_cross_layer_operations() {
    // Test proof-intensive operations across layers
    let start_time = std::time::Instant::now();

    for i in 0..200 {
        // Create expressions
        let expr1 = app(lam(var(0)), var(1));
        let expr2 = app(lam(var(1)), var(0));
        let expr3 = app(lam(app(var(0), var(1))), lam(var(0)));

        // Generate multiple proofs
        let _beta_proof1 = prove_beta_reduction(expr1.clone());
        let _eval_proof1 = prove_evaluation(expr1.clone());
        let _norm_proof1 = prove_normalization(expr1);

        let _beta_proof2 = prove_beta_reduction(expr2.clone());
        let _eval_proof2 = prove_evaluation(expr2.clone());
        let _norm_proof2 = prove_normalization(expr2);

        let _beta_proof3 = prove_beta_reduction(expr3.clone());
        let _eval_proof3 = prove_evaluation(expr3.clone());
        let _norm_proof3 = prove_normalization(expr3);

        // Do physics operations
        let _ = add(i, i + 1);
        let _ = mul(i, 2);
    }

    let duration = start_time.elapsed();
    println!(
        "200 proof-intensive cross-layer operations completed in {:?}",
        duration
    );
}

#[test]
fn test_snapshot_rollback_stress_test() {
    // Test snapshot and rollback under stress
    let mut memory_manager = MemoryManager::new();
    let start_time = std::time::Instant::now();

    // Create multiple snapshots and rollbacks
    for i in 0..100 {
        // Allocate some memory
        let block = memory_manager.allocate(100).unwrap();

        // Take snapshot
        assert!(memory_manager.snapshot().is_ok());

        // Modify memory
        let data = vec![i as u8; 50];
        assert!(memory_manager.write_memory(&block, 0, &data).is_ok());

        // Take another snapshot
        assert!(memory_manager.snapshot().is_ok());

        // Modify again
        let new_data = vec![(i + 1) as u8; 25];
        assert!(memory_manager.write_memory(&block, 0, &new_data).is_ok());

        // Rollback to previous snapshot
        assert!(memory_manager.rollback().is_ok());

        // Verify we're back to previous state
        let read_data = memory_manager.read_memory(&block, 0, 25).unwrap();
        assert_eq!(read_data, vec![i as u8; 25]);

        // Rollback to initial snapshot
        assert!(memory_manager.rollback().is_ok());

        // Free memory
        assert!(memory_manager.free(block).is_ok());
    }

    let duration = start_time.elapsed();
    println!(
        "100 snapshot/rollback operations completed in {:?}",
        duration
    );

    // Verify no memory leaks
    let (_total, _freed, active) = memory_manager.get_memory_stats();
    assert_eq!(active, 0);
}
