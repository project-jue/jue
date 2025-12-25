# TCO Compiler Integration - Final Implementation Plan

## Expert Guidance Summary

All questions have been answered by the expert. Key decisions:

| Question          | Decision                                           |
| ----------------- | -------------------------------------------------- |
| Jump offsets      | Use `offset = else_start_idx - cond_jump_idx - 1`  |
| Lambda body       | Always compile in tail position (`true`)           |
| Begin/sequence    | Last expression inherits tail position             |
| TailCall failures | Handle exactly like Call failures (validate first) |
| Single-pass       | Preferred approach - simpler and correct           |

---

## Implementation Steps with Expert Guidance

### Step 2.1: Add Tail Context Parameter

**File**: `jue_world/src/physics_integration/physics_compiler.rs`

```rust
/// Compile AST to Physics-World bytecode with tail context tracking
/// 
/// # Arguments
/// * `ast` - The AST node to compile
/// * `in_tail_position` - Whether this expression is in tail position (last expr that determines return value)
pub fn compile_to_physics_with_tail_context(
    &mut self,
    ast: &AstNode,
    in_tail_position: bool,
) -> Result<Vec<OpCode>, CompilationError> {
    match ast {
        AstNode::Call { function, arguments, .. } => {
            self.compile_call(function, arguments, in_tail_position)
        }
        AstNode::If { condition, then_branch, else_branch, .. } => {
            self.compile_if(condition, then_branch, else_branch, in_tail_position)
        }
        AstNode::Let { bindings, body, .. } => {
            self.compile_let(bindings, body, in_tail_position)
        }
        AstNode::Letrec { bindings, body, .. } => {
            self.compile_letrec(bindings, body, in_tail_position)
        }
        AstNode::Lambda { parameters, body, .. } => {
            self.compile_lambda(parameters, body)
            // Lambda body ALWAYS in tail position (per expert guidance)
        }
        // ... other cases
    }
}

/// Backward-compatible wrapper
pub fn compile_to_physics(&mut self, ast: &AstNode) -> Result<Vec<OpCode>, CompilationError> {
    self.compile_to_physics_with_tail_context(ast, false)
}
```

### Step 2.2: Fix compile_if Jump Offsets

**Expert guidance**: Offset = `else_start_idx - cond_jump_idx - 1`

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
    let skip_else_jump_idx = bytecode.len() - 1;

    // Compile else branch
    let else_start_idx = bytecode.len();
    bytecode.extend(self.compile_to_physics_with_tail_context(else_branch, in_tail_position)?);

    // Patch conditional jump: jump to else_start_idx if condition is false
    // offset = else_start_idx - cond_jump_idx - 1
    let else_offset = (else_start_idx as i16) - (cond_jump_idx as i16) - 1;
    if let OpCode::JmpIfFalse(offset) = &mut bytecode[cond_jump_idx] {
        *offset = else_offset;
    }

    // Patch skip-else jump: jump past else branch
    let skip_else_offset = (bytecode.len() as i16) - (skip_else_jump_idx as i16) - 1;
    if let OpCode::Jmp(offset) = &mut bytecode[skip_else_jump_idx] {
        *offset = skip_else_offset;
    }

    Ok(bytecode)
}
```

### Step 2.3: Update compile_call for TailCall Emission

**Expert guidance**: Emit `TailCall` when `in_tail_position=true`

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

    // Emit Call or TailCall based on position
    if in_tail_position {
        bytecode.push(OpCode::TailCall(arguments.len() as u16));
    } else {
        bytecode.push(OpCode::Call(arguments.len() as u16));
    }

    Ok(bytecode)
}
```

### Step 2.4: Update compile_lambda (Body Always in Tail Position)

**Expert guidance**: Lambda body should ALWAYS be compiled in tail position

```rust
pub fn compile_lambda(
    &mut self,
    parameters: &[String],
    body: &AstNode,
) -> Result<Vec<OpCode>, CompilationError> {
    let mut bytecode = Vec::new();

    // Create new environment for lambda
    self.environment.push_scope();

    // Add parameters to environment
    for (i, param) in parameters.iter().enumerate() {
        self.environment.add_variable(param.clone(), i);
    }

    // Compile lambda body - ALWAYS in tail position (per expert guidance)
    let body_bytecode = self.compile_to_physics_with_tail_context(body, true)?;

    // Pop environment scope
    self.environment.pop_scope();

    // Create closure
    bytecode.push(OpCode::MakeClosure(parameters.len(), body_bytecode.len()));
    bytecode.extend(body_bytecode);

    Ok(bytecode)
}
```

### Step 2.5: Update compile_let and compile_letrec

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

    // Compile each binding (values NOT in tail position)
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

