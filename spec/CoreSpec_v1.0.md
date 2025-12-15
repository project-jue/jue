# **Project Jue: Core World Specification v1.0**

## **1. Overview & Architectural Role**

Core World is the **immutable formal kernel** of Project Jue. It defines the mathematical meaning of all computations via pure, untyped λ-calculus.

**Core World is NOT an execution engine.** It is a **verification library**. Its sole purpose is to answer formal questions about λ-term equivalence, providing the bedrock upon which Jue-World's proof-carrying compilation depends.

**Primary Functions:**
1.  **Define Meaning:** Provide the canonical translation target for Jue constructs.
2.  **Verify Equivalence:** Formally prove that two λ-terms have the same meaning.
3.  **Generate Proofs:** Construct evidence for semantic-preserving transformations.

**Guarantees:**
*   **Purity:** All functions are deterministic and side-effect free.
*   **Extensionality:** Terms are considered equivalent under βη-reduction.
*   **Trusted Base:** The implementation is small, auditable, and frozen after verification.

## **2. Data Model: The λ-Term**

Core World operates on a single data type: the λ-term with De Bruijn indices.

### **2.1. Term Definition (`core_expr::CoreExpr`)**
```rust
// MUST BE EXACTLY THIS DEFINITION.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CoreExpr {
    /// De Bruijn index. `Var(0)` is the innermost bound variable.
    Var(usize),
    /// Lambda abstraction. `Lam(Box<CoreExpr>)` binds a new variable in its body.
    Lam(Box<CoreExpr>),
    /// Function application.
    App(Box<CoreExpr>, Box<CoreExpr>),
}
```
*   **De Bruijn Convention:** `Var(0)` refers to the variable bound by the *immediately enclosing* `Lam`. `Var(n)` refers to the variable bound by the `n`-th `Lam` up the syntax tree.
*   **Immutability:** All terms are immutable values. Operations return new terms.

### **2.2. Canonical Semantics: βη-Equivalence**
The *meaning* of a term is its **βη-normal form**. Two terms are **semantically equivalent** if they share the same normal form (or both diverge).
*   **β-reduction (Beta):** `(λ.M) N →β [N/0]M`
*   **η-reduction (Eta):** `λ.(M 0) →η M` , provided `0` is not free in `M`.

## **3. Core Operations & Reduction Semantics**

The following operations define the term algebra. They must be implemented as pure functions.

### **3.1. Lifting (Shifting)**
**Signature:** `lift(term: CoreExpr, cutoff: usize, amount: isize) -> CoreExpr`
*   **Purpose:** Adjusts free De Bruijn indices. `lift(M, c, d)` increments all free variables in `M` with index ≥ `c` by `d`. (`d` can be negative for lowering).
*   **Rules:**
    *   `lift(Var(i), c, d) = Var(i+d)` if `i ≥ c`, else `Var(i)`
    *   `lift(Lam(body), c, d) = Lam(lift(body, c+1, d))`
    *   `lift(App(f, a), c, d) = App(lift(f, c, d), lift(a, c, d))`

### **3.2. Substitution**
**Signature:** `subst(term: CoreExpr, target_idx: usize, replacement: CoreExpr) -> CoreExpr`
*   **Purpose:** Performs `[N/k]M` – substitutes `replacement` (N) for variable `target_idx` (k) in `term` (M).
*   **Formal Rules:**
    1.  `[N/k]k = N`
    2.  `[N/k]n = n-1` if `n > k`
    3.  `[N/k]n = n` if `n < k`
    4.  `[N/k](λ.M) = λ([↑(N)/k+1]M)` where `↑(N)` lifts free variables in N by 1.
    5.  `[N/k](M₁ M₂) = ([N/k]M₁) ([N/k]M₂)`

### **3.3. β-Reduction (Single-Step, Call-by-Name)**
**Signature:** `beta_reduce(term: CoreExpr) -> CoreExpr`
*   **Strategy:** **Outermost, call-by-name.** Reduces the *leftmost, outermost* β-redex. This is **normal order** reduction.
*   **Behavior:** Finds the first occurrence of `App(Lam(body), arg)` and returns `subst(*body, 0, *arg)`. If no redex exists, returns the input unchanged.

### **3.4. η-Reduction (Single-Step)**
**Signature:** `eta_reduce(term: CoreExpr) -> Option<CoreExpr>`
*   **Rule:** If `term` is of the form `Lam(box App(func, Var(0)))` and `Var(0)` is not free in `func`, returns `Some(func)`. Otherwise, returns `None`.

### **3.5. Normalization**
**Signature:** `normalize(term: CoreExpr, step_limit: usize) -> Result<CoreExpr, NormalizationError>`
*   **Purpose:** Iteratively applies `beta_reduce` and `eta_reduce` until neither applies (the βη-normal form) or a step limit is hit.
*   **Error:** `NormalizationError::StepLimitExceeded` if the limit is reached before finding a normal form.

### **3.6. α-Equivalence**
**Signature:** `alpha_equiv(a: CoreExpr, b: CoreExpr) -> bool`
*   **Purpose:** Determines if two terms are identical up to renaming of bound variables (De Bruijn index adjustment).
*   **Implementation:** A straightforward structural comparison is sufficient with De Bruijn indices. `Lam(x)` is α-equivalent to `Lam(y)` if `x` is α-equivalent to `y`.

## **4. Proof System**

