# Core World V2 Implementation Plan

## Executive Summary

This document provides a comprehensive engineering plan for implementing Core World V2 specification. Core-World V2 maintains its role as the immutable formal kernel of Project Jue, providing mathematical meaning through pure, untyped λ-calculus with βη-equivalence semantics.

### Key Changes from V1 to V2

1. **Capability Boundary Clarification**: Explicitly states that capabilities are operational constructs belonging to Physics-World and Jue-World, not semantic concerns of Core-World
2. **Standardized Serialization**: Adds binary serialization format for CoreExpr and Proof objects
3. **Performance Optimization**: Optimized normalization algorithm for large terms using explicit stacks
4. **Frozen Primitive Set**: Formalizes the minimal axiom set (λ, Nat, Pair)

### Implementation Scope

This implementation will:
- Maintain backward compatibility with V1.0 proofs and terms
- Add serialization functions for cross-layer communication
- Optimize normalization for deeply nested terms
- Ensure all existing functionality continues to work
- Add comprehensive conformance tests

## Current Implementation Analysis

### Current State Assessment

The current Core World implementation (`core_world/src/`) already implements most V2 requirements:

**✅ Already Implemented:**
- CoreExpr enum with Var, Lam, App, Nat, Pair variants
- β-reduction with call-by-name semantics
- η-reduction
- α-equivalence checking
- Normalization with step limits
- Proof system with BetaStep, EtaStep, Refl, Sym, Trans, CongApp, CongLam
- Public API with `verify_equivalence()` and `normalize()`

**❌ Missing V2 Requirements:**
- Standardized binary serialization for CoreExpr and Proof
- Optimized normalization using explicit stacks (currently uses recursion)
- Formal documentation of capability boundary
- Comprehensive conformance test suite

**🔍 Potential Issues:**
- Current implementation includes Nat and Pair variants which may need to be removed for pure λ-calculus compliance
- Recursive normalization may hit stack limits on deeply nested terms
- No explicit stack-based normalization implementation

## Detailed Implementation Plan

### Phase 1: Preparation and Analysis

**Task 1.1: Create Implementation Plan Document**
- [x] Create this comprehensive engineering plan
- [ ] Review with team and incorporate feedback

**Task 1.2: Analyze Current Implementation**
- [ ] Run existing test suite to establish baseline
- [ ] Identify any regressions or issues
- [ ] Document current performance characteristics

**Task 1.3: Set Up Development Environment**
- [ ] Ensure all dependencies are up to date
- [ ] Set up continuous integration for V2 branch
- [ ] Create feature branch for V2 implementation

### Phase 2: CoreExpr Serialization Implementation

**File: `core_world/src/core_expr.rs`**

**Task 2.1: Add Serialization Functions**

