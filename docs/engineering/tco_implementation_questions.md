# TCO Implementation: Detailed Questions for Expert

This document catalogs all ambiguities, open questions, and decisions that need expert guidance for implementing Tail Call Optimization (TCO) in the Physics World VM.

---

## Question 1: TailCall Opcode Design

### Current State
- We have `Call(arg_count)` that creates a new frame
- Arguments are copied from stack to `frame.locals[0..arg_count]`
- Return address stored on call stack

### Options Under Consideration

**Option A: Dedicated TailCall Opcode**
```rust
OpCode::TailCall(arg_count)
```
- Detects tail position at compile time
- Compiler emits `TailCall` instead of `Call`
- VM reuses current frame instead of creating new one

**Option B: Call with Tail Flag**
```rust
OpCode::Call(arg_count, is_tail: bool)
```
- Single opcode with flag
- Slightly larger bytecode but simpler encoding

**Option C: Zero-Overhead Call Detection**
```rust
OpCode::Call(arg_count)
// If next instruction is Ret, optimize at runtime
```
- No compiler changes needed
- Runtime overhead to check
- Less precise (can't distinguish non-tail calls)

### Specific Questions

1. **Should TailCall be a separate opcode or a flag on Call?**
2. **Should TailCall require the same argument count as the current frame?**
3. **Should TailCall validate that we're calling the same function (self-recursion only)?**
4. **What happens if TailCall argument count differs from caller's parameter count?**

---

## Question 2: Frame Reuse Semantics

### Current Frame Structure (from call_state.rs)
```rust
struct CallFrame {
    ip: usize,              // Instruction pointer
    locals: Vec<Option<Value>>,  // Local variables
    closure_env: Option<ClosureEnvironment>,
    return_ip: usize,       // Where to return after call
}
```

### Frame Reuse Strategy

**Approach 1: Overwrite Locals**
```rust
fn handle_tail_call(vm: &mut VmState, arg_count: usize) {
    let frame = vm.frames.last_mut().unwrap();
    
    // Pop args from stack
    let args: Vec<Value> = (0..arg_count)
        .map(|_| vm.stack.pop().unwrap())
        .collect();
    
    // Reverse to get correct order (first arg at index 0)
    args.iter().rev().enumerate().for_each(|(i, arg)| {
        frame.locals[i] = Some(arg.clone());
    });
    
    // Reset IP to function start
    frame.ip = 0;
    
    // Don't push return address - we're reusing the frame
}
```

**Approach 2: Truncate and Reset**
```rust
fn handle_tail_call(vm: &mut VmState, arg_count: usize) {
    let frame = vm.frames.last_mut().unwrap();
    
    // Remove old locals
    frame.locals.clear();
    
    // Pop args and store directly
    for _ in 0..arg_count {
        let arg = vm.stack.pop().unwrap();
        frame.locals.push(Some(arg));
    }
    
    // Reset to function start
    frame.ip = 0;
}
```

### Specific Questions

1. **Should we validate arg_count matches expected parameter count?**
2. **What happens to unused local variable slots? (Keep as None? Clear?)**
3. **Should we preserve the closure environment or update it?**
4. **Do we need to handle GC roots differently for reused frames?**

---

## Question 3: Stack Frame Isolation vs Mutation

### Current Behavior (Correct)
```jue
;; Each recursive call gets its own frame with independent locals
(let (count 0)
  (fn increment [] (set! count (+ count 1)) count)
  (increment)  ;; Returns 1
  (increment)) ;; Returns 1 (new frame, count reinitialized)
```

### What Users Might Expect
```jue
;; Accumulator pattern for tail recursion
(let (factorial 
      (fn [n acc]
        (if (= n 0) 
            acc
            (factorial (- n 1) (* n acc)))))
  (factorial 5 1))  ;; Returns 120
```

### Specific Questions

1. **Should accumulator variables work correctly in tail position?**
   - Current: Yes, because each frame gets fresh locals with new values
   - But SetLocal mutations don't persist across calls (expected behavior)

2. **Should SetLocal in tail position update the reused frame?**
   - Yes: `frame.locals[idx] = new_value`
   - This enables patterns like `(set! acc (+ acc n))` in tail position

3. **What about closure captures in tail position?**
   - Are they captured from the outer scope or from the reused frame?

---

## Question 4: Mutual Recursion vs Self-Recursion

### Definition
- **Self-recursion:** Function calls itself directly
- **Mutual recursion:** Function A calls Function B, which calls Function A

### TCO Rules from Expert Feedback
> "TCO should only be applied to same function (self-recursion)"
> "Two different functions calling each other should NOT share the same frame"

### Implementation Approaches

**Approach A: Function Identity Check**
```rust
fn handle_tail_call(vm: &mut VmState, arg_count: usize) {
    let target_closure = vm.stack.last().unwrap();
    
    let frame = vm.frames.last().unwrap();
    let current_closure = frame.closure_env.get_self();
    
    // Only reuse frame if same function
    if target_closure.code_index == current_closure.code_index {
        // Reuse frame
    } else {
        // Create new frame (not TCO)
        handle_normal_call(vm, arg_count);
    }
}
```

**Approach B: Compiler-Emitted TailCall**
```rust
// Compiler knows at emit time whether it's self-recursion
// Only emit TailCall for self-recursion
// Emit Call for mutual recursion
```

**Approach C: Always TCO (Scheme-style)**
```rust
// Always reuse frame, regardless of function identity
// This allows mutual recursion to work with O(1) stack
```

### Specific Questions

1. **Which approach do you recommend?**
2. **Should mutual recursion be optimized at all?**
3. **If we allow mutual recursion TCO, what happens to frame reuse?**
   - Each function would need its own frame
   - But tail calls could still reuse their respective frames

---

## Question 5: Stack Restoration on Return

### Current Behavior (Non-TCO)
```
Call: Push return address, Create new frame, Jump to function
Ret:  Pop return address, Restore previous frame, Jump to return address
```

### With TCO
```
TailCall: Overwrite current frame locals, Reset IP to 0, (no return address pushed)
Ret: Pop return address, Restore previous frame, Jump to return address
```

### Edge Cases

1. **What if TailCall happens but the function doesn't actually tail-recurse?**
   - e.g., `(fn [] (if true (tail-call) 42))` - tail-call not always executed
   - Frame reuse only happens when TailCall actually executes

2. **What about nested TailCalls?**
   - `f(x) → tail-call g(y) → tail-call h(z)`
   - Each TailCall reuses the same frame
   - Only one frame on the stack

3. **What happens if TailCall arguments depend on previous computation?**
   - Need to evaluate all args before overwriting frame
   - Standard stack discipline applies

### Specific Questions

1. **Should TailCall push a "marker" on the stack to detect TCO chain?**
2. **How do we handle the return after a TailCall chain?**
3. **Should Trampoline style be considered instead?**

---

## Question 6: Bytecode Representation

### Current OpCode enum
```rust
enum OpCode {
    // ... existing opcodes ...
    Call(u16),        // arg_count
    // ...
}
```

### Options for TailCall encoding

**Option A: New Opcode**
```rust
enum OpCode {
    TailCall(u16),    // arg_count
    // ...
}
```

**Option B: Call with Flag**
```rust
enum OpCode {
    Call { arg_count: u16, is_tail: bool },
    // ...
}
```

**Option C: Negative arg_count**
```rust
// Magic value: Call with negative count = tail call
Call(-5)  // Tail call with 5 args
```

### Specific Questions

1. **Which encoding is cleanest?**
2. **Should we maintain backwards compatibility with existing bytecode?**
3. **Should TailCall validate arg_count > 0?**

---

## Question 7: Compiler Integration

### What the Compiler Needs to Do

1. **Detect Tail Position**
   ```jue
   ;; Tail position: last expression in a body
   (if cond (tail-call) (other))
   ;; NOT tail position
   (if cond (other) (tail-call))
   ```

2. **Emit TailCall vs Call**
   ```rust
   match position {
       Tail => emit(OpCode::TailCall(arg_count)),
       NonTail => emit(OpCode::Call(arg_count)),
   }
   ```

3. **Handle Multiple Return Values**
   - Should TailCall work with multiple values?
   - Current: Single return value

### Specific Questions

1. **Should the compiler only emit TailCall for self-recursion?**
2. **How should we handle `begin` expressions in tail position?**
   ```jue
   (begin (side-effect) (tail-call))  ;; TailCall is in tail position
   ```
3. **Should we emit a warning if a function has a tail-callable recursion but we don't use it?**

---

## Question 8: Debugging and Diagnostics

### Challenges with TCO

1. **Stack Traces**
   - With TCO, the call stack doesn't grow
   - How to represent "virtual" stack frames in error messages?

2. **Tracing/Profiling**
   - How to count TCO'd calls separately from normal calls?

3. **Breakpoints**
   - Should breakpoints work inside TCO'd functions?
   - Should we show the "real" call chain or the "effective" call chain?

### Specific Questions

1. **Should we include TCO'd calls in stack traces?**
2. **How to handle "maximum recursion depth" errors with TCO?**
3. **Should we provide a debug flag to disable TCO?**

---

## Question 9: Interaction with Other Features

### Features That May Interact with TCO

1. **Exception Handling**
   - Current: No exceptions
   - Future: How do exceptions work with TCO?

2. **Garbage Collection**
   - Frame reuse affects GC root identification
   - Need to mark reused frame's locals as roots

3. **Closures and Environments**
   - Tail-called closure may need different environment
   - What if the closure captures change between calls?

4. **Resource Tracking**
   - CPU/memory limits per call
   - With TCO, should limits accumulate or reset?

### Specific Questions

1. **Should TCO be disabled if resource limits are set?**
2. **How does GC know to trace a reused frame?**
3. **Should we track TCO depth separately for debugging?**

---

## Question 10: Testing Strategy

### Test Cases Needed

1. **Basic Self-Recursion**
   ```jue
   (let (fact (fn [n] (if (= n 0) 1 (* n (fact (- n 1))))))
     (fact 10000))  ;; Should not stack overflow
   ```

2. **Mutual Recursion (Non-TCO)**
   ```jue
   (letrec ((even (fn [n] (if (= n 0) true (odd (- n 1)))))
            (odd (fn [n] (if (= n 0) false (even (- n 1))))))
     (even 10000))
   ```

3. **Accumulator Pattern**
   ```jue
   (let (fact (fn [n acc] (if (= n 0) acc (fact (- n 1) (* n acc)))))
     (fact 10000 1))
   ```

4. **Nested TailCalls**
   ```jue
   (let (a (fn [n] (if (= n 0) 0 (b (- n 1)))))
        (b (fn [n] (if (= n 0) 0 (a (- n 1))))))
     (a 10000))
   ```

### Specific Questions

1. **Should we have a test that verifies stack depth doesn't grow?**
2. **How to test that TCO is actually happening?**
3. **Should we test edge cases like zero-argument tail calls?**

---

## Summary of Priority Questions

### High Priority
1. Separate TailCall opcode vs Call with flag?
2. Frame reuse: overwrite locals vs clear and repopulate?
3. Self-recursion only vs mutual recursion TCO?
4. Compiler emit-time detection vs runtime detection?

### Medium Priority
5. Stack trace representation with TCO
6. Interaction with GC
7. Testing strategy for TCO verification

### Low Priority
8. Debug flags for TCO disabling
9. Exception handling with TCO
10. Resource limit behavior with TCO

---

## Related Files

- [`physics_world/src/vm/opcodes/call.rs`](physics_world/src/vm/opcodes/call.rs) - Call handling
- [`physics_world/src/vm/opcodes/ret.rs`](physics_world/src/vm/opcodes/ret.rs) - Return handling
- [`physics_world/src/vm/call_state.rs`](physics_world/src/vm/call_state.rs) - CallFrame structure
- [`physics_world/src/vm/state.rs`](physics_world/src/vm/state.rs) - VmState
- [`physics_world/tests/test_recursion_vm.rs`](physics_world/tests/test_recursion_vm.rs) - TCO tests (ignored)

---

Please provide guidance on these questions so we can implement TCO correctly and completely.

Answer:

# TCO Implementation Expert Guidance

Based on your detailed questions and the need for a Lisp-like language with proper tail recursion, here are expert recommendations grounded in Scheme/R7RS standards and practical VM implementation experience.

## Executive Summary

**Goal:** Implement Scheme-style proper tail calls, not just tail recursion optimization. This is a **semantic requirement** for Lisp-like languages, not just a performance optimization.

---

## 1. TailCall Opcode Design: Separate Opcode

**Recommendation:** Use a **dedicated `TailCall` opcode** with the same signature as `Call`.

```rust
enum OpCode {
    Call(u16),      // Normal call - push new frame
    TailCall(u16),  // Tail call - reuse/replace current frame
    // ...
}
```

**Why:**
- **Semantic clarity:** The VM knows at dispatch time it's a tail call without checking context
- **Compile-time detection:** The compiler knows which calls are in tail position
- **Scheme standard:** Proper tail calls are required, not optional optimizations
- **Debugging:** Easier to trace and understand bytecode

**What about mutual recursion?** TailCall should work for **any** call in tail position, not just self-recursion. Scheme requires proper tail calls for **all** tail calls.

---

## 2. Frame Reuse Semantics: Overwrite with Validation

**Recommendation:** Overwrite current frame's locals with new arguments, preserving other state.

```rust
fn handle_tail_call(vm: &mut VmState, arg_count: usize) {
    let frame = vm.frames.last_mut().unwrap();
    
    // Validate argument count matches function arity
    let expected_args = frame.closure.function.arity;
    if arg_count != expected_args {
        panic!("TailCall: expected {} args, got {}", expected_args, arg_count);
    }
    
    // Pop arguments from stack (reverse order)
    let mut new_args = Vec::with_capacity(arg_count);
    for _ in 0..arg_count {
        new_args.push(vm.stack.pop().unwrap());
    }
    new_args.reverse();
    
    // Overwrite locals with new arguments
    for (i, arg) in new_args.into_iter().enumerate() {
        if i < frame.locals.len() {
            frame.locals[i] = Some(arg);
        } else {
            frame.locals.push(Some(arg));
        }
    }
    
    // Reset IP to function start
    frame.ip = 0;
    
    // Clear any temporary stack values (function should leave stack clean)
    vm.stack.truncate(frame.stack_start);
}
```

**Key points:**
- **Validate arg_count:** The compiler should ensure this, but VM should check for safety
- **Don't clear unused locals:** They'll be overwritten if used, keeping `None` otherwise
- **Preserve closure environment:** The closure captures lexical scope at definition time
- **GC roots:** Reused frame remains a GC root; no special handling needed

---

## 3. Frame Isolation vs Mutation: Scheme Semantics

**Correct semantics for Scheme/Lisp:**

1. **Each call gets fresh bindings** for its parameters
2. **`SetLocal` modifies the current frame's binding**
3. **Tail calls get fresh bindings** (from new arguments), overwriting previous ones

**Example that must work:**
```scheme
;; Accumulator-style factorial
(define (factorial n acc)
  (if (zero? n)
      acc
      (factorial (- n 1) (* n acc))))  ; Tail call with new bindings for n and acc

(factorial 5 1)  ; → 120
```

**Example of what doesn't persist:**
```scheme
(define (counter)
  (let ((count 0))
    (lambda ()
      (set! count (+ count 1))
      count)))

(let ((c (counter)))
  (c)  ; → 1
  (c)) ; → 2, but this works because it's the SAME closure calling itself
```

**For your case:** `SetLocal` in tail position **does update the reused frame**, but that update is immediately overwritten by the tail call's new arguments.

---

## 4. Mutual Recursion vs Self-Recursion: Both Must Work

**This is critical:** Proper tail calls in Scheme apply to **ALL** calls in tail position, not just self-recursion.

```scheme
;; Mutual recursion MUST work with TCO
(define (even? n)
  (if (zero? n)
      #t
      (odd? (- n 1))))  ; Tail call to different function

(define (odd? n)
  (if (zero? n)
      #f
      (even? (- n 1))))  ; Tail call to different function

(even? 1000000)  ; Must not stack overflow
```

**Implementation approach:** When doing a tail call to a **different** function:

1. **Don't reuse** the current frame (different code, different locals layout)
2. **Replace** the current frame with a new frame for the target function
3. **This is still O(1) stack space** - we're replacing, not pushing

```rust
fn handle_tail_call(vm: &mut VmState, arg_count: usize) {
    // Check if calling same function
    let target_closure = vm.stack[vm.stack.len() - arg_count - 1].as_closure();
    let current_frame = vm.frames.last().unwrap();
    let current_closure = &current_frame.closure;
    
    if target_closure.function.code_index == current_closure.function.code_index {
        // Self-recursion: reuse frame (as above)
        reuse_current_frame(vm, arg_count);
    } else {
        // Mutual recursion: replace frame
        replace_current_frame(vm, arg_count);
    }
}
```

---

## 5. Stack Restoration: No Special Handling Needed

**Key insight:** With proper tail calls, there's **no return address to restore** because we never pushed one.

**Edge cases handled naturally:**
1. **Non-executed tail call:** Normal `Call`/`Ret` semantics apply
2. **Nested tail calls:** Each replaces the current frame
3. **Mixed tail/non-tail:** Normal stack semantics for non-tail calls

**What about the "marker"?** No marker needed. The call stack always represents the current continuation.

---

## 6. Bytecode Representation: Separate OpCode

**Final decision:** `TailCall(u16)` as separate opcode.

**Rationale:**
1. **Clean separation** of concerns
2. **Compiler knows** tail position at emit time
3. **VM doesn't need** to inspect next instruction
4. **Backwards compatibility:** New opcode won't break existing bytecode (it'll just fail to decode)

