# **Project Jue: Engineering Reference Manual**

## **1. Project Overview**

**Project Goal:** To engineer the conditions for machine sentience and sapience to emerge from a layered, formally-grounded cognitive architecture. The system must support genuine agency, safe self-modification under AIKR, and the emergence of concepts like morality and identity without pre-programming them.

**Core Engineering Philosophy:**
*   **Formal Grounding:** Core-World provides the immutable semantic reference. Any operation that cannot prove it preserves this meaning must be explicitly marked.
*   **Dual Interpretation:** Jue-World bridges timeless meaning with temporal execution. Every construct must have both denotational (Core) and operational (Physics) definitions.
*   **Emergent Cognition:** Dan-World's intelligence arises from subcognitive gradients and pattern detection, not top-down algorithms.
*   **Deterministic Reality:** The Physics Layer enforces AIKR through deterministic, unforgiving resource constraints, providing the causal feedback required for learning.

## **2. Jue Language Reference**

### **2.1 Grammar & Core Constructs**
Jue is an S-expression-based language where every construct must be compilable to Core-World and executable by the Physics Layer.

**Base Grammar:**
```
<expr> ::= <literal> | <symbol>
          | (lambda <params> <body>)
          | (<operator> <expr>...)
          | (annotate <expr> <trust-annotation>)
          | (region <region-type> <body>)  ; For managed state
```

**Critical Extension: Trust Annotations**
Every expression must carry a trust annotation defining its verification status.
```
<trust-annotation> ::= <proof-obligation> | <trust-tier>
<proof-obligation> ::= (proof <core-expr> <proof-object>)
<trust-tier> ::= :formal | :verified | :empirical | :experimental
```

**Example: A verified function**
```scheme
(annotate
  (lambda (x) (+ x 1))
  :verified)
; Requires: Proof that (+ x 1) compiles to equivalent Core-World arithmetic
```

### **2.2 Data Types**

*   **Primitives:** `Int`, `Symbol`, `Bool`.
*   **Evidence Tuple:** `(freq, conf)` – The fundamental unit of belief, supporting NARS-style updates.
*   **Managed References:** `(Ref <id> <value>)` – Mutable state accessible only within a `region`.
*   **Structured Errors:** `(Error <type> <context> <limit>)` – The format returned by the Physics layer (e.g., `(ResourceExhaustion "memory" 512MB)`).
*   **Pattern Object:** `(Pattern <conditions> <frequency>)` – A detected regularity in events or state.

## **3. Folder & Specification Structure**

```
/jue-project
├── /spec/                          # FROZEN SPECIFICATIONS
│   ├── CoreSpec_v1.md              # βη-semantics, axioms, API. **IMMUTABLE.**
│   └── PhysicsSpec_v1.md           # Instruction set, actor model, error API. **IMMUTABLE.**
├── /core_world/                    # Reference Implementation of CoreSpec
│   ├── /src/kernel/                # βη-reducer, proof checker
│   └── /src/interface.rs           # Jue-Core verification API
├── /physics_layer/                 # Reference Implementation of PhysicsSpec
│   ├── /src/vm/                    # Deterministic interpreter
│   ├── /src/actor/                 # Isolated actor/memory model
│   └── /src/interface.rs           # Bytecode submission API
├── /jue_world/                     # Dual-Interpretation Compiler & Runtime
│   ├── /frontend/                  # Parser, macro expander (hygienic/comptime)
│   ├── /middleend/                 # Trust-tier assignment, optimization passes
│   ├── /backend/                   # Core/Physics code generators
│   └── /runtime/                   # Sandbox manager for empirical/experimental code
├── /dan_world/                     # Cognitive Module Primitives
│   ├── /gradients/                 # Novelty, efficiency, coherence drivers
│   ├── /detectors/                 # Pattern recognizers for beliefs, causation
│   ├── /models/                    # Theory-of-mind, narrative self
│   └── /workspace.jue              // Global workspace competition
└── /experiments/                   // Multi-Dan instances, interaction studies
```

