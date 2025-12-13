use core_world::proof_checker::prove_normalization;

use core_world::proof_checker::prove_evaluation;

use core_world::proof_checker::prove_beta_reduction;

use core_world::core_expr::var;

use core_world::core_expr::lam;

use core_world::core_expr::app;

use std::time::Instant;

#[test]
pub(crate) fn test_proof_generation_performance() {
    let start_time = Instant::now();

    // Test proof generation
    for _i in 0..200 {
        let expr = app(lam(var(0)), var(1));
        let _beta_proof = prove_beta_reduction(expr.clone());
        let _eval_proof = prove_evaluation(expr.clone());
        let _norm_proof = prove_normalization(expr);
    }

    let duration = start_time.elapsed();
    println!("600 proofs generated in {:?}", duration);
}
