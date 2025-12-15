/// Core-World library module
/// This module contains the formal λ-calculus kernel with relational semantics
use crate::{core_expr::CoreExpr, proof_checker::Proof};

pub mod core_expr;
pub mod core_kernel;
pub mod proof_checker;

/// The primary export: verifies that a proof correctly establishes term equivalence.
pub fn verify_equivalence(proof: Proof) -> Result<(CoreExpr, CoreExpr), VerifyError> {
    match proof_checker::verify(&proof) {
        Ok(result) => Ok(result),
        Err(proof_error) => Err(VerifyError::ProofRuleViolation(proof_error.to_string())),
    }
}

/// Utility: Returns the βη-normal form of a term, if reachable within limits.
/// Used for debugging and specification, not for runtime.
pub fn normalize(term: CoreExpr, step_limit: usize) -> Result<CoreExpr, NormalizationError> {
    core_kernel::normalize_with_limit(term, step_limit)
}

/// Public error types.
pub enum VerifyError {
    InvalidProofStructure,
    ProofRuleViolation(String),
}

pub enum NormalizationError {
    StepLimitExceeded(usize),
}
