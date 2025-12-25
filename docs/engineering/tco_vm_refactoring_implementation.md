# TCO VM Refactoring Implementation Plan

## Overview

This document provides the step-by-step implementation plan for fixing the TCO frame reuse issue by implementing **Option A (Separate Local Stack)**. This follows the expert guidance and the detailed plan in `tco_vm_refactoring_plan.md`.

## Current Problem

The VM's `SetLocal` opcode pops from the value stack, but after `Call` truncates the stack, the expected values are gone. This breaks frame reuse in `handle_tail_call`.

## Solution: Two-Storage Model

Each `CallFrame` will have two separate storage areas:
1. **`locals: Vec<Value>`** - Arguments and local variables (stable, frame-scoped)
2. **`value_stack: Vec<Value>`** - Expression evaluation stack (ephemeral, call-scoped)

## Implementation Steps

### Phase 1: CallFrame Structure Modification

#### 1.1 Update CallFrame in `physics_world/src/vm/call_state.rs`

```rust
pub struct CallFrame {
    /// Local variables including function arguments
    pub locals: Vec<Value>,
    /// Instruction pointer for this frame
    pub ip: usize,
    /// Reference to the closure being executed
    pub closure: GcPtr<Closure>,
    /// Stack start position for this frame's evaluation
    pub stack_start: usize,
}

impl CallFrame {
    pub fn new(closure: GcPtr<Closure>, stack_start: usize) -> Self {
        let arg_count = closure.fn_type.arg_count();
        let mut locals = Vec::with_capacity(arg_count);
        
        // Pre-allocate locals vector with Nil values for arguments
        locals.resize(arg_count, Value::Nil);
        
        Self {
            locals,
            ip: 0,
            closure,
            stack_start,
        }
    }
}
```

### Phase 2: Update Opcodes

#### 2.1 GetLocal - Read from locals (no change needed, already correct)

```rust
pub fn handle_get_local(vm: &mut VmState, offset: u16) -> Result<(), VmError> {
    let frame = vm.call_stack.last_mut().ok_or(VmError::StackUnderflow)?;
    let offset_usize = offset as usize;
    
    if offset_usize >= frame.locals.len() {
        return Err(VmError::UndefinedVariable(offset));
    }
    
    vm.value_stack.push(frame.locals[offset_usize].clone());
    Ok(())
}
```

#### 2.2 SetLocal - Write to locals (remove stack pop)

```rust
pub fn handle_set_local(vm: &mut VmState, offset: u16) -> Result<(), VmError> {
    let frame = vm.call_stack.last_mut().ok_or(VmError::StackUnderflow)?;
    let offset_usize = offset as usize;
    
    // Ensure locals vector is large enough
    if offset_usize >= frame.locals.len() {
        frame.locals.resize(offset_usize + 1, Value::Nil);
    }
    
    // Value should be at the top of value_stack
    // Copy it to locals and remove from stack
    if let Some(value) = vm.value_stack.pop() {
        frame.locals[offset_usize] = value;
        Ok(())
    } else {
        Err(VmError::StackUnderflow)
    }
}
```

### Phase 3: Fix Call and TailCall

#### 3.1 Handle Call - Copy args to locals, truncate stack

```rust
pub fn handle_call(vm: &mut VmState, arg_count: u16) -> Result<(), VmError> {
    let callee = vm.value_stack.last()
        .ok_or(VmError::StackUnderflow)?
        .clone();
    
    let closure = match callee {
        Value::Closure(c) => c,
        Value::Builtin(b) => {
            return execute_builtin(vm, &b.name, arg_count);
        }
        _ => return Err(VmError::TypeError("Expected function".to_string())),
    };
    
    // Verify argument count
    if closure.fn_type.arg_count() != arg_count as usize {
        return Err(VmError::ArgumentCountMismatch {
            expected: closure.fn_type.arg_count(),
            got: arg_count as usize,
        });
    }
    
    // Copy arguments from value_stack to new frame's locals
    let stack_start = vm.value_stack.len() - arg_count as usize;
    let mut new_locals = Vec::with_capacity(closure.fn_type.arg_count());
    
    for i in 0..arg_count as usize {
        new_locals.push(vm.value_stack[stack_start + i].clone());
    }
    
    // Truncate value_stack to remove arguments (they're now in locals)
    vm.value_stack.truncate(stack_start);
    
    // Pop the callee closure
    vm.value_stack.pop();
    
    // Push new frame onto call stack
    let mut frame = CallFrame::new(closure, stack_start);
    frame.locals = new_locals;
    vm.call_stack.push(frame);
    
    Ok(())
}
```

#### 3.2 Handle TailCall - Reuse frame, replace locals

```rust
pub fn handle_tail_call(vm: &mut VmState, arg_count: u16) -> Result<(), VmError> {
    let callee = vm.value_stack.last()
        .ok_or(VmError::StackUnderflow)?
        .clone();
    
    let closure = match callee {
        Value::Closure(c) => c,
        Value::Builtin(_) => {
            // For builtins, we can't do TCO - fall back to regular call
            return handle_call(vm, arg_count);
        }
        _ => return Err(VmError::TypeError("Expected function".to_string())),
    };
    
    // Verify argument count
    if closure.fn_type.arg_count() != arg_count as usize {
        return Err(VmError::ArgumentCountMismatch {
            expected: closure.fn_type.arg_count(),
            got: arg_count as usize,
        });
    }
    
    let frame = vm.call_stack.last_mut().ok_or(VmError::StackUnderflow)?;
    
    // Reuse the current frame for tail call
    frame.closure = closure;
    frame.ip = 0;
    
    // Clear and replace locals with new arguments
    frame.locals.clear();
    
    let args_start = vm.value_stack.len() - arg_count as usize;
    for i in 0..arg_count as usize {
        frame.locals.push(vm.value_stack[args_start + i].clone());
    }
    
    // Truncate value_stack to remove arguments and callee
    vm.value_stack.truncate(args_start);
    vm.value_stack.pop(); // Remove callee
    
    Ok(())
}
```

