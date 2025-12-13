# **De Bruijn Index Cheat Sheet - Rust Implementation Guide**

---

## **Core Implementation Rules (Must Follow Exactly)**

### **1. Formal Substitution Definition**
```rust
// MUST implement exactly:
// [N/k]k = N
// [N/k]n = n-1       if n > k
// [N/k]n = n         if n < k
// [N/k](λM) = λ([↑(N)/k+1]M)
// [N/k](M₁ M₂) = ([N/k]M₁)([N/k]M₂)

fn substitute(expr: CoreExpr, target: usize, replacement: CoreExpr) -> CoreExpr {
    match expr {
        CoreExpr::Var(index) => {
            if index == target {
                replacement
            } else if index > target {
                CoreExpr::Var(index - 1)  // CORRECT: binder removed above
            } else {
                CoreExpr::Var(index)      // binder below, unchanged
            }
        }
        CoreExpr::Lam(body) => {
            // Lift free vars in replacement by 1 when going under binder
            let lifted = lift(replacement.clone(), 1, 0);
            CoreExpr::Lam(Box::new(substitute(*body, target + 1, lifted)))
        }
        CoreExpr::App(func, arg) => {
            CoreExpr::App(
                Box::new(substitute(*func, target, replacement.clone())),
                Box::new(substitute(*arg, target, replacement)),
            )
        }
    }
}
```

### **2. Lifting Implementation (Critical for Correctness)**
```rust
// ↑d(N) with cutoff c: increment free variables ≥ c by d
// This matches the formal FV(λM) = {k | k+1 ∈ FV(M)} property

fn lift(expr: CoreExpr, amount: usize, cutoff: usize) -> CoreExpr {
    match expr {
        CoreExpr::Var(index) => {
            if index >= cutoff {
                CoreExpr::Var(index + amount)
            } else {
                CoreExpr::Var(index)
            }
        }
        CoreExpr::Lam(body) => {
            // IMPORTANT: cutoff + 1 when going under lambda
            CoreExpr::Lam(Box::new(lift(*body, amount, cutoff + 1)))
        }
        CoreExpr::App(func, arg) => {
            CoreExpr::App(
                Box::new(lift(*func, amount, cutoff)),
                Box::new(lift(*arg, amount, cutoff)),
            )
        }
    }
}
```

### **3. Beta Reduction (Call-by-Value for Lisp)**
```rust
// For Lisp-like language, use call-by-value semantics:
// 1. Reduce function to WHNF
// 2. Reduce argument
// 3. Substitute

fn beta_reduce_cbv(expr: CoreExpr) -> CoreExpr {
    match expr {
        CoreExpr::App(func, arg) => {
            match beta_reduce_cbv(*func) {
                CoreExpr::Lam(body) => {
                    // Function is lambda, reduce argument first (call-by-value)
                    let reduced_arg = beta_reduce_cbv(*arg);
                    substitute(*body, 0, reduced_arg)
                }
                reduced_func => {
                    // Function not a lambda, return reduced application
                    CoreExpr::App(Box::new(reduced_func), arg)
                }
            }
        }
        CoreExpr::Lam(body) => {
            CoreExpr::Lam(Box::new(beta_reduce_cbv(*body)))
        }
        CoreExpr::Var(_) => expr,
    }
}
```

### **4. Alpha Equivalence (Fixed Version)**
```rust
// Two expressions are α-equivalent if they're identical after
// normalizing binder indices

fn alpha_equiv(a: &CoreExpr, b: &CoreExpr, env: Vec<(usize, usize)>) -> bool {
    match (a, b) {
        (CoreExpr::Var(i), CoreExpr::Var(j)) => {
            // Look up in environment or compare directly
            env.iter()
                .find(|(ai, _)| ai == i)
                .map(|(_, bj)| bj == j)
                .unwrap_or(i == j)
        }
        (CoreExpr::Lam(body_a), CoreExpr::Lam(body_b)) => {
            // Extend environment with mapping for new binder
            let new_depth = env.len();
            let mut new_env = env.clone();
            new_env.push((new_depth, new_depth));
            alpha_equiv(body_a, body_b, new_env)
        }
        (CoreExpr::App(f1, a1), CoreExpr::App(f2, a2)) => {
            alpha_equiv(f1, f2, env.clone()) && alpha_equiv(a1, a2, env)
        }
        _ => false,
    }
}
```

---

## **Common Rust Implementation Pitfalls**

### **1. Off-by-One Errors in Substitution**
```rust
// WRONG - doesn't account for binder removal
if index > target_index {
    CoreExpr::Var(index)  // Should be index - 1!
}

// CORRECT
if index > target_index {
    CoreExpr::Var(index - 1)  // Binder at target_index was removed
}
```

