



**14. What is allowed to mutate?**
You banned free mutation — good. But:
• can Dan-World rewrite Jue-World code?
• can Jue-World rewrite itself?
• can physics influence semantics indirectly?

Mutation without a theory of permission is how agents eat their own axioms.


14A:
## **14. What is allowed to mutate?**

This is the security model for self-modification. We need a **permission lattice**.

### **The Mutation Permission Matrix:**

| Layer          | Can Mutate          | What                      | Approval Required                                       |
| -------------- | ------------------- | ------------------------- | ------------------------------------------------------- |
| **Physics**    | Never               | Nothing                   | N/A (frozen)                                            |
| **Core-World** | Never               | Nothing                   | N/A (frozen)                                            |
| **Jue-World**  | Its own macros      | Syntax, compilation rules | Formal proof + empirical testing                        |
| **Dan-World**  | Module parameters   | Weights, thresholds       | Module consensus + resource budget                      |
| **Dan-World**  | Module connections  | Which modules activate    | Global workspace vote + stability check                 |
| **Dan-World**  | Self-representation | Narrative, identity       | Tiered: minor=auto, major=vote, radical=human oversight |

**Critical Rule:** **Mutation can only move downward in abstraction, never upward.**

```
        Can mutate things at SAME or LOWER abstraction
        ─────────────────────────────────────────────►
        
High:   Narrative   → can change which modules activate
        │
        ▼
        Modules     → can change their Jue code
        │
        ▼
        Jue Code    → can change compilation rules
        │
        ▼
Low:    Core/Physics → CANNOT CHANGE (frozen)
```

**How Jue-World Rewrites Itself:**
- Jue macros can expand to different Core-World terms
- **But:** Must carry proof of semantic equivalence
- If proof impossible, demote to "experimental" status
- Experimental code runs in sandbox with resource limits

**How Dan-World Rewrites Jue-World:**
1. Dan module proposes Jue code change
2. Change is classified by trust tier:
   - Formal: requires proof (rare)
   - Verified: requires extensive testing
   - Empirical: requires successful pilot run
   - Experimental: requires isolation and monitoring
3. Approval depends on tier and risk level

**Preventing Axiom-Eating:**
```jue
// Critical axioms are marked as "foundational"
axiom non_contradiction = {
    statement: "¬(P ∧ ¬P)",
    trust_level: :foundational,
    mutation_requires: [:human_oversight, :multiple_votes, :backup_snapshot]
}

// Attempting to mutate foundational axioms triggers:
// 1. Full system snapshot
// 2. Human notification (if configured)
// 3. Fork of system with change in isolated branch
```

**Key Insight:** Mutation isn't "allowed" or "disallowed"—it's **tiered by risk**, with higher risk requiring more validation.