Proofs are certificates of semantic equivalence. They are data that can be serialized, stored, and independently verified.

### **4.1. Proof Object (`proof::Proof`)**
```rust
#[derive(Clone, Serialize, Deserialize)]
pub enum Proof {
    /// A single β-reduction step. `redex` must be of the form `App(Lam(body), arg)`.
    BetaStep { redex: CoreExpr, contractum: CoreExpr },
    /// A single η-reduction step.
    EtaStep { redex: CoreExpr, contractum: CoreExpr },
    /// A reflexive proof: a term is equivalent to itself.
    Refl(CoreExpr),
    /// Symmetry of equivalence. `proof` proves `B ≡ A`.
    Sym(Box<Proof>),
    /// Transitivity of equivalence. `proof_a` proves `A ≡ B`, `proof_b` proves `B ≡ C`.
    Trans { proof_a: Box<Proof>, proof_b: Box<Proof> },
    /// Congruence for application. `proof_f` proves `F ≡ G`, `proof_a` proves `A ≡ B`.
    CongApp { proof_f: Box<Proof>, proof_a: Box<Proof> },
    /// Congruence for abstraction. `proof_b` proves `M ≡ N`.
    CongLam { proof_b: Box<Proof> },
}
```

### **4.2. Proof Verification**
**Signature:** `proof::verify(proof: &Proof) -> Result<(CoreExpr, CoreExpr), ProofError>`
*   **Purpose:** **Validates the internal consistency of a proof.** If valid, returns the pair of equivalent terms `(left, right)` that the proof concludes.
*   **Key Change:** The verifier does **not** take a target term as input. It inspects the proof object alone to determine what equivalence it proves.
*   **Process:** Recursively checks each rule. E.g., for `Trans { proof_a, proof_b }`, it verifies `proof_a` yields `(A, B)`, `proof_b` yields `(B', C)`, and then checks that `B` and `B'` are α-equivalent.

### **4.3. Proof Construction (Utilities)**
Helper functions to build valid proofs:
*   `prove_beta(redex: CoreExpr) -> Proof`
*   `prove_eta(redex: CoreExpr) -> Proof`
*   `prove_normalization(term: CoreExpr, steps: usize) -> Result<Proof, NormalizationError>`

## **5. Public API (`lib.rs`)**

Core World exposes a minimal, frozen API. All other functions are internal.

```rust
// core_world/src/lib.rs

/// The primary export: verifies that a proof correctly establishes term equivalence.
pub fn verify_equivalence(
    proof: Proof
) -> Result<(CoreExpr, CoreExpr), VerifyError> {
    proof::verify(&proof)
}

/// Utility: Returns the βη-normal form of a term, if reachable within limits.
/// Used for debugging and specification, not for runtime.
pub fn normalize(
    term: CoreExpr,
    step_limit: usize
) -> Result<CoreExpr, NormalizationError> {
    kernel::normalize(term, step_limit)
}

/// Public error types.
pub enum VerifyError {
    InvalidProofStructure,
    ProofRuleViolation(String),
}

pub enum NormalizationError {
    StepLimitExceeded(usize),
}
```

## **6. Mandatory Breaking Changes from Current Code**

The current `core_world` code must be refactored to comply with this spec.

1.  **Remove `eval_relation.rs` entirely.** Core World does not evaluate; it reduces. Execution semantics are defined by the Physics World.
2.  **Fix `core_kernel::beta_reduce`.** Implement **call-by-name (normal order)** as specified. Remove any call-by-value logic or recursion limit warnings.
3.  **Implement η-reduction.** Add the `eta_reduce` function and integrate it into `normalize`.
4.  **Rewrite `proof_checker.rs`.** Adopt the new `Proof` enum. The `verify` function must be changed to the signature and logic described in Section 4.2. The `ProvenExpr` struct and the old `verify_proof(proof, expr)` function are deleted.
5.  **Define the public API.** Create `lib.rs` to export **only** `verify_equivalence` and `normalize`.

## **7. Conformance Tests**

A conforming implementation must pass the following tests:

```rust
// Test 1: β-Reduction Correctness (Call-by-Name)
let term = app(lam(var(0)), var(1)); // (λ.0) 1
let reduced = kernel::beta_reduce(term.clone());
assert_eq!(reduced, var(1));

// Test 2: η-Reduction
let term = lam(app(var(0), var(1))); // λ.(0 1)
assert!(kernel::eta_reduce(term).is_none()); // Should NOT reduce (0 is free in body)

let term_eta = lam(app(var(1), var(0))); // λ.(1 0)
assert_eq!(kernel::eta_reduce(term_eta), Some(var(1))); // SHOULD reduce to 1

// Test 3: Proof Verification
let redex = app(lam(var(0)), var(42)); // (λ.0) 42
let proof = proof::prove_beta(redex.clone());
let (left, right) = api::verify_equivalence(proof).unwrap();
assert!(kernel::alpha_equiv(left, redex));
assert_eq!(right, var(42));

// Test 4: Normalization finds identity function
let omega = lam(app(var(0), var(0))); // λ.(0 0)
let term = app(omega.clone(), omega); // (λ.(0 0)) (λ.(0 0))
let result = api::normalize(term, 1000);
assert!(matches!(result, Err(NormalizationError::StepLimitExceeded(_))));
```

This specification defines Core World v1.0. Implementation is now a direct translation of these rules into code.