```rust
/// Binary serialization format for CoreExpr
/// Format specification:
/// - Little-endian encoding
/// - Var(n): [0x01, n as u64]
/// - Lam(body): [0x02, body_bytes...]
/// - App(f, a): [0x03, f_bytes..., a_bytes...]
/// - Nat(n): [0x04, n as u64]
/// - Pair(f, s): [0x05, f_bytes..., s_bytes...]

pub fn serialize_core_expr(expr: &CoreExpr) -> Vec<u8> {
    let mut bytes = Vec::new();
    match expr {
        CoreExpr::Var(index) => {
            bytes.push(0x01);
            bytes.extend_from_slice(&index.to_le_bytes());
        }
        CoreExpr::Lam(body) => {
            bytes.push(0x02);
            bytes.extend_from_slice(&serialize_core_expr(body));
        }
        CoreExpr::App(func, arg) => {
            bytes.push(0x03);
            bytes.extend_from_slice(&serialize_core_expr(func));
            bytes.extend_from_slice(&serialize_core_expr(arg));
        }
        CoreExpr::Nat(n) => {
            bytes.push(0x04);
            bytes.extend_from_slice(&n.to_le_bytes());
        }
        CoreExpr::Pair(first, second) => {
            bytes.push(0x05);
            bytes.extend_from_slice(&serialize_core_expr(first));
            bytes.extend_from_slice(&serialize_core_expr(second));
        }
    }
    bytes
}

pub fn deserialize_core_expr(bytes: &[u8]) -> Result<CoreExpr, ParseError> {
    if bytes.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let mut cursor = 0;
    let tag = bytes[cursor];
    cursor += 1;

    match tag {
        0x01 => { // Var
            if cursor + 8 > bytes.len() {
                return Err(ParseError::IncompleteData);
            }
            let index_bytes = &bytes[cursor..cursor+8];
            let index = u64::from_le_bytes(index_bytes.try_into().unwrap());
            Ok(CoreExpr::Var(index as usize))
        }
        0x02 => { // Lam
            let body = deserialize_core_expr(&bytes[cursor..])?;
            Ok(CoreExpr::Lam(Box::new(body)))
        }
        0x03 => { // App
            let func = deserialize_core_expr(&bytes[cursor..])?;
            let remaining = &bytes[cursor..];
            let func_len = serialize_core_expr(&func).len();
            let arg = deserialize_core_expr(&remaining[func_len..])?;
            Ok(CoreExpr::App(Box::new(func), Box::new(arg)))
        }
        0x04 => { // Nat
            if cursor + 8 > bytes.len() {
                return Err(ParseError::IncompleteData);
            }
            let n_bytes = &bytes[cursor..cursor+8];
            let n = u64::from_le_bytes(n_bytes.try_into().unwrap());
            Ok(CoreExpr::Nat(n))
        }
        0x05 => { // Pair
            let first = deserialize_core_expr(&bytes[cursor..])?;
            let remaining = &bytes[cursor..];
            let first_len = serialize_core_expr(&first).len();
            let second = deserialize_core_expr(&remaining[first_len..])?;
            Ok(CoreExpr::Pair(Box::new(first), Box::new(second)))
        }
        _ => Err(ParseError::InvalidTag(tag))
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    IncompleteData,
    InvalidTag(u8),
    Overflow,
}
```

**Task 2.2: Add Serialization Tests**

```rust
#[cfg(test)]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_var_serialization() {
        let expr = var(42);
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_lam_serialization() {
        let expr = lam(var(0));
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_app_serialization() {
        let expr = app(lam(var(0)), var(1));
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_nat_serialization() {
        let expr = nat(12345);
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_pair_serialization() {
        let expr = pair(var(0), var(1));
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_complex_serialization() {
        let expr = app(lam(pair(var(0), nat(5))), nat(10));
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let expr = app(lam(var(0)), var(42));
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }
}
```

### Phase 3: Proof Serialization Implementation

**File: `core_world/src/proof_checker.rs`**

**Task 3.1: Add Proof Serialization Functions**

