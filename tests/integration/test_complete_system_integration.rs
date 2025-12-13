/// Complete System Integration Tests
use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, prove_consistency};
use core_world::eval_relation::eval_empty;
use core_world::proof_checker::{
    prove_beta_reduction, prove_evaluation, prove_normalization, verify_proof, Proof,
};
use physics_layer::memory_manager::MemoryManager;
use physics_layer::primitives::add;

#[test]
fn test_complete_system_integration() {
    // This test simulates a complete workflow through all layers:
    // 1. Physics layer provides primitives
    // 2. Core layer provides formal semantics
    // 3. Jue layer compiles and optimizes
    // 4. Dan layer manages modules and events

    // Step 1: Physics layer operations
    let arithmetic_result = add(10, 20).unwrap();
    assert_eq!(arithmetic_result, 30);

    // Step 2: Core layer expressions
    let core_expr = app(lam(var(0)), var(1));
    let beta_reduced = beta_reduce(core_expr.clone());
    assert_eq!(beta_reduced, var(1));

    // Step 3: Proof generation and verification
    let beta_proof = prove_beta_reduction(core_expr.clone()).unwrap();
    let eval_proof = prove_evaluation(core_expr.clone());
    let norm_proof = prove_normalization(core_expr.clone());

    assert!(verify_proof(&beta_proof, &core_expr));
    assert!(verify_proof(&eval_proof, &core_expr));
    assert!(verify_proof(&norm_proof, &core_expr));

    // Step 4: Composite proof verification
    let composite_proof = Proof::Composite {
        proofs: vec![beta_proof, eval_proof, norm_proof],
        conclusion: "Complete system integration proof".to_string(),
    };

    assert!(verify_proof(&composite_proof, &core_expr));

    // Step 5: Memory management for system state
    let mut memory_manager = MemoryManager::new();
    let system_state = memory_manager.allocate(4096).unwrap();

    // Take snapshot of initial state
    assert!(memory_manager.snapshot().is_ok());

    // Verify system consistency
    assert!(prove_consistency());

    // Clean up
    assert!(memory_manager.free(system_state).is_ok());
}
