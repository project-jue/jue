# Phase 2: FFI Functions for Arithmetic Operations

## Overview

This document details the implementation plan for registering FFI functions for arithmetic operations (add, mul, sub, div) to enable expressions like `(add 1 2)` and `(mul 3.5 2.0)` to work correctly.

## Current State Analysis

### FFI System Architecture

The current FFI system consists of:

1. **HostFunction enum** (`physics_world/src/types/host.rs`):
   - Contains system functions: `ReadSensor`, `WriteActuator`, `GetWallClockNs`, etc.
   - **No arithmetic operations** currently defined

2. **FfiRegistry** (`jue_world/src/ffi_system/global_ffi_registry.rs`):
   - Maps function names to `FfiFunction` structs
   - Stores capability indices
   - Provides `find_function()` lookup

3. **Standard Functions** (`jue_world/src/ffi_system/standard_functions.rs`):
   - Only registers: `read-sensor`, `write-actuator`, `get-wall-clock`
   - **No arithmetic functions registered**

4. **Compiler Integration** (`jue_world/src/physics_integration/physics_compiler.rs`):
   - `compile_call()` emits `HostCall` opcode for FFI calls
   - Looks up function via `ffi_registry.find_function()`

### The Problem

When compiling code like `(add 1 2)`:
1. Parser creates `AstNode::Call { function: Symbol("add"), arguments: [...] }`
2. `compile_call()` compiles arguments, then function
3. Function compiles to `Symbol(index)` (string pool entry)
4. Finally emits `Call(2)` opcode
5. **VM tries to call Symbol("add") as a closure, which fails**

### Root Cause

The compiler treats `Symbol("add")` as a symbol reference, not as a function to invoke via FFI.

## Implementation Options

### Option A: Add Arithmetic HostFunctions

**Approach:** Extend `HostFunction` enum with arithmetic variants and implement them in the VM.

**Pros:**
- Consistent with existing FFI design
- Centralized implementation
- Can add capability checks per operation

**Cons:**
- Requires VM opcode implementation
- More invasive change

### Option B: Native Opcode Mapping

**Approach:** Modify `compile_call()` to detect special symbol names and emit native opcodes directly.

**Pros:**
- No VM changes needed
- Faster execution (direct opcode)
- Simpler implementation

**Cons:**
- Hardcoded symbol names
- Less extensible
- Duplicates logic in compiler

### Option C: Hybrid Approach

**Approach:** Keep FFI for complex operations, use native opcodes for simple arithmetic.

**Pros:**
- Best of both worlds
- Flexible
- Can evolve over time

**Cons:**
- More complex design
- Two code paths to maintain

---

## Implementation Plan (Option A: Add HostFunctions)

### Step 1: Extend HostFunction Enum

**File:** `physics_world/src/types/host.rs`

```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostFunction {
    // Existing...
    ReadSensor = 0,
    WriteActuator = 1,
    GetWallClockNs = 2,
    SpawnActor = 3,
    TerminateActor = 4,
    NetworkSend = 5,
    NetworkReceive = 6,
    PersistWrite = 7,
    PersistRead = 8,
    
    // New arithmetic operations (continue numbering)
    IntAdd = 9,
    IntSub = 10,
    IntMul = 11,
    IntDiv = 12,
    IntMod = 13,
    FloatAdd = 14,
    FloatSub = 15,
    FloatMul = 16,
    FloatDiv = 17,
    IntToFloat = 18,
    FloatToInt = 19,
}
```

**Questions for Expert Feedback:**
1. Should we use separate `IntAdd`/`FloatAdd` or a single `Add` that dispatches based on operand types?
2. Should we continue the enum numbering from 8 or restart for new functions?
3. Should we add overflow checking for integer operations?

### Step 2: Implement VM Handler

**File:** `physics_world/src/vm/opcodes/host_call.rs`

Need to add handling for new `HostFunction` variants in the VM's `handle_host_call()` function.

**Questions for Expert Feedback:**
1. Should arithmetic operations require any capabilities, or be available to all tiers?
2. Should we add error handling for division by zero?
3. Should we implement integer overflow detection?

### Step 3: Register FFI Functions

**File:** `jue_world/src/ffi_system/standard_functions.rs`