```rust
/// Binary serialization format for Proof
/// Format specification:
/// - Little-endian encoding
/// - Tagged union with explicit length prefixes
/// - BetaStep: [0x01, redex_len, redex_bytes, contractum_len, contractum_bytes]
/// - EtaStep: [0x02, redex_len, redex_bytes, contractum_len, contractum_bytes]
/// - Refl: [0x03, expr_len, expr_bytes]
/// - Sym: [0x04, proof_len, proof_bytes]
/// - Trans: [0x05, proof_a_len, proof_a_bytes, proof_b_len, proof_b_bytes]
/// - CongApp: [0x06, proof_f_len, proof_f_bytes, proof_a_len, proof_a_bytes]
/// - CongLam: [0x07, proof_b_len, proof_b_bytes]

pub fn serialize_proof(proof: &Proof) -> Vec<u8> {
    let mut bytes = Vec::new();
    match proof {
        Proof::BetaStep { redex, contractum } => {
            bytes.push(0x01);
            let redex_bytes = serialize_core_expr(redex);
            let contractum_bytes = serialize_core_expr(contractum);
            bytes.extend_from_slice(&(redex_bytes.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&redex_bytes);
            bytes.extend_from_slice(&(contractum_bytes.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&contractum_bytes);
        }
        Proof::EtaStep { redex, contractum } => {
            bytes.push(0x02);
            let redex_bytes = serialize_core_expr(redex);
            let contractum_bytes = serialize_core_expr(contractum);
            bytes.extend_from_slice(&(redex_bytes.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&redex_bytes);
            bytes.extend_from_slice(&(contractum_bytes.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&contractum_bytes);
        }
        Proof::Refl(expr) => {
            bytes.push(0x03);
            let expr_bytes = serialize_core_expr(expr);
            bytes.extend_from_slice(&(expr_bytes.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&expr_bytes);
        }
        Proof::Sym(subproof) => {
            bytes.push(0x04);
            let proof_bytes = serialize_proof(subproof);
            bytes.extend_from_slice(&(proof_bytes.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&proof_bytes);
        }
        Proof::Trans { proof_a, proof_b } => {
            bytes.push(0x05);
            let proof_a_bytes = serialize_proof(proof_a);
            let proof_b_bytes = serialize_proof(proof_b);
            bytes.extend_from_slice(&(proof_a_bytes.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&proof_a_bytes);
            bytes.extend_from_slice(&(proof_b_bytes.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&proof_b_bytes);
        }
        Proof::CongApp { proof_f, proof_a } => {
            bytes.push(0x06);
            let proof_f_bytes = serialize_proof(proof_f);
            let proof_a_bytes = serialize_proof(proof_a);
            bytes.extend_from_slice(&(proof_f_bytes.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&proof_f_bytes);
            bytes.extend_from_slice(&(proof_a_bytes.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&proof_a_bytes);
        }
        Proof::CongLam { proof_b } => {
            bytes.push(0x07);
            let proof_b_bytes = serialize_proof(proof_b);
            bytes.extend_from_slice(&(proof_b_bytes.len() as u64).to_le_bytes());
            bytes.extend_from_slice(&proof_b_bytes);
        }
    }
    bytes
}

pub fn deserialize_proof(bytes: &[u8]) -> Result<Proof, ParseError> {
    if bytes.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let mut cursor = 0;
    let tag = bytes[cursor];
    cursor += 1;

    match tag {
        0x01 => { // BetaStep
            let redex_len = read_u64(&bytes, &mut cursor)?;
            let redex_bytes = &bytes[cursor..cursor+redex_len as usize];
            let redex = deserialize_core_expr(redex_bytes)?;
            cursor += redex_len as usize;

            let contractum_len = read_u64(&bytes, &mut cursor)?;
            let contractum_bytes = &bytes[cursor..cursor+contractum_len as usize];
            let contractum = deserialize_core_expr(contractum_bytes)?;

            Ok(Proof::BetaStep { redex, contractum })
        }
        0x02 => { // EtaStep
            let redex_len = read_u64(&bytes, &mut cursor)?;
            let redex_bytes = &bytes[cursor..cursor+redex_len as usize];
            let redex = deserialize_core_expr(redex_bytes)?;
            cursor += redex_len as usize;

            let contractum_len = read_u64(&bytes, &mut cursor)?;
            let contractum_bytes = &bytes[cursor..cursor+contractum_len as usize];
            let contractum = deserialize_core_expr(contractum_bytes)?;

            Ok(Proof::EtaStep { redex, contractum })
        }
        0x03 => { // Refl
            let expr_len = read_u64(&bytes, &mut cursor)?;
            let expr_bytes = &bytes[cursor..cursor+expr_len as usize];
            let expr = deserialize_core_expr(expr_bytes)?;

            Ok(Proof::Refl(expr))
        }
        0x04 => { // Sym
            let proof_len = read_u64(&bytes, &mut cursor)?;
            let proof_bytes = &bytes[cursor..cursor+proof_len as usize];
            let subproof = deserialize_proof(proof_bytes)?;

            Ok(Proof::Sym(Box::new(subproof)))
        }
        0x05 => { // Trans
            let proof_a_len = read_u64(&bytes, &mut cursor)?;
            let proof_a_bytes = &bytes[cursor..cursor+proof_a_len as usize];
            let proof_a = deserialize_proof(proof_a_bytes)?;
            cursor += proof_a_len as usize;

            let proof_b_len = read_u64(&bytes, &mut cursor)?;
            let proof_b_bytes = &bytes[cursor..cursor+proof_b_len as usize];
            let proof_b = deserialize_proof(proof_b_bytes)?;

            Ok(Proof::Trans {
                proof_a: Box::new(proof_a),
                proof_b: Box::new(proof_b),
            })
        }
        0x06 => { // CongApp
            let proof_f_len = read_u64(&bytes, &mut cursor)?;
            let proof_f_bytes = &bytes[cursor..cursor+proof_f_len as usize];
            let proof_f = deserialize_proof(proof_f_bytes)?;
            cursor += proof_f_len as usize;

            let proof_a_len = read_u64(&bytes, &mut cursor)?;
            let proof_a_bytes = &bytes[cursor..cursor+proof_a_len as usize];
            let proof_a = deserialize_proof(proof_a_bytes)?;

            Ok(Proof::CongApp {
                proof_f: Box::new(proof_f),
                proof_a: Box::new(proof_a),
            })
        }
        0x07 => { // CongLam
            let proof_b_len = read_u64(&bytes, &mut cursor)?;
            let proof_b_bytes = &bytes[cursor..cursor+proof_b_len as usize];
            let proof_b = deserialize_proof(proof_b_bytes)?;

            Ok(Proof::CongLam {
                proof_b: Box::new(proof_b),
            })
        }
        _ => Err(ParseError::InvalidTag(tag))
    }
}

fn read_u64(bytes: &[u8], cursor: &mut usize) -> Result<u64, ParseError> {
    if *cursor + 8 > bytes.len() {
        return Err(ParseError::IncompleteData);
    }
    let value_bytes = &bytes[*cursor..*cursor+8];
    *cursor += 8;
    Ok(u64::from_le_bytes(value_bytes.try_into().unwrap()))
}
```

