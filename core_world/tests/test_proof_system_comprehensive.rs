/// Proof System Comprehensive Tests
use core_world::core_expr::{app, lam, var};
use core_world::proof_checker::{
    attach_proof, prove_alpha_equivalence, prove_beta_reduction,
    prove_consistency as prove_consistency_proof, prove_evaluation, prove_normalization,
    verify_proof, Proof, ProvenExpr,
};

#[test]
fn test_proof_verification_comprehensive() {
    // Test β-reduction proof
    let identity = lam(var(0));
    let y = var(1);
    let expr = app(identity, y);
    let proof = prove_beta_reduction(expr.clone()).unwrap();
    assert!(verify_proof(&proof, &expr));

    // Test α-equivalence proof
    let expr1 = lam(var(0));
    let expr2 = lam(var(0));
    let proof = prove_alpha_equivalence(expr1.clone(), expr2.clone()).unwrap();
    assert!(verify_proof(&proof, &expr1));

    // Test normalization proof
    let expr = app(lam(var(0)), var(1));
    let proof = prove_normalization(expr.clone());
    assert!(verify_proof(&proof, &expr));

    // Test evaluation proof
    let expr = app(lam(var(0)), var(1));
    let proof = prove_evaluation(expr.clone());
    assert!(verify_proof(&proof, &expr));

    // Test consistency proof
    let proof = prove_consistency_proof();
    assert!(verify_proof(&proof, &lam(var(0))));

    // Test multiple proofs for same expression
    let expr = app(lam(var(0)), var(1));
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());
    let norm_proof = prove_normalization(expr.clone());

    assert!(verify_proof(&beta_proof, &expr));
    assert!(verify_proof(&eval_proof, &expr));
    assert!(verify_proof(&norm_proof, &expr));

    // Create a composite proof
    let composite_proof = Proof::Composite {
        proofs: vec![beta_proof, eval_proof, norm_proof],
        conclusion: "Complete proof verification".to_string(),
    };

    assert!(verify_proof(&composite_proof, &expr));
}

#[test]
fn test_proven_expr_comprehensive() {
    // Test simple proven expression
    let expr = lam(var(0));
    let proof = prove_evaluation(expr.clone());
    let proven_expr = attach_proof(expr.clone(), proof);

    assert!(proven_expr.verify());
    assert_eq!(proven_expr.expr, expr);

    // Test complex proven expression
    let expr = app(lam(var(0)), var(1));
    let proof = prove_beta_reduction(expr.clone()).unwrap();
    let proven_expr = ProvenExpr::new(expr.clone(), proof);

    assert!(proven_expr.verify());
    assert_eq!(proven_expr.expr, expr);

    // Test multiple proofs for same expression
    let expr_clone = expr.clone();
    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr_clone.clone());

    let beta_proven = ProvenExpr::new(expr.clone(), beta_proof);
    let eval_proven = ProvenExpr::new(expr_clone, eval_proof);

    assert!(beta_proven.verify());
    assert!(eval_proven.verify());
}

#[test]
fn test_composite_proof_comprehensive() {
    // Test simple composite proof
    let expr = app(lam(var(0)), var(1));

    let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
    let eval_proof = prove_evaluation(expr.clone());

    let composite_proof = Proof::Composite {
        proofs: vec![beta_proof, eval_proof],
        conclusion: "β-reduction and evaluation".to_string(),
    };

    assert!(verify_proof(&composite_proof, &expr));

    // Test complex composite proof
    let expr_clone = expr.clone();
    let norm_proof = prove_normalization(expr_clone.clone());

    let complex_composite = Proof::Composite {
        proofs: vec![
            prove_beta_reduction(expr.clone()).unwrap(),
            prove_evaluation(expr.clone()),
            norm_proof,
        ],
        conclusion: "Complete proof verification".to_string(),
    };

    assert!(verify_proof(&complex_composite, &expr));

    // Test nested composite proofs
    let inner_composite = Proof::Composite {
        proofs: vec![
            prove_beta_reduction(expr.clone()).unwrap(),
            prove_evaluation(expr.clone()),
        ],
        conclusion: "Inner composite".to_string(),
    };

    let outer_composite = Proof::Composite {
        proofs: vec![inner_composite, prove_normalization(expr.clone())],
        conclusion: "Outer composite".to_string(),
    };

    assert!(verify_proof(&outer_composite, &expr));
}

#[test]
fn test_invalid_proof_comprehensive() {
    // Test invalid α-equivalence proof
    let expr1 = lam(var(0));
    let expr2 = lam(var(1));

    let proof = Proof::AlphaEquivalence {
        expr1: expr1.clone(),
        expr2: expr2.clone(),
    };

    assert!(!verify_proof(&proof, &expr1));

    // Test invalid β-reduction proof
    let expr = lam(var(0));
    let invalid_reduced = var(999);

    let proof = Proof::BetaReduction {
        original: expr.clone(),
        reduced: invalid_reduced,
        step: "invalid reduction".to_string(),
    };

    assert!(!verify_proof(&proof, &expr));

    // Test invalid normalization proof
    let expr = lam(var(0));
    let invalid_normal_form = var(999);

    let proof = Proof::Normalization {
        original: expr.clone(),
        normal_form: invalid_normal_form,
        steps: vec![],
    };

    assert!(!verify_proof(&proof, &expr));
}
