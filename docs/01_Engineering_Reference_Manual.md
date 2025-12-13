# **Project Jue: Engineering Reference Manual**

## **1. Project Overview**

**Project Goal:**
Create a hybrid AGI system built on a formally verified core (Core-World), an optimized execution layer (Jue-World), and an emergent cognitive layer (Dan-World). The system must support safe self-modification, proof-driven optimization, and modular cognitive processes.

**System Philosophy:**

* **Formal guarantees:** All critical operations are backed by proofs in Core-World.
* **Performance and practicality:** Jue-World executes efficiently, compiles proofs into optimized bytecode.
* **Emergent cognition:** Dan-World supports modular, asynchronous, event-driven cognition, capable of experimental growth under controlled trust protocols.

---

## **2. Jue Language Reference**

### **2.1 Grammar Overview**

Jue is a **S-expression-based language** with extensions for proof annotations, macros, and concurrency.

```
<expr> ::= <number>
         | <symbol>
         | (lambda <params> <body>)
         | (<operator> <arg1> <arg2> ... <argN>)
         | (quote <expr>)
         | (macro <name> <body>)
         | (proof-obligation <description> <expr>)
```

**Extended Syntax:**

* `fn [x] (+ x 1)` → shorthand for `(lambda (x) (+ x 1))`
* `match x (1 "one") (_ "other")` → pattern matching with proof obligations
* `annotate <core-expr> <proof>` → associates a Core-World proof with an expression

### **2.2 Data Types**

* **Primitive types:** Numbers, symbols, booleans
* **Compound types:** Lists, closures, macros
* **Proof annotations:** Attached to expressions to specify correctness constraints
* **Persistent structures:** Immutable maps and lists, supporting versioning and rollback

### **2.3 Operators and Built-ins**

* Arithmetic: `+`, `-`, `*`, `/`
* Logical: `and`, `or`, `not`
* Comparison: `=`, `<`, `>`, `<=`, `>=`
* Control: `if`, `cond`, `match`, `let`
* Core translation: `core-number`, `core-var`, `core-lam`, `core-app`
* Proof helpers: `prove-number`, `prove-var-lookup`, `prove-lambda`, `annotate`

---

## **3. Folder Structure**

**Root directory:**

```
/jue-project
├── /core-world       # Formal kernel (λ-calculus)
│   ├── core.rs       # CoreExpr definitions
│   ├── proof.rs      # Proof structures and verification logic
│   └── tests/        # Unit tests for core semantics
│

├── /jue-world        # Optimized execution & compiler
│   ├── parser.jue    # S-expression parser
│   ├── compiler.jue  # Compiler to Core-World
│   ├── eval.jue      # Optimized evaluator
│   ├── macros.jue    # Macro system
│   ├── runtime.jue   # Concurrency & runtime primitives
│   └── proofs/       # Proof obligations per optimization
│

├── /dan-world        # Cognitive layer
│   ├── modules/      # Perceptual, affective, memory, planning modules
│   ├── workspace.jue # Global workspace
│   ├── event-system.jue # Message passing, scheduler
│   └── mutation.jue  # Self-modification protocols
│

├── /physics          # Rust VM for low-level execution
│   ├── vm.rs         # Physical machine definitions
│   ├── ops.rs        # 12 primitive operations
│   └── tests/        # VM verification tests
│

├── /docs             # Documentation, diagrams, reference guides
└── /experiments      # Scripts for evolution, stress tests
```

**Notes:**

* Each `/jue-world` file should clearly indicate the **proof obligations it generates**.
* Modules in Dan-World must include **micro-kernels** capable of validating proposed changes before committing.

---

## **4. Architecture and Layer Interactions**

**Hierarchy:**

```
Core-World (Formal Kernel)
      ↑ Proof Obligations
Jue-World (Execution Engine)
      ↑ Verified / annotated code
Dan-World (Cognitive Ecology)
      ↑ Emergent, experimental modules
Physics Layer (Rust VM)
      ↑ Executes all bytecode
```

