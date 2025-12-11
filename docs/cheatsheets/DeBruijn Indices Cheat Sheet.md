Here’s a **comprehensive De Bruijn Index Cheat Sheet** for both practical and advanced scenarios. I’ve included shifting, substitution, reduction, recursive patterns, and some tricky edge cases you often encounter in homoiconic interpreters or compilers. I’ll use **0-indexing** (0 = nearest binder).

---

# **De Bruijn Index Cheat Sheet**

---

## 1. Basic Representation

| Lambda Calculus       | De Bruijn       |
| --------------------- | --------------- |
| `λx. x`               | `λ 0`           |
| `λx. λy. x`           | `λ λ 1`         |
| `λx. λy. y`           | `λ λ 0`         |
| `λx. λy. x y`         | `λ λ 1 0`       |
| `λx. λy. λz. x (y z)` | `λ λ λ 2 (1 0)` |

**Pattern:** Count the number of binders between occurrence and binder. Closest binder = 0, next outer = 1, etc.

---

## 2. Shifting (↑)

**Goal:** Avoid capture when inserting terms under new binders.

* `↑d t` shifts all free variables ≥ cutoff `c` by `d`.

**Example:** Insert term `t` under 2 new lambdas.

```text
Original: 1
Shift by 2: 3
```

* **Rule:**

  ```
  x >= c → x + d
  x < c  → x
  ```

* **Pattern for nested shift:**

  ```
  shift(d, c, λ t1) = λ shift(d, c+1, t1)
  shift(d, c, t1 t2) = shift(d, c, t1) shift(d, c, t2)
  shift(d, c, x) = x+d if x >= c else x
  ```

---

## 3. Single Substitution (Capture-Avoiding)

**Goal:** Replace index `k` with term `s`.

* `t[k ↦ s]`:

```text
x = variable index in t
if x == k: replace with shift(k, 0, s)
if x < k: leave as x
if x > k: decrement by 1 (binder removed)
```

* Example:

```text
Term: λ λ 1 0
Substitute λ0 for index 1
Shift λ0 up by 1 → λ1
Replace index 1 → λ1 0
```

* **Pattern:**

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

**Pattern:**

1. Order substitutions from largest index to smallest.
2. Shift each `si` up by number of lambdas it will pass under.
3. Apply sequentially.

* Example:

```text
Term: λ λ λ 2 1 0
Substitute index 2 → s1, index 1 → s2
Shift s1 up by 2 (for outer lambdas)
Replace indices sequentially → safe, capture-free
```

---

## 5. Beta Reduction

**Pattern:** `(λ t) s → t[0 ↦ s]`

1. Shift `s` up by 1 (for new lambda context).
2. Substitute index 0 in `t` with shifted `s`.
3. Shift result down by 1.

**Example:**

```text
(λ λ 1 0) (λ0)
Step1: Shift λ0 → λ1
Step2: Substitute 0 → λ1
Step3: Shift down → λ λ λ1 0
```

---

## 6. Nested Shifts for Recursion

Recursive lambdas, e.g., Y combinator:

```text
Y = λ f. (λ x. f (x x)) (λ x. f (x x))
De Bruijn: λ (λ 1 (0 0)) (λ 1 (0 0))
```

* **Pattern:**
  When substituting under recursion, **increment shift by number of enclosing lambdas** each time you go deeper.
* **Use case:** Fixpoints, lazy evaluation.

---

## 7. Let-binding Translation

Convert `let` into lambda form:

```text
(let ((x 1) (y 2)) (+ x y)) 
→ λ λ (+ 1 0)
```

* **Pattern:** Treat `let` as a series of lambdas.
* **Shifting:** Each subsequent let-binding increases the depth for the remaining body.

---

## 8. Advanced Macro Expansion (Hygiene)

* Use De Bruijn indices to safely inline or reorder code.

**Pattern:**

```text
Macro expansion: generate λ-terms with indices
Shift inserted terms by depth of macro insertion
Substitute safely to avoid accidental capture
```

* Example: Reordering `let` blocks:

```text
Original: λ 0 1
Insert new binding: shift inserted term ↑1 → λ (shifted term) 0 1
```

---

## 9. Typed Lambda Calculus (Polymorphism)

* Both **type variables** and **term variables** can use De Bruijn indices.
* Substitution rules apply at two layers:

```text
t[term-var ↦ term]
T[type-var ↦ type]
```

* Shifting required in both layers.

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

* **Pattern:** Avoids hash-table lookups or name resolution at runtime.

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

## 12. Edge Cases / Gotchas

| Case                             | Note                                                            |
| -------------------------------- | --------------------------------------------------------------- |
| Free variables                   | Index ≥ current depth, must shift properly                      |
| Empty lambda                     | λ 0 → valid, but index 0 refers to the binder                   |
| Insertion under multiple binders | Shift by the number of binders inserted                         |
| Recursive substitution           | Must carefully manage nested shifts to avoid accidental capture |
| Combining typed and term indices | Keep separate counters to avoid confusion                       |

---

## 13. Cheat Patterns Summary

1. **Indexing**: Closest binder = 0, next = 1, …
2. **Shift**: `↑d t` increments free variables ≥ cutoff `c`.
3. **Single substitution**: Shift insertion, replace index, shift back.
4. **Multiple substitutions**: Order by descending indices, shift each accordingly.
5. **Beta reduction**: `λ t s → t[0 ↦ ↑1 s] ↓1`
6. **Recursive lambdas**: Shift on each recursion depth increment.
7. **Let → λ translation**: Sequential lambdas with increasing depth.
8. **Macro insertion**: Shift inserted code by depth to maintain hygiene.
9. **Typed De Bruijn**: Separate indices for types and terms.
10. **Closure evaluation**: Indices map directly to environment slots.
11. **Pattern matching / rewrite**: Compare numeric structure after normalization.
12. **Edge cases**: Free variables, nested lambdas, empty lambdas, recursive substitutions.

---


Extra:
The subtlety that often trips people up with De Bruijn indices and the usual `subst`/`shift` rules. Let’s walk through it carefully and clarify the logic.

---

### **1. Core idea of `subst(k, s, t)`**

We want to **replace variable `k` in `t` with `s`**. The key points are:

1. `k` is the **De Bruijn index we are targeting**.
2. `s` is the term we are substituting in, which may contain free variables.
3. `t` is the term where the substitution happens.

The subtlety comes when `t` is **under binders (lambdas)**: the free variables in `s` may now be **deeper in scope**, so we need to shift them.

---

### **2. The shift rule**

`↑d s` means: **increment all free variables in `s` that are ≥ `c` by `d`**.

* `d` = how much the free variables need to be bumped
* `c` = cutoff; variables below `c` are bound and should not be shifted

When substituting `s` under `n` additional lambdas, the free variables in `s` must be incremented by `n` to reflect the **new depth**.

---

### **3. Correct substitution formula**

If `t` is:

```text
t = λ. t'    ; one binder above t'
```

Then substitution proceeds recursively:

```text
subst(k, s, x) =
    if x == k: shift(depth, 0, s)    ; shift free vars in s by number of enclosing binders
    if x < k: x                       ; variable bound by some outer binder
    if x > k: x - 1                   ; because the binder at k is removed
```

Where `depth` is the **number of lambdas between the original target index and the current position**.

This is the important clarification: **it’s not “current depth” or “target index” alone**, it’s the difference between the two.

---

### **4. Beta reduction example**

Let’s carefully redo your example:

```text
(λ λ 1 0) (λ 0)
```

* Step 0: Identify the substitution target: index 0 in the outermost application.
* Step 1: Shift the argument `(λ 0)` **up by 1**, because it will go **under one binder** after substitution.

```text
shift(1, 0, λ 0) → λ 1
```

* Step 2: Substitute `0 → λ1` in the body `λ 1 0` (inside one binder):

```text
λ 1 0
```

* Inside the inner lambda:

  * Index `0` is **the one we are substituting for**, but now **under one binder**, so we use:

```text
shift(1, 0, λ1) → λ 2
```

