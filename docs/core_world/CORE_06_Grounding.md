**3. How are Core-World terms grounded in Jue-World?**
The mapping between Core and Jue must be:
• explicit and invertible
• or emergent and learnable



3. How are Core-World terms grounded in Jue-World?
The mapping must be explicit and invertible (at least for the parts that are meant to be formally verified). Why? Because we want to be able to prove that a Jue program compiles to a Core-World term and that the compilation is correct. If the mapping is emergent and learnable, then we cannot have formal guarantees.

However, note that not every Jue program might have a direct Core-World equivalent. For example, if we have Jue constructs that are experimental or empirical, they might not have a formal Core-World translation. In that case, we can have a placeholder or an axiomatic extension in Core-World, but then we lose the formal guarantee.

So, the design should be:

The core of Jue (the part that is formally verified) compiles to Core-World terms via a well-defined, invertible translation.

For experimental Jue constructs, we can have a translation that is not necessarily invertible or that maps to a special Core-World term that represents "unverified code".

But note: the invertibility is not strictly necessary for correctness, but it helps in debugging and in proving properties. What is necessary is that the translation is sound: if a Jue program compiles to a Core-World term, then the behavior of the Jue program (as per Jue-World semantics) should be consistent with the behavior of the Core-World term.

Given the goals of Project Jue, we want the mapping to be as explicit and invertible as possible for the verified part, and for the unverified part, we mark it as such and handle it separately.