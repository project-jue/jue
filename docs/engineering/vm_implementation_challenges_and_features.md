# VM Implementation Challenges and Lisp Language Features

## Executive Summary

This document captures the current challenges in implementing a VM for a Lisp-like language and enumerates the features required for a complete Lisp runtime.

---

## Part 1: Current Implementation Challenges

### 1.1 The Core Problem: Stack Management

The fundamental challenge is managing the interaction between:
- **Stack-based argument passing** (values pushed before Call)
- **Locals-based access** (GetLocal/SetLocal for function arguments)
- **Call frame preservation** (restoring caller state after return)

#### Current Stack Layout

```
Main program (top-level):
Stack: [arg1, arg2, closure]  ← Call(2) pops closure, leaves [arg1, arg2]
                                GetLocal(0) reads arg2 (stack top)
                                GetLocal(1) reads arg1

After Call:
Call frame created with:
  - stack_start = 1 (points to arg1)
  - locals = [arg2, arg1] (args in order: last-to-first)
```

#### The Asymmetry Problem

| Operation     | Index | Accesses                |
| ------------- | ----- | ----------------------- |
| `GetLocal(0)` | 0     | Last argument pushed    |
| `GetLocal(1)` | 1     | Second-to-last argument |

This means `stack_start + GetLocal(0)` gives the **last** argument, not the first.

### 1.2 Known Issues with Current Implementation

#### Issue 1: Frame Preservation Across Recursive Calls

**Problem**: When function A calls function B recursively, the return address and stack state from the outer A call are lost.

**Trace of broken behavior**:
```
Call depth 1: frame A pushed (return_ip=10, stack_start=0)
Call depth 2: frame B pushed (return_ip=20, stack_start=0)
  - Ret pops frame B
  - Truncates to stack_start=0 (removes frame A's state!)
  - Returns to return_ip=20 (stale)
```

**Root cause**: `original_stack_size` is not being computed correctly to preserve the entire call chain.

#### Issue 2: Tail Call Optimization (TCO)

**Current state**: TCO is disabled (`let is_tail_recursive = false`) because:
- All closures use `code_index=0`
- Every recursive call appears tail-recursive
- This would incorrectly reuse frames

**Required fix**: Unique `code_index` per closure definition, not per call site.

#### Issue 3: Locals Restoration Logic

**Problem**: The restoration logic in `handle_ret` attempts to detect if values are "already preserved":

```rust
let already_preserved = if i < vm.stack.len() {
    vm.stack[i] == local
} else {
    false
};
```

This comparison fails when the same logical value is represented differently (e.g., different HeapPtr for equivalent closures).

### 1.3 Debugging Questions for an Expert

#### Q1: Frame Preservation Model
```
For nested calls like: main → f → f → f
What should the call_stack look like after 3 Call instructions?
What should happen on each Ret?
```

#### Q2: Stack Truncation Point
```
Frame has: stack_start=2, original_stack_size=?
If caller pushed [a, b] then called with args [x, y]:
Stack before Call: [a, b, x, y, closure]
Stack after Call pops: [a, b, x, y]
What should original_stack_size be to preserve [a, b]?
```

#### Q3: Locals vs Stack Values
```
If GetLocal(0) reads from vm.stack[stack_start + 0]:
- Should locals be stored separately?
- Or should locals be "aliased" to stack positions?
- What happens when SetLocal modifies a local?
```

#### Q4: Return Value Position
```
After function executes, return value is on stack at position stack_start?
Or should it be pushed after truncation?
```

### 1.4 Test Case Analysis

#### test_deep_call_stack (Tail-recursive factorial)

```lisp
;; Expected: fact(3, 1) = 6
;; Implementation: fact(n, acc) = if n==0 then acc else fact(n-1, n*acc)
```

**Bytecode**:
```
0: GetLocal(0)   ;; n
1: Int(0)
2: Eq
3: JmpIfFalse(6) ;; if n != 0, jump to recursive case
4: GetLocal(1)   ;; acc - base case return
5: Ret
6: GetLocal(0)   ;; n - recursive case
7: Int(1)
8: Sub           ;; n - 1
9: GetLocal(0)   ;; n
10: GetLocal(1)  ;; acc
11: Mul          ;; n * acc
12: MakeClosure(0, 0)
13: Call(2)      ;; fact(n-1, n*acc)
14: Ret
```

