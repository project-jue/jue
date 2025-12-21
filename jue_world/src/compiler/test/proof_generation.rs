use crate::compiler::core_compilation::{
    compile_ast_to_core_expr, generate_comprehensive_proof, generate_simple_proof,
};
use crate::parser::parse;
use core_world::proof_checker::{verify, Proof};

#[test]
fn test_simple_lambda_proof_generation() {
    // Test: (λx.x) y → y
    let source = "(λ (x) x) y";
    let ast = parse(source).unwrap();
    let core_expr = compile_ast_to_core_expr(&ast).unwrap();

    // Generate proof
    let proof = generate_simple_proof(&core_expr);

    assert!(
        proof.is_some(),
        "Proof generation should succeed for simple lambda"
    );

    let proof = proof.unwrap();

    // Verify the proof
    let result = verify(&proof);
    assert!(result.is_ok(), "Proof verification should succeed");

    let (left, right) = result.unwrap();
    println!("Proof verified: {} ≡ {}", left, right);
}

#[test]
fn test_complex_lambda_proof_generation() {
    // Test: ((λx.λy.x) a) b → a
    let source = "((λ (x) (λ (y) x)) a) b";
    let ast = parse(source).unwrap();
    let core_expr = compile_ast_to_core_expr(&ast).unwrap();

    // Generate comprehensive proof
    let proof = generate_comprehensive_proof(&core_expr);

    assert!(
        proof.is_some(),
        "Comprehensive proof generation should succeed"
    );

    let proof = proof.unwrap();

    // Verify the proof
    let result = verify(&proof);
    assert!(
        result.is_ok(),
        "Comprehensive proof verification should succeed"
    );

    let (left, right) = result.unwrap();
    println!("Comprehensive proof verified: {} ≡ {}", left, right);
}

#[test]
fn test_identity_function_proof() {
    // Test identity function: (λx.x) 42 → 42
    let source = "(λ (x) x) 42";
    let ast = parse(source).unwrap();
    let core_expr = compile_ast_to_core_expr(&ast).unwrap();

    // Generate proof
    let proof = generate_simple_proof(&core_expr);

    assert!(
        proof.is_some(),
        "Proof generation should succeed for identity function"
    );

    let proof = proof.unwrap();

    // Verify the proof
    let result = verify(&proof);
    assert!(
        result.is_ok(),
        "Proof verification should succeed for identity function"
    );

    let (left, right) = result.unwrap();
    println!("Identity proof verified: {} ≡ {}", left, right);
}

#[test]
fn test_normal_form_proof() {
    // Test expression that's already in normal form
    let source = "42";
    let ast = parse(source).unwrap();
    let core_expr = compile_ast_to_core_expr(&ast).unwrap();

    // Generate proof
    let proof = generate_simple_proof(&core_expr);

    assert!(
        proof.is_some(),
        "Proof generation should succeed for normal form"
    );

    let proof = proof.unwrap();

    // Should be a reflexivity proof
    match proof {
        Proof::Refl(_) => println!("Correctly generated reflexivity proof"),
        _ => panic!("Expected reflexivity proof for normal form"),
    }

    // Verify the proof
    let result = verify(&proof);
    assert!(
        result.is_ok(),
        "Reflexivity proof verification should succeed"
    );
}

#[test]
fn test_lambda_application_proof() {
    // Test: (λx.λy.x y) a b → a b
    let source = "((λ (x) (λ (y) (x y))) a) b";
    let ast = parse(source).unwrap();
    let core_expr = compile_ast_to_core_expr(&ast).unwrap();

    // Generate proof
    let proof = generate_simple_proof(&core_expr);

    assert!(
        proof.is_some(),
        "Proof generation should succeed for lambda application"
    );

    let proof = proof.unwrap();

    // Verify the proof
    let result = verify(&proof);
    assert!(
        result.is_ok(),
        "Proof verification should succeed for lambda application"
    );

    let (left, right) = result.unwrap();
    println!("Lambda application proof verified: {} ≡ {}", left, right);
}