```rust
pub fn create_standard_ffi_registry() -> super::global_ffi_registry::FfiRegistry {
    let mut registry = super::global_ffi_registry::FfiRegistry::new();

    // Register arithmetic operations (no capability required)
    registry.register_function(super::global_ffi_registry::FfiFunction {
        name: "add".to_string(),
        host_function: HostFunction::IntAdd,
        required_capability: Capability::None, // or create new capability
        parameter_types: vec!["Int".to_string(), "Int".to_string()],
        return_type: "Int".to_string(),
        documentation: "Add two integers".to_string(),
        location: SourceLocation::default(),
    });
    
    // ... similar for mul, sub, div, mod
    // ... add float variants: fadd, fmul, fsub, fdiv
    
    registry
}
```

**Questions for Expert Feedback:**
1. Should arithmetic require a capability, or be universally available?
2. Should we use "Int" and "Float" as type strings, or something more specific?
3. Should we add overload resolution for mixed-type operations?

### Step 4: Update Compiler (Optional)

Currently `compile_call()` already handles FFI calls via `compile_ffi_call()`. The issue is that `(add 1 2)` is parsed as a `Call` node with `Symbol("add")`, not as an `FfiCall` node.

**Option 4a:** Detect special symbol names in `compile_call()` and route to FFI:
```rust
pub fn compile_call(
    &mut self,
    function: &AstNode,
    arguments: &[AstNode],
    in_tail_position: bool,
) -> Result<Vec<OpCode>, CompilationError> {
    // Check if this is an FFI function call
    if let AstNode::Symbol(name) = function {
        if let Some(ffi_func) = self.ffi_registry.registry.find_function(name) {
            return self.compile_ffi_call(name, arguments, &SourceLocation::default());
        }
    }
    // ... existing call logic
}
```

**Option 4b:** Require users to use explicit `ffi-call` syntax:
```jue
(ffi-call "add" 1 2)  ; Explicit FFI call
(add 1 2)              ; Error: unknown function
```

**Questions for Expert Feedback:**
1. Should we auto-detect FFI functions by name, or require explicit syntax?
2. If auto-detect, what happens to user-defined functions named "add"?
3. Should we have a naming convention (e.g., `math:add`)?

---

## Test Plan

### Tests to Unignore (Currently Ignored)

From `test_physics_world_integration_comprehensive.rs`:
- `test_float_arithmetic_integration`
- `test_variable_environment_integration`
- `test_nested_scope_variable_resolution`
- `test_performance_many_operations`
- `test_formal_tier_no_capability_checks`

From `test_recursive_function_execution.rs`:
- `test_factorial_recursion_*` (4 tests)
- `test_fibonacci_recursion`
- Multiple others requiring arithmetic

### New Tests to Add

```rust
#[test]
fn test_int_arithmetic_ffi() {
    let source = r#"(add 1 2)"#;
    let ast = parse(source).unwrap();
    let (bytecode, _) = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
    
    // Should contain HostCall for add
    assert!(bytecode.iter().any(|op| matches!(op, OpCode::HostCall { .. })));
    
    // Execute
    let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
    assert_eq!(vm.run().unwrap(), Value::Int(3));
}

#[test]
fn test_float_arithmetic_ffi() {
    let source = r#"(fadd 1.5 2.5)"#;
    // ... similar test
}
```

---

## Dependencies and Risks

### Dependencies
1. `HostFunction` enum extension → VM implementation
2. FFI registration → Compiler detection
3. VM handler → Integration tests

### Risks
1. **VM Integration:** The VM must properly handle new HostFunction variants
2. **Type Errors:** No type checking in compiler; runtime type errors possible
3. **Capability Confusion:** Need to decide if arithmetic requires capabilities

---

## Open Questions for Expert Feedback

1. **Type System:** Should we implement type inference/checking before this, or accept runtime type errors?

2. **Capability Model:** Should arithmetic be:
   - Available to all tiers (simplest)
   - Require a "Computation" capability
   - Have per-operation capabilities

3. **Naming Convention:** Should arithmetic use:
   - `add`, `mul`, `sub`, `div` (simple names)
   - `math:add`, `math:mul` (namespaced)
   - Different names to avoid conflicts with user code

4. **Mixed Types:** Should `(add 1 2.5)` work?
   - If yes, what are the rules for type promotion?
   - If no, when do we report the type error?

5. **Error Handling:** Should division by zero:
   - Return a special value (NaN, infinity)
   - Throw a runtime error
   - Panic

6. **Overflow:** Should integer overflow:
   - Wrap around (modular arithmetic)
   - Return error
   - Panic

