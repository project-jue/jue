/// Proof checker implementation according to CoreSpec v1.0
use crate::core_expr::CoreExpr;
use crate::core_kernel::{alpha_equiv, beta_reduce_step, eta_reduce, normalize};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Proof of equivalence between λ-calculus terms according to CoreSpec v1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Proof {
    /// A single β-reduction step. `redex` must be of the form `App(Lam(body), arg)`.
    BetaStep {
        redex: CoreExpr,
        contractum: CoreExpr,
    },
    /// A single η-reduction step.
    EtaStep {
        redex: CoreExpr,
        contractum: CoreExpr,
    },
    /// A reflexive proof: a term is equivalent to itself.
    Refl(CoreExpr),
    /// Symmetry of equivalence. `proof` proves `B ≡ A`.
    Sym(Box<Proof>),
    /// Transitivity of equivalence. `proof_a` proves `A ≡ B`, `proof_b` proves `B ≡ C`.
    Trans {
        proof_a: Box<Proof>,
        proof_b: Box<Proof>,
    },
    /// Congruence for application. `proof_f` proves `F ≡ G`, `proof_a` proves `A ≡ B`.
    CongApp {
        proof_f: Box<Proof>,
        proof_a: Box<Proof>,
    },
    /// Congruence for abstraction. `proof_b` proves `M ≡ N`.
    CongLam { proof_b: Box<Proof> },
}

/// Error type for proof verification failures.
#[derive(Debug, Error, PartialEq)]
pub enum ProofError {
    #[error("Invalid beta step: {0}")]
    InvalidBetaStep(String),
    #[error("Invalid eta step: {0}")]
    InvalidEtaStep(String),
    #[error("Invalid reflexivity: {0}")]
    InvalidReflexivity(String),
    #[error("Invalid symmetry: {0}")]
    InvalidSymmetry(String),
    #[error("Invalid transitivity: {0}")]
    InvalidTransitivity(String),
    #[error("Invalid congruence for application: {0}")]
    InvalidCongApp(String),
    #[error("Invalid congruence for abstraction: {0}")]
    InvalidCongLam(String),
}

/// Verify a proof and return the pair of equivalent terms it proves.
/// Signature: `verify(proof: &Proof) -> Result<(CoreExpr, CoreExpr), ProofError>`
pub fn verify(proof: &Proof) -> Result<(CoreExpr, CoreExpr), ProofError> {
    match proof {
        Proof::BetaStep { redex, contractum } => {
            // Verify that one β-reduction step transforms redex to contractum
            let actual_contractum = beta_reduce_step(redex.clone());
            if alpha_equiv(actual_contractum.clone(), contractum.clone()) {
                Ok((redex.clone(), contractum.clone()))
            } else {
                Err(ProofError::InvalidBetaStep(format!(
                    "Beta reduction of {:?} should yield {:?}, but got {:?}",
                    redex, contractum, actual_contractum
                )))
            }
        }

        Proof::EtaStep { redex, contractum } => {
            // Verify that one η-reduction step transforms redex to contractum
            let actual_contractum = eta_reduce(redex.clone());
            if alpha_equiv(actual_contractum.clone(), contractum.clone()) {
                Ok((redex.clone(), contractum.clone()))
            } else {
                Err(ProofError::InvalidEtaStep(format!(
                    "Eta reduction of {:?} should yield {:?}, but got {:?}",
                    redex, contractum, actual_contractum
                )))
            }
        }

        Proof::Refl(expr) => {
            // Reflexivity: any term is equivalent to itself
            Ok((expr.clone(), expr.clone()))
        }

        Proof::Sym(subproof) => {
            // Symmetry: if subproof proves A ≡ B, then Sym(subproof) proves B ≡ A
            let (a, b) = verify(subproof)?;
            Ok((b, a))
        }

        Proof::Trans { proof_a, proof_b } => {
            // Transitivity: if proof_a proves A ≡ B and proof_b proves B ≡ C, then Trans proves A ≡ C
            let (a, b) = verify(proof_a)?;
            let (c, d) = verify(proof_b)?;

            if alpha_equiv(b.clone(), c.clone()) {
                Ok((a, d))
            } else {
                Err(ProofError::InvalidTransitivity(format!(
                    "Middle terms don't match: {:?} ≠ {:?}",
                    b, c
                )))
            }
        }

        Proof::CongApp { proof_f, proof_a } => {
            // Congruence for application: if proof_f proves F ≡ G and proof_a proves A ≡ B,
            // then CongApp proves (F A) ≡ (G B)
            let (f, g) = verify(proof_f)?;
            let (a, b) = verify(proof_a)?;

            let app1 = CoreExpr::App(Box::new(f.clone()), Box::new(a.clone()));
            let app2 = CoreExpr::App(Box::new(g.clone()), Box::new(b.clone()));

            Ok((app1, app2))
        }

        Proof::CongLam { proof_b } => {
            // Congruence for abstraction: if proof_b proves M ≡ N, then CongLam proves (λ.M) ≡ (λ.N)
            let (m, n) = verify(proof_b)?;

            let lam1 = CoreExpr::Lam(Box::new(m.clone()));
            let lam2 = CoreExpr::Lam(Box::new(n.clone()));

            Ok((lam1, lam2))
        }
    }
}

