# core_world De Bruijn & Call‑By‑Name Specification

This document replaces the previous CBV‑oriented cheat sheet. It is normative for **Core‑World** and is written to be directly actionable for Rust implementation and testing.

Core‑World is **not** an interpreter, evaluator, or runtime. It is a *semantic kernel*: a rewriting system that defines when two programs mean the same thing.

Execution, optimization, cost, time, and resource usage belong to higher layers (Jue‑World, Physics). This document intentionally excludes them.

---

## 0. Design Commitments (Non‑Negotiable)

Core‑World:

• Uses **pure λ‑calculus with De Bruijn indices**
• Uses **Call‑By‑Name (normal order, weak head)** semantics
• Defines **equivalence**, not performance
• Treats code as **structure**, not action
• Has **no notion of value vs expression**

If a design choice forces eagerness, values, or execution order observation, it does not belong here.

---

## 1. Mental Model (Read This First)

Core‑World is a **rewriting calculus**, not an interpreter.

Reduction is a *logical entailment*, not a step of computation. A term reduces because it *must*, not because we chose to execute it.

Once this is internalized, Call‑By‑Name is no longer an optimization choice — it is the only strategy that does not smuggle execution assumptions into meaning.

---

## 2. Core Syntax (Strategy‑Neutral)

```rust
enum CoreExpr {
    Var(usize),                 // De Bruijn index
    Lam(Box<CoreExpr>),         // λ‑abstraction
    App(Box<CoreExpr>, Box<CoreExpr>),
}
```

Notes:
• There is no Value variant
• There is no environment
• There is no evaluator state

All meaning arises from structure alone.

---

## 3. De Bruijn Binding Semantics

An index refers to its binder by relative depth:

• `0` → nearest enclosing λ
• `1` → next outer λ
• `n` → binder reached after skipping `n` lambdas

α‑equivalent named terms map to identical structures. Renaming is erased *by construction*.

---

## 4. Shifting (Index Lifting)

Shifting preserves binding when a term crosses a λ boundary.

```rust
// ↑amount(expr) with cutoff
fn lift(expr: CoreExpr, amount: usize, cutoff: usize) -> CoreExpr {
    match expr {
        CoreExpr::Var(i) => {
            if i >= cutoff { CoreExpr::Var(i + amount) }
            else { CoreExpr::Var(i) }
        }
        CoreExpr::Lam(body) =>
            CoreExpr::Lam(Box::new(lift(*body, amount, cutoff + 1))),
        CoreExpr::App(f, a) =>
            CoreExpr::App(
                Box::new(lift(*f, amount, cutoff)),
                Box::new(lift(*a, amount, cutoff)),
            ),
    }
}
```

Shifting is **not evaluation**. It exists only to make substitution correct.

---

## 5. Substitution (Meta‑Operation)

Substitution defines *meaning preservation*. It is not required to be executed eagerly.

Formal rule:

```
[N / k]k = N
[N / k]n = n − 1        if n > k
[N / k]n = n            if n < k
[N / k](λM) = λ([↑¹(N) / k+1]M)
[N / k](M₁ M₂) = ([N / k]M₁)([N / k]M₂)
```

Rust reference implementation:

```rust
fn substitute(expr: CoreExpr, target: usize, replacement: CoreExpr) -> CoreExpr {
    match expr {
        CoreExpr::Var(i) => {
            if i == target { replacement }
            else if i > target { CoreExpr::Var(i - 1) }
            else { CoreExpr::Var(i) }
        }
        CoreExpr::Lam(body) => {
            let lifted = lift(replacement.clone(), 1, 0);
            CoreExpr::Lam(Box::new(substitute(*body, target + 1, lifted)))
        }
        CoreExpr::App(f, a) => CoreExpr::App(
            Box::new(substitute(*f, target, replacement.clone())),
            Box::new(substitute(*a, target, replacement)),
        ),
    }
}
```

This definition is **authoritative**.

---

## 6. Call‑By‑Name Reduction Semantics

Reduction is constrained by *where* it may occur.

Rules:

• Arguments are **never evaluated first**
• No reduction occurs **inside λ bodies**
• Only the **leftmost outermost** redex may reduce

This is weak head normal order.

---

## 7. β‑Reduction Rule (CBN)

```
(λM) N  →  [N / 0]M
```

Important:
• `N` is substituted *as‑is*
• `N` may be duplicated
• `N` may never be evaluated

---

## 8. Evaluation Contexts (Formal Gatekeeper)

```text
E ::= [ ] | (E M)
```

A reduction is valid **iff** the term matches `E[(λM) N]`.

This single rule forbids all eager behavior.

---

## 9. Single‑Step Reduction (Reference)

```rust
fn beta_reduce_once(expr: CoreExpr) -> Option<CoreExpr> {
    match expr {
        CoreExpr::App(f, a) => match *f {
            CoreExpr::Lam(body) => Some(substitute(*body, 0, *a)),
            other => beta_reduce_once(other)
                .map(|f2| CoreExpr::App(Box::new(f2), a)),
        },
        _ => None, // no descent under λ
    }
}
```

Descending under `Lam` is **forbidden**.

---

## 10. Normalization (Meaning Extraction)

```rust
fn normalize(expr: CoreExpr) -> CoreExpr {
    let mut cur = expr;
    while let Some(next) = beta_reduce_once(cur.clone()) {
        cur = next;
    }
    cur
}
```

Normalization may diverge. Core‑World does not short‑circuit divergence.

---

## 11. Canonical Examples

### Example 1 — Unused Argument

```
(λ0) Ω  →  0
```

Ω is never evaluated.

---

### Example 2 — Duplication

```
(λ(0 0)) N  →  (N N)
```

Duplication is semantic, not optional.

---

### Example 3 — Shadowing

```
λ( (λ0) 0 )  →  λ0
```

Inner binder correctly shadows outer.

---

## 12. Testing Requirements

Core tests MUST include:

• Deep shadowing
• Free variable preservation
• Unused divergent arguments
• Nested substitution under binders

Tests must assert **structure**, not step counts.

---

## 13. Explicit Non‑Goals

Core‑World deliberately excludes:

• Call‑By‑Value
• Laziness / sharing
• Thunks
• Environments
• Cost models
• Side effects

Those belong elsewhere.

---

## Final Insight

De Bruijn indices define *what binds to what*.

Call‑By‑Name defines *when meaning is forced*.

Core‑World exists so that every higher layer can be wrong in new and interesting ways — without corrupting truth at the bottom.