---

## Success Criteria

- [ ] `add`, `mul`, `sub`, `div`, `mod` FFI functions registered
- [ ] `fadd`, `fmul`, `fsub`, `fdiv` FFI functions registered
- [ ] `(add 1 2)` returns `Int(3)`
- [ ] `(fadd 1.5 2.5)` returns `Float(4.0)`
- [ ] At least 10 previously ignored tests now pass
- [ ] No regressions in existing tests

---

## Timeline Estimate

| Task                      | Effort        | Risk   |
| ------------------------- | ------------- | ------ |
| Extend HostFunction enum  | 1 hour        | Low    |
| Implement VM handlers     | 2 hours       | Medium |
| Register FFI functions    | 1 hour        | Low    |
| Update compiler detection | 1 hour        | Low    |
| Unignore and run tests    | 30 min        | Low    |
| **Total**                 | **5.5 hours** | -      |



Response:

# Expert Guidance: Arithmetic FFI Implementation Strategy

## **Executive Summary**

You're on the right track with **Option A (Add HostFunctions)**, but there are critical design decisions needed. Based on your safety-critical context and Lisp-like language goals, here's the recommended approach.

## **1. HostFunction Enum Design**

### **Recommended: Separate Int/Float Functions**
```rust
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostFunction {
    // Existing system functions...
    ReadSensor = 0,
    WriteActuator = 1,
    // ...
    
    // New arithmetic functions
    IntAdd = 9,
    IntSub = 10,
    IntMul = 11,
    IntDiv = 12,
    IntMod = 13,
    
    FloatAdd = 14,
    FloatSub = 15,
    FloatMul = 16,
    FloatDiv = 17,
    
    // Type conversions
    IntToFloat = 18,
    FloatToInt = 19,
    
    // Comparison operations (useful for conditionals)
    IntEq = 20,
    IntLt = 21,
    IntGt = 22,
    
    FloatEq = 23,
    FloatLt = 24,
    FloatGt = 25,
}
```

**Why separate Int/Float?**
1. **Type Safety**: Catches type errors at registration/capability level
2. **Performance**: No runtime type dispatch overhead
3. **Simplicity**: Clear semantics, easier to debug
4. **Safety**: Prevents unexpected type conversions

**Numbering**: Continue from current last value (8). Check your serialization uses `#[repr(u8)]` or similar.

## **2. VM Handler Implementation**

### **Critical Safety Decisions**

#### **Division by Zero Handling**
```rust
match host_function {
    HostFunction::IntDiv => {
        let b = vm.stack.pop().unwrap();
        let a = vm.stack.pop().unwrap();
        
        let b_int = b.as_int().ok_or(VmError::TypeError)?;
        if b_int == 0 {
            // Return a special error value, NOT panic
            vm.stack.push(Value::Error("division by zero".to_string()));
            return Ok(());
        }
        vm.stack.push(Value::Int(a.as_int().ok_or(VmError::TypeError)? / b_int));
    }
    HostFunction::FloatDiv => {
        let b = vm.stack.pop().unwrap();
        let a = vm.stack.pop().unwrap();
        
        let b_float = b.as_float().ok_or(VmError::TypeError)?;
        if b_float == 0.0 {
            // For floats, return Infinity (consistent with IEEE 754)
            vm.stack.push(Value::Float(f64::INFINITY));
            return Ok(());
        }
        vm.stack.push(Value::Float(a.as_float().ok_or(VmError::TypeError)? / b_float));
    }
}
```

#### **Integer Overflow Handling**
```rust
match host_function {
    HostFunction::IntAdd => {
        let b = vm.stack.pop().unwrap();
        let a = vm.stack.pop().unwrap();
        
        let a_int = a.as_int().ok_or(VmError::TypeError)?;
        let b_int = b.as_int().ok_or(VmError::TypeError)?;
        
        // Checked arithmetic for safety
        match a_int.checked_add(b_int) {
            Some(result) => vm.stack.push(Value::Int(result)),
            None => vm.stack.push(Value::Error("integer overflow".to_string())),
        }
    }
    // Similar for IntSub, IntMul
}
```

**Why error values instead of panic?**
1. **Determinism**: VM continues execution, can be handled by caller
2. **Safety**: No abrupt termination in safety-critical systems
3. **Debuggability**: Error can be logged, inspected, recovered

## **3. Capability Model**

### **Recommended: Arithmetic as "Compute" Capability**

