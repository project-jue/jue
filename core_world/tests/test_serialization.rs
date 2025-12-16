/// Comprehensive serialization tests for Core-World V2
/// Tests cover all serialization functionality according to CoreSpec v2.0
use core_world::{
    core_expr::{app, lam, nat, pair, var, ParseError},
    deserialize_core_expr, deserialize_proof,
    proof_checker::{prove_beta, prove_eta, Proof, ProofParseError},
    serialize_core_expr, serialize_proof,
};

/// Test CoreExpr serialization roundtrip for all variants
#[test]
fn test_core_expr_roundtrip_all_variants() {
    // Test Var
    let var_expr = var(42);
    let serialized = serialize_core_expr(&var_expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(var_expr, deserialized);

    // Test Lam
    let lam_expr = lam(var(0));
    let serialized = serialize_core_expr(&lam_expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(lam_expr, deserialized);

    // Test App
    let app_expr = app(lam(var(0)), var(1));
    let serialized = serialize_core_expr(&app_expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(app_expr, deserialized);

    // Test Nat
    let nat_expr = nat(12345);
    let serialized = serialize_core_expr(&nat_expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(nat_expr, deserialized);

    // Test Pair
    let pair_expr = pair(var(0), var(1));
    let serialized = serialize_core_expr(&pair_expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(pair_expr, deserialized);
}

/// Test complex nested expressions
#[test]
fn test_complex_nested_expressions() {
    // Test deeply nested lambda
    let nested_lam = lam(lam(lam(var(2))));
    let serialized = serialize_core_expr(&nested_lam);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(nested_lam, deserialized);

    // Test complex application
    let complex_app = app(lam(app(var(1), var(0))), lam(var(0)));
    let serialized = serialize_core_expr(&complex_app);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(complex_app, deserialized);

    // Test expression with Nat and Pair
    let complex_expr = app(lam(pair(var(0), nat(5))), nat(10));
    let serialized = serialize_core_expr(&complex_expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(complex_expr, deserialized);
}

/// Test error handling for CoreExpr deserialization
#[test]
fn test_core_expr_error_handling() {
    // Test empty input
    let result = deserialize_core_expr(&[]);
    assert!(matches!(result, Err(ParseError::EmptyInput)));

    // Test incomplete data for Var
    let incomplete_var = vec![0x01]; // Only tag, missing 8 bytes for index
    let result = deserialize_core_expr(&incomplete_var);
    assert!(matches!(result, Err(ParseError::IncompleteData)));

    // Test incomplete data for Nat
    let incomplete_nat = vec![0x04]; // Only tag, missing 8 bytes for value
    let result = deserialize_core_expr(&incomplete_nat);
    assert!(matches!(result, Err(ParseError::IncompleteData)));

    // Test invalid tag
    let invalid_tag = vec![0xFF];
    let result = deserialize_core_expr(&invalid_tag);
    assert!(matches!(result, Err(ParseError::InvalidTag(0xFF))));

    // Test incomplete App (missing argument)
    let func = lam(var(0));
    let func_serialized = serialize_core_expr(&func);
    let mut incomplete_app = vec![0x03]; // App tag
    incomplete_app.extend_from_slice(&func_serialized);
    // Missing argument bytes
    let result = deserialize_core_expr(&incomplete_app);
    assert!(matches!(result, Err(ParseError::IncompleteData)));
}

/// Test Proof serialization roundtrip for all variants
#[test]
fn test_proof_roundtrip_all_variants() {
    // Test BetaStep
    let redex = app(lam(var(0)), var(1));
    let beta_proof = prove_beta(redex.clone());
    let serialized = serialize_proof(&beta_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();
    let (left, right) = core_world::proof_checker::verify(&deserialized).unwrap();
    assert!(core_world::core_kernel::alpha_equiv(left, redex));
    assert_eq!(right, var(1));

    // Test EtaStep
    let redex = lam(app(var(1), var(0)));
    let eta_proof = prove_eta(redex.clone()).unwrap();
    let serialized = serialize_proof(&eta_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();
    let (left, right) = core_world::proof_checker::verify(&deserialized).unwrap();
    assert!(core_world::core_kernel::alpha_equiv(left, redex));
    assert_eq!(right, var(1));

    // Test Refl
    let expr = var(42);
    let refl_proof = Proof::Refl(expr.clone());
    let serialized = serialize_proof(&refl_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();
    let (left, right) = core_world::proof_checker::verify(&deserialized).unwrap();
    assert_eq!(left, expr);
    assert_eq!(right, expr);

    // Test Sym
    let redex = app(lam(var(0)), var(1));
    let beta_proof = prove_beta(redex.clone());
    let sym_proof = Proof::Sym(Box::new(beta_proof));
    let serialized = serialize_proof(&sym_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();
    let (left, right) = core_world::proof_checker::verify(&deserialized).unwrap();
    assert_eq!(left, var(1));
    assert!(core_world::core_kernel::alpha_equiv(right, redex));

    // Test Trans
    let inner_redex = app(lam(var(0)), var(1));
    let outer_redex = app(lam(var(0)), inner_redex.clone());
    let proof1 = prove_beta(outer_redex.clone());
    let proof2 = prove_beta(inner_redex.clone());
    let trans_proof = Proof::Trans {
        proof_a: Box::new(proof1),
        proof_b: Box::new(proof2),
    };
    let serialized = serialize_proof(&trans_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();
    let (left, right) = core_world::proof_checker::verify(&deserialized).unwrap();
    assert!(core_world::core_kernel::alpha_equiv(left, outer_redex));
    assert_eq!(right, var(1));

    // Test CongApp
    let f1 = lam(var(0));
    let a1 = var(1);
    let proof_f = Proof::Refl(f1.clone());
    let proof_a = Proof::Refl(a1.clone());
    let cong_app_proof = Proof::CongApp {
        proof_f: Box::new(proof_f),
        proof_a: Box::new(proof_a),
    };
    let serialized = serialize_proof(&cong_app_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();
    let (left, right) = core_world::proof_checker::verify(&deserialized).unwrap();
    let expected_left = app(f1.clone(), a1.clone());
    let expected_right = app(f1.clone(), a1.clone());
    assert!(core_world::core_kernel::alpha_equiv(left, expected_left));
    assert!(core_world::core_kernel::alpha_equiv(right, expected_right));

    // Test CongLam
    let m = var(0);
    let proof_b = Proof::Refl(m.clone());
    let cong_lam_proof = Proof::CongLam {
        proof_b: Box::new(proof_b),
    };
    let serialized = serialize_proof(&cong_lam_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();
    let (left, right) = core_world::proof_checker::verify(&deserialized).unwrap();
    let expected_left = lam(m.clone());
    let expected_right = lam(m.clone());
    assert!(core_world::core_kernel::alpha_equiv(left, expected_left));
    assert!(core_world::core_kernel::alpha_equiv(right, expected_right));
}

/// Test complex nested proof structures
#[test]
fn test_complex_nested_proofs() {
    // Test deeply nested proof with multiple constructs
    let redex = app(lam(var(0)), var(1));
    let beta_proof = prove_beta(redex.clone());
    let sym_proof = Proof::Sym(Box::new(beta_proof));
    let refl_proof = Proof::Refl(var(42));
    let trans_proof = Proof::Trans {
        proof_a: Box::new(sym_proof),
        proof_b: Box::new(refl_proof),
    };

    let serialized = serialize_proof(&trans_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();
    let result = core_world::proof_checker::verify(&deserialized);
    assert!(result.is_ok());

    // Test complex proof with CongApp and CongLam
    let f1 = lam(var(0));
    let _f2 = lam(var(0));
    let a1 = var(1);
    let _a2 = var(1);
    let proof_f = Proof::Refl(f1.clone());
    let proof_a = Proof::Refl(a1.clone());
    let cong_app_proof = Proof::CongApp {
        proof_f: Box::new(proof_f),
        proof_a: Box::new(proof_a),
    };
    let inner_refl = Proof::Refl(var(0));
    let cong_lam_proof = Proof::CongLam {
        proof_b: Box::new(inner_refl),
    };
    let complex_proof = Proof::Trans {
        proof_a: Box::new(cong_app_proof),
        proof_b: Box::new(cong_lam_proof),
    };

    let serialized = serialize_proof(&complex_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();
    let result = core_world::proof_checker::verify(&deserialized);
    assert!(result.is_ok());
}

/// Test error handling for Proof deserialization
#[test]
fn test_proof_error_handling() {
    // Test empty input
    let result = deserialize_proof(&[]);
    assert!(matches!(result, Err(ProofParseError::EmptyInput)));

    // Test invalid tag
    let invalid_tag = vec![0xFF];
    let result = deserialize_proof(&invalid_tag);
    assert!(matches!(result, Err(ProofParseError::InvalidTag(0xFF))));

    // Test incomplete BetaStep (missing contractum)
    let redex = app(lam(var(0)), var(1));
    let redex_serialized = serialize_core_expr(&redex);
    let mut incomplete_beta = vec![0x01]; // BetaStep tag
    incomplete_beta.extend_from_slice(&redex_serialized);
    // Missing contractum bytes
    let result = deserialize_proof(&incomplete_beta);
    assert!(matches!(
        result,
        Err(ProofParseError::CoreExprParseError(_))
    ));

    // Test incomplete Trans (missing second proof)
    let proof1 = prove_beta(app(lam(var(0)), var(1)));
    let proof1_serialized = serialize_proof(&proof1);
    let mut incomplete_trans = vec![0x05]; // Trans tag
    incomplete_trans.extend_from_slice(&proof1_serialized);
    // Missing second proof bytes
    let result = deserialize_proof(&incomplete_trans);
    assert!(matches!(result, Err(ProofParseError::IncompleteData)));
}

/// Test integration with existing functionality
#[test]
fn test_serialization_integration() {
    // Test that serialized expressions can be used in normalization
    let expr = app(lam(var(0)), var(42));
    let serialized = serialize_core_expr(&expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();

    let normalized = core_world::normalize(deserialized, 10).unwrap();
    assert_eq!(normalized, var(42));

    // Test that serialized proofs can be verified
    let redex = app(lam(var(0)), var(1));
    let proof = prove_beta(redex.clone());
    let serialized = serialize_proof(&proof);
    let deserialized = deserialize_proof(&serialized).unwrap();

    let result = core_world::verify_equivalence(deserialized);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert!(core_world::core_kernel::alpha_equiv(left, redex));
    assert_eq!(right, var(1));
}

/// Test edge cases and boundary conditions
#[test]
fn test_edge_cases() {
    // Test Var with maximum index
    let max_var = var(usize::MAX);
    let serialized = serialize_core_expr(&max_var);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(max_var, deserialized);

    // Test Nat with maximum value
    let max_nat = nat(u64::MAX);
    let serialized = serialize_core_expr(&max_nat);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(max_nat, deserialized);

    // Test deeply nested expressions (stress test)
    let mut deep_expr = var(0);
    for _ in 0..100 {
        deep_expr = lam(deep_expr);
    }
    let serialized = serialize_core_expr(&deep_expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(deep_expr, deserialized);

    // Test large proof structures
    let mut large_proof = prove_beta(app(lam(var(0)), var(1)));
    for _ in 0..50 {
        let beta_proof = prove_beta(app(lam(var(0)), var(1)));
        large_proof = Proof::Trans {
            proof_a: Box::new(large_proof),
            proof_b: Box::new(beta_proof),
        };
    }
    let serialized = serialize_proof(&large_proof);
    let deserialized = deserialize_proof(&serialized).unwrap();
    let result = core_world::proof_checker::verify(&deserialized);
    assert!(result.is_ok());
}

/// Test conformance with V2 specification examples
#[test]
fn test_v2_conformance_examples() {
    // Test 1: β-Reduction Correctness (Call-by-Name)
    let term = app(lam(var(0)), var(1)); // (λ.0) 1
    let reduced = core_world::core_kernel::beta_reduce_step(term.clone());
    assert_eq!(reduced, var(1));

    // Test 2: η-Reduction
    let term = lam(app(var(0), var(1))); // λ.(0 1)
    let eta_result = core_world::core_kernel::eta_reduce(term.clone());
    assert_eq!(eta_result, term); // Should NOT reduce (0 is free in body)

    let term_eta = lam(app(var(1), var(0))); // λ.(1 0)
    let eta_result_eta = core_world::core_kernel::eta_reduce(term_eta.clone());
    assert_ne!(eta_result_eta, term_eta); // SHOULD reduce to var(1)
    assert_eq!(eta_result_eta, var(1));

    // Test 3: Proof Verification
    let redex = app(lam(var(0)), var(42)); // (λ.0) 42
    let proof = prove_beta(redex.clone());
    let (left, right) = core_world::verify_equivalence(proof).unwrap();
    assert!(core_world::core_kernel::alpha_equiv(left, redex));
    assert_eq!(right, var(42));

    // Test 4: V2 Serialization Roundtrip
    let expr = app(lam(var(0)), var(42));
    let serialized = serialize_core_expr(&expr);
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(expr, deserialized);
}

/// Test serialization format compliance
#[test]
fn test_serialization_format_compliance() {
    // Test Var format: [0x01, n as u64]
    let var_expr = var(42);
    let serialized = serialize_core_expr(&var_expr);
    assert_eq!(serialized[0], 0x01);
    assert_eq!(serialized.len(), 9); // 1 byte tag + 8 bytes u64

    // Test Lam format: [0x02, body_bytes...]
    let lam_expr = lam(var(0));
    let serialized = serialize_core_expr(&lam_expr);
    assert_eq!(serialized[0], 0x02);

    // Test App format: [0x03, f_bytes..., a_bytes...]
    let app_expr = app(lam(var(0)), var(1));
    let serialized = serialize_core_expr(&app_expr);
    assert_eq!(serialized[0], 0x03);

    // Test Nat format: [0x04, n as u64]
    let nat_expr = nat(12345);
    let serialized = serialize_core_expr(&nat_expr);
    assert_eq!(serialized[0], 0x04);
    assert_eq!(serialized.len(), 9); // 1 byte tag + 8 bytes u64

    // Test Pair format: [0x05, f_bytes..., s_bytes...]
    let pair_expr = pair(var(0), var(1));
    let serialized = serialize_core_expr(&pair_expr);
    assert_eq!(serialized[0], 0x05);

    // Test Proof BetaStep format: [0x01, redex_bytes..., contractum_bytes...]
    let redex = app(lam(var(0)), var(1));
    let beta_proof = prove_beta(redex.clone());
    let serialized = serialize_proof(&beta_proof);
    assert_eq!(serialized[0], 0x01);

    // Test Proof Refl format: [0x03, expr_bytes...]
    let refl_proof = Proof::Refl(var(42));
    let serialized = serialize_proof(&refl_proof);
    assert_eq!(serialized[0], 0x03);
}