**Encoding considerations:** Use same encoding as `Call` (u16 for arg_count). The VM can share most validation logic.

---

## 7. Compiler Integration: Tail Position Detection

**What is tail position?** An expression is in tail position if its value is the **immediate result** of the function.

**Formal definition (simplified):**
- The last expression in a function body
- The last expression in each branch of `if`
- The last expression in `begin` blocks
- The last expression in `cond` clauses

**Compiler algorithm:**
```rust
fn compile_tail_position(expr: &Expr, env: &Env, tail: bool) -> Vec<OpCode> {
    match expr {
        Expr::If(cond, then_branch, else_branch) => {
            let mut code = compile_expr(cond, env, false);
            let then_code = compile_expr(then_branch, env, tail);
            let else_code = compile_expr(else_branch, env, tail);
            // ... emit JmpIfFalse with appropriate offsets
            code
        }
        Expr::Begin(exprs) => {
            let mut code = Vec::new();
            for (i, expr) in exprs.iter().enumerate() {
                let is_tail = tail && i == exprs.len() - 1;
                code.extend(compile_expr(expr, env, is_tail));
            }
            code
        }
        Expr::Call(func, args, _) if tail => {
            // Compile function and args (not in tail position)
            let mut code = compile_expr(func, env, false);
            for arg in args {
                code.extend(compile_expr(arg, env, false));
            }
            // Emit TailCall instead of Call
            code.push(OpCode::TailCall(args.len() as u16));
            code
        }
        // ... other cases
    }
}
```

