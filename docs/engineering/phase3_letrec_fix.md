# Phase 3: Fix Letrec Compilation for Recursive Lambdas

## Overview

This document details the implementation plan for fixing recursive function support using `letrec` bindings. The issue is that tests like `(let ((fact (lambda (n) ...))) (fact 5))` fail because variables bound in `let` are not visible within their own value expressions.

## Current State Analysis

### Let vs Letrec Semantics

**`let` binding** (current implementation):
```jue
(let ((fact (lambda (n) (if (<= n 1) 1 (* n (fact (- n 1)))))))
  (fact 5))
```
- Bindings are evaluated sequentially
- `fact` is NOT visible inside `(lambda (n) ...)` when it's being defined
- **Result:** "Variable not found: fact"

**`letrec` binding** (required for recursion):
```jue
(letrec ((fact (lambda (n) (if (<= n 1) 1 (* n (fact (- n 1)))))))
  (fact 5))
```
- All bindings are visible within all value expressions
- Enables mutual recursion
- **Result:** Works correctly

### Current Implementation

**File:** `jue_world/src/physics_integration/physics_compiler.rs` (lines 286-324)

```rust
pub fn compile_letrec(
    &mut self,
    bindings: &[(String, AstNode)],
    body: &AstNode,
    in_tail_position: bool,
) -> Result<Vec<OpCode>, CompilationError> {
    let mut bytecode = Vec::new();

    // Create new environment scope
    self.environment.push_scope();

    // First, register all binding names (so they're visible in the values)
    for (name, _value) in bindings {
        self.environment.add_variable(name.clone(), 0);  // Pre-register
    }

    // Now compile each binding
    for (name, value) in bindings {
        let value_bytecode = self.compile_to_physics_with_tail_context(value, false)?;
        bytecode.extend(value_bytecode);

        if let Some(index) = self.environment.get_variable_index(name) {
            bytecode.push(OpCode::SetLocal(index as u16));  // Store result
        }
    }

    // Compile body
    let body_bytecode = self.compile_to_physics_with_tail_context(body, in_tail_position)?;
    bytecode.extend(body_bytecode);

    self.environment.pop_scope();
    Ok(bytecode)
}
```

### The Problem

The `compile_letrec` function **does** pre-register variables, which should enable recursion. However, there are likely issues with:

1. **SetLocal behavior:** When compiled without a call frame, `SetLocal` fails (requires `top_level_locals`)
2. **Lambda closure environment:** The lambda's captured environment may not include the `fact` variable
3. **Two-pass evaluation:** Lambda compilation happens before `fact` is stored

---

## Root Cause Analysis

### Issue 1: SetLocal Without Call Frame

The `SetLocal` opcode requires a call frame to calculate stack positions. In standalone execution (top-level), this fails.

**Current fix in place:** `top_level_locals` field in `VmState`

**Verification needed:** Does this fix work correctly for letrec?

### Issue 2: Lambda Closure Capture

When compiling:
```jue
(letrec ((fact (lambda (n) (fact ...))))
  (fact 5))
```

The lambda `(lambda (n) (fact ...))` needs to capture a reference to the `fact` variable that's being defined. The current implementation may not be capturing this correctly.

**Questions for Expert Feedback:**
1. Should the lambda capture a **reference** to the variable slot, or the **value** at the time of closure creation?
2. For recursion, the lambda needs to see the variable even before it's set - is this supported?

### Issue 3: Environment Timing

In `compile_letrec`:
1. Pre-register `fact` in environment → `fact` has index 0
2. Compile lambda body → lambda captures `fact` at index 0
3. Store lambda in `fact` slot via `SetLocal`

The issue: When the lambda body is compiled, does it see `fact` in the environment? Yes, because we pre-register. But does it capture the **correct** reference?

---

## Implementation Options

### Option A: Fix SetLocal/GetLocal for Top-Level

**Approach:** Ensure `top_level_locals` is properly used when no call frame exists.