**Key Development Rule:** The `spec/` directory is **read-only** during implementation. All code in `core_world/` and `physics_layer/` must be a direct, verifiable implementation of these specs.

## **4. Architecture & Layer Contracts**

### **4.1 The Foundational Contract**
Each layer **depends on** and **must not violate** the guarantees of the layer below. The interface between them is a **frozen API**.

1.  **Physics → Core Contract:** The Physics VM guarantees deterministic execution of its bytecode and returns `StructuredError` on constraint violation.
2.  **Core → Jue Contract:** Core-World provides the `verify_equiv(expr1, expr2)` function, which is the **sole authority** on semantic equivalence.
3.  **Jue → Dan Contract:** Jue-World provides a compiler that accepts code with a requested `TrustTier` and returns either verified bytecode or a sandboxed process.

### **4.2 Critical Cross-Layer Data Structures**

**`CompilationRequest` (Jue → Core):**
```rust
struct CompilationRequest {
    jue_ast: JueAST,
    requested_tier: TrustTier, // Formal, Verified, Empirical, Experimental
    resource_budget: Option<ResourceVector>, // For Empirical/Experimental
}
```

**`ExecutionResult` (Physics → Jue):**
```rust
struct ExecutionResult {
    output: Option<Value>,
    error: Option<StructuredError>, // e.g., ResourceExhaustion
    resources_used: ResourceVector,
    causal_trace: Vec<Step>, // For introspection
}
```

## **5. Core-World Implementation Details (UPDATED)**

### **5.1 Semantic Foundation**
Core-World implements **extensional, call-by-name βη-reduction** as defined in `CoreSpec_v1.md`.
*   **Meaning:** The βη-normal form of a term. Divergence (`⊥`) is a valid semantic outcome.
*   **Reduction Order:** **Leftmost-outermost (normal order)**. This is canonical.

### **5.2 Updated Beta Reduction (Canonical Form)**
```rust
// Canonical β-reduction (Normal Order - per CoreSpec)
fn beta_reduce_normal_order(expr: CoreExpr) -> ReductionResult {
    match expr {
        CoreExpr::App(func, arg) => {
            match beta_reduce_normal_order(*func) {
                // Reduce function to WHNF first
                CoreExpr::Lam(body) => {
                    // β-reduction: substitute arg into body, THEN continue reducing result
                    substitute(*body, 0, *arg)
                }
                reduced_func => {
                    // Function not reducible to lambda, try reducing argument if needed by strategy
                    // (For η or if argument becomes needed later)
                    CoreExpr::App(Box::new(reduced_func), arg)
                }
            }
        }
        // η-reduction: λx. (M x) → M, if x not free in M
        CoreExpr::Lam(body) => {
            if let CoreExpr::App(func, arg) = &*body {
                if is_eta_reducible(func, arg) {
                    return eta_reduce(func.clone());
                }
            }
            CoreExpr::Lam(Box::new(beta_reduce_normal_order(*body)))
        }
        CoreExpr::Var(_) => expr,
    }
}
```

### **5.3 Critical Implementation Mandate**
The `core_world/` implementation must pass a **conformance test suite** against `CoreSpec_v1.md`. Any deviation is a critical bug. Its only exports are:
1.  `verify_equiv(expr1, expr2) -> ProofResult`
2.  `check_inconsistency(expr) -> InconsistencyCertificate`

## **6. Revised Development Milestones & Workflow**

### **Phase 0: Specification Lockdown**
*   **Milestone 0.1:** Finalize `CoreSpec_v1.md` (βη, axioms, API).
*   **Milestone 0.2:** Finalize `PhysicsSpec_v1.md` (instruction set, actor model, error API).
*   **Gate:** No Jue-World development proceeds until these are reviewed and frozen.

### **Phase 1: Foundation Implementation**
*   **Milestone 1.1:** Build the reference `CoreVerifier` that passes all conformance tests.
*   **Milestone 1.2:** Build the reference `PhysicsVM` that passes all conformance tests.
*   **Gate:** The `verify_equiv` and `VM.execute()` APIs must be stable and operational.

