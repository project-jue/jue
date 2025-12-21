#[test]
fn test_simple_proof_generation_works() {
    use crate::compiler::core_compilation::generate_simple_proof;
    use core_world::core_expr::{app, lam, var};
    use core_world::proof_checker::verify;

    // Create a simple lambda expression: (λx.x) y
    let expr = app(lam(var(0)), var(1));

    // Generate proof
    let proof = generate_simple_proof(&expr);

    assert!(proof.is_some(), "Proof generation should succeed");

    let proof = proof.unwrap();

    // Verify the proof
    let result = verify(&proof);
    assert!(result.is_ok(), "Proof verification should succeed");

    let (left, right) = result.unwrap();
    println!("Proof verified: {} ≡ {}", left, right);
}