pub fn compile_letrec(
    &mut self,
    bindings: &[(String, AstNode)],
    body: &AstNode,
    in_tail_position: bool,
) -> Result<Vec<OpCode>, CompilationError> {
    let mut bytecode = Vec::new();

    self.environment.push_scope();

    // Register all binding names first (for mutual recursion)
    for (name, _value) in bindings {
        self.environment.add_variable(name.clone(), 0);
    }

    // Compile each binding (values NOT in tail position)
    for (name, value) in bindings {
        let value_bytecode = self.compile_to_physics_with_tail_context(value, false)?;
        bytecode.extend(value_bytecode);

        if let Some(index) = self.environment.get_variable_index(name) {
            bytecode.push(OpCode::SetLocal(index as u16));
        }
    }

    // Compile body - propagate tail context
    let body_bytecode = self.compile_to_physics_with_tail_context(body, in_tail_position)?;

    self.environment.pop_scope();
    bytecode.extend(body_bytecode);
    Ok(bytecode)
}
```

### Step 2.6: Add Begin/Sequence Support

**Expert guidance**: Last expression inherits tail position; others don't

```rust
/// Compile begin/sequence expression
pub fn compile_begin(
    &mut self,
    expressions: &[AstNode],
    in_tail_position: bool,
) -> Result<Vec<OpCode>, CompilationError> {
    let mut bytecode = Vec::new();

    if expressions.is_empty() {
        // Empty begin: return nil
        return Ok(vec![OpCode::Nil]);
    }

    // All but last expression: NOT in tail position
    for expr in &expressions[0..expressions.len()-1] {
        bytecode.extend(self.compile_to_physics_with_tail_context(expr, false)?);
        // Pop result to keep stack clean (if expression has side effects)
        bytecode.push(OpCode::Pop);
    }

    // Last expression: inherits tail position
    if let Some(last) = expressions.last() {
        bytecode.extend(self.compile_to_physics_with_tail_context(last, in_tail_position)?);
    }

    Ok(bytecode)
}
```

### Step 2.7: Add TailCall Operation Counting

**File**: `physics_world/src/vm/opcodes/call.rs`

```rust
pub fn handle_tail_call(vm: &mut VmState, arg_count: u16) -> VmResult<()> {
    // Count as operation (per expert guidance)
    vm.total_ops += 1;
    vm.tail_ops += 1;  // Optional separate counter

    if vm.total_ops > vm.max_ops {
        return Err(VmError::CpuLimitExceeded {
            context: "tail call operation limit".to_string(),
            limit: vm.max_ops,
        });
    }

    // Validate before modifying frame
    let arg_count_usize = arg_count as usize;
    if vm.stack.len() < arg_count_usize + 1 {
        return Err(VmError::StackUnderflow);
    }

    let closure_idx = vm.stack.len() - arg_count_usize - 1;
    let closure_value = vm.stack[closure_idx].clone();

    let target_closure = closure_value.as_closure()
        .ok_or(VmError::TypeError {
            expected: "closure",
            got: closure_value.type_name(),
        })?;

    // Get current frame
    let current_frame = vm.frames.last_mut()
        .ok_or(VmError::NoActiveFrame)?;

    // Check if same function (self-recursion) or different (mutual recursion)
    let is_same_function = {
        let current_closure = &current_frame.closure;
        target_closure.function.code_index == current_closure.function.code_index
    };

    if is_same_function {
        reuse_frame_for_tail_call(vm, current_frame, arg_count_usize)?;
    } else {
        replace_frame_for_tail_call(vm, arg_count_usize, &target_closure)?;
    }

    Ok(())
}
```

### Step 2.8: Add Compiler Debug Flag

```rust
pub struct PhysicsWorldCompiler {
    pub tier: TrustTier,
    pub location: SourceLocation,
    pub capability_indices: Vec<Capability>,
    pub string_pool: Vec<String>,
    pub ffi_registry: FfiCallGenerator,
    pub environment: CompilationEnvironment,
    pub is_compiling_recursive_lambda: bool,
    /// Debug flag to disable TCO (per expert guidance)
    pub disable_tco: bool,
}

impl PhysicsWorldCompiler {
    pub fn new(tier: TrustTier) -> Self {
        Self {
            tier,
            location: SourceLocation::default(),
            capability_indices: Vec::new(),
            string_pool: Vec::new(),
            ffi_registry: FfiCallGenerator {
                registry: create_standard_ffi_registry(),
                location: SourceLocation::default(),
            },
            environment: CompilationEnvironment::new(),
            is_compiling_recursive_lambda: false,
            disable_tco: false,  // Default: TCO enabled
        }
    }
}