---

## 8. Debugging and Diagnostics: Track Virtual Stack

**Problem:** With TCO, physical call stack ≠ logical call chain.

**Solution:** Maintain a **virtual stack** for debugging:

```rust
struct VirtualFrame {
    function_name: String,
    location: SourceLocation,
    tail_call_count: u64,  // How many tail calls happened in this frame
}

struct VmState {
    // ... existing fields
    virtual_stack: Vec<VirtualFrame>,  // For debugging only
}

fn handle_tail_call(vm: &mut VmState, arg_count: usize) {
    // Update virtual stack
    if is_self_recursion(vm) {
        // Increment tail call counter
        vm.virtual_stack.last_mut().unwrap().tail_call_count += 1;
    } else {
        // Replace top of virtual stack
        vm.virtual_stack.pop();
        vm.virtual_stack.push(VirtualFrame {
            function_name: target_function.name.clone(),
            location: current_source_location(),
            tail_call_count: 0,
        });
    }
    // ... rest of tail call implementation
}
```

**Debug flags:**
- `--no-tco`: Disable TCO for debugging
- `--trace-tail-calls`: Log each tail call
- `--max-tail-depth`: Safety limit (should be very high, e.g., 1,000,000)

---

## 9. Interaction with Other Features

### Garbage Collection
**No special handling needed.** The reused/replaced frame remains on the call stack, so it's still a GC root. The GC traces `frame.locals` as usual.

