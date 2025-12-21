# Engineering Plan: Phase 1 - Foundation
**Duration**: 4 weeks
**Objective**: Implement lexical binding foundation and basic optimizations

## 📋 Overview
This phase establishes the core infrastructure for Jue's enhanced stack frame system, focusing on lexical binding, basic escape analysis, and tail call detection.

## 🗂️ File Modifications

### 1. **Physics World VM Changes**

#### `physics_world/src/vm/state.rs`
**Modifications**:
```rust
// Enhanced CallFrame structure
pub struct CallFrame {
    pub return_address: usize,
    pub stack_start: usize,
    pub locals: Vec<Value>,                // NEW: Lexical environment storage
    pub closed_over: HashMap<usize, Value>, // NEW: Closed-over variables
    pub recursion_depth: u32,
    pub is_tail_call: bool,                 // NEW: TCO tracking flag
    pub frame_id: u64,                     // NEW: For debugging/verification
}

// NEW: Tail call detection method
impl VmState {
    pub fn is_current_position_tail(&self) -> bool {
        if let Some(opcode) = self.current_opcode() {
            matches!(opcode, OpCode::Return | OpCode::TailCall(_))
        } else {
            false
        }
    }
}
```

#### `physics_world/src/vm/opcodes.rs`
**Modifications**:
```rust
// Add TailCall opcode
#[derive(Debug, Clone, PartialEq)]
pub enum OpCode {
    // ... existing opcodes
    Call(u16),
    TailCall(u16),      // NEW: Tail call (reuses stack frame)
}
```

#### `physics_world/src/vm/opcodes/call.rs`
**Modifications**:
```rust
// Enhanced handle_call with basic TCO detection
pub fn handle_call(vm: &mut VmState, function_ptr: u16) -> Result<(), VmError> {
    let is_tail_position = vm.is_current_position_tail();
    let arg_count = vm.read_u16()?;
    let args: Vec<Value> = vm.stack.drain((vm.stack.len() - arg_count as usize)..).collect();

    if is_tail_position {
        handle_tail_call(vm, function_ptr, args)?
    } else {
        // Regular call with enhanced CallFrame
        let current_depth = if let Some(last_frame) = vm.call_stack.last() {
            last_frame.recursion_depth + 1
        } else {
            1
        };

        if current_depth > vm.max_recursion_depth {
            return Err(VmError::recursion_limit_exceeded(
                vm.create_error_context(),
                vm.max_recursion_depth,
                current_depth
            ));
        }

        let call_frame = CallFrame {
            return_address: vm.ip + 2,
            stack_start: vm.stack.len() - arg_count as usize,
            locals: Vec::new(),
            closed_over: HashMap::new(),
            recursion_depth: current_depth,
            is_tail_call: false,
            frame_id: vm.next_frame_id(),
        };

        vm.call_stack.push(call_frame);
        vm.ip = function_ptr as usize;
        Ok(())
    }
}

// NEW: Basic tail call handler
pub fn handle_tail_call(vm: &mut VmState, function_ptr: u16, args: Vec<Value>) -> Result<(), VmError> {
    if vm.call_stack.is_empty() {
        return Err(VmError::stack_underflow(
            vm.create_error_context(),
            "TailCall",
            1,
            0
        ));
    }

    let current_frame = vm.call_stack.last_mut().unwrap();
    current_frame.is_tail_call = true;

    vm.stack.extend(args);
    vm.ip = function_ptr as usize;
    Ok(())
}
```

### 2. **Jue World Compiler Changes**

#### `jue_world/src/compiler/compiler.rs`
**Modifications**:
```rust
// NEW: Basic escape analysis
pub struct EscapeAnalysis {
    escaping_vars: HashSet<usize>,
    current_function: Option<FunctionId>,
}

// NEW: Basic tail call detection
pub fn is_tail_position(expr: &Expr, context: &CompilationContext) -> bool {
    match context.position {
        ExpressionPosition::FunctionBodyLast => true,
        ExpressionPosition::BlockLast => true,
        _ => false,
    }
}

// MODIFIED: Enhanced compile_function_call
pub fn compile_function_call(expr: &Expr, context: &mut CompilationContext) -> Result<Vec<OpCode>, CompileError> {
    let function_expr = &expr.args[0];
    let arg_exprs = &expr.args[1..];

    let mut bytecode = Vec::new();
    for arg in arg_exprs {
        bytecode.extend(compile_expression(arg, context)?);
    }

    let function_ptr = compile_expression(function_expr, context)?;
    bytecode.extend(function_ptr);

    if is_tail_position(expr, context) {
        bytecode.push(OpCode::TailCall(arg_exprs.len() as u16));
    } else {
        bytecode.push(OpCode::Call(arg_exprs.len() as u16));
    }

    Ok(bytecode)
}
```

