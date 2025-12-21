/// Proof checker implementation according to CoreSpec v1.0
use crate::core_expr::{deserialize_core_expr, serialize_core_expr, CoreExpr};
use crate::core_kernel::{alpha_equiv, beta_reduce_step, eta_reduce, normalize};
use serde::{Deserialize, Serialize};
use std::fmt;
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

/// Error type for proof serialization/deserialization failures.
#[derive(Debug, PartialEq)]
pub enum ProofParseError {
    EmptyInput,
    IncompleteData,
    InvalidTag(u8),
    InvalidLengthPrefix,
    CoreExprParseError(String),
}

impl fmt::Display for ProofParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProofParseError::EmptyInput => write!(f, "Empty input"),
            ProofParseError::IncompleteData => write!(f, "Incomplete data"),
            ProofParseError::InvalidTag(tag) => write!(f, "Invalid tag: {}", tag),
            ProofParseError::InvalidLengthPrefix => write!(f, "Invalid length prefix"),
            ProofParseError::CoreExprParseError(msg) => write!(f, "CoreExpr parse error: {}", msg),
        }
    }
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

/// Binary serialization format for Proof
/// Format specification:
/// - Little-endian encoding
/// - BetaStep: [0x01, redex_bytes..., contractum_bytes...]
/// - EtaStep: [0x02, redex_bytes..., contractum_bytes...]
/// - Refl: [0x03, expr_bytes...]
/// - Sym: [0x04, subproof_bytes...]
/// - Trans: [0x05, left_bytes..., right_bytes...]
/// - CongApp: [0x06, f_bytes..., a_bytes...]
/// - CongLam: [0x07, b_bytes...]
pub fn serialize_proof(proof: &Proof) -> Vec<u8> {
    let mut bytes = Vec::new();
    match proof {
        Proof::BetaStep { redex, contractum } => {
            bytes.push(0x01);
            bytes.extend_from_slice(&serialize_core_expr(redex));
            bytes.extend_from_slice(&serialize_core_expr(contractum));
        }
        Proof::EtaStep { redex, contractum } => {
            bytes.push(0x02);
            bytes.extend_from_slice(&serialize_core_expr(redex));
            bytes.extend_from_slice(&serialize_core_expr(contractum));
        }
        Proof::Refl(expr) => {
            bytes.push(0x03);
            bytes.extend_from_slice(&serialize_core_expr(expr));
        }
        Proof::Sym(subproof) => {
            bytes.push(0x04);
            bytes.extend_from_slice(&serialize_proof(subproof));
        }
        Proof::Trans { proof_a, proof_b } => {
            bytes.push(0x05);
            bytes.extend_from_slice(&serialize_proof(proof_a));
            bytes.extend_from_slice(&serialize_proof(proof_b));
        }
        Proof::CongApp { proof_f, proof_a } => {
            bytes.push(0x06);
            bytes.extend_from_slice(&serialize_proof(proof_f));
            bytes.extend_from_slice(&serialize_proof(proof_a));
        }
        Proof::CongLam { proof_b } => {
            bytes.push(0x07);
            bytes.extend_from_slice(&serialize_proof(proof_b));
        }
    }
    bytes
}

/// Deserialize a proof from binary format
pub fn deserialize_proof(bytes: &[u8]) -> Result<Proof, ProofParseError> {
    if bytes.is_empty() {
        return Err(ProofParseError::EmptyInput);
    }

    let mut cursor = 0;
    let tag = bytes[cursor];
    cursor += 1;

    match tag {
        0x01 => {
            // BetaStep
            let redex = deserialize_core_expr(&bytes[cursor..])
                .map_err(|e| ProofParseError::CoreExprParseError(format!("{:?}", e)))?;
            let redex_len = serialize_core_expr(&redex).len();
            let remaining = &bytes[cursor + redex_len..];
            if remaining.is_empty() {
                return Err(ProofParseError::IncompleteData);
            }
            let contractum = deserialize_core_expr(remaining)
                .map_err(|e| ProofParseError::CoreExprParseError(format!("{:?}", e)))?;
            Ok(Proof::BetaStep { redex, contractum })
        }
        0x02 => {
            // EtaStep
            let redex = deserialize_core_expr(&bytes[cursor..])
                .map_err(|e| ProofParseError::CoreExprParseError(format!("{:?}", e)))?;
            let redex_len = serialize_core_expr(&redex).len();
            let remaining = &bytes[cursor + redex_len..];
            if remaining.is_empty() {
                return Err(ProofParseError::IncompleteData);
            }
            let contractum = deserialize_core_expr(remaining)
                .map_err(|e| ProofParseError::CoreExprParseError(format!("{:?}", e)))?;
            Ok(Proof::EtaStep { redex, contractum })
        }
        0x03 => {
            // Refl
            let expr = deserialize_core_expr(&bytes[cursor..])
                .map_err(|e| ProofParseError::CoreExprParseError(format!("{:?}", e)))?;
            Ok(Proof::Refl(expr))
        }
        0x04 => {
            // Sym
            let subproof = deserialize_proof(&bytes[cursor..])?;
            Ok(Proof::Sym(Box::new(subproof)))
        }
        0x05 => {
            // Trans
            let proof_a = deserialize_proof(&bytes[cursor..])?;
            let proof_a_serialized = serialize_proof(&proof_a);
            let proof_a_len = proof_a_serialized.len();
            let remaining = &bytes[cursor + proof_a_len..];
            if remaining.is_empty() {
                return Err(ProofParseError::IncompleteData);
            }
            let proof_b = deserialize_proof(remaining)?;
            Ok(Proof::Trans {
                proof_a: Box::new(proof_a),
                proof_b: Box::new(proof_b),
            })
        }
        0x06 => {
            // CongApp
            let proof_f = deserialize_proof(&bytes[cursor..])?;
            let proof_f_serialized = serialize_proof(&proof_f);
            let proof_f_len = proof_f_serialized.len();
            let remaining = &bytes[cursor + proof_f_len..];
            if remaining.is_empty() {
                return Err(ProofParseError::IncompleteData);
            }
            let proof_a = deserialize_proof(remaining)?;
            Ok(Proof::CongApp {
                proof_f: Box::new(proof_f),
                proof_a: Box::new(proof_a),
            })
        }
        0x07 => {
            // CongLam
            let proof_b = deserialize_proof(&bytes[cursor..])?;
            Ok(Proof::CongLam {
                proof_b: Box::new(proof_b),
            })
        }
        _ => Err(ProofParseError::InvalidTag(tag)),
    }
}

#[cfg(test)]
#[path = "test/proof_checker_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "test/proof_checker_serialization_tests.rs"]
mod serialization_tests;
