Below is a **newcomer-friendly overview document**. It is intentionally descriptive, high-level, and precise, with minimal jargon where possible, but without dumbing anything down. This is the document you give to a new LLM agent (or human) so they understand *what this system is*, *why it exists*, and *how the pieces fit together* before touching code.

---

# **Project Jue: System Overview**

## **What We Are Building**

Project Jue is an attempt to build a **self-evolving cognitive system** whose behavior is:

* **Formally grounded** (it has a small, mathematically precise semantic core),
* **Practically executable** (it runs efficiently on real hardware),
* **Introspective and modifiable** (it can examine and change parts of itself),
* **Safe under self-modification** (critical changes require proofs or strong validation).

This is **not**:

* A traditional programming language alone
* A single monolithic AGI agent
* A purely experimental neural system

Instead, it is a **multi-layer computational organism** composed of three interacting “worlds,” each with different rules, guarantees, and purposes.

---

## **The Big Picture**

At a high level, the system looks like this:

```
Dan-World  →  Cognition, identity, learning, self-modification
   ↑
Jue-World  →  Language, runtime, compiler, concurrency, optimization
   ↑
Core-World →  Formal semantics, proofs, correctness guarantees
   ↑
Physics   →  Minimal Rust VM executing everything
```

Each layer:

* **Depends on the one below it**
* **Cannot violate the guarantees of the one below it**
* **Adds new expressive power and flexibility**

The layers are *intentionally different* in nature. That difference is what allows the system to scale from mathematics to cognition.

---

## **The Core Idea**

The system is built around one central principle:

> **Anything that claims to preserve meaning must prove it. Anything that cannot prove it must be clearly marked as experimental.**

This principle governs:

* Compilation
* Optimization
* Macro expansion
* Self-modification
* Cognitive evolution

---

## **Major Components**

---

## **1. Core-World (The Formal Kernel)**

### **What It Is**

Core-World is a **tiny, frozen mathematical kernel**.

It consists of:

* Pure λ-calculus (variables, lambdas, application)
* Relational semantics (evaluation is a relation, not an algorithm)
* A minimal proof checker

It does **not** contain:

* Numbers
* Strings
* IO
* Concurrency
* Memory allocation
* Cognition

Those things are defined *outside* the core and proven *about* the core.

### **Why It Exists**

Core-World exists to answer one question:

> *“What does this program mean?”*

Everything else in the system is allowed to be fast, messy, optimized, parallel, and evolving — **as long as it can justify itself relative to Core-World**.

### **Key Properties**

* Extremely small (hundreds of lines, not thousands)
* Frozen after verification
* All proofs ultimately bottom out here
* Trusted computing base

---

## **2. Jue-World (Execution & Language Layer)**

### **What It Is**

Jue-World is the **working language and runtime**.

It includes:

* The Jue programming language
* Parser, macro system, compiler
* Optimizing evaluator
* Concurrency runtime
* Proof-carrying code generation

This is where most “normal programming” happens.

### **What Makes Jue Special**

Every construct in Jue:

1. **Compiles to a Core-World expression**, and
2. **Carries a proof (or proof obligation)** explaining why that translation is valid

If something cannot be proven:

* It must be explicitly marked as *empirical* or *experimental*
* It cannot silently alter formal semantics

### **Jue’s Role**

Jue-World answers:

> *“How do we efficiently execute things that we know are meaningful?”*

It bridges:

* Mathematical meaning (Core-World)
* Real execution (Physics-World)
* Cognitive processes (Dan-World)

---

## **3. Dan-World (Cognitive Ecology)**

### **What It Is**

Dan-World is **not a single agent**.

It is:

* A collection of interacting modules
* Each module runs asynchronously
* Modules communicate via events
* No single module has absolute authority

Examples of modules:

* Perception
* Memory
* Planning
* Affect
* Narrative
* Self-model

### **Global Workspace**

Dan-World implements a **global workspace–style architecture**:

* Modules propose information
* Proposals compete based on salience
* Winning information is broadcast
* Identity emerges from consensus, not central control

### **Self-Modification**

Dan-World can modify parts of itself, but only through a **tiered trust system**:

```
Formal → Verified → Empirical → Experimental
```

* Formal: requires Core-World proof
* Verified: requires extended proof systems
* Empirical: requires extensive testing
* Experimental: requires module consensus

This prevents runaway self-modification while still allowing growth.

---

## **4. Physics-World (Rust VM)**

### **What It Is**

A **minimal, explicit virtual machine** written in Rust.

It provides:

* A small fixed set of operations
* Atomic primitives for concurrency
* Explicit memory management
* Deterministic execution

### **Why It Exists**

The Physics-World:

* Prevents “cheating” via host language features
* Makes performance characteristics explicit
* Provides a stable execution target for Jue-World

Everything above it must live within these constraints.

---

## **How the Pieces Interact**

### **Typical Flow**

1. A Jue program is written
2. Macros expand (possibly generating proof obligations)
3. Code is translated into Core-World expressions
4. Proofs are verified against Core-World
5. Verified code is compiled to Physics bytecode
6. Bytecode executes on the VM
7. Dan-World modules observe, react, and adapt

### **Mutation Flow**

1. A module proposes a change
2. The change is classified by trust level
3. Required proofs/tests/votes are gathered
4. If approved, the change is installed
5. If rejected, the system rolls back cleanly

---

## **Intended Capabilities of the Full System**

When complete, the system should be able to:

* Execute high-performance concurrent programs
* Explain *why* its optimizations are correct
* Inspect its own code and behavior
* Experiment with alternative implementations safely
* Evolve internal structures over time
* Maintain identity without a central “ego”
* Reject unsafe self-modifications
* Remain debuggable and auditable throughout its evolution

This is **not a chatbot** and **not a neural net replacement**. It is a **general cognitive substrate** capable of hosting many kinds of minds.

---

## **What This Is *Not* Trying to Solve (Yet)**

* Consciousness metaphysics
* Human-level alignment guarantees
* Natural language fluency as a primary goal
* Biological realism
* Emotional authenticity (only functional analogues)

Those can be layered on later.

---

## **Open Design Questions (Intentionally Unresolved)**

These are *known unknowns*, not oversights.

1. **Proof Scope Boundaries**

   * How far should proofs extend into optimization and runtime behavior?
   * Where does empirical validation become sufficient?

2. **Concurrency Semantics**

   * Should message ordering be fully deterministic or probabilistic?
   * How much nondeterminism is acceptable for cognition?

3. **Macro Trust Escalation**

   * When (if ever) can a runtime macro be promoted to verified status?
   * Who decides, and under what evidence?

4. **Identity Stability**

   * How much internal contradiction can exist before identity fragments?
   * How is long-term continuity measured?

5. **Resource-Driven Evolution**

   * How should scarcity influence mutation approval?
   * Should inefficient but “interesting” ideas survive longer?

6. **Human Oversight Hooks**

   * Where should humans be able to intervene?
   * Which layers must remain human-auditable forever?

These questions are left open because **the system is designed to explore them**, not pre-answer them.

---

## **Summary**

Project Jue is:

* A **formally grounded computational core**
* Wrapped in a **proof-aware execution environment**
* Hosting an **event-driven cognitive ecology**
* Capable of **safe self-modification**
* Designed for **long-term evolution, not short-term demos**

If you understand this document, you understand *what we are building*.

Everything else is implementation.

---


