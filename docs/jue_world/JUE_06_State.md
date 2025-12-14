
## **3. The State Conundrum: Functional vs. Imperative**

**The Problem:**
- Core-World is **purely functional** (no side effects)
- Cognition needs **state, mutation, identity over time**
- Physics layer provides **mutable memory cells**

**Options:**

**A. Monadic Style:** All effects via monads (like Haskell).
```jue
-- Pure functions return effect descriptions
readSensor :: () -> IO SensorReading
writeActuator :: Command -> IO ()
```
**Problem:** Verbose, hard for Dan to reason about.

**B. Linear Types:** Track uniqueness and mutation (like Rust).
```jue
-- Variables can be moved, not copied
mutate : (unique ref: Ref a) -> (a -> a) -> Ref a
```
**Problem:** Complex type system.

**C. Effect System:** Annotate effectful computations.
```jue
effectful readSensor() -> SensorReading @{io}
pure add(x, y) -> Number @{}
```

**My Advice:** **Linear regions with escape hatches.**
```jue
-- Core language is pure
-- But you can open "mutable regions"
region mutable {
  let x = alloc(5);  // Mutable reference
  x := 10;           // Mutation allowed here
  // Region ensures all mutations are contained
}
// x is now immutable again (or gone)

-- For Dan's self-modeling:
myMemory = region mutable {
  // Dan's working memory
  // Can be snapshotted, rolled back
}
```
