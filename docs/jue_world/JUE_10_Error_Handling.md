
## **7. The Error Handling: Total vs. Partial**

**The Problem:**
- Core-World has **no errors** (only divergence)
- Real programs have **partial functions, exceptions**
- Dan needs to **handle and learn from failures**

**Options:**

**A. Total Functions:** Everything returns a value.
```jue
-- Like Maybe/Either monads
divide : Number -> Number -> Maybe Number
divide x y = if y == 0 then Nothing else Just (x / y)
```

**B. Exceptions:** Jump out on error.
```jue
-- Traditional try/catch
try {
  result = 1 / 0;
} catch (e: DivisionByZero) {
  handleError(e);
}
```

**C. Effect Typing:** Errors in type system.
```jue
-- Functions declare possible errors
readFile :: Path -> String !{FileNotFound, PermissionDenied}
```

**My Advice:** **Result types with pattern matching.**
```jue
-- All partial operations return Result
type Result a = Ok a | Error ErrorType

divide x y = 
  if y == 0 
  then Error DivisionByZero 
  else Ok (x / y)

-- Forces explicit handling
case divide 10 0 {
  Ok result -> use(result)
  Error e -> handle(e)
}

-- But provide syntactic sugar
try? divide(10, 0)  -- Returns Error if fails
```