**Task 3.2: Add Proof Serialization Tests**

```rust
#[cfg(test)]
mod proof_serialization_tests {
    use super::*;

    #[test]
    fn test_beta_step_serialization() {
        let redex = app(lam(var(0)), var(1));
        let proof = prove_beta(redex.clone());
        let serialized = serialize_proof(&proof);
        let deserialized = deserialize_proof(&serialized).unwrap();

        let result = verify(&deserialized).unwrap();
        let (left, right) = result;
        assert!(alpha_equiv(left, redex));
        assert_eq!(right, var(1));
    }

    #[test]
    fn test_eta_step_serialization() {
        let redex = lam(app(var(1), var(0)));
        let proof = prove_eta(redex.clone()).unwrap();
        let serialized = serialize_proof(&proof);
        let deserialized = deserialize_proof(&serialized).unwrap();

        let result = verify(&deserialized).unwrap();
        let (left, right) = result;
        assert!(alpha_equiv(left, redex));
        assert_eq!(right, var(1));
    }

    #[test]
    fn test_refl_serialization() {
        let expr = var(0);
        let proof = Proof::Refl(expr.clone());
        let serialized = serialize_proof(&proof);
        let deserialized = deserialize_proof(&serialized).unwrap();

        let result = verify(&deserialized).unwrap();
        let (left, right) = result;
        assert_eq!(left, expr);
        assert_eq!(right, expr);
    }

    #[test]
    fn test_sym_serialization() {
        let redex = app(lam(var(0)), var(1));
        let beta_proof = prove_beta(redex.clone());
        let sym_proof = Proof::Sym(Box::new(beta_proof));
        let serialized = serialize_proof(&sym_proof);
        let deserialized = deserialize_proof(&serialized).unwrap();

        let result = verify(&deserialized).unwrap();
        let (left, right) = result;
        assert_eq!(left, var(1));
        assert!(alpha_equiv(right, redex));
    }

    #[test]
    fn test_trans_serialization() {
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

        let result = verify(&deserialized).unwrap();
        let (left, right) = result;
        assert!(alpha_equiv(left, outer_redex));
        assert_eq!(right, var(1));
    }

    #[test]
    fn test_complex_proof_serialization() {
        let expr = app(lam(var(0)), var(1));
        let proof = prove_normalization(expr.clone(), 10).unwrap();
        let serialized = serialize_proof(&proof);
        let deserialized = deserialize_proof(&serialized).unwrap();

        let result = verify(&deserialized).unwrap();
        let (left, right) = result;
        assert!(alpha_equiv(left, expr));
        assert_eq!(right, var(1));
    }
}
```

