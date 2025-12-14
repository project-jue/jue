## Core-World: the foundation of meaning

Core-World is the most abstract layer, so its ambiguities are the most dangerous.

**1. What is the unit of meaning?**
Is it:
• a Core-World term
• a Jue-World program
• a rewrite trajectory?

1. What is the unit of meaning?
In Core-World, the unit of meaning is a Core-World term (a λ-calculus expression) and its semantic interpretation. However, meaning in Core-World is not just the term but the equivalence class of terms under the defined rewrite rules (β, and possibly η). The meaning of a term is its behavior under reduction, or more formally, its normal form (if it exists). For non-terminating terms, we have to decide whether to consider them as having a meaning (like ⊥) or not.

But note: Core-World is not just about terms, but about the relationships between terms (rewrite rules) and the proofs of equivalence. So the unit of meaning is a term along with its semantic denotation (which could be a mathematical object in some model, but we don't have to go there; we can stay operational).

However, when we move to Jue-World, the unit of meaning might be a Jue program, which is a more complex structure that compiles to a Core-World term. But the meaning of that Jue program is exactly the meaning of its Core-World compilation.

So, to be precise:

In Core-World: the unit of meaning is a λ-term and its equivalence class (or its behavior under reduction).

In Jue-World: the unit of meaning is a Jue program, but its meaning is defined by the Core-World term it compiles to.



1. What is the unit of meaning?
Answer: A Core-World term's equivalence class under βη-reduction.

Reasoning: This gives you maximum flexibility while maintaining precision:

Normal form: λx.x means "identity function"

Equivalence class: λx.x, λy.y, λz.z are all the same meaning

Non-terminating: (λx.xx)(λx.xx) has no normal form, so its meaning is "divergence" (⊥)

Why not Jue-World programs? Because Jue programs include execution artifacts (order, state, side effects). Core-World should capture timeless meaning.

Why not rewrite trajectories? Because they're procedural, not semantic. Two different reduction paths to the same normal form should have the same meaning.

Recommendation: Implement Core-World with:

β-reduction (essential)

η-reduction (makes meaning extensional)

Congruence rules (if M ≡ N then λx.M ≡ λx.N, etc.)