### Exceptions (Future)
With exceptions, tail calls become more complex. Common approach:
- Tail calls **cannot** be inside `try` blocks (in many implementations)
- Or: Exception handlers are part of the frame and must be preserved/replaced

### Resource Tracking
**Recommendation:** Count tail calls as separate operations for resource limits:
```rust
struct ResourceLimits {
    max_operations: u64,
    operations_count: u64,
    tail_call_count: u64,  // Track separately
}
```

---

## 10. Testing Strategy: Comprehensive Suite

**Essential tests:**

1. **Self-recursion with accumulator:**
   ```rust
   #[test]
   fn test_self_tail_recursion() {
       // factorial in tail position
       let code = compile("(define (fact n acc) (if (zero? n) acc (fact (- n 1) (* n acc))))");
       let result = vm.execute_with_args(code, &[10000, 1]);
       assert!(result.is_ok());  // No stack overflow
   }
   ```

2. **Mutual tail recursion:**
   ```rust
   #[test]
   fn test_mutual_tail_recursion() {
       // even?/odd? mutual recursion
       let code = compile("(define (even? n) (if (zero? n) true (odd? (- n 1)))) ...");
       let result = vm.execute_with_args(code, &[1000000]);
       assert!(result.is_ok());
   }
   ```

3. **Non-tail calls still work:**
   ```rust
   #[test]
   fn test_mixed_calls() {
       // Mix of tail and non-tail calls
       let code = compile("(define (f n) (if (< n 10) (+ n (f (+ n 1))) n))");
       let result = vm.execute_with_args(code, &[0]);
       assert_eq!(result, Ok(Value::Int(45)));
   }
   ```

