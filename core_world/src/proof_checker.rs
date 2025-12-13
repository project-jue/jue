/// Proof checker implementation
/// This module contains functions for verifying and attaching proofs to expressions
use crate::core_expr::CoreExpr;
use crate::core_kernel::{alpha_equiv, beta_reduce, normalize};
use crate::eval_relation::{eval_empty, EvalResult};
use std::fmt;

/// Proof type representing a formal proof of expression properties
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Proof {
    /// Proof of β-reduction: (λx.M) N → M[x→N]
    BetaReduction {
        original: CoreExpr,
        reduced: CoreExpr,
        step: String,
    },
    /// Proof of α-equivalence: M ≡ N
    AlphaEquivalence { expr1: CoreExpr, expr2: CoreExpr },
    /// Proof of normalization: expr →* normal_form
    Normalization {
        original: CoreExpr,
        normal_form: CoreExpr,
        steps: Vec<CoreExpr>,
    },
    /// Proof of evaluation: expr ⇒ value
    Evaluation { expr: CoreExpr, result: EvalResult },
    /// Proof of consistency: kernel properties hold
    Consistency,
    /// Composite proof combining multiple proofs
    Composite {
        proofs: Vec<Proof>,
        conclusion: String,
    },
}

impl fmt::Display for Proof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Proof::BetaReduction {
                original,
                reduced,
                step,
            } => {
                write!(f, "β-reduction: {} → {} ({})", original, reduced, step)
            }
            Proof::AlphaEquivalence { expr1, expr2 } => {
                write!(f, "α-equivalence: {} ≡ {}", expr1, expr2)
            }
            Proof::Normalization {
                original,
                normal_form,
                steps,
            } => {
                write!(
                    f,
                    "normalization: {} →* {} ({} steps)",
                    original,
                    normal_form,
                    steps.len()
                )
            }
            Proof::Evaluation { expr, result } => {
                write!(f, "evaluation: {} ⇒ {:?}", expr, result)
            }
            Proof::Consistency => {
                write!(f, "consistency: kernel properties verified")
            }
            Proof::Composite { proofs, conclusion } => {
                write!(
                    f,
                    "composite proof ({} subproofs): {}",
                    proofs.len(),
                    conclusion
                )
            }
        }
    }
}

/// Verify that a proof is correct for a given expression
pub fn verify_proof(proof: &Proof, expr: &CoreExpr) -> bool {
    match proof {
        Proof::BetaReduction {
            original, reduced, ..
        } => {
            // Check that the original expression matches the input
            // and that the reduction is actually a β-reduction
            if original != expr {
                return false;
            }

            // Perform the reduction and check it matches
            let actual_reduced = beta_reduce(original.clone());
            actual_reduced == *reduced
        }
        Proof::AlphaEquivalence { expr1, expr2 } => {
            // Check that one of the expressions matches the input
            // and that they are actually α-equivalent
            if expr1 != expr && expr2 != expr {
                return false;
            }

            // Check α-equivalence
            alpha_equiv(expr1.clone(), expr2.clone())
        }
        Proof::Normalization {
            original,
            normal_form,
            steps,
        } => {
            // Check that the original expression matches the input
            if original != expr {
                return false;
            }

            // Verify the normalization steps
            let mut current = original.clone();
            let mut verified_steps = Vec::new();

            for step in steps {
                let reduced = beta_reduce(current.clone());
                if reduced != *step {
                    return false;
                }
                verified_steps.push(reduced.clone());
                current = reduced;
            }

            // Check that the final step matches the normal form
            let final_normalized = normalize(original.clone());
            current == *normal_form && final_normalized == *normal_form
        }
        Proof::Evaluation {
            expr: proof_expr,
            result,
        } => {
            // Check that the expression matches the input
            if proof_expr != expr {
                return false;
            }

            // Perform evaluation and check it matches
            let actual_result = eval_empty(expr.clone());
            actual_result == *result
        }
        Proof::Consistency => {
            // Consistency proof doesn't depend on a specific expression
            // Just verify that kernel consistency holds
            crate::core_kernel::prove_kernel_consistency()
        }
        Proof::Composite { proofs, .. } => {
            // Verify all subproofs
            // Note: In a composite proof, subproofs might be for different expressions
            // that are related to the main expression, or they might be for the same expression.
            // However, the current implementation of verify_proof takes a single expr argument.
            // If we enforce that all subproofs must be valid for *this* expr, then we can't
            // have composite proofs that chain steps (A->B, B->C).
            // But based on the test case `test_composite_proof_semantics`, it seems we expect
            // subproofs to be valid for the *given* expression.
            // If a subproof is for a different expression, verify_proof(subproof, expr) should return false.
            proofs.iter().all(|subproof| verify_proof(subproof, expr))
        }
    }
}

/// Attach a proof to an expression, creating a proven expression
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenExpr {
    pub expr: CoreExpr,
    pub proof: Proof,
}

