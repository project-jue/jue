#![allow(warnings)]
/// Core-World library module
/// This module contains the formal λ-calculus kernel with relational semantics
///
/// # Capability Boundary (V2 Specification)
///
/// Core-World is the immutable formal kernel of Project Jue. It defines the mathematical
/// meaning of all computations via pure, untyped λ-calculus. Core-World does NOT reason
/// about capabilities - capabilities are operational constructs that belong to Physics-World
/// and Jue-World.
///
/// ## Core-World's Role in Capability Safety
///
/// When Jue-World compiles code that uses capabilities, it must prove that the
/// capability-wrapped operations preserve the underlying λ-calculus semantics.
/// Core-World plays a crucial role in this process:
///
/// 1. **Capability Safety**: Core-World verifies that the pure λ-calculus fragments
///    of capability-using code are semantically correct.
///
/// 2. **Proof Composition**: A proof that code with capabilities is correct can be
///    composed from:
///    - Core-World proofs about the pure λ-calculus fragments
///    - Physics-World guarantees about capability enforcement
///
/// 3. **Boundary**: The boundary between pure computation and capability-mediated
///    operations must be explicit in proofs.
///
/// ## Example: Capability Usage Proof Obligation
///
/// If Jue code uses `IoReadSensor` capability, the proof obligation is:
/// - The sensor read operation returns a value of type `Int`
/// - The surrounding λ-calculus code correctly handles that `Int`
/// - Core-World verifies the λ-calculus part
/// - Physics-World verifies the capability part
///
/// ## Usage Examples
///
/// For detailed usage examples, see the integration tests and the test suite.
/// The core functionality includes:
/// - Creating λ-calculus expressions with `var()`, `lam()`, `app()`
/// - Performing β-reduction and normalization
/// - Generating and verifying equivalence proofs
/// - Serializing/deserializing expressions and proofs
///
/// ## Best Practices
///
/// 1. **Pure Functions Only**: Core-World functions are deterministic and side-effect free.
/// 2. **Extensionality**: Terms are considered equivalent under βη-reduction.
/// 3. **Trusted Base**: The implementation is small, auditable, and frozen after verification.
/// 4. **Serialization**: Use standardized binary format for cross-layer communication.
use crate::{core_expr::CoreExpr, proof_checker::Proof};

pub mod core_expr;
pub mod core_kernel;
pub mod proof_checker;

// Re-export helper functions for convenience
pub use core_expr::{app, lam, nat, pair, var};
pub use core_kernel::alpha_equiv;
pub use proof_checker::prove_beta;

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

/// V2 Serialization: Serialize a CoreExpr to binary format.
/// Format specification: Little-endian encoding with tagged union structure.
/// - Var(n): [0x01, n as u64]
/// - Lam(body): [0x02, body_bytes...]
/// - App(f, a): [0x03, f_bytes..., a_bytes...]
/// - Nat(n): [0x04, n as u64]
/// - Pair(f, s): [0x05, f_bytes..., s_bytes...]
pub fn serialize_core_expr(expr: &CoreExpr) -> Vec<u8> {
    core_expr::serialize_core_expr(expr)
}

/// V2 Serialization: Deserialize a CoreExpr from binary format.
/// Returns ParseError if the input is malformed or incomplete.
pub fn deserialize_core_expr(bytes: &[u8]) -> Result<CoreExpr, ParseError> {
    core_expr::deserialize_core_expr(bytes)
}

/// V2 Serialization: Serialize a Proof to binary format.
/// Format specification: Little-endian encoding with tagged union structure.
/// - BetaStep: [0x01, redex_bytes..., contractum_bytes...]
/// - EtaStep: [0x02, redex_bytes..., contractum_bytes...]
/// - Refl: [0x03, expr_bytes...]
/// - Sym: [0x04, subproof_bytes...]
/// - Trans: [0x05, left_bytes..., right_bytes...]
/// - CongApp: [0x06, f_bytes..., a_bytes...]
/// - CongLam: [0x07, b_bytes...]
pub fn serialize_proof(proof: &Proof) -> Vec<u8> {
    proof_checker::serialize_proof(proof)
}

/// V2 Serialization: Deserialize a Proof from binary format.
/// Returns ProofParseError if the input is malformed or incomplete.
pub fn deserialize_proof(bytes: &[u8]) -> Result<Proof, ProofParseError> {
    proof_checker::deserialize_proof(bytes)
}

/// V2 Stack-based normalization: Returns the βη-normal form using explicit stack.
/// Optimized for large terms to avoid recursion limits.
/// Returns NormalizationError::StepLimitExceeded if the step limit is reached.
pub fn normalize_stack_based(
    term: CoreExpr,
    step_limit: usize,
) -> Result<CoreExpr, NormalizationError> {
    core_kernel::normalize_stack_based(term, step_limit)
}

/// Public error types.
#[derive(Debug)]
pub enum VerifyError {
    InvalidProofStructure,
    ProofRuleViolation(String),
}

#[derive(Debug)]
pub enum NormalizationError {
    StepLimitExceeded(usize),
}

/// Error type for CoreExpr serialization/deserialization failures.
pub use core_expr::ParseError;

/// Error type for Proof serialization/deserialization failures.
pub use proof_checker::ProofParseError;
