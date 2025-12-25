# TCO Frame Reuse Limitation Analysis

**Document Version:** 1.0  
**Date:** 2025-12-25  
**Author:** Code Analysis  
**Status:** Expert Guidance - Known Limitation

---

## 1. Current Implementation Summary

### 1.1 How `handle_tail_call` is Supposed to Work

The Physics VM implements Tail Call Optimization (TCO) through frame reuse in [`handle_tail_call`](physics_world/src/vm/opcodes/call.rs:239). The intended mechanism:

1. **Validate** the closure and recursion limits
2. **Extract** arguments from stack (without popping)
3. **Reuse** the existing call frame instead of creating a new one
4. **Replace** instructions with the new closure body
5. **Reset** instruction pointer to zero

The key design is in [`execute_tail_call_body`](physics_world/src/vm/opcodes/call.rs:314):

```rust
// Line 320: Calculate stack_start BEFORE popping
let stack_start = vm.stack.len() - arg_count as usize;

// Line 323: Pop the closure from the stack first
let _closure = vm.stack.pop().unwrap();

// Lines 327-332: Save COPY of arguments for locals
let args_start = vm.stack.len() - arg_count as usize;
let args: Vec<Value> = if args_start < vm.stack.len() {
    vm.stack[args_start..].to_vec()
} else {
    Vec::new()
};

// Lines 336-340: Reuse current call frame
if let Some(current_frame) = vm.call_stack.last_mut() {
    current_frame.is_tail_call = true;
    current_frame.stack_start = stack_start;
    current_frame.locals = args;  // Store arguments in frame.locals
}
```

The `CallFrame` structure stores locals separately from the value stack:

```rust
// From physics_world/src/vm/call_state.rs
pub struct CallFrame {
    pub locals: Vec<Value>,  // Arguments and local variables
    pub stack_start: usize,  // Position in value stack
    // ... other fields
}
```

### 1.2 How `SetLocal` and `GetLocal` Operate

[`GetLocal`](physics_world/src/vm/opcodes/stack_ops.rs:36) reads from `frame.locals`:

```rust
pub fn handle_get_local(vm: &mut VmState, offset: u16) -> Result<(), VmError> {
    let call_frame = vm.call_stack.last().ok_or(VmError::StackUnderflow)?;
    if offset_usize >= call_frame.locals.len() {
        return Err(VmError::StackUnderflow);
    }
    let value = call_frame.locals[offset_usize].clone();
    vm.stack.push(value);
    Ok(())
}
```

[`SetLocal`](physics_world/src/vm/opcodes/stack_ops.rs:52) pops from value stack and stores in `frame.locals`:

```rust
pub fn handle_set_local(vm: &mut VmState, offset: u16) -> Result<(), VmError> {
    if vm.stack.is_empty() {
        return Err(VmError::StackUnderflow);
    }
    let value = vm.stack.pop().unwrap();
    let call_frame = vm.call_stack.last_mut().ok_or(VmError::StackUnderflow)?;
    while offset_usize >= call_frame.locals.len() {
        call_frame.locals.push(Value::Nil);
    }
    call_frame.locals[offset_usize] = value;
    Ok(())
}
```

---

## 2. The Problem

### 2.1 Frame Reuse Mechanism Breaks Down

When `handle_tail_call` reuses a frame, it stores arguments in `frame.locals` and updates `frame.stack_start`. However, subsequent operations encounter issues:

1. **Stack Truncation**: After `Call`, the stack is truncated to remove arguments (see [`execute_closure_body`](physics_world/src/vm/opcodes/call.rs:164-167)):

   ```rust
   // Arguments are removed from value stack during normal Call
   vm.stack.truncate(original_stack_size);
   ```

2. **SetLocal Expects Stack Values**: `SetLocal` pops from the value stack (`vm.stack.pop().unwrap()`), but after the Call truncates the stack, the expected values are gone.

