## Formal Definition of De Bruijn Indices

De Bruijn indices are a notation for lambda terms that eliminates the need for variable names by using natural numbers to indicate binding relationships.

### 1. **Abstract Syntax**

Let Λ be the set of lambda terms using De Bruijn indices, defined inductively:

1. **Variable**: For any natural number n ∈ ℕ, n ∈ Λ  
   (Represents a bound variable, where n indicates the distance to its binder)

2. **Abstraction**: If M ∈ Λ, then λM ∈ Λ  
   (The λ binds all free variables in M by incrementing their indices by 1)

3. **Application**: If M, N ∈ Λ, then (M N) ∈ Λ

### 2. **Semantics: Variable Binding**

A De Bruijn index n refers to the λ binder encountered by counting outward n+1 λ's:

- **0** refers to the **innermost** enclosing λ (the most recent binder)
- **1** refers to the next enclosing λ
- **n** refers to the λ binder n+1 levels out

### 3. **Examples of Translation**

**Named variables → De Bruijn indices:**

```
λx. x              ↦ λ0
λx. λy. x          ↦ λλ1
λx. λy. λz. x z    ↦ λλλ2 0
(λx. x x)(λy. y)   ↦ (λ0 0)(λ0)
```

### 4. **Formal Properties**

Let FV(M) be the set of free variables in M (as natural numbers representing "unbound" indices):

```
FV(n) = {n}
FV(λM) = {k | k+1 ∈ FV(M)}  (each free variable shifts down by 1)
FV(M N) = FV(M) ∪ FV(N)
```

### 5. **Substitution [N/k]M**

The substitution of term N for index k in term M is defined recursively:

```
[N/k]k = N
[N/k]n = n-1       if n > k
[N/k]n = n         if n < k
[N/k](λM) = λ([↑(N)/k+1]M)  where ↑(N) increments all free variables in N by 1
[N/k](M₁ M₂) = ([N/k]M₁)([N/k]M₂)
```

### 6. **β-Reduction**

```
(λM) N →β [N/0]M
```

Where the substitution `[N/0]M` replaces all occurrences of 0 in M with N (appropriately adjusted).

### Key Insight

De Bruijn indices encode binding structure **positionally**: each variable indicates "how many λ's to skip" to find its binder, making α-equivalence (renaming of bound variables) syntactic identity. This eliminates the need for α-conversion in implementations and formal reasoning about lambda calculus.


# Deeper
# Deep Substitution Examples in De Bruijn Indices

Here are progressively deeper examples showing how substitution works with De Bruijn indices. Recall the substitution rules:

```
[N/k]k = N
[N/k]n = n-1    if n > k
[N/k]n = n      if n < k
[N/k](λM) = λ([↑(N)/k+1]M)
[N/k](M₁ M₂) = ([N/k]M₁)([N/k]M₂)
```

Where ↑(N) increments all free variables in N by 1.

## **Example 1: Simple Substitution**
```
Term: λ(0 1)  
Substitute: [2/1](λ(0 1))
Step 1: = λ([↑(2)/2](0 1))
Step 2: ↑(2) = 3  (free variable 2 becomes 3)
Step 3: [3/2](0 1) = ([3/2]0)([3/2]1)
         = (0)(3)  (since 0 < 2, 1 < 2)
Step 4: = λ(0 3)
Result: λ(0 3)
```

## **Example 2: Nested Abstraction**
```
Term: λλ(1 0)  
Substitute: [2/0](λλ(1 0))
Step 1: = λ([↑(2)/1](λ(1 0)))
Step 2: ↑(2) = 3
Step 3: [3/1](λ(1 0)) = λ([↑(3)/2](1 0))
Step 4: ↑(3) = 4
Step 5: [4/2](1 0) = ([4/2]1)([4/2]0)
         = (1)(0)  (since 1 < 2, 0 < 2)
Step 6: = λ(1 0)
Step 7: = λλ(1 0)
Result: λλ(1 0)  (no change - 0 was bound)
```

## **Example 3: Complex Substitution**
```
Term: λ(λ(2 1) 0)  
Substitute: [λ0/1](λ(λ(2 1) 0))
Step 1: = λ([↑(λ0)/2](λ(2 1) 0))
Step 2: ↑(λ0) = λ(↑(0)) = λ1
Step 3: [λ1/2](λ(2 1) 0) = ([λ1/2]λ(2 1))([λ1/2]0)
Step 4: [λ1/2]λ(2 1) = λ([↑(λ1)/3](2 1))
         ↑(λ1) = λ(↑(1)) = λ2
         [λ2/3](2 1) = ([λ2/3]2)([λ2/3]1)
         = (λ2)(1)  (since 2 < 3, 1 < 3)
         = λ(λ2 1)
Step 5: [λ1/2]0 = 0  (0 < 2)
Step 6: = (λ(λ2 1)) 0
Step 7: = λ((λ(λ2 1)) 0)
Result: λ((λ(λ2 1)) 0)
```

