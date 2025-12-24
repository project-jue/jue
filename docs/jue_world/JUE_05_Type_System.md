

## **2. The Type System Dilemma: Safety vs. Flexibility**

**The Problem:**
- Core-World is **untyped** λ-calculus (maximum flexibility)
- Real programs need **types** for safety and optimization
- Dan-World needs to **evolve types** as it learns

**Options:**

**A. Gradual Types:** Start untyped, add types incrementally.
```jue
-- Type annotations are optional
add :: (x: Number, y: Number) -> Number
add = λx.λy. x + y  -- Type checked

dynamicAdd = λx.λy. x + y  -- Runtime type checks
```

**B. Dependent Types:** Types can depend on values.
```jue
-- Enables strong verification
vector : (n: Nat) -> Type
append : (n m: Nat) -> Vector n -> Vector m -> Vector (n + m)
```
**Problem:** Complex, hard for Dan to invent.

**C. Manifest Contracts:** Types as runtime contracts.
```jue
-- Types are runtime predicates
type Number = {x | isNumber(x)}
type Even = {x: Number | x % 2 == 0}
```

**My Advice:** **Gradual types with runtime contracts.**
1. Start with simple types (Number, String, Function)
2. Add refinement types as contracts
3. Let Dan discover and propose new type predicates
4. Type checking can be:
   - **Formal:** With proof (for critical code)
   - **Empirical:** With runtime checks (for learning code)
   - **Experimental:** No checks (for exploration)

## Recursive Function Type Safety

**Current Implementation**: Recursive functions are fully supported with proper type inference and environment handling.

### Type System Support for Recursion

**Type Inference**: The compiler correctly handles recursive function type inference:
```jue
; Factorial function - type inferred as (Number) -> Number
(let [factorial (λn. (if (= n 0) 1 (* n (factorial (- n 1)))))])
  (factorial 5))

; Mutual recursion - types inferred correctly
(let [even? (λn. (if (= n 0) true (odd? (- n 1))))
      odd?  (λn. (if (= n 0) false (even? (- n 1))))]
  (even? 4))
```

**Trust Tier Type Checking**:
- **Formal Tier**: Recursive functions generate proof obligations for type preservation
- **Verified Tier**: Type checking ensures recursive calls maintain type consistency
- **Empirical Tier**: Runtime type validation for recursive function arguments
- **Experimental Tier**: Dynamic type checking during recursive execution

**Environment Type Safety**: Two-pass environment handling ensures:
1. **First Pass**: Captures recursive variable references with correct types
2. **Second Pass**: Validates recursive calls against inferred function types
3. **Closure Safety**: Type-safe closure environment creation and access

**Current Limitations**: While type inference works correctly for compilation, full type safety in execution requires:
- Complete arithmetic operator type checking
- If expression type validation
- Pattern matching type inference

---