### Phase 4: Fix Return Handling

```rust
pub fn handle_return(vm: &mut VmState) -> Result<(), VmError> {
    let return_value = vm.value_stack.pop()
        .ok_or(VmError::StackUnderflow)?;
    
    // Pop the current frame
    vm.call_stack.pop();
    
    // Push return value to value_stack
    if let Some(caller_frame) = vm.call_stack.last_mut() {
        vm.value_stack.push(return_value);
    } else {
        vm.value_stack.push(return_value);
    }
    
    Ok(())
}
```

### Phase 5: Update MakeClosure

```rust
pub fn handle_make_closure(vm: &mut VmState, arg_count: u16) -> Result<(), VmError> {
    // arg_count values on stack: [env_values..., code_ptr, arg_count]
    
    let actual_arg_count = vm.value_stack.len();
    if actual_arg_count < 2 {
        return Err(VmError::StackUnderflow);
    }
    
    let code_ptr = vm.value_stack[vm.value_stack.len() - 2];
    let code_ptr = match code_ptr {
        Value::CodePtr(p) => p,
        _ => return Err(VmError::TypeError("Expected code pointer".to_string())),
    };
    
    // Collect environment values
    let env_start = vm.value_stack.len() - 2 - arg_count as usize;
    let env_values: Vec<Value> = vm.value_stack[env_start..vm.value_stack.len() - 2]
        .iter()
        .cloned()
        .collect();
    
    // Create closure with environment and argument count
    let closure = Closure {
        fn_type: FunctionType::UserDefined {
            arg_count: arg_count as usize,
        },
        code_ptr,
        env: env_values,
    };
    
    // Clear the stack and push the closure
    vm.value_stack.truncate(env_start);
    vm.value_stack.push(Value::Closure(closure));
    
    Ok(())
}
```

## Files to Modify

1. `physics_world/src/vm/call_state.rs` - Add locals Vec to CallFrame
2. `physics_world/src/vm/state.rs` - Ensure value_stack is accessible
3. `physics_world/src/vm/opcodes/local_ops.rs` - Fix SetLocal to not pop from stack
4. `physics_world/src/vm/opcodes/call.rs` - Fix Call and TailCall to use new model
5. `physics_world/src/vm/opcodes/make_closure.rs` - Update closure creation
6. `physics_world/src/vm/opcodes/ret.rs` - Fix return handling

## Verification Checklist

- [ ] All existing tests pass (no regressions)
- [ ] Compiler TCO tests pass (14/14)
- [ ] VM execution tests pass (8/8)
- [ ] New TCO runtime tests pass (tail_call_with_set_local, mutual_tail_recursion, stack_depth_constant)
- [ ] Stack depth remains constant during tail recursion
- [ ] No stack overflow for deep recursion (10000+ calls)

## Effort Estimate

- **Phase 1**: 2-3 hours (CallFrame structure)
- **Phase 2**: 1-2 hours (Opcode updates)
- **Phase 3**: 2-3 hours (Call/TailCall fix)
- **Phase 4**: 1 hour (Return handling)
- **Phase 5**: 1 hour (MakeClosure)
- **Testing**: 2 hours (new tests and regression testing)

**Total**: 9-12 hours for full implementation

## Test Cases to Add

```rust
#[test]
fn test_tail_call_with_set_local() {
    // This should work with the new model
    let code = compile("
        (define (fact n acc)
          (if (= n 0)
              acc
              (let ((new-n (- n 1))
                    (new-acc (* n acc)))
                (fact new-n new-acc))))
        (fact 5 1)
    ");
    assert_eq!(vm.execute(code), Ok(Value::Int(120)));
}

#[test]
fn test_mutual_tail_recursion() {
    // Mutual tail recursion should work
    let code = compile("
        (define (even? n)
          (if (= n 0)
              #t
              (odd? (- n 1))))
        (define (odd? n)
          (if (= n 0)
              #f
              (even? (- n 1))))
        (even? 10000)
    ");
    assert_eq!(vm.execute(code), Ok(Value::Bool(true)));
}

#[test]
fn test_stack_depth_constant() {
    // Verify stack doesn't grow
    let start_depth = vm.value_stack.len();
    let code = compile_tail_recursive(10000);
    vm.execute(code);
    let end_depth = vm.value_stack.len();
    assert!(end_depth - start_depth < 10); // Allow small variance
}
```

## Implementation Order

1. **Start with Phase 1**: Modify CallFrame structure
2. **Then Phase 2**: Update GetLocal/SetLocal opcodes
3. **Then Phase 3**: Fix Call and TailCall
4. **Then Phase 4**: Fix Return handling
5. **Then Phase 5**: Update MakeClosure
6. **Finally**: Add tests and verify

## Related Documentation

- [docs/engineering/tco_frame_reuse_analysis.md](tco_frame_reuse_analysis.md)
- [docs/engineering/tco_implementation_complete.md](tco_implementation_complete.md)
- [docs/engineering/tco_vm_refactoring_plan.md](tco_vm_refactoring_plan.md)