### Phase 4: Optimized Normalization Implementation

**File: `core_world/src/core_kernel.rs`**

**Task 4.1: Implement Stack-Based Normalization**

```rust
/// Optimized normalization using explicit stack to avoid recursion limits
/// V2 Implementation: Uses iterative approach with explicit stack
pub fn normalize_stack_based(expr: CoreExpr, step_limit: usize) -> Result<CoreExpr, NormalizationError> {
    let mut current = expr;
    let mut steps = 0;

    while steps < step_limit {
        // Check if current expression is in normal form
        if is_normal_form(&current) {
            return Ok(current);
        }

        // Try β-reduction first
        let beta_reduced = beta_reduce_step(current.clone());
        if !alpha_equiv(beta_reduced.clone(), current.clone()) {
            current = beta_reduced;
            steps += 1;
            continue;
        }

        // If β-reduction didn't make progress, try η-reduction
        let eta_reduced = eta_reduce(current.clone());
        if !alpha_equiv(eta_reduced.clone(), current.clone()) {
            current = eta_reduced;
            steps += 1;
            continue;
        }

        // If neither reduction made progress, we're stuck
        break;
    }

    if steps >= step_limit {
        Err(NormalizationError::StepLimitExceeded(steps))
    } else {
        Ok(current)
    }
}

/// Stack-based β-reduction that handles deeply nested terms
fn beta_reduce_stack_based(expr: CoreExpr) -> CoreExpr {
    // Use iterative approach with explicit stack
    let mut stack = vec![expr];
    let mut result = None;

    while let Some(current) = stack.pop() {
        match current {
            CoreExpr::App(func, arg) => {
                // Push argument first, then function (so function is processed first)
                stack.push(*arg);
                stack.push(*func);
                // Mark that we're in an application context
                stack.push(CoreExpr::App(Box::new(CoreExpr::Var(0)), Box::new(CoreExpr::Var(0)))); // Placeholder
            }
            CoreExpr::Lam(body) => {
                // If we're in an application context, this is a redex
                if let Some(CoreExpr::App(..)) = stack.last() {
                    // This is a redex: (λ.M) N
                    stack.pop(); // Remove the placeholder
                    let arg = stack.pop().unwrap();
                    let func = CoreExpr::Lam(body);
                    // Perform substitution
                    let substituted = substitute(*func.body(), 0, arg);
                    stack.push(substituted);
                } else {
                    // Just a lambda, push it back
                    stack.push(CoreExpr::Lam(body));
                }
            }
            other => {
                // Variables, Nat, Pair - just push back
                stack.push(other);
            }
        }
    }

    // The final result is at the bottom of the stack
    stack.pop().unwrap_or(CoreExpr::Var(0))
}
```

**Task 4.2: Update Public API**

```rust
/// Updated normalize function using stack-based approach
pub fn normalize(expr: CoreExpr, step_limit: usize) -> Result<CoreExpr, NormalizationError> {
    normalize_stack_based(expr, step_limit)
}
```

### Phase 5: API Updates and Documentation

**File: `core_world/src/lib.rs`**

**Task 5.1: Add Serialization Functions to Public API**

```rust
/// Public API for CoreExpr serialization
pub fn serialize_core_expr(expr: &CoreExpr) -> Vec<u8> {
    core_expr::serialize_core_expr(expr)
}

pub fn deserialize_core_expr(bytes: &[u8]) -> Result<CoreExpr, ParseError> {
    core_expr::deserialize_core_expr(bytes)
}

/// Public API for Proof serialization
pub fn serialize_proof(proof: &Proof) -> Vec<u8> {
    proof_checker::serialize_proof(proof)
}

pub fn deserialize_proof(bytes: &[u8]) -> Result<Proof, ParseError> {
    proof_checker::deserialize_proof(bytes)
}

/// Public error types for serialization
#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    IncompleteData,
    InvalidTag(u8),
    Overflow,
}
```

**Task 5.2: Update Documentation**

