Here’s a comprehensive cheat sheet of **logical rules and transformations**, organized for both propositional and predicate logic. I’ll include basic equivalences, inference rules, laws, and some advanced patterns you can use in proofs, simplifications, and automated reasoning. This is meant to be exhaustive but also practical.

---

## **I. Basic Logical Operators**

| Symbol | Name          | Meaning            |
| ------ | ------------- | ------------------ |
| ¬A     | Negation      | Not A              |
| A ∧ B  | Conjunction   | A and B            |
| A ∨ B  | Disjunction   | A or B (inclusive) |
| A → B  | Implication   | If A then B        |
| A ↔ B  | Biconditional | A if and only if B |
| ⊥      | Contradiction | Always false       |
| ⊤      | Tautology     | Always true        |

---

## **II. Fundamental Equivalences**

### 1. **Negation Rules**

* Double negation: ¬(¬A) ≡ A
* De Morgan’s laws:
  ¬(A ∧ B) ≡ ¬A ∨ ¬B
  ¬(A ∨ B) ≡ ¬A ∧ ¬B
* Negation of implication: ¬(A → B) ≡ A ∧ ¬B

### 2. **Implication & Biconditional**

* Material implication: A → B ≡ ¬A ∨ B
* Biconditional as conjunction of implications: A ↔ B ≡ (A → B) ∧ (B → A)
* Biconditional alternative: A ↔ B ≡ (A ∧ B) ∨ (¬A ∧ ¬B)

### 3. **Commutativity**

* A ∧ B ≡ B ∧ A
* A ∨ B ≡ B ∨ A

### 4. **Associativity**

* (A ∧ B) ∧ C ≡ A ∧ (B ∧ C)
* (A ∨ B) ∨ C ≡ A ∨ (B ∨ C)

### 5. **Distributivity**

* A ∧ (B ∨ C) ≡ (A ∧ B) ∨ (A ∧ C)
* A ∨ (B ∧ C) ≡ (A ∨ B) ∧ (A ∨ C)

### 6. **Idempotent Laws**

* A ∧ A ≡ A
* A ∨ A ≡ A

### 7. **Absorption Laws**

* A ∧ (A ∨ B) ≡ A
* A ∨ (A ∧ B) ≡ A

### 8. **Identity and Domination**

* Identity: A ∧ ⊤ ≡ A, A ∨ ⊥ ≡ A
* Domination: A ∨ ⊤ ≡ ⊤, A ∧ ⊥ ≡ ⊥

### 9. **Negation Laws**

* A ∨ ¬A ≡ ⊤ (Law of excluded middle)
* A ∧ ¬A ≡ ⊥ (Contradiction law)

---

## **III. Inference Rules (Propositional Logic)**

### 1. **Modus Ponens**

* From A → B and A, infer B

### 2. **Modus Tollens**

* From A → B and ¬B, infer ¬A

### 3. **Disjunctive Syllogism**

* From A ∨ B and ¬A, infer B

### 4. **Hypothetical Syllogism**

* From A → B and B → C, infer A → C

### 5. **Constructive Dilemma**

* From (A → B) ∧ (C → D) and A ∨ C, infer B ∨ D

### 6. **Conjunction and Simplification**

* From A and B, infer A ∧ B
* From A ∧ B, infer A (or B)

### 7. **Addition**

* From A, infer A ∨ B

### 8. **Resolution Rule**

* From A ∨ B and ¬A ∨ C, infer B ∨ C

---

## **IV. Equivalences Involving Quantifiers (Predicate Logic)**

### 1. **Negation**

* ¬∀x P(x) ≡ ∃x ¬P(x)
* ¬∃x P(x) ≡ ∀x ¬P(x)

### 2. **Distributivity**

* ∀x (P(x) ∧ Q(x)) ≡ ∀x P(x) ∧ ∀x Q(x)
* ∃x (P(x) ∨ Q(x)) ≡ ∃x P(x) ∨ ∃x Q(x)

### 3. **Other Quantifier Transformations**

* ∀x (P(x) ∨ Q) ≡ (∀x P(x)) ∨ Q
* ∃x (P(x) ∧ Q) ≡ (∃x P(x)) ∧ Q
* ∀x P(x) → Q ≡ ∃x P(x) → Q
* (∃x P(x)) → Q ≡ ∀x (P(x) → Q)

---

## **V. Advanced Transformations / Patterns**

### 1. **Implication Chains**

* A → B → C ≡ A ∧ B → C ≡ A → (B → C)

### 2. **Contrapositive**

* A → B ≡ ¬B → ¬A

### 3. **Exportation / Importation**

* (A ∧ B) → C ≡ A → (B → C)

### 4. **Distributive Implication**

* A → (B ∧ C) ≡ (A → B) ∧ (A → C)
* (A ∨ B) → C ≡ (A → C) ∧ (B → C)

### 5. **Tautology Patterns**

* (A → B) ∧ (B → A) ≡ A ↔ B
* (A → B) ∨ (B → C) is often used in conditional proof patterns

### 6. **De Morgan Generalization**

* ¬(A₁ ∧ … ∧ An) ≡ ¬A₁ ∨ … ∨ ¬An
* ¬(A₁ ∨ … ∨ An) ≡ ¬A₁ ∧ … ∧ ¬An

### 7. **Consensus / Redundancy**

* (A ∧ B) ∨ (¬A ∧ C) ∨ (B ∧ C) ≡ (A ∧ B) ∨ (¬A ∧ C)
* Useful in simplifications for CNF/DNF

### 8. **Distribution for CNF/DNF**

* Distribute OR over AND for CNF: A ∨ (B ∧ C) ≡ (A ∨ B) ∧ (A ∨ C)
* Distribute AND over OR for DNF: A ∧ (B ∨ C) ≡ (A ∧ B) ∨ (A ∧ C)

### 9. **Absorption and Simplification**

* A ∨ (A ∧ B) ≡ A
* A ∧ (A ∨ B) ≡ A

---

## **VI. Proof Strategies / Transformations**

* **Conditional Proof:** Assume A, derive B → conclude A → B
* **Indirect Proof / Proof by Contradiction:** Assume ¬A, derive contradiction → conclude A
* **Case Analysis:** Break A ∨ B into two subcases, prove C in both → conclude C
* **Resolution-based Proof:** Convert to CNF and iteratively apply resolution to reach ⊥

---

## **VII. Common Shortcuts / Patterns**

* Implication: replace → with ¬A ∨ B to simplify in CNF/DNF
* Contrapositive is often easier for proof than direct →
* Combine De Morgan + distributivity for simplification
* Predicate logic: pull out constants from quantifiers when possible

---

This cheat sheet can be used both for **manual logical derivation** and **automated reasoning / SAT solver transformations**.