```rust
// Define a new capability tier
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Capability {
    // Existing capabilities
    SensorRead,
    ActuatorWrite,
    ClockAccess,
    
    // New for arithmetic
    Compute,  // Allows arithmetic operations
    ComputeFloating,  // Allows floating point (more expensive)
}
```

**Registration:**
```rust
registry.register_function(FfiFunction {
    name: "add".to_string(),
    host_function: HostFunction::IntAdd,
    required_capability: Capability::Compute,
    parameter_types: vec!["Int".to_string(), "Int".to_string()],
    return_type: "Int".to_string(),
    // ...
});

registry.register_function(FfiFunction {
    name: "fadd".to_string(),
    host_function: HostFunction::FloatAdd,
    required_capability: Capability::ComputeFloating,  // More restrictive
    // ...
});
```

**Why separate capabilities?**
1. **Resource Control**: Floating point might be expensive on embedded systems
2. **Security**: Restrict computation for untrusted code
3. **Accounting**: Track compute resource usage

## **4. Compiler Integration Strategy**

### **Recommended: Auto-detect with Symbol Table**

```rust
pub fn compile_call(
    &mut self,
    function: &AstNode,
    arguments: &[AstNode],
    in_tail_position: bool,
    symbol_table: &SymbolTable,  // Track local definitions
) -> Result<Vec<OpCode>, CompilationError> {
    match function {
        AstNode::Symbol(name) => {
            // 1. Check local definitions first
            if symbol_table.is_locally_defined(name) {
                return self.compile_local_call(name, arguments, in_tail_position);
            }
            
            // 2. Check FFI registry
            if let Some(ffi_func) = self.ffi_registry.find_function(name) {
                // Verify capability
                if !self.current_capabilities.contains(&ffi_func.required_capability) {
                    return Err(CompilationError::MissingCapability {
                        function: name.clone(),
                        required: ffi_func.required_capability,
                    });
                }
                
                return self.compile_ffi_call(name, arguments, ffi_func);
            }
            
            // 3. Treat as global symbol (for later linking)
            return self.compile_symbol_call(name, arguments, in_tail_position);
        }
        // ... handle other function expressions
    }
}
```

**Why this order?**
1. **Local shadowing**: User-defined `add` should override built-in
2. **Capability checking**: Early error if missing required capability
3. **Clear errors**: Users know why their code won't compile

## **5. Type System & Mixed Operations**

### **Recommendation: Strict Type Separation, Add Conversion Functions**

**No automatic mixed-type operations** - this avoids:
- Hidden performance costs (runtime checks)
- Unexpected type conversions
- Complexity in capability tracking

**Instead provide explicit conversions:**
```jue
(add 5 (float-to-int 3.14))  ; Explicit conversion
(fadd 2.5 (int-to-float 3))  ; Explicit conversion
```

**Type strings in FFI registration:**
```rust
parameter_types: vec!["Int".to_string(), "Int".to_string()],
// Not: vec!["Number".to_string()]  (too vague)
```

## **6. Naming Convention**

### **Recommendation: Use Prefix for Float Operations**

```rust
// Integer: add, sub, mul, div, mod
// Float: fadd, fsub, fmul, fdiv, fmod
// Comparison: eq, lt, gt (dispatches based on first arg type)
```

**Alternative: Namespaced approach** (if you plan many operations):
```jue
(math:int:add 1 2)
(math:float:add 1.5 2.5)
```

**Start with simple prefix**, add namespaces later if needed.

## **7. Error Handling Strategy**

### **Three-Tier Error Handling**

```rust
// In VM handler
fn handle_int_div(vm: &mut VmState) -> Result<(), VmError> {
    let b = vm.stack.pop().unwrap();
    let a = vm.stack.pop().unwrap();
    
    // 1. Type validation
    let (a_int, b_int) = match (a.as_int(), b.as_int()) {
        (Some(a), Some(b)) => (a, b),
        _ => {
            // Push error value, continue execution
            vm.stack.push(Value::TypeError("expected integers".to_string()));
            return Ok(());
        }
    };
    
    // 2. Domain validation (division by zero)
    if b_int == 0 {
        vm.stack.push(Value::DomainError("division by zero".to_string()));
        return Ok(());
    }
    
    // 3. Overflow check
    match a_int.checked_div(b_int) {
        Some(result) => vm.stack.push(Value::Int(result)),
        None => vm.stack.push(Value::OverflowError("integer overflow".to_string())),
    }
    
    Ok(())
}
```