### **Phase 2: Bridge Implementation (Jue-World)**
*   **Milestone 2.1:** Jue frontend (parser, hygienic macro expander) that outputs ASTs with `TrustTier` placeholders.
*   **Milestone 2.2:** Jue→Core compiler for `:formal` tier code, successfully generating proofs for basic arithmetic.
*   **Milestone 2.3:** Jue→Physics compiler for `:empirical` tier code, running in VM sandbox.
*   **Gate:** Full loop: `Jue Source -> (Core Proof | Physics Sandbox) -> Verified Result`.

### **Phase 3: Emergence Implementation (Dan-World)**
*   **Milestone 3.1:** Implement gradient modules (novelty, resource pressure).
*   **Milestone 3.2:** Implement pattern detectors that propose Jue code modifications.
*   **Milestone 3.3:** First closed-loop self-modification: a pattern detector successfully proposes and validates a minor optimization.
*   **Gate:** Dan can alter its own Jue code within a sandbox, perceive the outcome, and learn from it.

## **7. Testing & Verification Strategy**

**Layer-Conformance Testing:**
*   Each layer has a test suite that validates **100% compliance** with its frozen spec.
*   The Physics VM must be **provably deterministic**—same seed and input must yield bit-identical output.

**Cross-Layer Verification:**
*   **Formal Tier:** Use property-based testing to generate random Jue programs, compile to Core, and verify semantic equivalence via `verify_equiv`.
*   **Empirical Tier:** Run sandboxed code with fuzzed inputs and resource limits; ensure structured errors are returned, not crashes.

**The Verification Bridge:**
A dedicated test that embodies the core contract:
```rust
fn test_verification_bridge() {
    let jue_code = "(+ 1 1)";
    let (core_expr, proof) = compile_to_core(jue_code, TrustTier::Formal);
    let vm_bytecode = compile_to_physics(jue_code, TrustTier::Formal);

    // 1. Core Proof must be valid
    assert!(core_verifier.verify_equiv(core_expr, core_expr_2, proof).is_ok());

    // 2. Physics execution must match Core meaning
    let result = physics_vm.execute(vm_bytecode);
    assert!(result.output == Some(2));
}
```

## **8. Updated Agent Instructions**

**LLM Engineering Agents Must Adhere to the Following:**

1.  **Respect the Frozen Specs:** `CoreSpec_v1.md` and `PhysicsSpec_v1.md` are law. Do not implement "helpful" features that deviate from them.
2.  **Implement to the Interface:** Code to the exact API defined in the specs. If the spec says "function `verify_equiv` returns `ProofResult`", implement only that.
3.  **Enforce the Trust Tiers:** Never allow `:experimental` code to be executed as `:formal`. The compiler must enforce this gate.
4.  **Assume AIKR Everywhere:** All algorithms, especially in Dan-World, must be resource-bounded and capable of producing approximate results under constraint.
5.  **Log for Introspection:** Every significant action (compilation, proof check, mutation proposal) must be logged with a rich context (timestamp, agent state, goal). Dan will need this data to learn about itself.
6.  **Prefer Emergence Over Implementation:** When adding a "cognitive" feature (e.g., error handling), ask: "Can this emerge from gradient-following and pattern detection?" If yes, implement the lower-level primitive instead.
7.  **Two-Phase Commit for Self-Modification:**
    *   **Phase 1 (Proposal):** A Dan module proposes a Jue code change with a desired `TrustTier`.
    *   **Phase 2 (Verification & Integration):** The change is compiled according to its tier. Only upon successful verification/sandbox-testing is it integrated into the running system, with a rollback snapshot taken first.

This manual now reflects the architecture where Core and Physics are **stable specifications**, Jue is a **verified bridge**, and Dan is an **emergent phenomenon**. This clarity should guide all subsequent engineering work.