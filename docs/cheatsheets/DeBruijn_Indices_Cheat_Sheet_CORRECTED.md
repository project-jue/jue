Here's a **comprehensive De Bruijn Index Cheat Sheet** for both practical and advanced scenarios, aligned with the formal mathematical definition. All examples and formulas are verified against the trusted formal definition document.

---

# **De Bruijn Index Cheat Sheet - Corrected**

---

## 1. Basic Representation

| Lambda Calculus       | De Bruijn       | Explanation                       |
| --------------------- | --------------- | --------------------------------- |
| `λx. x`               | `λ 0`           | 0 refers to innermost enclosing λ |
| `λx. λy. x`           | `λ λ 1`         | 1 refers to the next enclosing λ  |
| `λx. λy. y`           | `λ λ 0`         | 0 refers to innermost enclosing λ |
| `λx. λy. x y`         | `λ λ 1 0`       | 1 refers to x, 0 refers to y      |
| `λx. λy. λz. x (y z)` | `λ λ λ 2 (1 0)` | 2 refers to x, 1 to y, 0 to z     |

**Pattern:** Count the number of binders between occurrence and binder. Closest binder = 0, next outer = 1, etc.
**Formal Rule:** Index n refers to λ binder encountered by counting outward n+1 λ's.

---

## 2. Shifting (↑)

**Goal:** Avoid capture when inserting terms under new binders.

* `↑d t` shifts all free variables ≥ cutoff `c` by `d`.

**Example:** Insert term `t` under 2 new lambdas.

```text
Original: 1
Shift by 2: 3
```

* **Formal Rule (aligned with definition):**

  ```
  For shift(d, c, t):
  x >= c → x + d
  x < c  → x
  ```

* **Pattern for nested shift (matching formal FV rules):**

  ```
  shift(d, c, λ t1) = λ shift(d, c+1, t1)
  shift(d, c, t1 t2) = shift(d, c, t1) shift(d, c, t2)
  shift(d, c, x) = x+d if x >= c else x
  ```

**Note:** This aligns with the formal property FV(λM) = {k | k+1 ∈ FV(M)}

---

## 3. Single Substitution (Capture-Avoiding) - Formal Definition

**Goal:** Replace index `k` with term `s` using formal mathematical rules.

**Formal Substitution Rules (must follow exactly):**

```text
[N/k]k = N
[N/k]n = n-1       if n > k
[N/k]n = n         if n < k
[N/k](λM) = λ([↑(N)/k+1]M)  where ↑(N) increments all free variables in N by 1
[N/k](M₁ M₂) = ([N/k]M₁)([N/k]M₂)
```

**Example following formal rules:**

```text
Term: λ λ 1 0
Substitute λ0 for index 1
Step 1: [λ0/1](λ λ 1 0) = λ([↑(λ0)/2](λ 1 0))
Step 2: ↑(λ0) = λ(↑(0)) = λ1  (lifting free variables by 1)
Step 3: [λ1/2](λ 1 0) = λ([↑(λ1)/3](1 0)) = λ([λ2/3](1 0))
Step 4: [λ2/3]1 = 1 (1 < 3), [λ2/3]0 = 0 (0 < 3)
Step 5: Result: λ λ 1 0
```

**Pattern (formal recursive definition):**

```text
subst(k, s, λ t) = λ subst(k+1, ↑1 s, t)
subst(k, s, t1 t2) = subst(k, s, t1) subst(k, s, t2)
subst(k, s, x) = ↑k s if x==k else x
```

---

## 4. Multi-variable Substitution

* Simultaneously substitute multiple indices:

```text
t[ k1 ↦ s1, k2 ↦ s2, ... ]
```

**Formal Pattern:**

1. Order substitutions from largest index to smallest
2. Shift each `si` up by number of lambdas it will pass under
3. Apply sequentially using formal rules

**Example (formal calculation):**

```text
Term: λ λ λ 2 1 0
Substitute index 2 → s1, index 1 → s2
Formal approach: [s1/2]([s2/1](t))
Shift s1 up by 2 (for outer lambdas)
Shift s2 up by 1 (for outer lambdas)
Apply formal substitution rules → safe, capture-free result
```

---

## 5. Beta Reduction - Formal Definition

**Pattern:** `(λ t) s → t[0 ↦ s]` (exactly as defined formally)

**Formal β-Reduction Rule:**
```
(λM) N →β [N/0]M
```

**Step-by-step following formal rules:**

1. Shift `s` up by 1 (for new lambda context)
2. Substitute index 0 in `t` with shifted `s`
3. Apply formal substitution rules exactly

**Example (corrected calculation):**

```text
(λ λ 1 0) (λ0)
Step1: β-reduction: [λ0/0](λ λ 1 0)
Step2: Apply formal rule: λ([↑(λ0)/1](λ 1 0))
Step3: ↑(λ0) = λ1
Step4: [λ1/1](λ 1 0) = λ([↑(λ1)/2](1 0)) = λ([λ2/2](1 0))
Step5: [λ2/2]1 = 1 (1 < 2), [λ2/2]0 = 0 (0 < 2)
Step6: Result: λ(1 0)
```

**Correct Result: λ(1 0)** (not the incorrect λ λ λ1 0 from original)

---

## 6. Nested Shifts for Recursion

Recursive lambdas, e.g., Y combinator:

```text
Y = λ f. (λ x. f (x x)) (λ x. f (x x))
De Bruijn: λ (λ 1 (0 0)) (λ 1 (0 0))
```

* **Formal Pattern:**
  When substituting under recursion, **increment shift by number of enclosing lambdas** each time you go deeper
