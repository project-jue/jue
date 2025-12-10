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

**Extended Syntax**:

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

## **5. Milestones and Verification Requirements**

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

## **6. Agent Instructions**

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

## **7. Reference Materials**

* Formal λ-calculus semantics
* Jue grammar specification and extended macros
* Event-driven concurrency patterns
* Proof-carrying code examples (constant folding, primitive inlining, beta-reduction)
* Persistent data structure patterns
* Cross-world synchronization protocols

---

This is a **foundational document** for the engineering team. Each LLM agent will work with this as a master guide. It is **unambiguous**, describes goals, interactions, verification points, and folder structures.

---

If you want, the next step is to **expand this into a full “implementation checklist per module”**, with concrete examples of Core ↔ Jue ↔ Dan code, proof obligations, and tests. That would let LLM agents execute confidently without managerial intervention.

Do you want me to start that next?