**Error value types in Value enum:**
```rust
enum Value {
    Int(i64),
    Float(f64),
    // Error values
    TypeError(String),
    DomainError(String),
    OverflowError(String),
    // ...
}
```

## **8. Implementation Priority**

### **Phase 1: Basic Integer Operations (2-3 days)**
1. Add `IntAdd`, `IntSub`, `IntMul`, `IntDiv`, `IntMod` to HostFunction
2. Implement VM handlers with overflow/zero checking
3. Register with `Capability::Compute`
4. Compiler auto-detection with shadowing

### **Phase 2: Float Operations (1-2 days)**
1. Add float variants
2. Register with `Capability::ComputeFloating`
3. Add conversion functions `int-to-float`, `float-to-int`

### **Phase 3: Comparison Operations (1 day)**
1. Add `eq`, `lt`, `gt` for int/float
2. Enable conditional expressions

### **Phase 4: Error Handling Refinement (1 day)**
1. Add error value types
2. Standard error reporting
3. Error recovery mechanisms

## **9. Testing Strategy**

### **Critical Tests to Add**

```rust
#[test]
fn test_int_arithmetic_capability() {
    // Test with/without Compute capability
    let code = "(add 1 2)";
    
    // With Compute capability - should work
    let bytecode = compile_with_capabilities(code, &[Capability::Compute]);
    let result = vm.execute(bytecode);
    assert_eq!(result, Ok(Value::Int(3)));
    
    // Without Compute capability - should fail compilation
    let result = compile_with_capabilities(code, &[]);
    assert!(matches!(result, Err(CompilationError::MissingCapability { .. })));
}

#[test]
fn test_type_safety() {
    // Mixing int and float should produce type error
    let code = "(add 1 2.5)";
    let bytecode = compile(code);
    let result = vm.execute(bytecode);
    assert!(matches!(result, Ok(Value::TypeError(_))));
}

#[test]
fn test_local_shadowing() {
    // User-defined add should shadow built-in
    let code = "
        (let (add (lambda (x y) 42))
          (add 1 2))";  // Should return 42, not 3
    let result = execute(code);
    assert_eq!(result, Ok(Value::Int(42)));
}
```

## **10. Key Design Decisions Summary**

| Decision             | Recommendation                  | Why                                    |
| -------------------- | ------------------------------- | -------------------------------------- |
| Int/Float separation | Separate functions              | Type safety, performance, clarity      |
| Error handling       | Return error values             | Deterministic, debuggable, recoverable |
| Capabilities         | `Compute` and `ComputeFloating` | Resource control, security             |
| Type mixing          | No auto-conversion              | Predictable, explicit is better        |
| Naming               | Prefix (`fadd`)                 | Simple, familiar, extensible           |
| Local shadowing      | Allowed                         | Language consistency, user flexibility |

## **11. Migration Path for Existing Tests**

For your ignored tests, they'll need:
1. **Add required capabilities** to test contexts
2. **Use correct function names** (`add` vs `fadd`)
3. **Handle explicit type conversions** where needed

**Example test update:**
```rust
// Before: Might have used implicit arithmetic
// After: Must use explicit FFI functions
let source = r#"
    (let ((x 5) (y 3))
      (add x y))  ; Explicit FFI call
"#;
```

## **12. Risk Mitigation**

1. **Backward Compatibility**: New capabilities won't break existing code (they don't use arithmetic)
2. **Performance**: Checked arithmetic has overhead but is essential for safety
3. **Complexity**: Start simple (prefix naming), add namespaces later if needed
4. **Testing**: Unignore tests gradually as each phase is implemented

## **Final Recommendation**

Implement **Phase 1 (Integer Operations)** first. This gives you:
- Working arithmetic for most use cases
- All the architectural patterns you need
- Ability to test with existing factorial/fibonacci tests
- Foundation for adding floats later

The key insight: **Don't try to build a full numeric tower** (automatic promotions, mixed types, etc.). Keep it simple, explicit, and safe - appropriate for a safety-critical Lisp-like language.

Your approach is sound. The main adjustments needed are:
1. Error handling via return values, not panics
2. Clear capability separation for int vs float
3. Strict type checking with explicit conversions
4. Local symbol shadowing of built-ins

This will give you a robust arithmetic foundation that aligns with your safety-critical goals.