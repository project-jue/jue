A **per-file, per-function checklist** for Project Jue. This will provide **LLM agents a precise roadmap**—every file, function, and proof obligation is enumerated, with clear instructions on validation before moving to the next step.

---

# **Project Jue: Detailed File & Function Checklist**

---

## **Folder Structure**

```
/jue-core
    core_expr.rs
    core_kernel.rs
    eval_relation.rs
    proof_checker.rs
/tests
    core_tests.rs
    jue_tests.rs
/jue-world
    parser.jue
    compiler.jue
    evaluator.jue
    macros.jue
    concurrency.jue
/dan-world
    module_kernel.jue
    event_loop.jue
    global_workspace.jue
    mutation_protocol.jue
    persistent_structures.jue
/physics
    primitives.rs
    atomic_ops.rs
    memory_manager.rs
```

---

## **Layer 1: Core-World (Rust)**

### **core_expr.rs**

| Function/Struct                   | Description           | Proof / Verification                      |
| --------------------------------- | --------------------- | ----------------------------------------- |
| `enum CoreExpr { Var, Lam, App }` | Base λ-calculus terms | Unit test construction & pattern matching |
| `impl Display for CoreExpr`       | Human-readable form   | Test string matches expected λ-notation   |

### **core_kernel.rs**

| Function                                           | Description              | Proof / Verification                                |
| -------------------------------------------------- | ------------------------ | --------------------------------------------------- |
| `fn beta_reduce(expr: CoreExpr) -> CoreExpr`       | Implements β-reduction   | Test `(λx.M) N → M[x→N]`                            |
| `fn alpha_equiv(a: CoreExpr, b: CoreExpr) -> bool` | α-equivalence check      | Test renaming correctness                           |
| `fn normalize(expr: CoreExpr) -> CoreExpr`         | Full normalization       | Property: `normalize(normalize(e)) == normalize(e)` |
| `fn prove_consistency()`                           | Check kernel consistency | Verify via external checker                         |

### **eval_relation.rs**

| Item                                                       | Description                    | Proof / Verification                                 |
| ---------------------------------------------------------- | ------------------------------ | ---------------------------------------------------- |
| `relation Eval(env: Env, expr: CoreExpr, value: CoreExpr)` | Relational semantics rules     | Ensure each rule corresponds to λ-calculus semantics |
| `rule var_lookup`                                          | Lookup variable in environment | Unit tests for all indices                           |
| `rule lam_intro`                                           | Closure formation              | Test closure binding correctness                     |
| `rule app_elim`                                            | Function application           | Test composition and argument evaluation             |

### **proof_checker.rs**

| Function                                                | Description                    | Proof / Verification             |
| ------------------------------------------------------- | ------------------------------ | -------------------------------- |
| `fn verify_proof(proof: Proof, expr: CoreExpr) -> bool` | Checks proof correctness       | Must pass for all CoreExpr cases |
| `fn attach_proof(expr: CoreExpr, proof: Proof)`         | Annotate expression with proof | Test round-trip annotation       |

---

## **Layer 2: Jue-World (Jue)**

### **parser.jue**

| Function                            | Description                   | Proof / Verification                      |
| ----------------------------------- | ----------------------------- | ----------------------------------------- |
| `parse_expr(input: string) -> Expr` | Converts input to AST         | Test parsing matches CoreExpr equivalents |
| `expand_syntax(expr)`               | Extended syntax → core syntax | Proof: `to_core(expr)` valid              |

### **compiler.jue**

| Function                         | Description                | Proof / Verification                   |
| -------------------------------- | -------------------------- | -------------------------------------- |
| `compile_to_core(expr)`          | Jue → CoreExpr             | Attach proof; verify with `validate()` |
| `compile_to_bytecode(core_expr)` | Core → bytecode            | Proof: bytecode evaluates equivalently |
| `optimize_expr(expr)`            | Constant folding, inlining | Proof: optimized == unoptimized        |

### **evaluator.jue**

| Function                | Description      | Proof / Verification                |
| ----------------------- | ---------------- | ----------------------------------- |
| `eval_jue(expr, env)`   | Evaluate Jue AST | Proof: result matches Core relation |
| `run_test_suite(exprs)` | Evaluate batch   | Proof obligations for each expr     |