**Pros:**
- Simple fix
- Solves immediate problem

**Cons:**
- May not address closure capture issues

### Option B: Fix Lambda Environment Capture

**Approach:** Modify lambda compilation to correctly capture variables from the enclosing `letrec` scope.

**Pros:**
- Addresses the real issue
- Better long-term solution

**Cons:**
- More complex
- Requires careful testing

### Option C: Two-Pass Compilation

**Approach:** For letrec, compile all lambdas first, then store them.

**Pros:**
- Clear separation
- Easier to debug

**Cons:**
- Changes compilation model
- May have edge cases

---

## Implementation Plan

### Step 1: Debug Current Letrec Behavior

Add detailed logging to understand what's happening:

```rust
pub fn compile_letrec(...) -> Result<Vec<OpCode>, CompilationError> {
    println!("=== compile_letrec ===");
    println!("Bindings: {:?}", bindings.len());
    
    self.environment.push_scope();
    
    // Register all names
    for (name, _) in bindings {
        let idx = self.environment.add_variable(name.clone(), 0);
        println!("Registered '{}' at index {}", name, idx);
    }
    
    // Compile each binding
    for (name, value) in bindings {
        let idx = self.environment.get_variable_index(name);
        println!("Compiling value for '{}' (index: {:?})", name, idx);
        
        let value_bytecode = self.compile_to_physics_with_tail_context(value, false)?;
        println!("  Generated {} opcodes", value_bytecode.len());
        
        bytecode.extend(value_bytecode);
        
        if let Some(i) = idx {
            bytecode.push(OpCode::SetLocal(i as u16));
            println!("  Added SetLocal({})", i);
        }
    }
    
    // ... rest of function
}
```

**Questions for Expert Feedback:**
1. Should we add verbose debugging output, or use a logging framework?
2. What specific bytecode patterns should we look for?

### Step 2: Verify Environment Visibility

Create a minimal test case to isolate the issue:

```jue
;; Minimal test - should work
(letrec ((x 42))
  x)

;; Lambda test - currently fails
(letrec ((f (lambda () 42)))
  (f))
```

**Expected behavior:** Both should return their respective values.

### Step 3: Fix SetLocal Handling

Ensure `handle_set_local` in the VM properly handles top-level execution:

```rust
// In physics_world/src/vm/opcodes/stack_ops.rs
pub fn handle_set_local(&mut self, index: u16) -> VmResult<()> {
    if let Some(frame) = self.call_stack.last_mut() {
        // Normal case: use frame-relative addressing
        let stack_idx = frame.stack_start + index as usize;
        let value = self.stack.pop()
            .ok_or(VmError::StackUnderflow)?;
        self.stack[stack_idx] = value;
    } else {
        // Top-level case: use top_level_locals
        let value = self.stack.pop()
            .ok_or(VmError::StackUnderflow)?;
        self.top_level_locals[index as usize] = Some(value);
    }
    Ok(())
}
```

**Verification needed:** Does `top_level_locals` have the right size?

### Step 4: Fix GetLocal Handling

Similarly ensure `GetLocal` works for top-level:

```rust
pub fn handle_get_local(&mut self, index: u16) -> VmResult<()> {
    let value = if let Some(frame) = self.call_stack.last_mut() {
        let stack_idx = frame.stack_start + index as usize;
        self.stack[stack_idx].clone()
    } else {
        // Top-level case
        self.top_level_locals[index as usize]
            .clone()
            .ok_or(VmError::UndefinedVariable)?
    };
    self.stack.push(value);
    Ok(())
}
```

---

## Test Plan

### Tests to Unignore

From `test_recursive_function_execution.rs`:
- `test_factorial_recursion_*` (4 tests)
- `test_fibonacci_recursion`
- `test_recursive_with_conditional_logic`
- `test_recursive_power_function`
- `test_nested_recursive_functions`
- `test_mutual_recursion_even_odd`