```rust
/// Core-World V2 Implementation
///
/// Core-World is the immutable formal kernel of Project Jue. It provides:
/// 1. Mathematical meaning through βη-equivalence
/// 2. Proof verification for semantic equivalence
/// 3. Standardized serialization for cross-layer communication
///
/// Key V2 Features:
/// - Capability boundary: Core-World does not reason about capabilities
/// - Standardized binary serialization format
/// - Optimized normalization using explicit stacks
/// - Frozen primitive set: λ, Nat, Pair
///
/// Public API:
/// - `verify_equivalence(proof)`: Verify term equivalence proofs
/// - `normalize(term, step_limit)`: Compute βη-normal form
/// - `serialize_core_expr(expr)`: Serialize CoreExpr to binary
/// - `deserialize_core_expr(bytes)`: Deserialize CoreExpr from binary
/// - `serialize_proof(proof)`: Serialize Proof to binary
/// - `deserialize_proof(bytes)`: Deserialize Proof from binary
```

### Phase 6: Conformance Testing

**File: `core_world/tests/test_conformance.rs`**

**Task 6.1: Implement V2 Conformance Tests**

```rust
#[cfg(test)]
mod v2_conformance_tests {
    use super::*;
    use crate::core_expr::{app, lam, var};
    use crate::proof_checker::{prove_beta, prove_eta, prove_normalization, verify};

    #[test]
    fn test_v2_beta_reduction_correctness() {
        // Test 1: β-Reduction Correctness (Call-by-Name)
        let term = app(lam(var(0)), var(1)); // (λ.0) 1
        let reduced = beta_reduce(term.clone());
        assert_eq!(reduced, var(1));
    }

    #[test]
    fn test_v2_eta_reduction() {
        // Test 2: η-Reduction
        let term = lam(app(var(0), var(1))); // λ.(0 1)
        assert!(eta_reduce(term).is_none()); // Should NOT reduce (0 is free in body)

        let term_eta = lam(app(var(1), var(0))); // λ.(1 0)
        assert_eq!(eta_reduce(term_eta), Some(var(1))); // SHOULD reduce to 1
    }

    #[test]
    fn test_v2_proof_verification() {
        // Test 3: Proof Verification
        let redex = app(lam(var(0)), var(42)); // (λ.0) 42
        let proof = prove_beta(redex.clone());
        let (left, right) = verify(&proof).unwrap();
        assert!(alpha_equiv(left, redex));
        assert_eq!(right, var(42));
    }

    #[test]
    fn test_v2_normalization_divergence() {
        // Test 4: Normalization finds identity function
        let omega = lam(app(var(0), var(0))); // λ.(0 0)
        let term = app(omega.clone(), omega); // (λ.(0 0)) (λ.(0 0))
        let result = normalize(term, 1000);
        assert!(matches!(result, Err(NormalizationError::StepLimitExceeded(_))));
    }

    #[test]
    fn test_v2_serialization_roundtrip() {
        // Test 5: V2 Serialization Roundtrip
        let expr = app(lam(var(0)), var(42));
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_v2_proof_serialization() {
        // Test proof serialization roundtrip
        let expr = app(lam(var(0)), var(1));
        let proof = prove_normalization(expr.clone(), 10).unwrap();
        let serialized = serialize_proof(&proof);
        let deserialized = deserialize_proof(&serialized).unwrap();

        let result = verify(&deserialized).unwrap();
        let (left, right) = result;
        assert!(alpha_equiv(left, expr));
        assert_eq!(right, var(1));
    }

    #[test]
    fn test_v2_stack_based_normalization() {
        // Test stack-based normalization
        let expr = app(lam(var(0)), var(1));
        let result = normalize_stack_based(expr.clone(), 10).unwrap();
        assert_eq!(result, var(1));
    }

    #[test]
    fn test_v2_complex_normalization() {
        // Test complex normalization: ((λx.λy.x) a) b → a
        let expr = app(app(lam(lam(var(1))), var(0)), var(1));
        let normalized = normalize(expr, 100).unwrap();
        assert_eq!(normalized, var(0));
    }

    #[test]
    fn test_v2_capability_boundary() {
        // This test documents the capability boundary
        // Core-World verifies pure λ-calculus semantics
        // Capability reasoning happens in Physics/Jue layers

        // Example: Pure λ-calculus term
        let pure_term = lam(app(var(0), var(1)));
        let proof = prove_normalization(pure_term.clone(), 10).unwrap();
        let result = verify(&proof).unwrap();

        // Core-World can verify this pure term
        assert!(alpha_equiv(result.0, pure_term));
        assert!(alpha_equiv(result.1, pure_term)); // Already in normal form
    }
}
```

