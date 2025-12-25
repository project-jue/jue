# TCO Implementation Plan (Based on Expert Guidance)

This document captures the expert recommendations and provides a concrete implementation plan for Tail Call Optimization (TCO) in the Physics World VM.

---

## Expert Guidance Summary

**Goal:** Implement Scheme-style **proper tail calls** (not just tail recursion optimization). This is a semantic requirement for Lisp-like languages.

**Key Decisions from Expert:**

1. **Separate `TailCall` opcode** (same signature as `Call`)
2. **Frame reuse with validation** - overwrite locals, validate arg_count
3. **Mutual recursion supported** - replace frame for different functions
4. **Self-recursion: reuse frame** | **Different function: replace frame**
5. **No special stack restoration** - proper tail calls don't push return address

---

## Implementation Phases

### Phase 1: TailCall Opcode and VM Support (1-2 days)

#### Step 1.1: Add TailCall OpCode

**File:** `physics_world/src/types/mod.rs` or wherever OpCode is defined

```rust
// Add to OpCode enum
pub enum OpCode {
    // ... existing opcodes ...
    Call(u16),        // Normal call - push new frame
    TailCall(u16),    // Tail call - reuse/replace current frame
    // ... other opcodes
}
```

#### Step 1.2: Implement handle_tail_call

**File:** `physics_world/src/vm/opcodes/call.rs` (add new function)

```rust
use super::super::{VmState, VmError, VmResult};

/// Handles the TailCall opcode - reuse or replace current frame
pub fn handle_tail_call(vm: &mut VmState, arg_count: u16) -> VmResult<()> {
    let arg_count = arg_count as usize;
    
    // Get target closure from stack
    let closure_pos = vm.stack.len().checked_sub(arg_count + 1)
        .ok_or(VmError::StackUnderflow)?;
    let closure_value = vm.stack[closure_pos].clone();
    let target_closure = closure_value.as_closure()
        .ok_or(VmError::TypeError("Expected closure for tail call"))?;
    
    // Get current frame
    let current_frame = vm.frames.last_mut()
        .ok_or(VmError::NoActiveFrame)?;
    
    // Check if same function (self-recursion) or different function (mutual recursion)
    let is_same_function = {
        let current_closure = &current_frame.closure;
        target_closure.function.code_index == current_closure.function.code_index
    };
    
    if is_same_function {
        // Self-recursion: reuse current frame
        reuse_frame_for_tail_call(vm, current_frame, arg_count)?;
    } else {
        // Mutual recursion: replace current frame
        replace_frame_for_tail_call(vm, arg_count, target_closure)?;
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

fn replace_frame_for_tail_call(
    vm: &mut VmState,
    arg_count: usize,
    target_closure: &Closure,
) -> VmResult<()> {
    // Pop arguments
    let mut args = Vec::with_capacity(arg_count);
    for _ in 0..arg_count {
        args.push(vm.stack.pop().unwrap());
    }
    args.reverse();
    
    // Create new frame for target function
    let new_frame = CallFrame {
        ip: 0,
        locals: args.into_iter().map(Some).collect(),
        closure_env: target_closure.env.clone(),
        closure: target_closure.clone(),
        return_ip: 0,  // No return - we're replacing
        stack_start: vm.stack.len(),  // Current stack position
    };
    
    // Replace current frame
    vm.frames.pop();
    vm.frames.push(new_frame);
    
    Ok(())
}
```

#### Step 1.3: Dispatch TailCall in VM

**File:** `physics_world/src/vm/execution.rs`

```rust
// In the instruction dispatch loop
match opcode {
    OpCode::Call(arg_count) => handle_call(vm, arg_count)?,
    OpCode::TailCall(arg_count) => handle_tail_call(vm, arg_count)?,
    // ... other opcodes
}
```

#### Step 1.4: Update tests to use TailCall

**File:** `physics_world/tests/test_recursion_vm.rs`

Remove `#[ignore]` from TCO tests and update bytecode:

```rust
#[test]
fn test_tail_recursion_no_stack_growth() {
    let bytecode = vec![
        OpCode::LoadString(0),
        OpCode::MakeClosure(0, 0),
        OpCode::SetLocal(0),
        OpCode::Int(100),
        OpCode::Int(0),
        OpCode::GetLocal(0),
        OpCode::TailCall(2),  // Changed from Call to TailCall
    ];
    
    let string_constants = vec![
        Value::String(
            "body:[GetLocal(0),Int(0),Eq,JmpIfFalse(2),GetLocal(1),Ret,GetLocal(0),Int(1),Sub,GetLocal(1),Int(1),Add,TailCall(2)]"
                .to_string())
    ];
    
    let mut vm = VmState::new(bytecode, string_constants, 1000, 1024, 1, 10000);
    let result = vm.run();
    assert!(result.is_ok(), "TCO should allow deep recursion");
}
```

---

### Phase 2: Compiler Integration (2-3 days)

#### Step 2.1: Tail Position Detection

**File:** `jue_world/src/compiler/` (or appropriate location)

```rust
/// Determines if an expression is in tail position
fn is_tail_position(expr: &Expr, in_tail_context: bool) -> bool {
    match expr {
        // Last expression in a sequence is in tail position
        Expr::Begin(exprs) => {
            exprs.last().map(|e| is_tail_position(e, in_tail_context)).unwrap_or(false)
        }
        // Last branch of if is in tail position
        Expr::If(cond, then_branch, else_branch) => {
            let then_tail = is_tail_position(then_branch, in_tail_context);
            let else_tail = is_tail_position(else_branch, in_tail_context);
            then_tail && else_tail  // Both branches must be tail
        }
        // Function call is in tail position if we're in tail context
        Expr::Call(_, _, _) => in_tail_context,
        // All other expressions are not tail positions
        _ => false,
    }
}
```

