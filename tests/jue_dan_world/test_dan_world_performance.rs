/// Test for Dan World performance
use core_world::core_expr::{app, lam, var};
use core_world::eval_relation::eval_empty;
use core_world::proof_checker::prove_evaluation;

#[test]
fn test_dan_world_performance() {
    // Test that Dan World operations are reasonably fast

    let start_time = std::time::Instant::now();

    // Test many core expressions (simulating Dan World usage)
    for i in 0..50 {
        // Create expressions that Dan World would use
        let expr = app(lam(var(i % 10)), var(i % 5));

        // Evaluate them
        let _result = eval_empty(expr.clone());

        // Generate proofs
        let _proof = prove_evaluation(expr);
    }

    let duration = start_time.elapsed();
    println!(
        "Dan World performance test: {:?} for 50 expressions",
        duration
    );

    // Should complete in reasonable time
    assert!(duration.as_secs() < 2, "Dan World operations too slow");
}