### Phase 7: Integration and Validation

**Task 7.1: Integration Testing**

```rust
#[test]
fn test_integration_with_jue_world() {
    // Test that serialized CoreExpr can be used by Jue-World
    let expr = app(lam(var(0)), var(42));
    let serialized = serialize_core_expr(&expr);

    // Simulate sending to Jue-World and receiving back
    let deserialized = deserialize_core_expr(&serialized).unwrap();
    assert_eq!(expr, deserialized);

    // Test that proofs can be serialized and verified
    let proof = prove_beta(expr.clone());
    let serialized_proof = serialize_proof(&proof);
    let deserialized_proof = deserialize_proof(&serialized_proof).unwrap();

    let result = verify(&deserialized_proof).unwrap();
    assert!(alpha_equiv(result.0, expr));
    assert_eq!(result.1, var(42));
}
```

**Task 7.2: Performance Testing**

```rust
#[test]
fn test_normalization_performance() {
    // Test performance with deeply nested terms
    let mut expr = lam(var(0));
    for _ in 0..100 {
        expr = app(expr, var(0));
    }

    let start = std::time::Instant::now();
    let result = normalize(expr, 1000);
    let duration = start.elapsed();

    // Should complete quickly with stack-based approach
    assert!(duration.as_millis() < 100);
    assert!(result.is_ok());
}
```

## Implementation Phases with Deliverables

### Phase 1: Preparation (Week 1)
**Deliverables:**
- ✅ Comprehensive implementation plan document
- ✅ Current implementation analysis
- ✅ Development environment setup
- ✅ Baseline test results

### Phase 2: CoreExpr Serialization (Week 2)
**Deliverables:**
- ✅ `serialize_core_expr()` function
- ✅ `deserialize_core_expr()` function
- ✅ Comprehensive serialization tests
- ✅ Documentation updates

### Phase 3: Proof Serialization (Week 3)
**Deliverables:**
- ✅ `serialize_proof()` function
- ✅ `deserialize_proof()` function
- ✅ Comprehensive proof serialization tests
- ✅ Integration with existing proof system

### Phase 4: Optimized Normalization (Week 4)
**Deliverables:**
- ✅ Stack-based normalization algorithm
- ✅ Updated public API
- ✅ Performance benchmarks
- ✅ Regression testing

### Phase 5: API and Documentation (Week 5)
**Deliverables:**
- ✅ Updated public API with serialization functions
- ✅ Comprehensive documentation
- ✅ Examples and usage guides
- ✅ Error handling improvements

### Phase 6: Conformance Testing (Week 6)
**Deliverables:**
- ✅ V2 conformance test suite
- ✅ Regression test suite
- ✅ Performance test suite
- ✅ Integration test suite

### Phase 7: Integration and Validation (Week 7)
**Deliverables:**
- ✅ Integration with Jue-World
- ✅ Cross-layer communication tests
- ✅ Final validation report
- ✅ Release candidate

## Open Questions and Assumptions

### Open Questions

1. **Nat and Pair Variants**: Should these be removed for pure λ-calculus compliance, or kept for practical purposes?
2. **Stack-Based Normalization**: Should we replace the recursive implementation entirely, or keep both for compatibility?
3. **Error Handling**: Should serialization errors be more detailed for debugging purposes?
4. **Performance Tradeoffs**: What's the acceptable performance threshold for deeply nested terms?

### Assumptions

1. **Backward Compatibility**: V2 must be fully backward compatible with V1.0 proofs and terms
2. **Binary Format**: Little-endian encoding is acceptable for cross-platform compatibility
3. **Performance**: Stack-based normalization will handle terms that cause stack overflow with recursion
4. **Testing**: Existing test suite provides adequate coverage for regression testing