### **2. Incorrect Lifting in Lambda Case**
```rust
// WRONG - using same cutoff
CoreExpr::Lam(Box::new(lift(*body, amount, cutoff)))

// CORRECT - increment cutoff under lambda
CoreExpr::Lam(Box::new(lift(*body, amount, cutoff + 1)))
```

### **3. Forgetting to Clone in Recursive Calls**
```rust
// WRONG - moves replacement
Box::new(substitute(*func, target, replacement))

// CORRECT - clone for each branch
Box::new(substitute(*func, target, replacement.clone()))
```

---

## **Parser Integration Example**

```rust
// Convert named variables to De Bruijn indices during parsing
struct Parser {
    bindings: Vec<String>,  // Stack of bound variable names
}

impl Parser {
    fn parse_lambda(&mut self, param: &str, body: &str) -> CoreExpr {
        self.bindings.push(param.to_string());
        let body_expr = self.parse_expr(body);
        self.bindings.pop();
        CoreExpr::Lam(Box::new(body_expr))
    }
    
    fn parse_variable(&self, name: &str) -> CoreExpr {
        // Convert name to De Bruijn index
        match self.bindings.iter().rposition(|n| n == name) {
            Some(pos) => {
                // Bound variable: distance from top of stack
                let index = self.bindings.len() - 1 - pos;
                CoreExpr::Var(index)
            }
            None => {
                // Free variable - handle specially (maybe keep name)
                CoreExpr::FreeVar(name.to_string())
            }
        }
    }
}
```

---

## **Testing Critical Cases**

```rust
#[test]
fn test_substitution_correctness() {
    // (λx. λy. x) z → λy. z
    let expr = CoreExpr::App(
        Box::new(CoreExpr::Lam(Box::new(CoreExpr::Lam(Box::new(
            CoreExpr::Var(1)
        ))))),
        Box::new(CoreExpr::Var(0)),
    );
    
    let result = beta_reduce(expr);
    // Should be: λ.0 (z with index 0)
    assert_eq!(result, CoreExpr::Lam(Box::new(CoreExpr::Var(0))));
}

#[test]
fn test_shadowing() {
    // λx. (λx. x) x → λ0 (λ0 0) 0
    // After β-reduction: λ0
    let expr = CoreExpr::Lam(Box::new(CoreExpr::App(
        Box::new(CoreExpr::Lam(Box::new(CoreExpr::Var(0)))),
        Box::new(CoreExpr::Var(0)),
    )));
    
    let result = normalize(expr);
    assert_eq!(result, CoreExpr::Lam(Box::new(CoreExpr::Var(0))));
}
```

---

## **Performance Optimizations for Rust**

### **1. Use `Rc<CoreExpr>` for Sharing**
```rust
use std::rc::Rc;

enum CoreExpr {
    Var(usize),
    Lam(Rc<CoreExpr>),     // Share common subexpressions
    App(Rc<CoreExpr>, Rc<CoreExpr>),
}
```

### **2. Memoize Substitution Results**
```rust
use std::collections::HashMap;

struct SubstitutionCache {
    cache: HashMap<(usize, Rc<CoreExpr>), Rc<CoreExpr>>,
}

impl SubstitutionCache {
    fn substitute(&mut self, expr: Rc<CoreExpr>, target: usize, replacement: Rc<CoreExpr>) -> Rc<CoreExpr> {
        let key = (target, replacement.clone());
        if let Some(cached) = self.cache.get(&key) {
            return cached.clone();
        }
        // ... compute and cache
    }
}
```

### **3. Tail-Recursive Normalization**
```rust
fn normalize_iterative(mut expr: CoreExpr) -> CoreExpr {
    loop {
        let reduced = beta_reduce_once(expr.clone());
        if reduced == expr {
            return expr;
        }
        expr = reduced;
    }
}
```

---

## **Debugging Aids**

```rust
impl CoreExpr {
    fn to_named(&self, bindings: &[String]) -> String {
        match self {
            CoreExpr::Var(index) => {
                if *index < bindings.len() {
                    bindings[bindings.len() - 1 - index].clone()
                } else {
                    format!("[{}]", index)
                }
            }
            CoreExpr::Lam(body) => {
                let new_name = format!("x{}", bindings.len());
                let mut new_bindings = bindings.to_vec();
                new_bindings.push(new_name.clone());
                format!("λ{}. {}", new_name, body.to_named(&new_bindings))
            }
            CoreExpr::App(func, arg) => {
                format!("({} {})", func.to_named(bindings), arg.to_named(bindings))
            }
        }
    }
}
```

---

## **Key Takeaways for Rust Implementation**

