#[test]
fn debug_edge_cases_proof_chain() {
    // Reproduce the exact failing test case from test_edge_cases
    let mut large_proof = core_world::proof_checker::prove_beta(core_world::core_expr::app(
        core_world::core_expr::lam(core_world::core_expr::var(0)),
        core_world::core_expr::var(1),
    ));

    println!("Initial proof: {:?}", large_proof);

    // Create a chain where each step reduces a different expression to 1
    // Start with (λ.0) 1 ≡ 1, then chain with 1 ≡ 1 (refl), then (λ.0) 2 ≡ 2, etc.
    for i in 1..50 {
        if i % 2 == 1 {
            // Add a reflexivity step
            let refl_proof = core_world::proof_checker::Proof::Refl(core_world::core_expr::var(1));
            println!("Step {}: Adding refl proof: {:?}", i, refl_proof);
            large_proof = core_world::proof_checker::Proof::Trans {
                proof_a: Box::new(large_proof),
                proof_b: Box::new(refl_proof),
            };
        } else {
            // Add another beta reduction
            let beta_proof = core_world::proof_checker::prove_beta(core_world::core_expr::app(
                core_world::core_expr::lam(core_world::core_expr::var(0)),
                core_world::core_expr::var(i + 1),
            ));
            println!("Step {}: Adding beta proof: {:?}", i, beta_proof);
            large_proof = core_world::proof_checker::Proof::Trans {
                proof_a: Box::new(large_proof),
                proof_b: Box::new(beta_proof),
            };
        }
        println!(
            "Step {}: Large proof size: {:?}",
            i,
            std::mem::size_of_val(&large_proof)
        );
    }

    println!("Final proof: {:?}", large_proof);

    let serialized = core_world::serialize_proof(&large_proof);
    println!("Serialized length: {}", serialized.len());

    let deserialized = core_world::deserialize_proof(&serialized);
    println!("Deserialized result: {:?}", deserialized);

    if let Ok(deserialized_proof) = deserialized {
        let result = core_world::proof_checker::verify(&deserialized_proof);
        println!("Verification result: {:?}", result);
        if let Err(e) = result {
            println!("Error details: {}", e);
        }
    }
}