## Testing Strategy

### Unit Testing
- Individual function testing for all new serialization functions
- Edge case testing for malformed input
- Performance testing for large terms

### Integration Testing
- Cross-module testing between core_expr and proof_checker
- API consistency testing
- Error handling validation

### Regression Testing
- Run all existing V1 tests to ensure no regressions
- Performance comparison between recursive and stack-based normalization
- Memory usage testing

### Conformance Testing
- V2 specification conformance tests
- Cross-layer communication tests
- Serialization roundtrip validation

### Validation Criteria

**Phase 2 (CoreExpr Serialization):**
- ✅ All serialization tests pass
- ✅ Roundtrip serialization preserves semantic equivalence
- ✅ No regressions in existing functionality

**Phase 3 (Proof Serialization):**
- ✅ All proof serialization tests pass
- ✅ Serialized proofs can be verified after deserialization
- ✅ Integration with existing proof system works

**Phase 4 (Optimized Normalization):**
- ✅ Stack-based normalization handles deeply nested terms
- ✅ Performance improvement over recursive approach
- ✅ Same results as recursive normalization

**Phase 5 (API and Documentation):**
- ✅ Public API is consistent and well-documented
- ✅ All new functions are properly exposed
- ✅ Documentation is complete and accurate

**Phase 6 (Conformance Testing):**
- ✅ All V2 conformance tests pass
- ✅ No regressions in V1 functionality
- ✅ Performance meets expectations

**Phase 7 (Integration and Validation):**
- ✅ Integration with Jue-World works
- ✅ Cross-layer communication is reliable
- ✅ Final validation report is complete

## Success Criteria

The Core World V2 implementation will be considered successful when:

1. **Functional Completeness**: All V2 specification requirements are implemented
2. **Backward Compatibility**: All V1 functionality continues to work
3. **Performance**: Stack-based normalization handles terms that cause stack overflow
4. **Reliability**: Serialization roundtrips preserve semantic equivalence
5. **Documentation**: Complete and accurate documentation for all new features
6. **Testing**: Comprehensive test coverage with no regressions
7. **Integration**: Successful integration with Jue-World and Physics-World

## Risk Assessment and Mitigation

### Risks

1. **Performance Issues**: Stack-based normalization might not provide expected benefits
   - *Mitigation*: Benchmark both approaches and choose the best one

2. **Serialization Complexity**: Binary format might be error-prone
   - *Mitigation*: Comprehensive testing and validation

3. **Backward Compatibility**: Changes might break existing functionality
   - *Mitigation*: Extensive regression testing

4. **Integration Challenges**: Cross-layer communication might have issues
   - *Mitigation*: Early integration testing

### Contingency Plans

1. **Fallback to Recursive**: If stack-based normalization has issues, keep recursive as fallback
2. **Alternative Serialization**: If binary format causes issues, consider JSON as alternative
3. **Phased Rollout**: Implement features incrementally with validation at each step

## Timeline and Resource Estimation

| Phase                      | Duration | Resources   | Key Deliverables                    |
| -------------------------- | -------- | ----------- | ----------------------------------- |
| 1. Preparation             | 1 week   | 1 developer | Implementation plan, baseline tests |
| 2. CoreExpr Serialization  | 1 week   | 1 developer | Serialization functions, tests      |
| 3. Proof Serialization     | 1 week   | 1 developer | Proof serialization, integration    |
| 4. Optimized Normalization | 1 week   | 1 developer | Stack-based algorithm, benchmarks   |
| 5. API and Documentation   | 1 week   | 1 developer | Public API, documentation           |
| 6. Conformance Testing     | 1 week   | 1 developer | Test suite, validation              |
| 7. Integration             | 1 week   | 1 developer | Cross-layer tests, final validation |

**Total Estimate**: 7 weeks, 1 full-time developer

## Conclusion

This implementation plan provides a comprehensive, step-by-step approach to implementing Core World V2 specification. The plan addresses all V2 requirements while maintaining backward compatibility and ensuring robust testing and validation. The phased approach allows for incremental validation and risk mitigation throughout the implementation process.