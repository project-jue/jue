use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::core_kernel::{alpha_equiv, beta_reduce, normalize, prove_consistency};
use core_world::eval_relation::{eval_empty, is_normal_form, EvalResult};
use core_world::proof_checker::{
    prove_beta_reduction, prove_evaluation, prove_normalization, verify_proof, Proof,
};
use physics_layer::memory_manager::MemoryManager;
use physics_layer::primitives::{add, div_i32, mul};
use std::time::Instant;

#[test]
fn test_beta_reduction_performance() {
    let start_time = Instant::now();

    // Test many beta reductions
    for i in 0..1000 {
        let identity = lam(var(0));
        let v = var(i % 10);
        let expr = app(identity, v);
        let _reduced = beta_reduce(expr);
    }

    let duration = start_time.elapsed();
    println!("1,000 beta reductions completed in {:?}", duration);
}

#[test]
fn test_normalization_performance() {
    let start_time = Instant::now();

    // Test many normalizations
    for i in 0..500 {
        let expr = app(app(lam(lam(var(1))), var(0)), var(1));
        let _normalized = normalize(expr);
    }

    let duration = start_time.elapsed();
    println!("500 normalizations completed in {:?}", duration);
}

#[test]
fn test_proof_generation_performance() {
    let start_time = Instant::now();

    // Test proof generation
    for i in 0..200 {
        let expr = app(lam(var(0)), var(1));
        let _beta_proof = prove_beta_reduction(expr.clone());
        let _eval_proof = prove_evaluation(expr.clone());
        let _norm_proof = prove_normalization(expr);
    }

    let duration = start_time.elapsed();
    println!("600 proofs generated in {:?}", duration);
}

#[test]
fn test_complex_expression_handling() {
    let start_time = Instant::now();

    // Create and process complex expressions
    for _ in 0..100 {
        let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));

        let normalized = normalize(deeply_nested.clone());
        let beta_proof = prove_beta_reduction(deeply_nested.clone());
        let eval_proof = prove_evaluation(deeply_nested.clone());
        let norm_proof = prove_normalization(deeply_nested.clone());

        assert_eq!(normalized, var(0));
        assert!(beta_proof.is_some());
        assert!(verify_proof(&eval_proof, &deeply_nested));
        assert!(verify_proof(&norm_proof, &deeply_nested));
    }

    let duration = start_time.elapsed();
    println!("100 complex expressions processed in {:?}", duration);
}