3. **TailCall Reuse Creates Mismatch**: When `handle_tail_call` reuses a frame, it updates `frame.locals` but the stack position (`frame.stack_start`) may be at a level where `SetLocal` expects to find values.

### 2.2 Concrete Failure Scenario

Consider a recursive factorial function:

```jue
(let ((fact (lambda (n)
  (if (eq n 0)
    1
    (* n (fact (- n 1)))))))
  (fact 5))
```

**Execution Flow (Simplified):**

1. Initial call: `fact(5)` pushes arguments onto stack
2. `handle_call` truncates stack, stores args in `frame.locals`
3. Recursive call: `TailCall(1)` with `n-1` on stack
4. `handle_tail_call` reuses frame, updates `frame.locals`
5. Function body executes `GetLocal(0)` - works, reads from `frame.locals`
6. Function body executes `SetLocal(0)` to bind new local - **FAILS**
   - `SetLocal` pops from `vm.stack`
   - Stack was truncated at step 2
   - Stack underflow or wrong value

---

## 3. Root Cause Analysis

### 3.1 Stack vs. Locals Dual Storage

The VM uses two storage locations for what should be unified:

| Operation  | Storage                     | Access Pattern             |
| ---------- | --------------------------- | -------------------------- |
| `GetLocal` | `frame.locals`              | Direct vector access       |
| `SetLocal` | `vm.stack` → `frame.locals` | Pop from stack, then write |
| Arguments  | `vm.stack` → `frame.locals` | Copied during Call         |

### 3.2 The Critical Mismatch

In [`execute_closure_body`](physics_world/src/vm/opcodes/call.rs:141-167):

```rust
// Step 1: Calculate original stack size
let original_stack_size = if vm.call_stack.is_empty() {
    vm.stack.len() - arg_count as usize
} else {
    vm.call_stack.last().unwrap().stack_start + vm.call_stack.last().unwrap().locals.len()
};

// Step 2: Copy arguments to locals
let args: Vec<Value> = vm.stack[args_start..].to_vec();

// Step 3: TRUNCATE stack (removes arguments from value stack)
vm.stack.truncate(original_stack_size);
```

After truncation:
- `frame.locals` contains the arguments ✓
- `vm.stack` is at `stack_start` position ✓
- **But `SetLocal` still expects to pop from `vm.stack`**

### 3.3 Why TCO Makes This Worse

For normal calls, the flow works because:
1. New frame created with fresh `locals`
2. Arguments copied from stack to `locals`
3. Stack truncated
4. No `SetLocal` expected until after function executes

