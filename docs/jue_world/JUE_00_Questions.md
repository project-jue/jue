## **Jue-World: The Bridge's Critical Ambiguities**




---
---

---


---


---

## **Synthesis: Jue-World Design Principles**

Based on these analyses, I recommend:

### **1. Evaluation Strategy:** 
- **Default:** Call-by-value with proof of equivalence to normal order
- **Allow:** Multiple strategies with different trust levels
- **Goal:** Let Dan discover optimal evaluation for different code patterns

### **2. Type System:**
- **Core:** Gradual types with runtime contracts
- **Extension:** Let Dan propose new type predicates
- **Verification:** Types can be formal (proved) or empirical (tested)

### **3. State Management:**
- **Default:** Immutable values, pure functions
- **Escape:** Mutable regions with linear typing
- **Model:** State as explicit snapshots for rollback

### **4. Concurrency:**
- **Model:** Message passing between isolated processes
- **Scheduler:** Deterministic round-robin
- **Communication:** Immutable messages only

### **5. Macros:**
- **Default:** Hygienic macros
- **Escape:** Explicit capture when needed
- **Safety:** Proof of preservation or experimental marking

### **6. Resources:**
- **Accounting:** Hybrid static/runtime/dynamic
- **Budgets:** Explicit resource limits
- **Learning:** Dan builds resource prediction models

### **7. Errors:**
- **Model:** Result types (Ok/Error)
- **Handling:** Pattern matching mandatory
- **Learning:** Errors become training data

---

## **The Jue Compiler Architecture**

```rust
// High-level structure
struct JueCompiler {
    // Phases
    parser: Parser,
    macro_expander: MacroExpander,
    type_checker: GradualTypeChecker,
    resource_analyzer: ResourceAnalyzer,
    core_translator: CoreTranslator,
    proof_generator: ProofGenerator,
    code_generator: CodeGenerator,
    
    // Policies
    trust_policy: TrustPolicy,  // Formal/Empirical/Experimental
    resource_policy: ResourcePolicy,
    optimization_policy: OptimizationPolicy,
}
```

**Key Innovation:** The compiler doesn't just produce code—it produces **justified code**:
- For each optimization: proof of equivalence
- For each evaluation strategy: proof or empirical evidence
- For each resource claim: proof or learned model

---

## **The Dan-Jue Feedback Loop**

Most importantly: **Jue should be designed for Dan's use:**

```jue
// Dan doesn't write Jue directly
// Dan proposes code changes through:
module codeProposer {
    // 1. Genetic programming
    proposeMutation(existingCode) -> newCode
    
    // 2. Learned patterns
    proposeFromPattern(observedPattern) -> newCode
    
    // 3. External learning (from humans, other Dans)
    incorporateExternal(codeExample) -> adaptedCode
}

// All proposals go through:
// 1. Compilation (with proofs/checks)
// 2. Testing (in sandbox)
// 3. Evaluation (by other modules)
// 4. Possibly adoption
```

---

**This design makes Jue-World a *living bridge*:** 
- Formally grounded in Core-World
- Practically efficient on Physics
- Adaptable to Dan's needs
- Safe through proofs and sandboxing

**Are these resolutions specific enough to guide implementation?** Would you like me to elaborate on any particular aspect before we move to Dan-World's ambiguities?



Yes, the Jue World spec leaves several **critical unanswered questions**. This is expected for a v1.0 spec, but they must be resolved before implementation. Here are the key ambiguities, sorted by priority.

