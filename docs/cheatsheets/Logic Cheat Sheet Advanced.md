Alright, we’re moving into **meta-level and advanced logical reasoning**—the stuff that sits *on top of propositional and predicate logic*. Here we deal with **modalities, structural rules, multi-valued logics, fixed-point reasoning, and other higher-order transformations**. This cheat sheet is designed to give you patterns, transformations, and inference principles you can use beyond the basics.

---

## **I. Modal Logic (Necessity & Possibility)**

### **Operators**

| Symbol | Name        | Meaning               |
| ------ | ----------- | --------------------- |
| □A     | Necessity   | A is necessarily true |
| ◇A     | Possibility | A is possibly true    |

### **Basic Equivalences**

* □A ≡ ¬◇¬A (Necessity as dual of possibility)
* ◇A ≡ ¬□¬A (Possibility as dual of necessity)

### **Key Rules**

* **K Axiom:** □(A → B) → (□A → □B)
* **T Axiom (Reflexivity):** □A → A
* **4 Axiom (Transitivity):** □A → □□A
* **5 Axiom (Euclidean):** ◇A → □◇A

### **Advanced Patterns**

* □(A ∧ B) ≡ □A ∧ □B
* ◇(A ∨ B) ≡ ◇A ∨ ◇B
* Modal contraposition: □A → □B ≡ ¬□B → ¬□A

---

## **II. Temporal Logic**

### **Operators**

| Symbol | Meaning                              |
| ------ | ------------------------------------ |
| G A    | Globally / Always in the future      |
| F A    | Eventually / Some time in the future |
| X A    | Next state                           |
| A U B  | A Until B                            |

### **Equivalences & Transformations**

* ¬G A ≡ F ¬A
* ¬F A ≡ G ¬A
* A U B ≡ B ∨ (A ∧ X(A U B))
* G A ≡ A ∧ X(G A)

### **Proof Patterns**

* **Inductive temporal proof:** Show base case (now) and inductive step (next)
* **Co-inductive reasoning:** Prove invariants hold forever using G

---

## **III. Higher-Order Logic (HOL)**

### **Key Concepts**

* Functions as first-class objects: predicates can take predicates as arguments
* Lambda abstraction: λx. P(x)
* Quantification over functions: ∀f ∃x P(f(x))

### **Common Transformations**

* Beta reduction: (λx. F(x)) a → F(a)
* Eta reduction: λx. F(x) x ≡ F
* Skolemization at higher order: replace ∃f ∀x P(f(x), x) with f as witness

### **Proof Tactics**

* **Functional extensionality:** f = g ≡ ∀x f(x) = g(x)
* **Higher-order unification:** solve equations between predicates/functions

---

## **IV. Non-classical & Multi-valued Logics**

### **Many-valued logic (3VL or more)**

* Truth values: {True, False, Unknown/Undefined}
* Operators extended:

  * ¬Unknown ≡ Unknown
  * A ∧ Unknown ≡ Unknown
  * A ∨ Unknown ≡ True if A=True else Unknown

### **Fuzzy logic**

* Truth: t ∈ [0,1]
* Conjunction: min(t₁, t₂)
* Disjunction: max(t₁, t₂)
* Implication often: max(1 - t₁, t₂)

### **Paraconsistent Logic**

* A ∧ ¬A can be non-contradictory
* Useful for reasoning under inconsistent knowledge bases

---

## **V. Structural & Sequent Rules (Proof Theory)**

### **Sequents**

* Γ ⊢ Δ means: from assumptions Γ, we can derive conclusions Δ

### **Structural Rules**

* **Weakening:** Γ ⊢ Δ ⇒ Γ, A ⊢ Δ
* **Contraction:** Γ, A, A ⊢ Δ ⇒ Γ, A ⊢ Δ
* **Exchange:** reorder assumptions: Γ, A, B, Δ ⊢ Θ ⇒ Γ, B, A, Δ ⊢ Θ

### **Cut Rule**

* If Γ ⊢ A and Γ, A ⊢ Δ then Γ ⊢ Δ
* Important for modular proofs

---

## **VI. Fixed-Point & Recursive Reasoning**

### **Least / Greatest Fixed Points**

* μX. F(X): least fixed point (minimal solution)
* νX. F(X): greatest fixed point (maximal solution)

### **Application**

* Recursive definitions in logic programming (Prolog, Datalog)
* Temporal and modal reasoning: G and F as ν and μ fixed points

---

## **VII. Algebraic & Lattice Logic**

* **Lattice laws**:

  * Join: A ∨ B (supremum)
  * Meet: A ∧ B (infimum)
  * Absorption: A ∧ (A ∨ B) = A; A ∨ (A ∧ B) = A

* Useful for knowledge representation and domain lattices

* **Heyting Algebra (Intuitionistic Logic)**

  * Rejects law of excluded middle
  * Implication: A → B ≡ largest X such that A ∧ X ≤ B

---

## **VIII. Meta-Logical Transformations**

* **Reflection:** reasoning about provability: □A ≡ “A is provable”

* **Self-reference:** used in Godelian constructions

* **Consistency & Soundness Checks:**

  * ⊢ A means A is derivable (syntactic)
  * ⊨ A means A is semantically true

* **Transformations**

  * From proof systems to model theory: Γ ⊢ A ⇒ Γ ⊨ A
  * Conservative extensions: adding new axioms without invalidating old theorems

---

## **IX. Advanced Proof Patterns / Tactics**

* **Cut-elimination:** remove intermediate lemmas to simplify sequent proofs
* **Normalization / Canonical Forms:** transform formulas to CNF/DNF, or to modal normal forms
* **Tableau Method:** expand formulas systematically, close branches to check satisfiability
* **Resolution for HOL & Predicate Logic:** generalized unification with functions and quantifiers

---

## **X. Common Meta-Patterns**

| Pattern                  | Transformation / Use                                       |
| ------------------------ | ---------------------------------------------------------- |
| Contrapositive           | A → B ≡ ¬B → ¬A, often easier in proof                     |
| Exportation              | (A ∧ B) → C ≡ A → (B → C)                                  |
| Distribution             | Modal and temporal operators can often distribute over ∧/∨ |
| Fixed-point unfolding    | μX. F(X) ≡ F(μX. F(X)) (for induction)                     |
| Reflection / Provability | □A ↔ Provable(A) for meta-reasoning                        |

---

This cheat sheet gives you **everything you need to layer higher-order reasoning, temporal/modal thinking, and meta-logic on top of classical logic**.