### 3. **New Files**

#### `physics_world/src/vm/verification.rs`
```rust
use crate::vm::error::{VmError, VerificationError};
use crate::vm::state::CallFrame;

pub trait Verifiable {
    fn verify_invariants(&self) -> Result<(), VerificationError>;
    fn generate_proof_context(&self) -> ProofContext;
}

impl Verifiable for CallFrame {
    fn verify_invariants(&self) -> Result<(), VerificationError> {
        if self.stack_start > self.locals.len() {
            return Err(VerificationError::StackConsistency {
                frame_id: self.frame_id,
                detail: "stack_start exceeds locals length".to_string(),
            });
        }

        if self.recursion_depth > MAX_VERIFIABLE_DEPTH {
            return Err(VerificationError::RecursionDepth {
                frame_id: self.frame_id,
                depth: self.recursion_depth,
                limit: MAX_VERIFIABLE_DEPTH,
            });
        }

        if self.is_tail_call && self.recursion_depth == 0 {
            return Err(VerificationError::TailCallConsistency {
                frame_id: self.frame_id,
                detail: "tail call flag set on root frame".to_string(),
            });
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ProofContext {
    pub frame_id: u64,
    pub stack_start: usize,
    pub locals_count: usize,
    pub closed_over_count: usize,
    pub recursion_depth: u32,
    pub is_tail_call: bool,
}
```

#### `physics_world/src/vm/test/verification_tests.rs`
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::state::VmState;
    use crate::types::Value;

    #[test]
    fn test_call_frame_verification() {
        let mut frame = CallFrame {
            return_address: 0,
            stack_start: 0,
            locals: vec![Value::Int(42)],
            closed_over: HashMap::new(),
            recursion_depth: 1,
            is_tail_call: false,
            frame_id: 1,
        };

        assert!(frame.verify_invariants().is_ok());

        frame.stack_start = 2;
        assert!(matches!(
            frame.verify_invariants(),
            Err(VerificationError::StackConsistency { .. })
        ));
    }
}
```

## 📋 Implementation Plan

### Week 1: Core Infrastructure
```markdown
[ ] Modify CallFrame structure in physics_world/src/vm/state.rs
[ ] Add TailCall opcode to physics_world/src/vm/opcodes.rs
[ ] Implement basic tail call detection in compiler
[ ] Create verification.rs with basic verification support
[ ] Update handle_call to use new CallFrame structure
[ ] Add frame_id generation to VmState
```

### Week 2: Basic Functionality
```markdown
[ ] Implement is_current_position_tail() in VmState
[ ] Create basic handle_tail_call function
[ ] Update compile_function_call with tail call detection
[ ] Add basic escape analysis to compiler
[ ] Implement Verifiable trait for CallFrame
[ ] Create initial verification tests
```

### Week 3: Integration and Testing
```markdown
[ ] Integrate new CallFrame with existing VM operations
[ ] Update all existing tests to work with new structure
[ ] Add verification calls to critical VM operations
[ ] Create comprehensive test suite for new features
[ ] Performance benchmarking of basic implementation
[ ] Memory usage analysis
```

### Week 4: Documentation and Cleanup
```markdown
[ ] Update architecture documentation
[ ] Create developer guide for new features
[ ] Add examples showing lexical binding usage
[ ] Clean up any technical debt
[ ] Final testing and bug fixing
[ ] Prepare for Phase 2 transition
```

## 🎯 Success Criteria
- ✅ CallFrame supports lexical binding
- ✅ Basic tail call detection works
- ✅ Verification infrastructure is in place
- ✅ All existing tests still pass
- ✅ New features don't break existing functionality
