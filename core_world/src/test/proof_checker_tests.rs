use super::*;
use crate::core_expr::{app, lam, var};

#[test]
fn test_beta_step_proof() {
    // Test: (λ.0) 1 → 1
    let redex = app(lam(var(0)), var(1));
    let proof = prove_beta(redex.clone());

    let result = verify(&proof);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert!(alpha_equiv(left, redex));
    assert_eq!(right, var(1));
}

#[test]
fn test_eta_step_proof() {
    // Test: λ.(1 0) → 1 (where 0 is not free in 1)
    let redex = lam(app(var(1), var(0)));
    let proof = prove_eta(redex.clone()).unwrap();

    let result = verify(&proof);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert!(alpha_equiv(left, redex));
    assert_eq!(right, var(1));
}

#[test]
fn test_refl_proof() {
    let expr = var(0);
    let proof = Proof::Refl(expr.clone());

    let result = verify(&proof);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert_eq!(left, expr);
    assert_eq!(right, expr);
}

#[test]
fn test_sym_proof() {
    // Create a beta step proof, then apply symmetry
    let redex = app(lam(var(0)), var(1));
    let beta_proof = prove_beta(redex.clone());
    let sym_proof = Proof::Sym(Box::new(beta_proof));

    let result = verify(&sym_proof);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert_eq!(left, var(1));
    assert!(alpha_equiv(right, redex));
}

#[test]
fn test_trans_proof() {
    // Create two beta step proofs that can be chained with transitivity
    // Start with: (λ.0) ((λ.0) 1)
    // First reduction: (λ.0) ((λ.0) 1) → (λ.0) 1
    // Second reduction: (λ.0) 1 → 1
    let inner_redex = app(lam(var(0)), var(1)); // (λ.0) 1
    let outer_redex = app(lam(var(0)), inner_redex.clone()); // (λ.0) ((λ.0) 1)

    let proof1 = prove_beta(outer_redex.clone()); // (λ.0) ((λ.0) 1) → (λ.0) 1
    let proof2 = prove_beta(inner_redex.clone()); // (λ.0) 1 → 1

    let trans_proof = Proof::Trans {
        proof_a: Box::new(proof1),
        proof_b: Box::new(proof2),
    };

    let result = verify(&trans_proof);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert!(alpha_equiv(left, outer_redex));
    assert_eq!(right, var(1));
}

#[test]
fn test_cong_app_proof() {
    // Test congruence for application: if F ≡ G and A ≡ B, then F A ≡ G B
    let f1 = lam(var(0));
    let f2 = lam(var(0)); // Same as f1, so F ≡ G
    let a1 = var(1);
    let a2 = var(1); // Same as a1, so A ≡ B

    let proof_f = Proof::Refl(f1.clone());
    let proof_a = Proof::Refl(a1.clone());

    let cong_app_proof = Proof::CongApp {
        proof_f: Box::new(proof_f),
        proof_a: Box::new(proof_a),
    };

    let result = verify(&cong_app_proof);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();

    let expected_left = app(f1, a1);
    let expected_right = app(f2, a2);

    assert!(alpha_equiv(left, expected_left));
    assert!(alpha_equiv(right, expected_right));
}

#[test]
fn test_cong_lam_proof() {
    // Test congruence for abstraction: if M ≡ N, then λ.M ≡ λ.N
    let m = var(0);
    let n = var(0); // Same as m, so M ≡ N

    let proof_b = Proof::Refl(m.clone());

    let cong_lam_proof = Proof::CongLam {
        proof_b: Box::new(proof_b),
    };

    let result = verify(&cong_lam_proof);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();

    let expected_left = lam(m);
    let expected_right = lam(n);

    assert!(alpha_equiv(left, expected_left));
    assert!(alpha_equiv(right, expected_right));
}

#[test]
fn test_normalization_proof() {
    // Test normalization proof for a simple expression
    let expr = app(lam(var(0)), var(1)); // (λ.0) 1
    let proof = prove_normalization(expr.clone(), 10).unwrap();

    let result = verify(&proof);
    assert!(result.is_ok());
    let (left, right) = result.unwrap();
    assert!(alpha_equiv(left, expr));
    assert_eq!(right, var(1));
}

#[test]
fn test_invalid_beta_step() {
    // Test invalid beta step verification
    let redex = var(0);
    let contractum = var(1);
    let invalid_proof = Proof::BetaStep { redex, contractum };

    let result = verify(&invalid_proof);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ProofError::InvalidBetaStep(_)
    ));
}

#[test]
fn test_invalid_eta_step() {
    // Test invalid eta step verification
    let redex = var(0); // Not eta-reducible
    let contractum = var(1);
    let invalid_proof = Proof::EtaStep { redex, contractum };

    let result = verify(&invalid_proof);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), ProofError::InvalidEtaStep(_)));
}