/// Generate a proof for a single β-reduction step.
pub fn prove_beta(redex: CoreExpr) -> Proof {
    let contractum = beta_reduce_step(redex.clone());
    Proof::BetaStep { redex, contractum }
}

/// Generate a proof for a single η-reduction step.
pub fn prove_eta(redex: CoreExpr) -> Result<Proof, ProofError> {
    let contractum = eta_reduce(redex.clone());
    if alpha_equiv(contractum.clone(), redex.clone()) {
        Err(ProofError::InvalidEtaStep(format!(
            "Expression {:?} is not eta-reducible",
            redex
        )))
    } else {
        Ok(Proof::EtaStep { redex, contractum })
    }
}

/// Generate a proof that `term` normalizes to its normal form through a sequence of steps.
pub fn prove_normalization(term: CoreExpr, step_limit: usize) -> Result<Proof, ProofError> {
    let normal_form = normalize(term.clone());
    let mut current = term.clone();
    let mut steps = Vec::new();

    // Collect reduction steps
    for _ in 0..step_limit {
        if alpha_equiv(current.clone(), normal_form.clone()) {
            break;
        }

        let next = beta_reduce_step(current.clone());
        if alpha_equiv(next.clone(), current.clone()) {
            // Try eta reduction if beta reduction didn't make progress
            let eta_next = eta_reduce(current.clone());
            if !alpha_equiv(eta_next.clone(), current.clone()) {
                steps.push(Proof::EtaStep {
                    redex: current.clone(),
                    contractum: eta_next.clone(),
                });
                current = eta_next;
                continue;
            }
            break;
        }

        steps.push(Proof::BetaStep {
            redex: current.clone(),
            contractum: next.clone(),
        });
        current = next;
    }

    if !alpha_equiv(current.clone(), normal_form.clone()) {
        return Err(ProofError::InvalidTransitivity(format!(
            "Normalization did not complete within {} steps",
            step_limit
        )));
    }

    // Build the proof using transitivity
    if steps.is_empty() {
        // Already in normal form
        Ok(Proof::Refl(term))
    } else if steps.len() == 1 {
        // Single step
        Ok(steps.into_iter().next().unwrap())
    } else {
        // Multiple steps: chain them together with transitivity
        let mut steps_iter = steps.into_iter();
        let mut proof = steps_iter.next().unwrap();
        for step in steps_iter {
            proof = Proof::Trans {
                proof_a: Box::new(proof),
                proof_b: Box::new(step),
            };
        }
        Ok(proof)
    }
}

#[cfg(test)]
mod tests {
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
}