1. **Follow formal rules exactly** - no shortcuts
2. **Pay attention to cutoff parameters** in lifting operations
3. **Clone strategically** - avoid unnecessary clones but don't move shared data
4. **Test edge cases** - shadowing, free variables, deep nesting
5. **Consider evaluation strategy** - call-by-value for Lisp, call-by-name for λ-calculus
6. **Use Rust's type system** to prevent invalid states
7. **Implement debugging views** to convert back to named variables

---

## **Quick Reference for Common Operations**

| Operation              | Implementation Pattern                                 |
| ---------------------- | ------------------------------------------------------ |
| **Shift free vars**    | `lift(expr, amount, cutoff)` with `cutoff + 1` under λ |
| **Substitute**         | Decrement indices > target, lift replacement under λ   |
| **Beta reduce**        | For CBV: reduce arg, then `substitute(body, 0, arg)`   |
| **Alpha compare**      | Compare with environment mapping indices               |
| **Parse to De Bruijn** | Maintain stack of bindings, count distance             |

This guide ensures your Rust implementation matches the formal λ-calculus semantics while leveraging Rust's safety features.



## ** Let me clarify how this applies to your Lisp-like language parser: **
## **Applicability for Lisp Parser: YES, with these adjustments**

### **1. Lisp vs. Pure λ-Calculus Differences**

| Aspect             | Pure λ-Calculus (Cheatsheet) | Lisp-like Language (Your Case)                          |
| ------------------ | ---------------------------- | ------------------------------------------------------- |
| **Evaluation**     | Often call-by-name           | Usually **call-by-value** (like Scheme) or call-by-need |
| **Data Types**     | Only functions               | Atoms, lists, numbers, booleans, etc.                   |
| **Special Forms**  | None                         | `if`, `cond`, `let`, `define`, `quote`, etc.            |
| **Variable Scope** | Lexical, De Bruijn           | Lexical, but may have dynamic aspects                   |

### **2. How to Adapt the Cheatsheet for Lisp**

#### **Core Representation Changes**
```rust
// Your AST will be richer than just λ-calculus
enum Expr {
    // Lambda calculus core
    Var(usize),                    // De Bruijn index
    Lam(Vec<String>, Box<Expr>),   // Parameters + body (names kept for debugging)
    App(Box<Expr>, Vec<Expr>),     // Function + multiple args (Lisp style)
    
    // Lisp-specific constructs
    Atom(String),
    Number(i64),
    Bool(bool),
    List(Vec<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),  // condition, then, else
    Let(Vec<(String, Expr)>, Box<Expr>),  // bindings, body
    Quote(Box<Expr>),
}
```

#### **Parsing Strategy: Two-Pass Approach**
```rust
// FIRST PASS: Parse to named representation
fn parse_to_named(source: &str) -> NamedExpr {
    // Returns something like:
    // NamedExpr::Lam(vec!["x", "y"], NamedExpr::App("+", vec!["x", "y"]))
}

// SECOND PASS: Convert to De Bruijn indices
fn to_de_bruijn(named: NamedExpr) -> Result<DeBruijnExpr, Error> {
    let mut converter = DeBruijnConverter::new();
    converter.convert(named)
}

struct DeBruijnConverter {
    env: Vec<String>,  // Stack of bound variable names
}

impl DeBruijnConverter {
    fn convert_lambda(&mut self, params: Vec<String>, body: NamedExpr) -> DeBruijnExpr {
        // Push parameters onto environment
        self.env.extend(params.clone());
        
        // Convert body (now can resolve param names to indices)
        let converted_body = self.convert(body);
        
        // Pop parameters
        for _ in params {
            self.env.pop();
        }
        
        DeBruijnExpr::Lam(params, Box::new(converted_body))
    }
    
    fn convert_variable(&self, name: &str) -> DeBruijnExpr {
        // Find name in environment, counting from top
        match self.env.iter().rposition(|n| n == name) {
            Some(index) => {
                // Distance from top = De Bruijn index
                DeBruijnExpr::Var(self.env.len() - 1 - index)
            }
            None => DeBruijnExpr::FreeVar(name.to_string()),
        }
    }
}
```

### **3. Which Parts of the Cheatsheet You NEED**

#### **CRITICAL (Must implement exactly)**
1. **Substitution rules** - For β-reduction in function application
2. **Lifting/shifting** - For substitution under lambdas
3. **Index resolution** - Converting names to indices during parsing

#### **LESS CRITICAL (Can simplify for Lisp)**
1. **Alpha equivalence** - Useful but not strictly needed for evaluation
2. **η-reduction** - Nice optimization but not required
3. **Complex macro hygiene** - Unless you're implementing advanced macros

### **4. Lisp-Specific Simplifications**