From `test_simple_recursion.rs`:
- `test_simple_recursive_base_case`
- `test_single_recursion_step`

### Test Cases to Add

```rust
#[test]
fn test_letrec_simple_value() {
    let source = r#"
        (letrec ((x 42))
          x)
    "#;
    let ast = parse(source).unwrap();
    let (bytecode, _) = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
    
    let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
    assert_eq!(vm.run().unwrap(), Value::Int(42));
}

#[test]
fn test_letrec_simple_lambda() {
    let source = r#"
        (letrec ((f (lambda () 42)))
          (f))
    "#;
    // ... should return Int(42)
}

#[test]
fn test_letrec_self_recursion() {
    let source = r#"
        (letrec ((fact (lambda (n)
                          (if (<= n 1)
                              1
                              (* n (fact (- n 1)))))))
          (fact 5))
    "#;
    // ... should return Int(120)
}

#[test]
fn test_letrec_mutual_recursion() {
    let source = r#"
        (letrec ((even (lambda (n) (if (= n 0) true (odd (- n 1)))))
                  (odd (lambda (n) (if (= n 0) false (even (- n 1))))))
          (even 4))
    "#;
    // ... should return Bool(true)
}
```

---

## Known Issues and Edge Cases

### Edge Case 1: Empty Letrec
```jue
(letrec () body)
```
Should compile to just `body`.

### Edge Case 2: Non-Lambda Values
```jue
(letrec ((x 42) (y x)) y)
```
Should `y` see `x`? In standard Scheme, yes. This requires all names to be registered before any values are compiled.

### Edge Case 3: Nested Letrec
```jue
(letrec ((a (letrec ((b 1)) b))) a)
```
Should work correctly.

### Edge Case 4: Recursive Value (Not Lambda)
```jue
(letrec ((x (+ x 1))) x)
```
Infinite loop - acceptable to either fail compilation or hang at runtime.

---

## Open Questions for Expert Feedback

1. **Value Visibility:** Should all letrec-bound variables be visible to all value expressions, or only to subsequent expressions?

2. **Type of Recursion:** Should we support:
   - Self-recursion only (simpler)
   - Mutual recursion (more powerful but complex)

3. **Error Handling:** If a letrec binding references an undefined variable:
   - Compile-time error?
   - Runtime error?
   - Silent undefined behavior?

4. **Performance:** Should we optimize non-recursive letrec to regular let?

5. **Closure Semantics:** When a lambda captures a letrec variable:
   - Should it capture the variable slot (allows mutation)?
   - Should it capture the current value (simpler, but breaks recursion)?
   - Should it capture a reference that updates?

---

## Success Criteria

- [ ] `(letrec ((x 42)) x)` returns `42`
- [ ] `(letrec ((f (lambda () 42))) (f))` returns `42`
- [ ] `(letrec ((fact (lambda (n) ...))) (fact 5))` works for factorial
- [ ] Mutual recursion `(even/odd)` works
- [ ] All 18 currently ignored letrec tests pass
- [ ] No regressions in existing tests

---

## Dependencies and Risks

### Dependencies
1. `top_level_locals` fix in VM (already partially done)
2. SetLocal/GetLocal handlers working correctly
3. Lambda closure capture working correctly

### Risks
1. **Closure Capture Bug:** If lambdas don't capture letrec variables correctly, recursion will fail
2. **VM Integration:** Changes to VM opcodes may have unintended side effects
3. **Test Coverage:** Some edge cases may not be covered by existing tests

---

## Timeline Estimate

| Task                          | Effort        | Risk   |
| ----------------------------- | ------------- | ------ |
| Debug current letrec behavior | 1 hour        | Low    |
| Verify SetLocal/GetLocal fix  | 1 hour        | Low    |
| Fix lambda closure capture    | 2 hours       | Medium |
| Test all cases                | 1 hour        | Low    |
| Unignore tests                | 30 min        | Low    |
| **Total**                     | **5.5 hours** | -      |