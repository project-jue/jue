use core_world::proof_checker::prove_normalization;

use core_world::proof_checker::prove_evaluation;

use core_world::proof_checker::prove_beta_reduction;

use core_world::core_kernel::normalize;

use core_world::core_expr::var;

use core_world::core_expr::lam;

use core_world::core_expr::app;
use core_world::proof_checker::verify_proof;

use std::time::Instant;

#[test]
pub(crate) fn test_complex_expression_handling() {
    let start_time = Instant::now();

    // Create and process complex expressions
    for _ in 0..100 {
        let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));

        let normalized = normalize(deeply_nested.clone());
        let beta_proof = prove_beta_reduction(deeply_nested.clone());
        let eval_proof = prove_evaluation(deeply_nested.clone());
        let norm_proof = prove_normalization(deeply_nested.clone());

        // The normalization produces lam(var(0)) which is correct
        // because the expression (λx.(λy.y) x) (λz.z) reduces to (λy.y)
        // which is represented as lam(var(0)) in De Bruijn indices
        assert_eq!(normalized, lam(var(0)));
        assert!(beta_proof.is_some());
        assert!(verify_proof(&eval_proof, &deeply_nested));
        assert!(verify_proof(&norm_proof, &deeply_nested));
    }

    let duration = start_time.elapsed();
    println!("100 complex expressions processed in {:?}", duration);
}
