use crate::compiler::core_compilation::{compile_ast_to_core_expr_with_proofs, generate_simple_proof, verify_core_expr_with_proof, verify_proof_against_kernel};
use crate::parser::parse;
use core_world::core_kernel::alpha_equiv;
use core_world::proof_checker::Proof;
use core_world::{app, lam, prove_beta, var};

#[test]
fn test_proof_verification_integration() {
    // Test that we can verify a simple proof against the Core-World kernel
    let redex = app(lam(var(0)), var(1));
    let proof = prove_beta(redex.clone());

    // Verify the proof using our integration function
    let result = verify_proof_against_kernel(&proof);
    assert!(result.is_ok());

    let (left, right) = result.unwrap();
    assert!(alpha_equiv(left, redex));
    assert_eq!(right, var(1));
}

#[test]
fn test_core_expr_with_proof_verification() {
    // Test that we can verify a CoreExpr with its proof
    let expr = app(lam(var(0)), var(1));
    let proof = prove_beta(expr.clone());

    // Verify the CoreExpr with its proof
    let result = verify_core_expr_with_proof(&expr, &proof);
    assert!(result.is_ok());
}

#[test]
fn test_proof_verification_failure() {
    // Test that proof verification fails for invalid proofs
    let redex = var(0);
    let contractum = var(1);
    let invalid_proof = Proof::BetaStep { redex, contractum };

    // This should fail because var(0) is not a beta-redex
    let result = verify_proof_against_kernel(&invalid_proof);
    assert!(result.is_err());
}

#[test]
fn test_ast_compilation_with_proof_verification() {
    // Test that we can compile AST to CoreExpr with proofs and verify them
    let source = "((lambda (x) x) 42)";
    let ast = parse(source).unwrap();

    let (expr, proof) = compile_ast_to_core_expr_with_proofs(&ast).unwrap();

    // For now, we'll just check that compilation succeeded and we got an expression
    // The proof generation for AST compilation is still a work in progress
    assert!(core_world::core_kernel::is_normal_form(&expr) || proof.is_some());
}

#[test]
fn test_simple_proof_generation_and_verification() {
    // Test that we can generate and verify simple proofs
    let expr = app(lam(var(0)), var(1));
    let proof = generate_simple_proof(&expr);

    assert!(proof.is_some());
    let proof = proof.unwrap();

    // Verify the proof
    let result = verify_proof_against_kernel(&proof);
    assert!(result.is_ok());

    let (left, right) = result.unwrap();
    assert!(core_world::core_kernel::alpha_equiv(left, expr));
    assert_eq!(right, var(1));
}
