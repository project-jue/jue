/// Test for mutation protocol foundations
use core_world::core_expr::{lam, var};
use core_world::proof_checker::{prove_evaluation, prove_normalization, verify_proof, Proof};

#[test]
fn test_mutation_protocol_foundations() {
    // Test core expression patterns for mutation protocol

    // Code expressions
    let mutation_code = lam(var(0));
    let proof_code = lam(var(0));

    // Verify they can be used in proofs
    let eval_proof = prove_evaluation(mutation_code.clone());
    assert!(verify_proof(&eval_proof, &mutation_code));

    let norm_proof = prove_normalization(proof_code.clone());
    assert!(verify_proof(&norm_proof, &proof_code));

    // Test that proofs can be combined (foundation for consensus)
    let composite_proof = Proof::Composite {
        proofs: vec![eval_proof, norm_proof],
        conclusion: "Mutation protocol foundation test".to_string(),
    };

    assert!(verify_proof(&composite_proof, &mutation_code));
}
