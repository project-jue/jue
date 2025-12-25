# Phase 2: Compiler Integration for Tail Call Optimization

This document provides a detailed implementation plan for integrating TCO into the Jue compiler based on the actual codebase structure.

## Current Compiler Architecture

The Jue compiler is located in [`jue_world/src/physics_integration/physics_compiler.rs`](jue_world/src/physics_integration/physics_compiler.rs) with:

- **Main entry**: `PhysicsWorldCompiler::compile_to_physics()` (lines 85-125)
- **Function call compilation**: `compile_call()` (lines 159-179) - currently emits `OpCode::Call`
- **Lambda compilation**: `compile_lambda()` (lines 181-208)
- **Let/letrec**: `compile_let()`, `compile_letrec()` (lines 210-278)
- **If expressions**: `compile_if()` (lines 323-364)

## Key Observation: Single-Pass Compilation

The current compiler uses a **single-pass compilation strategy** - each `compile_*` method emits bytecode immediately. This means TCO integration requires:

1. Adding a tail-context parameter to track position
2. Modifying `compile_call()` to emit `TailCall` when in tail position
3. Modifying `compile_if()` to propagate tail context to branches
4. Modifying `compile_lambda()` to accept tail parameter for body

---

## Implementation Steps

### Step 2.1: Add Tail Position Parameter to Compiler Methods

**File**: `jue_world/src/physics_integration/physics_compiler.rs`

**Change**: Modify `compile_to_physics` to accept a tail context parameter:

```rust
/// Compile AST to Physics-World bytecode
/// 
/// # Arguments
/// * `ast` - The AST node to compile
/// * `in_tail_position` - Whether this expression is in tail position
pub fn compile_to_physics_with_tail_context(
    &mut self,
    ast: &AstNode,
    in_tail_position: bool,
) -> Result<Vec<OpCode>, CompilationError> {
    match ast {
        AstNode::Call { function, arguments, .. } => {
            self.compile_call(function, arguments, in_tail_position)
        }
        // ... other cases updated similarly
    }
}
```

**Rationale**: This allows tracking whether we're in tail position without changing the API for external callers.

### Step 2.2: Update compile_call for Tail Position

**Current code** (lines 159-179):
```rust
pub fn compile_call(
    &mut self,
    function: &AstNode,
    arguments: &[AstNode],
) -> Result<Vec<OpCode>, CompilationError> {
    let mut bytecode = Vec::new();

    // Compile arguments in reverse order (stack grows upwards)
    for arg in arguments.iter().rev() {
        bytecode.extend(self.compile_to_physics(arg)?);
    }

    // Compile function
    bytecode.extend(self.compile_to_physics(function)?);

    // Add call instruction
    bytecode.push(OpCode::Call(arguments.len() as u16));

    Ok(bytecode)
}
```

**Updated code**:
```rust
pub fn compile_call(
    &mut self,
    function: &AstNode,
    arguments: &[AstNode],
    in_tail_position: bool,
) -> Result<Vec<OpCode>, CompilationError> {
    let mut bytecode = Vec::new();

    // Compile arguments in reverse order (NOT in tail position)
    for arg in arguments.iter().rev() {
        bytecode.extend(self.compile_to_physics_with_tail_context(arg, false)?);
    }

    // Compile function (NOT in tail position)
    bytecode.extend(self.compile_to_physics_with_tail_context(function, false)?);

    // Add call or tailcall instruction based on position
    if in_tail_position {
        bytecode.push(OpCode::TailCall(arguments.len() as u16));
    } else {
        bytecode.push(OpCode::Call(arguments.len() as u16));
    }

    Ok(bytecode)
}
```

### Step 2.3: Update compile_if for Tail Context Propagation

**Current code** (lines 323-364) - The if expression needs to handle tail context correctly:

```rust
pub fn compile_if(
    &mut self,
    condition: &AstNode,
    then_branch: &AstNode,
    else_branch: &AstNode,
    in_tail_position: bool,
) -> Result<Vec<OpCode>, CompilationError> {
    let mut bytecode = Vec::new();

    // Compile condition (never in tail position)
    bytecode.extend(self.compile_to_physics_with_tail_context(condition, false)?);

    // Compile then branch - propagate tail context
    bytecode.extend(self.compile_to_physics_with_tail_context(then_branch, in_tail_position)?);

    // Compile else branch - propagate tail context
    bytecode.extend(self.compile_to_physics_with_tail_context(else_branch, in_tail_position)?);

    Ok(bytecode)
}
```

**Critical consideration**: For TCO to work correctly, the `compile_if` method needs jump offsets. The current implementation uses placeholder jumps that get patched. We need to ensure the jump patching works correctly when tail calls are present.