## **Example 4: Multiple Free Variables**
```
Term: λ(3 2 1 0)  
Substitute: [λ0/1](λ(3 2 1 0))
Step 1: = λ([↑(λ0)/2](3 2 1 0))
Step 2: ↑(λ0) = λ1
Step 3: [λ1/2](3 2 1 0) = (3 2 λ1 0) after applying substitution to each:
        [λ1/2]3 = 2 (3 > 2 → 3-1 = 2)
        [λ1/2]2 = 1 (2 = 2 → λ1)
        [λ1/2]1 = 1 (1 < 2 → 1)
        [λ1/2]0 = 0 (0 < 2 → 0)
Step 4: = λ(2 1 λ1 0)
Result: λ(2 1 λ1 0)
```

## **Example 5: Deeply Nested**
```
Term: λλ(λ(4 3 2) 1 0)  
Substitute: [λ0/2](λλ(λ(4 3 2) 1 0))
Step 1: = λ([↑(λ0)/3](λ(λ(4 3 2) 1 0)))
Step 2: ↑(λ0) = λ1
Step 3: [λ1/3](λ(λ(4 3 2) 1 0)) = λ([↑(λ1)/4](λ(4 3 2) 1 0))
Step 4: ↑(λ1) = λ2
Step 5: [λ2/4](λ(4 3 2) 1 0) = ([λ2/4]λ(4 3 2))([λ2/4]1)([λ2/4]0)
Step 6: [λ2/4]λ(4 3 2) = λ([↑(λ2)/5](4 3 2))
         ↑(λ2) = λ3
         [λ3/5](4 3 2) = (3 2 λ3) (since 4 > 5? Let's compute properly)
         Actually: [λ3/5]4 = 3 (4 < 5? Wait, 4 < 5, so 4 remains 4)
         Correction: 4 < 5 → 4, 3 < 5 → 3, 2 < 5 → 2
         So [λ3/5](4 3 2) = (4 3 2)
         This seems wrong... Let me recompute carefully.

Better approach: Let's compute systematically:

[λ2/4]λ(4 3 2) = λ([↑(λ2)/5](4 3 2))
↑(λ2) = λ(↑(2)) = λ3
[λ3/5](4 3 2) = ([λ3/5]4) ([λ3/5]3) ([λ3/5]2)
[λ3/5]4 = 4 (4 < 5)
[λ3/5]3 = 3 (3 < 5)  
[λ3/5]2 = 2 (2 < 5)
So = (4 3 2)

Thus: [λ2/4]λ(4 3 2) = λ(4 3 2)

Step 7: [λ2/4]1 = 1 (1 < 4)
Step 8: [λ2/4]0 = 0 (0 < 4)
Step 9: = (λ(4 3 2) 1 0)
Step 10: = λ(λ(4 3 2) 1 0)
Step 11: = λλ(λ(4 3 2) 1 0)

Result: λλ(λ(4 3 2) 1 0)  (no change - 2 was bound two levels up)
```

## **Example 6: β-Reduction Example**
```
Term: (λ(λ(2 1 0)))(λ0)  →β  ?
First, note: (λM)N →β [N/0]M
Here M = λ(2 1 0), N = λ0

So: [λ0/0]λ(2 1 0) = λ([↑(λ0)/1](2 1 0))
↑(λ0) = λ1
[λ1/1](2 1 0) = ([λ1/1]2)([λ1/1]1)([λ1/1]0)
[λ1/1]2 = 1 (2 > 1 → 2-1 = 1)
[λ1/1]1 = λ1 (1 = 1 → λ1)
[λ1/1]0 = 0 (0 < 1)
= (1 λ1 0)
Thus: λ(1 λ1 0)

Result: λ(1 λ1 0)
```

## **Key Insight**
The trickiest part is when substituting under λ-binders:
1. The substitution index `k` increases by 1 for each λ we go under
2. The substituted term `N` must be "lifted" (↑) for each λ we go under
3. Free variables in the body adjust based on their relationship to `k`

This ensures that variables maintain their correct binding relationships even after substitution removes or adds abstraction layers.