For TCO calls, the flow breaks because:
1. **Same frame reused** with updated `locals`
2. **Same `stack_start`** (not a new frame's position)
3. `SetLocal` attempts to pop from stack expecting new values
4. **Stack is at old position** - missing new argument values

---

## 4. Solution Approaches

### Option A: Separate Local Stack

**Approach:** Keep locals in a dedicated stack per frame, decoupled from the value stack.

**Implementation:**
```rust
struct CallFrame {
    locals_stack: Vec<Value>,  // Separate stack for locals
    value_stack: Vec<Value>,   // Existing value stack
    // ... other fields
}

// SetLocal pushes to locals_stack instead of popping from value_stack
pub fn handle_set_local(vm: &mut VmState, offset: u16) -> Result<(), VmError> {
    let value = vm.stack.pop().unwrap();  // Still validate stack has value
    let call_frame = vm.call_stack.last_mut().ok_or(VmError::StackUnderflow)?;
    while offset_usize >= call_frame.locals_stack.len() {
        call_frame.locals_stack.push(Value::Nil);
    }
    call_frame.locals_stack[offset_usize] = value;
    Ok(())
}
```

**Trade-off Analysis:**

| Aspect                        | Assessment                                                  |
| ----------------------------- | ----------------------------------------------------------- |
| **Implementation Complexity** | Medium - Requires restructuring CallFrame and all local ops |
| **Performance Impact**        | Low - One extra push, but simpler semantics                 |
| **Memory Impact**             | Low - Additional Vec per frame, but predictable             |
| **Compatibility**             | High - No change to Call opcode semantics                   |

**Pros:**
- Clean separation of concerns
- No interference between value stack and locals
- Frame reuse works correctly
- Easy to debug and reason about

**Cons:**
- Memory overhead per frame
- Requires careful synchronization during frame creation
- Changes API surface area

### Option B: Heap-Allocated Frames

**Approach:** Allocate frames on the heap with stable addresses, store locals in the frame struct directly.

**Implementation:**
```rust
struct CallFrame {
    locals: Vec<Value>,           // Direct storage, not on any stack
    return_ip: usize,
    stack_start: usize,           // Position in value stack
    saved_instructions: Option<Vec<OpCode>>,
    // ... other fields
}

// Tail call reuses frame by updating IP
fn handle_tail_call(vm: &mut VmState, closure_body: Vec<OpCode>) -> Result<(), VmError> {
    if let Some(frame) = vm.call_stack.last_mut() {
        // Reuse same frame - locals already there
        frame.locals.clear();
        frame.locals.extend(new_args);
        vm.instructions = closure_body;
        vm.ip = 0;
    }
    Ok(())
}
```

**Trade-off Analysis:**

| Aspect                        | Assessment                                                 |
| ----------------------------- | ---------------------------------------------------------- |
| **Implementation Complexity** | High - Requires heap allocation strategy, Rc/Box decisions |
| **Performance Impact**        | Medium - Allocation on first call, reuse on TCO            |
| **Memory Impact**             | Medium - Heap allocation overhead per frame                |
| **Compatibility**             | Low - Major changes to call stack management               |

**Pros:**
- Stable frame addresses for debugging
- Natural fit for TCO - just update IP
- Easy to add frame introspection
- Good for memory profiling

**Cons:**
- Heap allocation overhead
- Complex lifetime management
- More complex error handling
- Significant refactoring required

### Option C: Trampoline/Bounce Mechanism

**Approach:** For non-self-recursive tail calls, return to dispatcher which jumps to new closure instead of frame reuse.

**Implementation:**
```rust
// In execution engine
match instruction {
    OpCode::TailCall(arg_count) => {
        // Return a special "TailCallContinuation" instead of reusing frame
        return Ok(InstructionResult::TailCallContinuation {
            closure_ptr,
            arg_count,
        });
    }
    // ... other instructions
}

// In VM runner
fn run_with_trampoline(vm: &mut VmState) -> Result<Value, VmError> {
    loop {
        match vm.step()? {
            InstructionResult::TailCallContinuation { closure_ptr, arg_count } => {
                // Bounce back to dispatcher, load new closure
                vm.instructions = load_closure_body(closure_ptr)?;
                vm.ip = 0;
                // Don't push new frame - just continue
            }
            InstructionResult::Finished(value) => return Ok(value),
            InstructionResult::Continue => continue,
        }
    }
}
```

**Trade-off Analysis:**

| Aspect                        | Assessment                                                        |
| ----------------------------- | ----------------------------------------------------------------- |
| **Implementation Complexity** | Medium - Requires new InstructionResult variant, dispatcher logic |
| **Performance Impact**        | Medium - One extra dispatch per tail call                         |
| **Memory Impact**             | Low - No extra memory, just different flow                        |
| **Compatibility**             | High - Minimal changes to frame structure                         |

**Pros:**
- Avoids complex frame reuse logic
- Works with existing frame structure
- Easy to implement in phases
- Clear separation between TCO and normal calls

**Cons:**
- Performance overhead per tail call
- Changes execution model significantly
- More complex control flow
- May not work for all tail call patterns

---

## 5. Recommendation

### Primary Recommendation: Option A (Separate Local Stack)

**Rationale:**

1. **Minimal Semantic Change**: The change is localized to how `SetLocal`/`GetLocal` access data, not the entire call semantics.

2. **Predictable Memory Behavior**: Each frame has its own local stack with predictable growth patterns.

3. **Works with Existing Frame Reuse**: The TCO mechanism already updates `frame.locals`; we just need `SetLocal` to write there directly.

4. **Implementation Path**: Can be implemented incrementally:
   - Phase 1: Add `locals_stack` to `CallFrame`
   - Phase 2: Modify `SetLocal` to write to `locals_stack`
   - Phase 3: Update `handle_tail_call` to use new structure
   - Phase 4: Remove old local handling

### Secondary Recommendation: Option C (Trampoline) as Fallback

If Option A proves too invasive, Option C provides a working solution with different trade-offs. The trampoline mechanism is well-understood and used in production VMs (e.g., Scala, Racket).

### Implementation Plan for Option A

1. **Modify CallFrame** to include `locals_stack: Vec<Value>`
2. **Update `handle_set_local`** to write to `frame.locals_stack` instead of popping from stack
3. **Update `handle_get_local`** to read from `frame.locals_stack`
4. **Update `handle_tail_call`** to use new local storage
5. **Add validation** to ensure `SetLocal` only writes within bounds
6. **Add tests** for frame reuse scenarios

### Immediate Workaround

For the current codebase, a **hotfix** can be applied to `handle_tail_call`:

```rust
fn execute_tail_call_body(
    vm: &mut VmState,
    closure_body: Vec<OpCode>,
    arg_count: u16,
) -> Result<(), VmError> {
    // Calculate stack_start BEFORE popping
    let stack_start = vm.stack.len() - arg_count as usize;
    
    // Pop the closure
    let _closure = vm.stack.pop().unwrap();
    
    // Copy arguments to locals (preserving them for SetLocal)
    let args_start = vm.stack.len() - arg_count as usize;
    let args: Vec<Value> = if args_start < vm.stack.len() {
        vm.stack[args_start..].to_vec()
    } else {
        Vec::new()
    };
    
    // FIX: Push arguments onto stack at correct position
    // This ensures SetLocal can pop them
    for arg in &args {
        vm.stack.push(arg.clone());
    }
    
    // Reuse the current call frame
    if let Some(current_frame) = vm.call_stack.last_mut() {
        current_frame.is_tail_call = true;
        current_frame.stack_start = stack_start;
        current_frame.locals = args.clone();
    }
    
    vm.instructions = closure_body;
    vm.ip = 0;
    
    Ok(())
}
```

This workaround maintains the stack invariant that `SetLocal` expects while preserving the TCO frame reuse semantics.

---

## 6. Code Evidence Summary

| File                                                                                     | Lines   | Evidence                                                       |
| ---------------------------------------------------------------------------------------- | ------- | -------------------------------------------------------------- |
| [`physics_world/src/vm/opcodes/call.rs`](physics_world/src/vm/opcodes/call.rs)           | 314-347 | `execute_tail_call_body` - frame reuse logic                   |
| [`physics_world/src/vm/opcodes/call.rs`](physics_world/src/vm/opcodes/call.rs)           | 121-236 | `execute_closure_body` - stack truncation                      |
| [`physics_world/src/vm/opcodes/stack_ops.rs`](physics_world/src/vm/opcodes/stack_ops.rs) | 36-50   | `handle_get_local` - reads from `frame.locals`                 |
| [`physics_world/src/vm/opcodes/stack_ops.rs`](physics_world/src/vm/opcodes/stack_ops.rs) | 52-73   | `handle_set_local` - pops from stack, writes to `frame.locals` |

---

## 7. References

- [TCO Implementation Complete](docs/engineering/tco_implementation_complete.md)
- [VM Implementation Challenges](docs/engineering/vm_implementation_challenges_and_features.md)
- [Call State Module](physics_world/src/vm/call_state.rs)
- [Execution Engine](physics_world/src/vm/execution.rs)

---

**Document Status:** Ready for Expert Review  
**Next Action:** Implement hotfix or select solution approach