**Key Concepts:**

1. **Core-World**: Pure λ-calculus; defines relational semantics and proof checker. Immutable and frozen after verification.
2. **Jue-World**: Bridges Core-World and Dan-World. Handles compilation, optimizations, macros, and concurrency runtime. Every transformation produces a proof obligation.
3. **Dan-World**: Event-driven cognitive modules with mutation protocols. Supports four trust levels for self-modification: `experimental → empirical → verified → formal`.
4. **Physics Layer**: Minimal Rust VM with 12 primitive operations. Provides atomicity and concurrency primitives for Jue/Dan-World.

**Cross-World Bridges:**

* `ToCoreWithProof` interface in Rust/Jue: ensures every Jue construct has a Core-World representation and a verified proof.
* `synchronize-worlds`: periodic synchronization of optimizations, new primitives, and mutations.
* `mutation-level` protocol: governs self-modification hierarchy.

---

## **5. Core-World Implementation Details**

### **5.1 Formal Substitution Rules**

The substitution operation `[N/k]M` replaces variable `k` with expression `N` in expression `M`. The formal rules are:

```rust
// MUST implement exactly:
// [N/k]k = N
// [N/k]n = n-1       if n > k
// [N/k]n = n         if n < k
// [N/k](λM) = λ([↑(N)/k+1]M)
// [N/k](M₁ M₂) = ([N/k]M₁)([N/k]M₂)

fn substitute(expr: CoreExpr, target: usize, replacement: CoreExpr) -> CoreExpr {
    match expr {
        CoreExpr::Var(index) => {
            if index == target {
                replacement
            } else if index > target {
                CoreExpr::Var(index - 1)  // CORRECT: binder removed above
            } else {
                CoreExpr::Var(index)      // binder below, unchanged
            }
        }
        CoreExpr::Lam(body) => {
            // Lift free vars in replacement by 1 when going under binder
            let lifted = lift(replacement.clone(), 1, 0);
            CoreExpr::Lam(Box::new(substitute(*body, target + 1, lifted)))
        }
        CoreExpr::App(func, arg) => {
            CoreExpr::App(
                Box::new(substitute(*func, target, replacement.clone())),
                Box::new(substitute(*arg, target, replacement)),
            )
        }
    }
}
```

**Critical Note:** When substituting under a lambda, the replacement expression must be lifted to account for the new binder. This prevents variable capture and ensures correctness.

### **5.2 Lifting Implementation**

The lifting operation `↑^d_c M` increments free variables ≥ `c` by `d` in expression `M`:

```rust
// ↑d(N) with cutoff c: increment free variables ≥ c by d
// This matches the formal FV(λM) = {k | k+1 ∈ FV(M)} property

fn lift(expr: CoreExpr, amount: usize, cutoff: usize) -> CoreExpr {
    match expr {
        CoreExpr::Var(index) => {
            if index >= cutoff {
                CoreExpr::Var(index + amount)
            } else {
                CoreExpr::Var(index)
            }
        }
        CoreExpr::Lam(body) => {
            // IMPORTANT: cutoff + 1 when going under lambda
            CoreExpr::Lam(Box::new(lift(*body, amount, cutoff + 1)))
        }
        CoreExpr::App(func, arg) => {
            CoreExpr::App(
                Box::new(lift(*func, amount, cutoff)),
                Box::new(lift(*arg, amount, cutoff)),
            )
        }
    }
}
```

**Key Insight:** The cutoff parameter increases by 1 when going under a lambda to account for the new binder. This ensures that free variables are correctly identified and shifted.

### **5.3 Beta Reduction (Call-by-Value Semantics)**

For Lisp-like languages, use call-by-value semantics:

```rust
// For Lisp-like language, use call-by-value semantics:
// 1. Reduce function to WHNF
// 2. Reduce argument
// 3. Substitute

fn beta_reduce_cbv(expr: CoreExpr) -> CoreExpr {
    match expr {
        CoreExpr::App(func, arg) => {
            match beta_reduce_cbv(*func) {
                CoreExpr::Lam(body) => {
                    // Function is lambda, reduce argument first (call-by-value)
                    let reduced_arg = beta_reduce_cbv(*arg);
                    substitute(*body, 0, reduced_arg)
                }
                reduced_func => {
                    // Function not a lambda, return reduced application
                    CoreExpr::App(Box::new(reduced_func), arg)
                }
            }
        }
        CoreExpr::Lam(body) => {
            CoreExpr::Lam(Box::new(beta_reduce_cbv(*body)))
        }
        CoreExpr::Var(_) => expr,
    }
}
```

**Process Flow:**
1. Reduce the function expression to Weak Head Normal Form (WHNF)
2. Reduce the argument expression completely
3. Perform substitution of the argument into the function body

### **5.4 Alpha Equivalence (Environment-Based Comparison)**

Two expressions are α-equivalent if they're identical after normalizing binder indices:

```rust
// Two expressions are α-equivalent if they're identical after
// normalizing binder indices

fn alpha_equiv(a: &CoreExpr, b: &CoreExpr, env: Vec<(usize, usize)>) -> bool {
    match (a, b) {
        (CoreExpr::Var(i), CoreExpr::Var(j)) => {
            // Look up in environment or compare directly
            env.iter()
                .find(|(ai, _)| ai == i)
                .map(|(_, bj)| bj == j)
                .unwrap_or(i == j)
        }
        (CoreExpr::Lam(body_a), CoreExpr::Lam(body_b)) => {
            // Extend environment with mapping for new binder
            let new_depth = env.len();
            let mut new_env = env.clone();
            new_env.push((new_depth, new_depth));
            alpha_equiv(body_a, body_b, new_env)
        }
        (CoreExpr::App(f1, a1), CoreExpr::App(f2, a2)) => {
            alpha_equiv(f1, f2, env.clone()) && alpha_equiv(a1, a2, env)
        }
        _ => false,
    }
}
```

**Approach:** The environment tracks the mapping between binder indices in the two expressions being compared, allowing for correct comparison of variables under different binding contexts.

### **5.5 Common Implementation Pitfalls**

#### **Off-by-One Errors in Substitution**

```rust
// WRONG - doesn't account for binder removal
if index > target_index {
    CoreExpr::Var(index)  // Should be index - 1!
}

// CORRECT
if index > target_index {
    CoreExpr::Var(index - 1)  // Binder at target_index was removed
}
```

#### **Incorrect Lifting in Lambda Case**

```rust
// WRONG - using same cutoff
CoreExpr::Lam(Box::new(lift(*body, amount, cutoff)))

// CORRECT - increment cutoff under lambda
CoreExpr::Lam(Box::new(lift(*body, amount, cutoff + 1)))
```

#### **Forgetting to Clone in Recursive Calls**

```rust
// WRONG - moves replacement
Box::new(substitute(*func, target, replacement))

// CORRECT - clone for each branch
Box::new(substitute(*func, target, replacement.clone()))
```

---

## **6. Milestones and Verification Requirements**

### **Milestone 1: Core-World Verification**

* Verify CoreExpr semantics are consistent (`beta_reduce`, `alpha_equiv`, `normalize`)
* Ensure relational semantics match Core-World rules
* Unit tests for all λ-calculus constructs

### **Milestone 2: Jue-World Compiler & Evaluator**

* Jue → Core translation works for all constructs
* Proof obligations are correctly generated for every transformation
* Basic optimizations verified (constant folding, primitive inlining)
* Concurrency runtime handles event loops without deadlocks

### **Milestone 3: Dan-World Modules**

* Micro-kernels validate all proposed module mutations
* Event-driven message passing works with delivery guarantees
* Global workspace integrates module outputs with salience-based competition

### **Milestone 4: Integration**

