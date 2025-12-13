
use core_world::proof_checker::Proof;

use core_world::proof_checker::prove_normalization;

use core_world::proof_checker::prove_evaluation;

use core_world::proof_checker::prove_beta_reduction;

use core_world::core_expr::var;

use core_world::core_expr::lam;

use core_world::core_expr::app;

/// Test Proof System Across All Layers
#[test]
pub(crate) fn test_cross_layer_proof_system() {
    // Test that proof system works consistently across all layers

    // Create a single core expression that represents operations from different layers
    let core_expr = app(lam(var(0)), var(1)); // Represents Jue compilation and physics operation

    // Generate proofs for the same expression
    let proof1 = prove_beta_reduction(core_expr.clone()).unwrap();
    let proof2 = prove_evaluation(core_expr.clone());
    let proof3 = prove_normalization(core_expr.clone());

    // All proofs should verify correctly for the same expression
    assert!(verify_proof(&proof1, &core_expr));
    assert!(verify_proof(&proof2, &core_expr));
    assert!(verify_proof(&proof3, &core_expr));

    // Test composite proof for the same expression
    let cross_layer_proof = Proof::Composite {
        proofs: vec![proof1, proof2, proof3],
        conclusion: "Cross-layer proof verification".to_string(),
    };

    // The composite proof should verify against the expression
    assert!(verify_proof(&cross_layer_proof, &core_expr));

    // Test that kernel consistency holds across all layers
    assert!(prove_consistency());
}