/// Then in compile_call:
if in_tail_position && !self.disable_tco {
    bytecode.push(OpCode::TailCall(arguments.len() as u16));
} else {
    bytecode.push(OpCode::Call(arguments.len() as u16));
}
```

---

## Comprehensive Test Suite

```rust
// jue_world/tests/test_tco_compiler.rs

#[test]
fn test_tail_call_factorial() {
    // (define (fact n acc) (if (zero? n) acc (fact (- n 1) (* n acc))))
    let ast = parse_jue("(define (fact n acc) (if (zero? n) acc (fact (- n 1) (* n acc))))");
    let (bytecode, _) = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
    
    // Should have exactly one TailCall for recursive case
    let tail_calls: Vec<_> = bytecode.iter()
        .filter(|op| matches!(op, OpCode::TailCall(_)))
        .collect();
    
    assert_eq!(tail_calls.len(), 1);
}

#[test]
fn test_nested_tail_calls() {
    // f calls g in tail position, g calls h in tail position
    let ast = parse_jue("
        (define (f x) (g (+ x 1)))
        (define (g y) (h (* y 2)))
        (define (h z) z)
        (f 10000)
    ");
    let (bytecode, _) = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
    
    // Both f->g and g->h should be tail calls
    let tail_calls: Vec<_> = bytecode.iter()
        .filter(|op| matches!(op, OpCode::TailCall(_)))
        .collect();
    
    assert_eq!(tail_calls.len(), 2);
}

#[test]
fn test_tail_call_in_conditionals() {
    // Tail call in both then and else branches
    let ast = parse_jue("
        (define (f n)
          (if (even? n)
              (g (/ n 2))
              (g (+ n 1))))
    ");
    let (bytecode, _) = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
    
    let tail_calls: Vec<_> = bytecode.iter()
        .filter(|op| matches!(op, OpCode::TailCall(_)))
        .collect();
    
    // Should have 2 TailCalls (both branches)
    assert_eq!(tail_calls.len(), 2);
}

#[test]
fn test_non_tail_call_not_optimized() {
    // (+ 1 (factorial 5)) - the call is NOT in tail position
    let ast = parse_jue("(+ 1 (fact 5))");
    let (bytecode, _) = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
    
    let calls: Vec<_> = bytecode.iter()
        .filter(|op| matches!(op, OpCode::Call(_)))
        .collect();
    
    let tail_calls: Vec<_> = bytecode.iter()
        .filter(|op| matches!(op, OpCode::TailCall(_)))
        .collect();
    
    assert!(!calls.is_empty(), "Should have regular Call");
    assert!(tail_calls.is_empty(), "Should NOT have TailCall");
}

#[test]
fn test_lambda_body_tail_position() {
    // Lambda body should always compile as tail position
    let ast = parse_jue("((lambda (n) (if (= n 0) 1 (* n (self (- n 1))))) 10)");
    let (bytecode, _) = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
    
    // The recursive call inside lambda should be TailCall
    let tail_calls: Vec<_> = bytecode.iter()
        .filter(|op| matches!(op, OpCode::TailCall(_)))
        .collect();
    
    assert!(!tail_calls.is_empty(), "Lambda body tail call should be optimized");
}
```

---

## Effort Estimate

| Task                            | Hours       |
| ------------------------------- | ----------- |
| Add tail position parameter     | 1           |
| Fix compile_if jump offsets     | 1           |
| Implement compile_call TailCall | 1           |
| Update lambda, let, letrec      | 1           |
| Add begin/sequence support      | 1           |
| Add operation counting          | 0.5         |
| Add debug flag                  | 0.5         |
| Create test suite               | 2           |
| **Total**                       | **8 hours** |

---

## Verification Checklist

- [ ] Jump offsets compute correctly (offset = else_start - cond_jump - 1)
- [ ] Lambda bodies always compile with tail position
- [ ] compile_call emits TailCall only when in_tail_position=true
- [ ] compile_if propagates tail context to both branches
- [ ] compile_let/letrec propagate tail context to body
- [ ] Begin/sequence handles tail position correctly
- [ ] Tail calls count toward CPU limits
- [ ] Debug flag can disable TCO
- [ ] All compiler tests pass
- [ ] Non-tail calls still use regular Call opcode
- [ ] Existing tests continue to pass

---

## Files to Modify

| File                                                    | Change                                     |
| ------------------------------------------------------- | ------------------------------------------ |
| `jue_world/src/physics_integration/physics_compiler.rs` | Add tail context, emit TailCall, fix jumps |
| `jue_world/src/vm/opcodes/call.rs`                      | Add operation counting                     |
| `jue_world/tests/test_tco_compiler.rs`                  | New test file                              |