* Step 3: Replace `0` → `λ 2`, leave `1` alone (it refers to outer lambda).

Result:

```text
λ λ λ1 0
```

✅ This matches your observation.

---

### **5. Key takeaway / general rule**

1. When substituting for `k`, **shift the replacement by the number of enclosing binders you have passed since leaving the binder for `k`**.
2. Do **not shift by current depth blindly**—the “distance from the target binder” is what matters.
3. After substitution, if you remove a binder (as in beta reduction), adjust indices **> k** by -1 to account for the removed binder.

---

### **6. Pseudocode for clarity**

```text
subst(k, s, x, depth):
    if x == k:
        return shift(depth, 0, s)
    else if x < k:
        return x
    else:  # x > k
        return x - 1

subst(k, s, λ t, depth):
    return λ subst(k+1, s, t, depth+1)
```

* `depth` = number of lambdas between the original target and the current recursion level.
* `k+1` inside a lambda because the lambda **binds a new variable**.
* `depth+1` passed to the shift because free variables in `s` are now **one deeper**.

---


Here’s a **structured outline** of how substitution and shifting work with De Bruijn indices, designed to be clear for an LLM or for implementation. It emphasizes **depth, target index, and shift logic**.

---

# **De Bruijn Substitution & Shift Outline**

## **1. Definitions**

1. **Variable indices**

   * `x` = De Bruijn index of a variable in term `t`
   * `k` = target index to substitute

2. **Substitution term**

   * `s` = term being substituted in for `x = k`

3. **Depth tracking**

   * `depth` = number of lambda binders **between the target binder and the current recursion point**
   * Used to correctly shift free variables in `s`

4. **Shift operator**

   * `shift(d, c, s)` = increment all free variables in `s` ≥ cutoff `c` by `d`

---

## **2. Variable substitution logic**

Given `subst(k, s, x, depth)`:

1. **Case 1: x == k**

   * Replace with `shift(depth, 0, s)`
   * Rationale: `s` may contain free variables that are now under `depth` new binders

2. **Case 2: x < k**

   * Variable is **bound by an outer lambda**, leave as is

3. **Case 3: x > k**

   * Variable refers to a binder that will be removed after substitution
   * Decrement index: `x - 1`

---

## **3. Lambda recursion**

For `subst(k, s, λ t, depth)`:

1. **Increment target index**: `k+1` because the new lambda **binds a variable**
2. **Increment depth**: `depth+1` because free variables in `s` are now **under one more binder**
3. **Recurse**: `λ subst(k+1, s, t, depth+1)`

---

## **4. Application / Beta Reduction**

For `(λ t1) t2`:

1. **Step 1: Shift argument `t2`**

   * `t2' = shift(1, 0, t2)` because it will go under **one lambda** after substitution

2. **Step 2: Substitute `0 → t2'` in `t1`**

3. **Step 3: Adjust indices for removed binder**

   * Decrement all variables **> 0** by 1 if necessary

---

## **5. Key principles / tips**

1. **Shift by “distance from target”**

   * Not “current depth” alone
   * Ensures free variables in `s` remain correctly scoped

2. **Always increment k inside lambdas**

   * Each lambda introduces a new binding that moves the target index

3. **Always increment depth inside lambdas**

   * Each lambda increases the number of enclosing binders for `s`

4. **Order of operations matters**

   * Shift first → substitute → adjust indices if a binder is removed

---

## **6. Example Walkthrough**

Term:

```text
(λ λ 1 0) (λ 0)
```

* **Target index**: 0 (outermost variable)
* **Argument**: `(λ 0)`

### Steps

1. Shift argument: `shift(1, 0, λ 0)` → `λ 1`
2. Substitute in body `λ 1 0` (under one lambda):

   * `depth = 1`
   * Replace index 0 → `shift(1, 0, λ 1)` → `λ 2`
3. Final term after beta reduction: `λ λ λ 1 0`

✅ This matches the correct De Bruijn result.

---

This outline captures **all the depth, shift, and recursion logic** clearly, making it easier for an LLM or algorithm to implement `subst` correctly without off-by-one errors.