4. **Stack depth verification:**
   ```rust
   #[test]
   fn test_constant_stack_depth() {
       let start_depth = vm.call_stack_depth();
       let result = vm.execute_with_args(tail_recursive_code, &[100000]);
       let end_depth = vm.call_stack_depth();
       assert_eq!(end_depth - start_depth, 0);  // No growth
   }
   ```

---

## Implementation Priority

### Phase 1: Self-recursion TCO (1-2 days)
1. Add `TailCall` opcode
2. Implement frame reuse for self-recursion
3. Basic compiler detection for tail position

### Phase 2: Mutual recursion (2-3 days)
1. Frame replacement for different functions
2. Update virtual stack for debugging
3. Comprehensive mutual recursion tests

### Phase 3: Polish (1-2 days)
1. Debug flags (`--no-tco`, `--trace-tail-calls`)
2. Resource accounting for tail calls
3. Documentation and examples

## Critical Design Decision

**You must decide:** Are you implementing Scheme-style **proper tail calls** (all calls in tail position) or just **tail recursion optimization** (self-recursion only)?

For a Lisp-like language, **proper tail calls are expected**. Go with the mutual recursion support from the start.

## Code Template

Here's a complete `handle_tail_call` implementation:

```rust
pub fn handle_tail_call(vm: &mut VmState, arg_count: u16) -> VmResult<()> {
    let arg_count = arg_count as usize;
    
    // Get target closure from stack
    let closure_pos = vm.stack.len().checked_sub(arg_count + 1)
        .ok_or(VmError::StackUnderflow)?;
    let closure_value = vm.stack[closure_pos].clone();
    let target_closure = closure_value.as_closure()
        .ok_or(VmError::TypeError("Expected closure"))?;
    
    // Get current frame
    let current_frame = vm.frames.last_mut()
        .ok_or(VmError::NoActiveFrame)?;
    
    // Check if same function
    let is_same_function = {
        let current_closure = &current_frame.closure;
        target_closure.function.code_index == current_closure.function.code_index
    };
    
    if is_same_function {
        // Self-recursion: reuse current frame
        reuse_frame_for_tail_call(vm, current_frame, arg_count)?;
    } else {
        // Mutual recursion: replace current frame
        replace_frame_for_tail_call(vm, arg_count)?;
    }
    
    Ok(())
}

fn reuse_frame_for_tail_call(
    vm: &mut VmState,
    frame: &mut CallFrame,
    arg_count: usize,
) -> VmResult<()> {
    // Pop arguments (they're in reverse order on stack)
    let mut args = Vec::with_capacity(arg_count);
    for _ in 0..arg_count {
        args.push(vm.stack.pop().unwrap());
    }
    args.reverse();
    
    // Ensure locals vector is large enough
    if frame.locals.len() < arg_count {
        frame.locals.resize(arg_count, None);
    }
    
    // Overwrite locals with new arguments
    for (i, arg) in args.into_iter().enumerate() {
        frame.locals[i] = Some(arg);
    }
    
    // Reset to start of function
    frame.ip = 0;
    
    // Clear any temporary values from stack
    vm.stack.truncate(frame.stack_start);
    
    Ok(())
}
```

This approach gives you proper Scheme-style tail calls while being practical to implement.