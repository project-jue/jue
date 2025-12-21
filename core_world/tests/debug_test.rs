#[test]
fn debug_edge_case() {
    // Reproduce the failing test case from test_edge_cases
    let mut large_proof = core_world::proof_checker::prove_beta(core_world::core_expr::app(
        core_world::core_expr::lam(core_world::core_expr::var(0)),
        core_world::core_expr::var(1)
    ));

    println!("Initial proof: {:?}", large_proof);

    for i in 0..50 {
        let beta_proof = core_world::proof_checker::prove_beta(core_world::core_expr::app(
            core_world::core_expr::lam(core_world::core_expr::var(0)),
            core_world::core_expr::var(1)
        ));
        println!("Step {}: Creating beta proof: {:?}", i, beta_proof);

        large_proof = core_world::proof_checker::Proof::Trans {
            proof_a: Box::new(large_proof),
            proof_b: Box::new(beta_proof),
        };
        println!("Step {}: Large proof size: {:?}", i, std::mem::size_of_val(&large_proof));
    }

    println!("Final proof: {:?}", large_proof);

    let serialized = core_world::serialize_proof(&large_proof);
    println!("Serialized length: {}", serialized.len());

    let deserialized = core_world::deserialize_proof(&serialized);
    println!("Deserialized result: {:?}", deserialized);

    if let Ok(deserialized_proof) = deserialized {
        let result = core_world::proof_checker::verify(&deserialized_proof);
        println!("Verification result: {:?}", result);
    }
}