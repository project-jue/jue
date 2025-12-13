/// Test for core expression stress
use core_world::core_expr::{app, lam, var};
use core_world::eval_relation::eval_empty;
use core_world::proof_checker::prove_evaluation;
use std::time::Instant;

#[test]
fn test_core_expression_stress() {
    // Test many core expressions (simulating Dan World usage)
    let start_time = Instant::now();

    for i in 0..100 {
        // Create expressions that Dan World would use
        let expr = app(lam(var(i % 10)), var(i % 5));

        // Evaluate them
        let _result = eval_empty(expr.clone());

        // Generate proofs
        let _proof = prove_evaluation(expr);
    }

    let duration = start_time.elapsed();
    println!(
        "Core expression stress test: {:?} for 100 expressions",
        duration
    );
}
