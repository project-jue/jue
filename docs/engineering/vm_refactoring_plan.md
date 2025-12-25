# VM Refactoring Plan: Unified Calling Convention

## Reference: Expert Feedback Summary

### Core Problem
The current implementation mixes two models:
- **Model A**: Stack-based arguments (GetLocal reads from `vm.stack[stack_start + i]`)
- **Model B**: Locals-based arguments (GetLocal reads from `frame.locals[i]`)

This causes asymmetry and breaks recursive calls.

### Solution: Unified Calling Convention

```
BEFORE Call:
Stack: [arg1, arg2, ..., argN, closure]

DURING Call:
1. Pop closure
2. Copy [arg1, ..., argN] → frame.locals
3. Pop arguments from stack (stack becomes empty for this frame)
4. frame.stack_start = stack.len() (points to empty stack)

AFTER Call:
Stack: []  (clean slate for function)
Locals: [arg1, arg2, ..., argN]
```

## Implementation Steps

### Step 1: Fix handle_call
1. Pop closure from stack
2. Copy arguments to `frame.locals` (in order: first arg at index 0)
3. Truncate stack to remove arguments (clean slate)
4. Set `frame.stack_start = stack.len()` (where empty stack begins)

### Step 2: Fix handle_ret
1. Pop return value from stack
2. Pop current frame
3. Truncate stack to `frame.stack_start`
4. Push return value for caller
5. Restore caller's IP

### Step 3: Fix GetLocal/SetLocal
1. GetLocal: Read from `frame.locals[offset]`
2. SetLocal: Write to `frame.locals[offset]`
3. Remove any stack-based fallback logic

### Step 4: Fix Test Cases
Update test cases to use new argument ordering:
- Main program pushes args first, then closure
- GetLocal(0) reads first argument

## Test Verification

### test_simple_recursion
```
fact(n) = if n==0 then 0 else fact(n-1)
Expected: fact(1) = 0
```

### test_deep_call_stack
```
fact(n, acc) = if n==0 then acc else fact(n-1, n*acc)
Expected: fact(3, 1) = 6
```

## Implementation Files

1. `physics_world/src/vm/opcodes/call.rs` - Call handler
2. `physics_world/src/vm/opcodes/ret.rs` - Return handler
3. `physics_world/src/vm/opcodes/stack_ops.rs` - GetLocal/SetLocal
4. `physics_world/tests/test_closure_execution.rs` - Update tests

## Success Criteria

1. All closure execution tests pass
2. Recursive calls work correctly
3. Frame isolation verified (nested calls don't corrupt each other)