#### **Call-by-Value Evaluation (Like Scheme)**
```rust
// Simplified - no need for full normal-order reduction
fn eval(expr: &Expr, env: &Environment) -> Result<Value, Error> {
    match expr {
        Expr::Var(index) => env.lookup(*index),
        Expr::Lam(params, body) => Ok(Value::Closure(params.clone(), body.clone(), env.clone())),
        Expr::App(func, args) => {
            // 1. Evaluate function (call-by-value)
            let func_val = eval(func, env)?;
            
            // 2. Evaluate all arguments (call-by-value)
            let arg_vals = args.iter()
                .map(|arg| eval(arg, env))
                .collect::<Result<Vec<_>, _>>()?;
            
            // 3. Apply function to arguments
            apply(func_val, &arg_vals)
        }
        Expr::If(cond, then, els) => {
            match eval(cond, env)? {
                Value::Bool(true) => eval(then, env),
                Value::Bool(false) => eval(els, env),
                _ => Err(Error::TypeError),
            }
        }
        // ... other cases
    }
}
```

#### **Let as Sugar (Not Primitive)**
```rust
// Desugar during parsing or conversion phase
// let ((x 1) (y 2)) body
// becomes
// ((lambda (x y) body) 1 2)

fn desugar_let(bindings: Vec<(String, Expr)>, body: Expr) -> Expr {
    let params: Vec<String> = bindings.iter().map(|(name, _)| name.clone()).collect();
    let args: Vec<Expr> = bindings.into_iter().map(|(_, expr)| expr).collect();
    
    Expr::App(
        Box::new(Expr::Lam(params, Box::new(body))),
        args,
    )
}
```

### **5. Practical Implementation Order**

**Phase 1: Basic Parser** (Weeks 1-2)
```rust
// 1. Parse to named AST
// 2. Simple interpreter with named variables (hash map environment)
// 3. No De Bruijn yet - get the language working first
```

**Phase 2: Add De Bruijn** (Week 3)
```rust
// 1. Add conversion pass from named to De Bruijn
// 2. Update interpreter to use De Bruijn indices
// 3. Implement substitution and β-reduction
```

**Phase 3: Optimizations** (Week 4+)
```rust
// 1. Add α-equivalence for optimization
// 2. Implement η-reduction
// 3. Add compile-time evaluation
```

### **6. Which Formal Rules Matter Most for Lisp**

From the cheatsheet, focus on:

1. **Substitution Rule 3** (for lambdas) - Most critical for correctness
   ```
   [N/k](λM) = λ([↑(N)/k+1]M)
   ```

2. **Free Variable Rule** - For understanding when to lift
   ```
   FV(λM) = {k | k+1 ∈ FV(M)}
   ```

3. **β-reduction** - But simplified for call-by-value
   ```
   (λM) N →β [N/0]M  (but only after evaluating N to a value)
   ```

### **7. Example: Lisp `lambda` to De Bruijn**

```lisp
; Lisp code
(lambda (x y) 
  ((lambda (z) (+ x z)) 
   (* y 2)))

; After parsing to named AST
NamedExpr::Lam(
  vec!["x", "y"],
  NamedExpr::App(
    NamedExpr::Lam(vec!["z"], NamedExpr::App("+", vec!["x", "z"])),
    vec![NamedExpr::App("*", vec!["y", 2])]
  )
)

; After De Bruijn conversion
DeBruijnExpr::Lam(
  vec!["x", "y"],  // Keep names for debugging
  Box::new(DeBruijnExpr::App(
    Box::new(DeBruijnExpr::Lam(
      vec!["z"],
      Box::new(DeBruijnExpr::App(
        Box::new(DeBruijnExpr::FreeVar("+")),
        vec![DeBruijnExpr::Var(1), DeBruijnExpr::Var(0)]  // x=1, z=0
      ))
    )),
    vec![DeBruijnExpr::App(
      Box::new(DeBruijnExpr::FreeVar("*")),
      vec![DeBruijnExpr::Var(0), DeBruijnExpr::Number(2)]  // y=0
    )]
  ))
)
```

### **8. Recommendation: Start Simple**

1. **Don't implement full cheatsheet immediately** - Start with Phase 1 above
2. **Add De Bruijn incrementally** - Once named interpreter works
3. **Use the cheatsheet as reference** for tricky cases (nested lambdas, shadowing)
4. **Test thoroughly** with Lisp examples, not just λ-calculus

**Bottom line:** The cheatsheet provides the theoretical foundation, but your implementation will be simpler because:
- Lisp is call-by-value
- You have richer data types
- You can keep variable names for debugging
- Many cases (atoms, numbers) don't need De Bruijn treatment

**Use the cheatsheet for:** 
- Understanding how substitution works
- Implementing β-reduction correctly
- Handling edge cases (shadowing, free variables)
- Macro hygiene if you add macros later

**Simplify for:** 
- Evaluation strategy (call-by-value)
- Data types (atoms, lists don't need De Bruijn)
- Special forms (transform to λ-calculus where possible)