### Step 2.4: Update compile_lambda for Tail Context

**Current code** (lines 181-208):
```rust
pub fn compile_lambda(
    &mut self,
    parameters: &[String],
    body: &AstNode,
    in_tail_position: bool,  // NEW PARAMETER
) -> Result<Vec<OpCode>, CompilationError> {
    let mut bytecode = Vec::new();

    // Create new environment for lambda
   _scope();

    // self.environment.push Add parameters to environment
    for (i, param) in parameters.iter().enumerate() {
        self.environment.add_variable(param.clone(), i);
    }

    // Compile lambda body - propagate tail context
    let body_bytecode = self.compile_to_physics_with_tail_context(body, in_tail_position)?;

    // Pop environment scope
    self.environment.pop_scope();

    // Create closure
    bytecode.push(OpCode::MakeClosure(parameters.len(), body_bytecode.len()));
    bytecode.extend(body_bytecode);

    Ok(bytecode)
}
```

### Step 2.5: Update compile_let and compile_letrec

Let bindings need to propagate tail context to the body:

```rust
pub fn compile_let(
    &mut self,
    bindings: &[(String, AstNode)],
    body: &AstNode,
    in_tail_position: bool,
) -> Result<Vec<OpCode>, CompilationError> {
    let mut bytecode = Vec::new();

    // Create new environment scope
    self.environment.push_scope();

    // Compile each binding (values not in tail position)
    for (name, value) in bindings {
        let value_bytecode = self.compile_to_physics_with_tail_context(value, false)?;
        bytecode.extend(value_bytecode);

        let index = self.environment.add_variable(name.clone(), 0);
        bytecode.push(OpCode::SetLocal(index as u16));
    }

    // Compile body - propagate tail context
    let body_bytecode = self.compile_to_physics_with_tail_context(body, in_tail_position)?;

    self.environment.pop_scope();
    bytecode.extend(body_bytecode);
    Ok(bytecode)
}
```

### Step 2.6: Update compile_define

Top-level defines don't need tail context since they don't return values:

```rust
pub fn compile_define(
    &mut self,
    name: String,
    value: &AstNode,
) -> Result<Vec<OpCode>, CompilationError> {
    let value_bytecode = self.compile_to_physics_with_tail_context(value, false)?;
    // ... rest unchanged
}
```

### Step 2.7: Update Top-Level Entry Point

**File**: `jue_world/src/physics_integration/physics_compiler.rs` (lines 481-507)

```rust
pub fn compile_to_physics_world(
    ast: &AstNode,
    tier: TrustTier,
) -> Result<(Vec<OpCode>, Vec<Value>), CompilationError> {
    let mut compiler = PhysicsWorldCompiler::new(tier);
    
    // Top-level expressions are NOT in tail position
    let mut bytecode = compiler.compile_to_physics_with_tail_context(ast, false)?;
    
    // ... rest unchanged
}
```

### Step 2.8: Add Helper Method for Backward Compatibility

```rust
/// Compile to physics - wrapper for backward compatibility
pub fn compile_to_physics(&mut self, ast: &AstNode) -> Result<Vec<OpCode>, CompilationError> {
    self.compile_to_physics_with_tail_context(ast, false)
}
```

---

## If Expression Jump Offset Issue

**Problem**: The current `compile_if` implementation has a bug in jump offset patching (lines 351-361):

```rust
// Bug: recompiling else_branch changes bytecode length
let else_branch_start = bytecode.len() - self.compile_to_physics(else_branch)?.len();
```

This recomputes `else_branch` bytecode, which is incorrect. For TCO to work correctly, we need to fix this:

```rust
pub fn compile_if(
    &mut self,
    condition: &AstNode,
    then_branch: &AstNode,
    else_branch: &AstNode,
    in_tail_position: bool,
) -> Result<Vec<OpCode>, CompilationError> {
    let mut bytecode = Vec::new();

    // Compile condition (never in tail position)
    bytecode.extend(self.compile_to_physics_with_tail_context(condition, false)?);

    // Reserve space for conditional jump
    bytecode.push(OpCode::JmpIfFalse(0)); 
    let cond_jump_idx = bytecode.len() - 1;

    // Compile then branch - propagate tail context
    bytecode.extend(self.compile_to_physics_with_tail_context(then_branch, in_tail_position)?);

    // Reserve space for jump over else branch
    bytecode.push(OpCode::Jmp(0));
    let then_jump_idx = bytecode.len() - 1;

    // Compile else branch - propagate tail context
    let else_start_idx = bytecode.len();
    bytecode.extend(self.compile_to_physics_with_tail_context(else_branch, in_tail_position)?);

    // Patch conditional jump (jump to else branch if false)
    if let OpCode::JmpIfFalse(offset) = &mut bytecode[cond_jump_idx] {
        *offset = (else_start_idx as i16) - (cond_jump_idx as i16);
    }

    // Patch then branch jump (skip else branch)
    if let OpCode::Jmp(offset) = &mut bytecode[then_jump_idx] {
        *offset = (bytecode.len() as i16) - (then_jump_idx as i16);
    }

    Ok(bytecode)
}
```