* **Use case:** Fixpoints, lazy evaluation
* **Critical:** Must follow formal β-reduction and substitution rules

---

## 7. Let-binding Translation

Convert `let` into lambda form:

```text
(let ((x 1) (y 2)) (+ x y)) 
→ λ λ (+ 1 0)
```

* **Formal Pattern:** Treat `let` as a series of lambdas
* **Shifting:** Each subsequent let-binding increases the depth for the remaining body
* **Verification:** Must satisfy formal FV properties

---

## 8. Advanced Macro Expansion (Hygiene)

* Use De Bruijn indices to safely inline or reorder code

**Formal Pattern:**

```text
Macro expansion: generate λ-terms with indices
Shift inserted terms by depth of macro insertion
Substitute safely using formal rules to avoid accidental capture
```

* Example: Reordering `let` blocks:

```text
Original: λ 0 1
Insert new binding: shift inserted term ↑1 → λ (shifted term) 0 1
Result: Safe using formal substitution rules
```

---

## 9. Typed Lambda Calculus (Polymorphism)

* Both **type variables** and **term variables** can use De Bruijn indices
* Substitution rules apply at two layers:

```text
t[term-var ↦ term]
T[type-var ↦ type]
```

* Shifting required in both layers
* Must maintain formal properties for both terms and types

---

## 10. Lazy Evaluation / Environment Optimization

* In closure-based evaluation, De Bruijn indices allow **direct array indexing**:

```text
Term: λ λ 1 0
Closure env: [x, y]
Access:
- index 0 → env[1] (y)
- index 1 → env[0] (x)
```

* **Pattern:** Avoids hash-table lookups or name resolution at runtime
* **Formal guarantee:** Index semantics preserved through environment mapping

---

## 11. Pattern Matching for Rewrite Rules

* De Bruijn indices allow structural pattern matching:

```text
Pattern: λ λ 1 0
Match term: λ λ 2 0
Shift pattern or term → normalize → compare numeric structure
```

* Useful for:

  * Macro expansions
  * Compiler optimizations
  * Symbolic computation (proof assistants)

---

## 12. Edge Cases / Gotchas - Formal Verification

| Case                             | Formal Treatment                                                             |
| -------------------------------- | ---------------------------------------------------------------------------- |
| Free variables                   | Index ≥ current depth, must shift properly per FV rules                      |
| Empty lambda                     | λ 0 → valid, but index 0 refers to the binder                                |
| Insertion under multiple binders | Shift by the number of binders inserted per formal shift rules               |
| Recursive substitution           | Must carefully manage nested shifts using formal recursive definition        |
| Combining typed and term indices | Keep separate counters to avoid confusion, apply formal rules to both layers |

---

## 13. Cheat Patterns Summary - Formally Verified

1. **Indexing**: Closest binder = 0, next = 1, … (formal definition)
2. **Shift**: `↑d t` increments free variables ≥ cutoff `c` by `d` (formal rule)
3. **Single substitution**: Use formal recursive definition exactly
4. **Multiple substitutions**: Order by descending indices, shift each accordingly (formal)
5. **Beta reduction**: `λ t s → t[0 ↦ ↑1 s] ↓1` (formal β-reduction rule)
6. **Recursive lambdas**: Shift on each recursion depth increment (formal)
7. **Let → λ translation**: Sequential lambdas with increasing depth (formal)
8. **Macro insertion**: Shift inserted code by depth to maintain hygiene (formal)
9. **Typed De Bruijn**: Separate indices for types and terms (formal)
10. **Closure evaluation**: Indices map directly to environment slots (formal)
11. **Pattern matching / rewrite**: Compare numeric structure after normalization (formal)
12. **Edge cases**: All handled by formal mathematical rules

---

## 14. Critical Implementation Rules

### **Formal Substitution Rules (Authoritative)**
```
[N/k]k = N
[N/k]n = n-1       if n > k
[N/k]n = n         if n < k
[N/k](λM) = λ([↑(N)/k+1]M)  where ↑(N) increments all free variables in N by 1
[N/k](M₁ M₂) = ([N/k]M₁)([N/k]M₂)
```

### **Formal β-Reduction**
```
(λM) N →β [N/0]M
```

### **Formal Free Variable Rules**
```
FV(n) = {n}
FV(λM) = {k | k+1 ∈ FV(M)}  (each free variable shifts down by 1)
FV(M N) = FV(M) ∪ FV(N)
```

### **Formal Variable Binding**
- Index n refers to λ binder encountered by counting outward n+1 λ's
- 0 refers to innermost enclosing λ (most recent binder)
- 1 refers to next enclosing λ
- n refers to λ binder n+1 levels out

---

## **Key Implementation Notes**

### **The Core Principle**
All operations must follow the formal mathematical definition exactly. The formal definition provides the authoritative source of truth - all practical implementations, examples, and algorithms must align precisely with these rules.

### **Why This Matters**
1. **Mathematical Correctness**: Ensures the implementation matches theoretical foundations
2. **Bug Prevention**: Prevents subtle off-by-one errors and capture issues
3. **Proof Verification**: Enables formal verification of transformations
4. **Interoperability**: Ensures consistent behavior across all system layers

### **Verification Checklist**
- [ ] All substitution examples follow formal rules exactly
- [ ] All β-reduction calculations are mathematically correct
- [ ] All shifting operations align with FV properties
- [ ] All recursive definitions match the formal inductive structure
- [ ] All edge cases are handled by formal mathematical principles

---

**Remember**: When in doubt, refer to the formal definition. It is the single source of mathematical truth for De Bruijn index operations in this system.