**Current failure**: ArithmeticOverflow at depth 10
- Recursion is working (reaches depth 10)
- But stack grows unboundedly (no TCO)
- Integer overflow on large multiplication

---

## Part 2: Required Features for Lisp-Like Language

### 2.1 Core Runtime Features

| Feature                | Status  | Priority |
| ---------------------- | ------- | -------- |
| Function calls/returns | Partial | Critical |
| Tail Call Optimization | Missing | Critical |
| Closures (lambdas)     | Partial | Critical |
| Variable capture       | Missing | High     |
| Proper tail recursion  | Missing | Critical |
| Continuations          | Missing | Medium   |

### 2.2 Homoiconicity Features

**Definition**: Code is data; the program representation can be manipulated as data.

| Feature     | Description             | Implementation         |
| ----------- | ----------------------- | ---------------------- |
| Quote       | Prevent evaluation      | `'(+ 1 2)` → `(+ 1 2)` |
| Quasi-quote | Template with escape    | `` `(a ,x b) ``        |
| Unquote     | Insert value into quote | `` ,x ``               |
| Eval        | Evaluate data as code   | `(eval ast)`           |
| Apply       | Apply function to args  | `(apply fn args)`      |
| Read        | Parse text to AST       | Reader/parser          |
| Print       | AST to text             | Printer                |

### 2.3 Lisp-Specific VM Features

#### Dynamic Scope vs Lexical Scope

```lisp
;; Lexical scope (standard)
(let ((x 1))
  (let ((f (lambda () x)))
    (let ((x 2)) (f))))  ;; Returns 1 (captures outer x)

;; Dynamic scope (rare, for reference)
(let ((x 1))
  (let ((f (lambda () x)))
    (let ((x 2)) (f))))  ;; Returns 2 (uses current x)
```

**VM requirement**: Closures must carry their defining environment.

#### Proper Tail Recursion (Required for Lisp)

Lisp systems must support unbounded recursion:

```lisp
;; This must not overflow stack
(define (loop i)
  (if (> i 1000000)
      i
      (loop (+ i 1))))
(loop 0)
```

**Solutions**:
1. **TCO**: Reuse call frame for tail calls
2. **Trampoline**: Convert to loop in host language
3. **Continuation-passing style (CPS)**: Transform to iteration

### 2.4 Memory Management

| Feature            | Status  | Notes                    |
| ------------------ | ------- | ------------------------ |
| Garbage collection | Missing | Required for Lisp        |
| Generational GC    | Missing | Performance optimization |
| Weak references    | Missing | For caches, cycles       |

### 2.5 Error Handling

| Feature          | Status  |
| ---------------- | ------- |
| Condition system | Missing |
| Restarts         | Missing |
| Stack unwinding  | Partial |

---

## Part 3: Proposed Path Forward

### Phase 1: Fix the Fundamentals

1. **Complete basic function calls**
   - Fix frame preservation
   - Verify all closure tests pass
   - Document the calling convention

2. **Implement TCO**
   - Add unique `code_index` to closures
   - Enable tail call detection
   - Frame reuse for tail calls

### Phase 2: Complete Lisp Features

3. **Add homoiconicity primitives**
   - Quote/eval infrastructure
   - AST data type
   - Reader implementation

4. **Add GC**
   - Mark-and-sweep for simplicity
   - Reference counting for cycles

### Phase 3: Advanced Features

5. **Continuations** (if needed)
   - First-class control flow
   - Call/cc implementation

---

## Appendix A: Glossary

| Term          | Definition                                                            |
| ------------- | --------------------------------------------------------------------- |
| TCO           | Tail Call Optimization - reusing stack frame for tail-recursive calls |
| Homoiconicity | Code and data have same representation                                |
| Closure       | Function plus its captured environment                                |
| Lexical scope | Variable binding determined by source structure                       |
| Dynamic scope | Variable binding determined by call chain                             |

## Appendix B: References

- [Scheme R7RS](https://small.r7rs.org/attachment/r7rs.pdf) - Standard for Lisp with proper tail recursion
- [Lisp VM Patterns](https://stackoverflow.com/questions/6098273/implementing-a-lisp-virtual-machine) - Stack-based VM patterns
- [Tail Recursion in VMs](https://blog.josephmorag.com/posts/tco-vm/) - TCO implementation strategies