#### Step 2.2: Compile Tail Calls

```rust
fn compile_expr(expr: &Expr, env: &Env, tail: bool) -> Vec<OpCode> {
    match expr {
        Expr::Call(func, args, _) => {
            let mut code = Vec::new();
            
            // Compile function (not in tail position)
            code.extend(compile_expr(func, env, false));
            
            // Compile arguments (not in tail position)
            for arg in args {
                code.extend(compile_expr(arg, env, false));
            }
            
            // Emit Call or TailCall based on position
            if tail {
                code.push(OpCode::TailCall(args.len() as u16));
            } else {
                code.push(OpCode::Call(args.len() as u16));
            }
            
            code
        }
        // ... other expression types
    }
}
```

---

### Phase 3: Testing and Validation (1-2 days)

#### Step 3.1: Comprehensive Test Suite

```rust
// test_tco_implementation.rs

#[test]
fn test_self_tail_recursion_factorial() {
    // Test: factorial(10000, 1) should return 10000!
    // Without TCO: stack overflow
    // With TCO: completes successfully
    let bytecode = compile_jue("(define (fact n acc) (if (zero? n) acc (fact (- n 1) (* n acc))))");
    let mut vm = VmState::new(bytecode, vec![], 1000, 1024, 1, 1000000);
    let result = vm.run();
    assert!(result.is_ok());
}

#[test]
fn test_mutual_tail_recursion_even_odd() {
    // Test: even?(1000000) should return true
    // Without TCO: stack overflow  
    // With TCO: completes successfully
    let bytecode = compile_jue("(define (even? n) (if (zero? n) #t (odd? (- n 1))))");
    let mut vm = VmState::new(bytecode, vec![], 1000, 1024, 1, 1000000);
    let result = vm.run();
    assert!(result.is_ok());
}

#[test]
fn test_stack_depth_constant() {
    // Verify stack depth doesn't grow during tail recursion
    let bytecode = compile_jue("(define (countdown n) (if (zero? n) 0 (countdown (- n 1))))");
    let mut vm = VmState::new(bytecode, vec![], 1000, 1024, 1, 100000);
    
    let initial_depth = vm.call_stack.len();
    let result = vm.run();
    let final_depth = vm.call_stack.len();
    
    assert!(result.is_ok());
    assert_eq!(final_depth, initial_depth, "Stack should not grow with TCO");
}
```

---

## Files to Modify

| File                                       | Change                             | Priority |
| ------------------------------------------ | ---------------------------------- | -------- |
| `physics_world/src/types/mod.rs`           | Add `TailCall(u16)` to OpCode enum | High     |
| `physics_world/src/vm/opcodes/call.rs`     | Add `handle_tail_call` function    | High     |
| `physics_world/src/vm/execution.rs`        | Dispatch TailCall opcode           | High     |
| `physics_world/tests/test_recursion_vm.rs` | Remove `#[ignore]`, update tests   | High     |
| `jue_world/src/compiler/`                  | Add tail position detection        | Medium   |
| `jue_world/src/compiler/`                  | Emit TailCall in tail position     | Medium   |
| `physics_world/src/vm/state.rs`            | Add virtual stack for debugging    | Low      |

---

## Expected Behavior

### Before TCO
```
(fact 5 1)  ;; Stack: [main, fact(5), fact(4), fact(3), fact(2), fact(1)]
             ;; If n=10000: stack overflow
```

### After TCO (Self-Recursion)
```
(fact 10000 1)  ;; Stack: [main, fact(10000)]  (frame reused)
                 ;; No stack growth, completes successfully
```

### After TCO (Mutual Recursion)
```
(even? 1000000)  ;; Stack: [main, even?(1000000)]  (frame replaced)
                   ;; No stack growth, completes successfully
```

---

## Verification Checklist

- [ ] `TailCall` opcode added to OpCode enum
- [ ] `handle_tail_call` implemented with frame reuse/replacement
- [ ] VM dispatches TailCall correctly
- [ ] Self-recursion TCO works (factorial test passes)
- [ ] Mutual recursion TCO works (even?/odd? test passes)
- [ ] Stack depth doesn't grow during tail recursion
- [ ] Non-tail calls still work correctly
- [ ] Compiler emits TailCall in tail position
- [ ] Debug output shows tail call information

---

## Related Documentation

- Expert guidance: [`docs/engineering/tco_implementation_questions.md`](docs/engineering/tco_implementation_questions.md)
- Code review summary: [`docs/engineering/vm_code_review_summary.md`](docs/engineering/vm_code_review_summary.md)
- Recursion tests: [`physics_world/tests/test_recursion_vm.rs`](physics_world/tests/test_recursion_vm.rs)

---

## Implementation Notes

1. **Arg Count Validation:** The VM should validate that arg_count matches the function's arity
2. **Closure Environment:** Preserve the closure's lexical environment
3. **GC Compatibility:** No special handling needed - reused frame remains a GC root
4. **Debug Support:** Consider adding virtual stack tracking for debugging
5. **Resource Limits:** Tail calls should count toward operation limits