### **macros.jue**

| Function             | Description     | Proof / Verification                 |
| -------------------- | --------------- | ------------------------------------ |
| `expand_macro(expr)` | Macro expansion | Attach `to_core` proof per expansion |

### **concurrency.jue**

| Function                       | Description         | Proof / Verification       |
| ------------------------------ | ------------------- | -------------------------- |
| `spawn_module_process(module)` | Event-driven module | Deadlock-free verification |
| `send_event(target, event)`    | Messaging           | Delivery guarantee test    |

---

## **Layer 3: Dan-World (Jue)**

### **module_kernel.jue**

| Function                                | Description              | Proof / Verification                      |
| --------------------------------------- | ------------------------ | ----------------------------------------- |
| `micro_kernel_validate(proposal)`       | Validate module proposal | Reject invalid code                       |
| `module_propose()`                      | Create proposal          | Must attach proof or empirical validation |
| `install_new_version(component, proof)` | Update module            | Only after verification                   |

### **event_loop.jue**

| Function                 | Description           | Proof / Verification           |
| ------------------------ | --------------------- | ------------------------------ |
| `receive_event(mailbox)` | Fetch event           | Test delivery consistency      |
| `process_event(event)`   | Handle event          | Ensure correct module response |
| `module_loop(mailbox)`   | Continuous event loop | Test deadlock-free execution   |

### **global_workspace.jue**

| Function                       | Description               | Proof / Verification                   |
| ------------------------------ | ------------------------- | -------------------------------------- |
| `compute_salience(event)`      | Evaluate event importance | Threshold check correctness            |
| `publish_to_all(event)`        | Broadcast event           | Ensure correct propagation             |
| `subscribe_to_all_modules(fn)` | Event subscription        | Verify module receives relevant events |

### **mutation_protocol.jue**

| Function                                | Description            | Proof / Verification                        |
| --------------------------------------- | ---------------------- | ------------------------------------------- |
| `mutate(component, new_version, level)` | Four-level mutation    | Each level verified with proofs/tests/votes |
| `consensus_reached?(votes)`             | Experimental promotion | Must correctly enforce majority             |

### **persistent_structures.jue**

| Function                 | Description        | Proof / Verification         |
| ------------------------ | ------------------ | ---------------------------- |
| `persistent_map()`       | Immutable map      | Test versioning correctness  |
| `assoc(map, key, value)` | Insert new version | Ensure old version unchanged |

---

## **Layer 4: Physics-World (Rust)**

### **primitives.rs**

| Function    | Description    | Proof / Verification               |
| ----------- | -------------- | ---------------------------------- |
| `add(a, b)` | Addition       | Unit test, verify output           |
| `sub(a, b)` | Subtraction    | Unit test                          |
| `mul(a, b)` | Multiplication | Unit test                          |
| `div(a, b)` | Division       | Unit test & division-by-zero check |

### **atomic_ops.rs**

| Function                | Description     | Proof / Verification |
| ----------------------- | --------------- | -------------------- |
| `atomic_add(ptr, val)`  | Atomic addition | Verify thread safety |
| `atomic_swap(ptr, val)` | Atomic swap     | Property-based tests |

### **memory_manager.rs**

| Function         | Description          | Proof / Verification                          |
| ---------------- | -------------------- | --------------------------------------------- |
| `allocate(size)` | Allocate memory      | Ensure immutability for persistent structures |
| `free(ptr)`      | Free memory          | Verify safe deallocation                      |
| `snapshot()`     | Take memory snapshot | Rollback test verification                    |

---

## **Cross-Layer Verification Checklist**

1. Every Jue construct → CoreExpr translation must include **proof attachment**.
2. Every CoreExpr evaluation → must be **consistent with relational semantics**.
3. Dan-World modules can only mutate after **correct proof verification or consensus**.
4. Event-driven communication must be **deadlock-free and guaranteed delivery**.
5. Persistent structures must maintain **immutable historical versions**.
6. Unit tests for every function must include **proof validation** where applicable.
7. Integration tests must validate **cross-world consistency** before advancing.

---