---

## Testing Strategy

### Test 2.1: Basic Tail Recursion

```rust
#[test]
fn test_compiler_tail_call_factorial() {
    // Compile: (define (fact n acc) (if (zero? n) acc (fact (- n 1) (* n acc))))
    let ast = parse_jue("(define (fact n acc) (if (zero? n) acc (fact (- n 1) (* n acc))))");
    let (bytecode, _) = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
    
    // Verify TailCall is emitted for the recursive call
    let tail_calls: Vec<_> = bytecode.iter()
        .filter(|op| matches!(op, OpCode::TailCall(_)))
        .collect();
    
    assert_eq!(tail_calls.len(), 1, "Should have exactly one TailCall for recursive case");
}
```

### Test 2.2: Mutual Tail Recursion

```rust
#[test]
fn test_compiler_tail_call_mutual_recursion() {
    // Compile: (define (even? n) (if (zero? n) #t (odd? (- n 1))))
    //           (define (odd? n) (if (zero? n) #f (even? (- n 1))))
    let ast = parse_jue("(define (even? n) (if (zero? n) #t (odd? (- n 1))))");
    let (bytecode, _) = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
    
    // Verify TailCall is emitted
    let tail_calls: Vec<_> = bytecode.iter()
        .filter(|op| matches!(op, OpCode::TailCall(_)))
        .collect();
    
    assert!(!tail_calls.is_empty(), "Should have TailCall for recursive odd? call");
}
```

### Test 2.3: Non-Tail Call (Regular Recursion)

```rust
#[test]
fn test_compiler_regular_call_not_tail() {
    // Compile: (define (fact n) (* n (fact (- n 1))))
    // This is NOT in tail position - should use Call, not TailCall
    let ast = parse_jue("(define (fact n) (* n (fact (- n 1))))");
    let (bytecode, _) = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
    
    let call_ops: Vec<_> = bytecode.iter()
        .filter(|op| matches!(op, OpCode::Call(_)))
        .collect();
    
    let tail_calls: Vec<_> = bytecode.iter()
        .filter(|op| matches!(op, OpCode::TailCall(_)))
        .collect();
    
    assert!(!call_ops.is_empty(), "Should have regular Call");
    assert!(tail_calls.is_empty(), "Should NOT have TailCall (multiply not in tail position)");
}
```

---

## Files to Modify

| File                                                    | Change                                                    | Priority |
| ------------------------------------------------------- | --------------------------------------------------------- | -------- |
| `jue_world/src/physics_integration/physics_compiler.rs` | Add `in_tail_position` parameter to all compile methods   | High     |
| `jue_world/src/physics_integration/physics_compiler.rs` | Update `compile_call` to emit `TailCall` in tail position | High     |
| `jue_world/src/physics_integration/physics_compiler.rs` | Fix jump offset patching in `compile_if`                  | High     |
| `jue_world/src/physics_integration/physics_compiler.rs` | Add backward-compatible wrapper methods                   | Medium   |
| `jue_world/tests/test_tco_compiler.rs`                  | Add compiler-level TCO tests                              | High     |

---

## Effort Estimate

| Task                                        | Estimated Effort |
| ------------------------------------------- | ---------------- |
| Add tail position parameter to all methods  | 1-2 hours        |
| Implement TailCall emission in compile_call | 1 hour           |
| Fix compile_if jump patching                | 1 hour           |
| Update all call sites                       | 1 hour           |
| Add comprehensive tests                     | 2 hours          |
| **Total**                                   | **6-8 hours**    |

---

## Verification Checklist

- [ ] `compile_call` emits `TailCall` when `in_tail_position=true`
- [ ] `compile_call` emits `Call` when `in_tail_position=false`
- [ ] `compile_if` correctly propagates tail context to both branches
- [ ] `compile_lambda` correctly propagates tail context to body
- [ ] `compile_let`/`compile_letrec` correctly propagates tail context to body
- [ ] Jump offsets in if expressions are computed correctly
- [ ] Compiler tests pass for tail recursion patterns
- [ ] Non-tail calls still use regular `Call` opcode
- [ ] Existing tests continue to pass