/// Test for proof generation stress
use core_world::core_expr::{app, lam, var};
use core_world::proof_checker::{prove_beta_reduction, prove_evaluation, prove_normalization};
use std::time::Instant;

#[test]
fn test_proof_generation_stress() {
    // Test proof generation under load
    let start_time = Instant::now();

    let expr = app(lam(var(0)), var(1));

    for _ in 0..50 {
        // Generate multiple types of proofs
        let _beta_proof = prove_beta_reduction(expr.clone());
        let _eval_proof = prove_evaluation(expr.clone());
        let _norm_proof = prove_normalization(expr.clone());
    }

    let duration = start_time.elapsed();
    println!(
        "Proof generation stress test: {:?} for 150 proofs",
        duration
    );
}