* Synchronization protocol bridges Core ↔ Jue ↔ Dan
* Proof obligations verified before committing optimizations
* Safe self-modification experiments succeed without violating trust hierarchy

### **Milestone 5: Stress & Evolution Experiments**

* Cognitive loops operate for extended virtual time
* Evolutionary pressure experiments with resource constraints
* Snapshots maintain consistent system states

---

## **7. Testing Strategy**

### **Core-World Testing**

**Unit Tests:** Individual algorithm verification (β-reduction, α-equivalence)

```rust
#[test]
fn test_substitution_correctness() {
    // (λx. λy. x) z → λy. z
    let expr = CoreExpr::App(
        Box::new(CoreExpr::Lam(Box::new(CoreExpr::Lam(Box::new(
            CoreExpr::Var(1)
        ))))),
        Box::new(CoreExpr::Var(0)),
    );

    let result = beta_reduce(expr);
    // Should be: λ.0 (z with index 0)
    assert_eq!(result, CoreExpr::Lam(Box::new(CoreExpr::Var(0))));
}

#[test]
fn test_shadowing() {
    // λx. (λx. x) x → λ0 (λ0 0) 0
    // After β-reduction: λ0
    let expr = CoreExpr::Lam(Box::new(CoreExpr::App(
        Box::new(CoreExpr::Lam(Box::new(CoreExpr::Var(0)))),
        Box::new(CoreExpr::Var(0)),
    )));

    let result = normalize(expr);
    assert_eq!(result, CoreExpr::Lam(Box::new(CoreExpr::Var(0))));
}
```

**Property-Based Tests:** Mathematical property validation using proptest
**Integration Tests:** Cross-component interaction verification
**Stress Tests:** Performance under large expression loads

### **Jue-World Testing**

* **Compiler Tests:** Semantic preservation verification
* **Optimization Tests:** Proof-carrying optimization validation
* **Concurrency Tests:** Event-driven runtime verification
* **Integration Tests:** Jue-to-CoreExpr compilation pipeline

### **Dan-World Testing**

* **Event Loop Tests:** Module communication validation
* **Mutation Protocol Tests:** Trust-level validation verification
* **Integration Tests:** Cognitive module interaction testing
* **Stress Tests:** High-volume event processing

### **Physics Layer Testing**

* **Atomic Operation Tests:** Thread-safety verification
* **Memory Management Tests:** Garbage collection validation
* **VM Tests:** Bytecode execution correctness
* **Integration Tests:** Cross-layer execution verification

---

## **8. Agent Instructions**

**LLM Engineering Agents Must:**

1. Follow **folder structure** strictly; separate concerns.
2. Ensure **proof obligations are attached** to every compiler/evaluator/mutation step.
3. Test each new construct **before moving to the next milestone**.
4. Validate **Core-World equivalence** for any Jue-World addition.
5. Respect **four-level mutation protocol** in Dan-World; never bypass formal or verified checks.
6. Use persistent data structures; avoid in-place mutations outside experimental trust levels.
7. Annotate all code with descriptive comments explaining proof relevance and expected behavior.
8. Log all changes, mutations, and module votes for auditing.

---

## **9. Reference Materials**

* Formal λ-calculus semantics
* Jue grammar specification and extended macros
* Event-driven concurrency patterns
* Proof-carrying code examples (constant folding, primitive inlining, beta-reduction)
* Persistent data structure patterns
* Cross-world synchronization protocols
* [DeBruijn_CBV_Cheat_Sheet_Rust.md](docs/cheatsheets/DeBruijn_CBV_Cheat_Sheet_Rust.md) for complete implementation details

---

This is a **foundational document** for the engineering team. Each LLM agent will work with this as a master guide. It is **unambiguous**, describes goals, interactions, verification points, and folder structures.

---

If you want, the next step is to **expand this into a full "implementation checklist per module"**, with concrete examples of Core ↔ Jue ↔ Dan code, proof obligations, and tests. That would let LLM agents execute confidently without managerial intervention.

Do you want me to start that next?