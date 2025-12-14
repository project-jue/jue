# De Bruijn Indices and Call‑By‑Name Semantics

This document is intentionally split into two layers:

1. **A strategy‑neutral formal calculus** based on De Bruijn indices. This layer defines syntax, binding, shifting, and substitution purely as mathematical objects.
2. **A Call‑By‑Name (CBN) operational semantics** that specifies *how* and *when* β‑reduction is permitted to occur, without requiring eager substitution.

This separation is deliberate. It prevents evaluation strategy from leaking into the core representation and allows alternative runtimes (CBV, lazy, graph‑based) to coexist without altering meaning.

---

## Layer 1 — Strategy‑Neutral De Bruijn Calculus

### 1. Abstract Syntax

Let Λ be the set of lambda terms defined inductively:

• **Variable**: for any natural number n ∈ ℕ, `n ∈ Λ`

• **Abstraction**: if `M ∈ Λ`, then `λM ∈ Λ`

• **Application**: if `M, N ∈ Λ`, then `(M N) ∈ Λ`

A term is *well‑formed* if every index refers to an enclosing abstraction.

---

### 2. Binding Semantics

A De Bruijn index refers to a binder by relative depth:

• `0` refers to the nearest enclosing λ
• `1` refers to the next outer λ
• `n` refers to the λ encountered after skipping `n` binders

Binding structure is positional. α‑equivalent named terms map to identical De Bruijn representations.

---

### 3. Shifting (Index Lifting)

Shifting adjusts free variable indices when a term crosses a binder boundary.

Define `↑ᵈ(M)` as *shifting by d*:

• `↑ᵈ(n) = n + d`
• `↑ᵈ(λM) = λ(↑ᵈ(M))`
• `↑ᵈ(M N) = (↑ᵈ(M) ↑ᵈ(N))`

Shifting is not evaluation. It is a structural transformation used to preserve binding correctness.

---

### 4. Free Variables

Let `FV(M)` denote the set of free indices in M:

• `FV(n) = {n}`
• `FV(λM) = {k | k + 1 ∈ FV(M)}`
• `FV(M N) = FV(M) ∪ FV(N)`

Free variables are defined purely for capture‑avoidance reasoning.

---

### 5. Substitution (Meta‑Operation)

Substitution is a *conceptual* operation, not an evaluation rule.

Define `[N / k]M` as replacing index `k` in `M` with term `N`:

• `[N / k]k = N`
• `[N / k]n = n − 1` if `n > k`
• `[N / k]n = n` if `n < k`
• `[N / k](λM) = λ([↑¹(N) / k + 1]M)`
• `[N / k](M₁ M₂) = ([N / k]M₁)([N / k]M₂)`

Substitution guarantees capture avoidance by lifting N when descending under binders.

No assumption is made about *when* substitution is executed.

---

## Layer 2 — Call‑By‑Name Operational Semantics

### 6. Evaluation Strategy

Call‑By‑Name evaluation is defined by *where reduction is allowed*, not by modifying substitution itself.

CBN rules:

• Arguments are **not evaluated before substitution**
• No reduction occurs **inside abstractions**
• Reduction occurs only at the **outermost leftmost application**

This is sometimes called *weak head reduction*.

---

### 7. β‑Reduction Rule (CBN)

The sole computational step is:

```
(λM) N → [N / 0]M
```

This rule applies **only** when the application is in evaluation position.

The argument `N` is substituted *as‑is*, without prior evaluation.

---

### 8. Evaluation Contexts

Define evaluation contexts `E`:

```
E ::= [ ] | (E M)
```

Reduction is permitted only when a term fits the form `E[(λM) N]`.

No evaluation is permitted inside `λM` or inside the argument position `N`.

---

### 9. Operational Consequences

• Arguments may be duplicated during substitution
• Divergence can occur even when arguments are unused
• Substitution may be delayed or represented via environments

The calculus specifies *meaning*, not implementation strategy.

---

## 10. Implementation Notes (Non‑Normative)

A runtime may:

• Delay substitution using closures
• Represent terms as graphs
• Cache substituted arguments
• Share unevaluated thunks

As long as observable behavior matches the CBN semantics above, the implementation is valid.

---

## Key Insight

De Bruijn indices define *what expressions mean*.

Call‑By‑Name defines *when meaning is forced*.

Conflating these layers leads to premature evaluation, accidental CBV semantics, and brittle runtimes. Keeping them separate preserves both rigor and flexibility.
