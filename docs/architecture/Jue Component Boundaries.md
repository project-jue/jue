Below is a **concrete, fully-fleshed set of component boundaries** for **juec** (the compiler) and **juerun** (the runtime), designed to support:

* Homoiconicity at AST level
* Self-modifying code
* REPL
* Future JIT and dynamic code loading
* Stable+clean architecture for early MVP
* Rust implementation using Pest + Cranelift

This serves as a **decoupled architecture map** for your implementation team and your LLM coding agents.

---

# **Jue Component Boundaries**

### *(juec = static compiler; juerun = dynamic runtime / interpreter / GC / JIT)*

---

# **1. High-Level Architecture Diagram**

```
          +----------------+
          |     juec       |
          |  Static Compiler|
          +----------------+
    (parsing + AST + IR + optional AOT JIT)
          |
          v
+-------------------+         +-------------------+
|   JUE IR (.jir)   | ----->  |  juerun (Runtime) |
|  Serialized AST + |         |   GC + VM/JIT     |
|  Optional Bytecode|         |   Dynamic Loader  |
+-------------------+         +-------------------+
```

---

# **2. juec – The Static Compiler**

**juec produces code artifacts** (AST → IR → machine code or bytecode).
It **does not run programs**, except optionally running constant-folded operations at compile time.

## **2.1. Responsibilities**

### **(1) Frontend**

* Lexing (Pest)
* Parsing (Pest)
* Building AST (your JueAST data structure)
* AST rewrites and macros (homoiconicity support)
* Hygiene and scope checking
* Type inference (optional at first)
* Static linking of modules
* Constant folding / partial evaluation

### **(2) Middle-End**

* AST → JUE IR lowering
* IR passes:

  * dead code elimination
  * inlining
  * tail-call analysis
  * purity annotation
* IR validation

### **(3) Backend**

Two possible backends:

#### **A. Cranelift AOT backend**

Outputs:

* Native binary
* `.o` object files
* Shared libraries `.so` / `.dll`
* Or just Cranelift IR

#### **B. Jue Bytecode backend**

Outputs:

* `.jbc` bytecode file
* Designed to be interpreted or JIT’d by juerun

### **(4) Artifact Output**

juec can produce:

| Artifact           | Purpose                                                                    |
| ------------------ | -------------------------------------------------------------------------- |
| **`.jir`**         | “Serialized AST”; needed for self-modifying code to load & manipulate ASTs |
| **`.jbc`**         | Bytecode for juerun                                                        |
| **`.so` / `.dll`** | Native plugin for dynamic loading                                          |
| **“.exe”**         | Final native binary, no runtime needed                                     |
| **`.clif`**        | Human-auditable Cranelift IR                                               |

---

# **2.2. Boundaries**

juec **must not contain:**

* the garbage collector
* the VM
* the runtime object model
* function invocation logic
* dynamic AST editing
* mutable code support

These belong to juerun.

juec **may contain:**

* a compile-time evaluator (interpreter for constant folding)
* an IR optimizer

---

# **3. juerun – The Runtime Environment**

This is where **the “soul” of Jue lives**:
homoiconicity, dynamic code loading, self-modification, JIT (future), GC, REPL.

## **3.1. Responsibilities**

### **(1) Runtime object model**

* Unified representation for:

  * numbers
  * strings
  * classes
  * functions
  * AST nodes (crucial)
* Stable memory layout for GC
* Distinct type tags or pointer compression

### **(2) Homoiconicity Engine**

Core subsystem.

Handles:

* Representation of ASTs at runtime
* `quote`, `splice`, quasiquote
* AST mutation (transformations, rewrites)
* Code-to-function compilation (“eval”)
* Code serialization and deserialization

This is deeply intertwined with GC.

### **(3) Execution Engine**

Two possible paths:

#### **A. Bytecode Interpreter**

Simple VM:

* stack-based or register-based
* executes `.jbc` from the compiler
* fast to implement

#### **B. JIT Engine (optional later)**

Using Cranelift:

* dynamic IR → machine code
* incremental recompilation
* inline caching for dynamic dispatch
* stack maps and shadow stack support for GC integration

### **(4) Dynamic Loader**

juerun should be able to load:

* `.jir` (AST)
* `.jbc` (bytecode)
* `.dll` / `.so` (native functions)
* Strings of Jue code (from user or program)

### **(5) Garbage Collector**

GC lives *entirely in juerun*, not juec.

Should include:

* tri-color marking
* generational nursery
* incremental mode (for interactive/agent tasks)
* AST/graph-aware marking

### **(6) Runtime standard library**

* File IO
* Networking
* Math, strings, lists
* AST utilities
* Serialization

### **(7) REPL**

Uses:

* parser (from juec’s frontend code)
* interpreter / evaluator
* dynamic linker

---

# **4. Boundary Summary Table**

| Feature                     | juec             | juerun          |
| --------------------------- | ---------------- | --------------- |
| Lexing/parsing              | ✓                | ✓ (repl)        |
| AST creation                | ✓                | ✓ (dynamic AST) |
| AST mutation                | ✗                | ✓               |
| Macroexpansion              | ✓                | ✓               |
| Compile-time evaluation     | ✓                | ✗               |
| Runtime evaluation (`eval`) | ✗                | ✓               |
| IR generation               | ✓                | ✗               |
| Bytecode generation         | ✓                | ✗               |
| Bytecode interpreter        | ✗                | ✓               |
| JIT compilation             | optional         | ✓               |
| GC                          | ✗                | ✓               |
| Static linking              | ✓                | ✗               |
| Dynamic module loading      | ✗                | ✓               |
| Native binary output        | ✓                | ✗               |
| REPL                        | ✗                | ✓               |
| Homoiconicity               | limited (macros) | full support    |

---

# **5. Why This Boundary Is Ideal for Jue**

### ✔ Supports full homoiconicity

We keep runtime ASTs and transformations in **juerun**, not the static compiler.

### ✔ Allows Jue to bootstrap itself later

Eventually Jue code will generate Jue code, load modules, and compile AST fragments.

### ✔ Prevents borrow-checker issues

GC & AST mutation sit in a separate, pointer-rich subsystem.

### ✔ Enables dynamic agents and self-modifying programs

This separation lets:

* juec be simple, fast, stable
* juerun be flexible and powerful

### ✔ Allows aggressive optimization

Static paths (juec) can be heavily optimized via Cranelift.
Dynamic paths (juerun) can JIT, patch, adapt.

---

# **6. What You Should Do Next**

I can generate a complete roadmap, but here’s the short version:

### **A. Lock in this architecture**

This gives your LLM agents a strong skeleton to work against.

### **B. Implement juec-first (recommended)**

* Parser
* AST
* Frontend
* Bytecode generator
* Simple IR

### **C. Implement juerun second**

* Object model
* GC
* Bytecode interpreter
* AST manipulation
* Minimal standard library
* REPL

### **D. Add dynamic code compilation later (phase 3 or 4)**

Either:

* Cranelift JIT
* or dynamic loading of `.dll` compilations from juec

---

# **If you want, I can next generate:**

### ✔ A full directory layout

### ✔ Detailed interface definitions

### ✔ Data structures for IR, bytecode, runtime objects

### ✔ A Jue→IR→bytecode pipeline diagram

### ✔ Or actual Rust project skeletons for both components

Just tell me what you want next.