### **1. Macro Expansion & Trust Tiers (High Priority)**
**The Ambiguity:** How do macros interact with the trust tier system? A `:formal` macro could expand to `:experimental` code, breaking the guarantee.
*   **Option A (Strict):** A macro inherits the tier of its *call site*. A `:formal` macro's expansion is forced to be `:formal`, requiring proofs for all generated code.
*   **Option B (Declared):** Macros declare their own tier (`:macro-empirical`). The compiler checks that the call site's tier is ≥ the macro's tier (e.g., you can't call an `:experimental` macro from `:formal` code).
*   **Option C (Unhygienic):** Macros are pure syntactic expansion before tier analysis. This is simplest but dangerous—one `:empirical` macro could poison a `:formal` codebase.

**Recommendation:** **Option B (Declared Tiers).** It's the only safe choice. The spec must add: `(defmacro <tier> <name> <args> <body>)`.

### **2. Type System Semantics (High Priority)**
**The Ambiguity:** Is the type system for documentation, proof generation, or runtime safety?
*   **Option A (Proof-Obligation):** Types generate proof obligations for the Core World path (e.g., `(+ :Int :Int -> :Int)` must be proven).
*   **Option B (Runtime Contract):** Types compile to runtime checks in the Physics World bytecode for `:empirical`/`:experimental` tiers, causing `StructuredError` on violation.
*   **Option C (Hint Only):** Types are ignored by the compiler and are only for Dan's self-documentation.

**Recommendation:** **Hybrid A/B.** For `:formal`/:verified`, types are proof obligations. For `:empirical`/:experimental`, they become runtime contracts. This aligns with the tier philosophy.

### **3. Error Handling Across Worlds (High Priority)**
**The Ambiguity:** What happens when a Jue-level error (e.g., `car` of a non-pair) occurs?
*   **Option A (Physics Error):** All errors are lowered to Physics World `OpCode` that can trap (e.g., `Car` checks heap tag). This pushes complexity to the VM.
*   **Option B (Jue Wrappers):** The Jue compiler inserts explicit checks and generates a Jue-level exception value (e.g., `(Error "car of non-pair")`).
*   **Option C (Undefined):** Erroneous operations have undefined behavior, trusting higher-level proofs.

**Recommendation:** **Option A for `:empirical`, Option C for `:formal`.** `:empirical` code needs safety; `:formal` code must be proven correct, making runtime checks redundant.

### **4. Module System & Code Organization (Medium Priority)**
**The Ambiguity:** How does Dan-World organize code? Is there a `module` or `namespace` construct?
*   **Option A (Flat):** No module system. All code is in one global scope. Simpler, but scales poorly.
*   **Option B (Physical Files):** Each file is a module; `(require "path")` includes it.
*   **Option C (Named Modules):** Explicit `(module <name> <body>)` with import/export.

**Recommendation:** **Start with Option A (Flat).** Add a module system in v2 once the core pipeline works. Document this as a known limitation.

### **5. Empirical Validation Threshold (Medium Priority)**
**The Ambiguity:** What constitutes "passing" the empirical test suite?
*   **Option A (Fixed Budget):** Run for N random inputs (e.g., 10,000) or Y seconds. If no `StructuredError`, it passes.
*   **Option B (Coverage Target):** Require branch/path coverage of the generated bytecode ≥ X%.
*   **Option C (Adaptive):** The required test rigor scales with the "risk" of the code (e.g., code that touches actor mailboxes requires more testing).

**Recommendation:** **Option A (Fixed Budget) for v1.0.** Specify the defaults (e.g., `:empirical` = 10,000 inputs, `:experimental` = 100 inputs). Keep it simple and measurable.

### **6. Memory Management Across Tiers (Medium Priority)**
**The Ambiguity:** Does the `:formal` compiler need to prove memory bounds, or just correctness?
*   **Option A (Correctness Only):** Core World proofs ignore memory. Physics World's arena enforces the limit empirically.
*   **Option B (Full Verification):** The `:formal` tier requires a proof that the computation stays within the `memory_limit` (e.g., via a sized-type system).
*   **Option C (Annotation):** Programmers add `:memory` annotations; the compiler checks them informally.

**Recommendation:** **Option A for v1.0.** AIKR memory limits are a Physics World enforcement. Adding formal memory proofs is a major research project; defer it.

### **7. Foreign Function Interface (FFI) (Low Priority, Critical Future)**
**The Ambiguity:** How does Jue code call Rust functions (for I/O, sensors, etc.)?
*   **Option A (Wrapper OpCode):** Define special `OpCode`s (e.g., `ReadSensor`) that the Physics World implements as host calls.
*   **Option B (Dynamic Binding):** Allow `(extern "rust" <fn-name>)` declarations that are linked at runtime.
*   **Option C (No FFI):** The Physics World provides all primitives; no direct host calls.

**Recommendation:** **Option A (Wrapper OpCode) for v1.0.** It's the safest and most deterministic. It forces a clear boundary: all external interaction is mediated by the Physics World spec.

### **8. Concurrency Primitives in Jue (Low Priority)**
**The Ambiguity:** Should Jue have `spawn` or `future` constructs, or is concurrency purely a Dan-World/Physics World scheduler concern?
*   **Option A (Explicit):** Jue has `(spawn <expr>)` which compiles to actor-creation instructions.
*   **Option B (Implicit):** Concurrency is managed by Dan-World modules; Jue sees a single-threaded world.

**Recommendation:** **Option B for v1.0.** Keep Jue simple. Dan's modules are the concurrent units; they communicate via messages, not shared Jue state.

### **9. Tooling & Debugging Output (Low Priority, High Practical Value)**
**The Ambiguity:** What debuggable output does the Jue compiler produce?
*   **Option A (Source Maps):** Generate mappings from Physics World bytecode positions back to Jue source.
*   **Option B (Step Log):** Annotate bytecode so the VM can log high-level operations (`"calling function f"`).
*   **Option C (Core Proof Trace):** For `:formal` code, output the proof tree as a debug artifact.

**Recommendation:** **Specify a `DebugInfo` struct in the `CompilationResult`.** It should contain a source map (Option A) and the core proof (Option C) if present. This is essential for Dan's introspection.

### **Summary: Spec Addenda Needed**
To resolve these, add these sections to the spec:

1.  **Section 8: Macro System** – Define `defmacro` with required tier declaration and hygiene rules.
2.  **Section 9: Type Annotations** – Define `:Int`, `:Bool`, `:Pair`, and their semantics per trust tier (proof vs. runtime check).
3.  **Section 10: Error Semantics** – Specify that `:formal` code has no runtime errors (by proof), while `:empirical` code traps via Physics World.
4.  **Section 11: Empirical Validation Protocol** – Define the default test count (10,000), input generation strategy, and pass/fail criteria.
5.  **Appendix A: Known Limitations (v1.0)** – Explicitly state: no module system, no FFI, no formal memory proofs, no Jue-level concurrency primitives.

These choices will make the spec **unambiguous for implementation**. The goal is to decide, not to leave perfect for later. For Project Jue, which path for macros and types seems most aligned with your vision?