impl ProvenExpr {
    /// Create a new proven expression
    pub fn new(expr: CoreExpr, proof: Proof) -> Self {
        Self { expr, proof }
    }

    /// Verify the proof attached to this expression
    pub fn verify(&self) -> bool {
        verify_proof(&self.proof, &self.expr)
    }
}

/// Generate a β-reduction proof for an expression
pub fn prove_beta_reduction(expr: CoreExpr) -> Option<Proof> {
    let reduced = beta_reduce(expr.clone());
    if reduced != expr {
        Some(Proof::BetaReduction {
            original: expr,
            reduced,
            step: "single β-reduction".to_string(),
        })
    } else {
        None
    }
}

/// Generate an α-equivalence proof for two expressions
pub fn prove_alpha_equivalence(expr1: CoreExpr, expr2: CoreExpr) -> Option<Proof> {
    if alpha_equiv(expr1.clone(), expr2.clone()) {
        Some(Proof::AlphaEquivalence { expr1, expr2 })
    } else {
        None
    }
}

/// Generate a normalization proof for an expression
pub fn prove_normalization(expr: CoreExpr) -> Proof {
    let normal_form = normalize(expr.clone());

    // Collect all intermediate steps
    let mut steps = Vec::new();
    let mut current = expr.clone();

    while current != normal_form {
        let next = beta_reduce(current.clone());
        if next == current {
            break; // No more reductions possible
        }
        steps.push(next.clone());
        current = next;
    }

    Proof::Normalization {
        original: expr,
        normal_form,
        steps,
    }
}

/// Generate an evaluation proof for an expression
pub fn prove_evaluation(expr: CoreExpr) -> Proof {
    let result = eval_empty(expr.clone());
    Proof::Evaluation { expr, result }
}

/// Generate a consistency proof for the kernel
pub fn prove_consistency() -> Proof {
    Proof::Consistency
}

/// Attach a proof to an expression
pub fn attach_proof(expr: CoreExpr, proof: Proof) -> ProvenExpr {
    ProvenExpr::new(expr, proof)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_expr::{app, lam, var};

    #[test]
    fn test_beta_reduction_proof() {
        // Create (λx.x) y → y
        let identity = lam(var(0));
        let y = var(1);
        let expr = app(identity, y);

        let proof = prove_beta_reduction(expr.clone()).unwrap();
        assert!(verify_proof(&proof, &expr));
    }

    #[test]
    fn test_alpha_equivalence_proof() {
        // Create two α-equivalent expressions: λx.x and λy.y
        let expr1 = lam(var(0));
        let expr2 = lam(var(0)); // Same structure

        let proof = prove_alpha_equivalence(expr1.clone(), expr2.clone()).unwrap();
        assert!(verify_proof(&proof, &expr1));
        assert!(verify_proof(&proof, &expr2));
    }

    #[test]
    fn test_normalization_proof() {
        let expr = app(lam(var(0)), var(1));
        let proof = prove_normalization(expr.clone());
        assert!(verify_proof(&proof, &expr));
    }

    #[test]
    fn test_evaluation_proof() {
        let expr = app(lam(var(0)), var(1));
        let proof = prove_evaluation(expr.clone());
        assert!(verify_proof(&proof, &expr));
    }

    #[test]
    fn test_consistency_proof() {
        let proof = prove_consistency();
        assert!(verify_proof(&proof, &lam(var(0)))); // Any expr should work
    }

    #[test]
    fn test_attach_proof() {
        let expr = lam(var(0));
        let proof = prove_evaluation(expr.clone());
        let proven_expr = attach_proof(expr.clone(), proof);

        assert!(proven_expr.verify());
        assert_eq!(proven_expr.expr, expr);
    }

    #[test]
    fn test_proven_expr_verify() {
        let expr = app(lam(var(0)), var(1));
        let proof = prove_beta_reduction(expr.clone()).unwrap();
        let proven_expr = ProvenExpr::new(expr, proof);

        assert!(proven_expr.verify());
    }

    #[test]
    fn test_composite_proof() {
        let expr = app(lam(var(0)), var(1));

        let beta_proof = prove_beta_reduction(expr.clone()).unwrap();
        let eval_proof = prove_evaluation(expr.clone());

        let composite_proof = Proof::Composite {
            proofs: vec![beta_proof, eval_proof],
            conclusion: "β-reduction and evaluation".to_string(),
        };

        assert!(verify_proof(&composite_proof, &expr));
    }

    #[test]
    fn test_invalid_proof() {
        let expr1 = lam(var(0));
        let expr2 = lam(var(1));

        // This should fail because the expressions are not α-equivalent
        let proof = Proof::AlphaEquivalence {
            expr1: expr1.clone(),
            expr2: expr2.clone(),
        };

        assert!(!verify_proof(&proof, &expr1));
    }
}
