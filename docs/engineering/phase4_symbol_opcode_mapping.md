# Phase 4: Symbol to Opcode Mapping

## Overview

This document details the implementation plan for mapping symbol names (like `add`, `mul`, `eq`) directly to VM opcodes, enabling a more efficient and natural syntax for arithmetic and comparison operations.

## Current State Analysis

### Current Behavior

Currently, `Symbol("add")` compiles to:
```rust
pub fn compile_symbol(&mut self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
    let symbol_index = self.get_string_index(name);
    Ok(vec![OpCode::Symbol(symbol_index)])  // Just stores the symbol name
}
```

This emits a `Symbol(index)` opcode that pushes a symbol value onto the stack. It doesn't perform any operation.

### Expected Behavior (Tests)

Tests expect:
```jue
(Symbol "add")  ;; or
(add 1 2)
```

To compile directly to:
```rust
OpCode::Add  // Or OpCode::FAdd for floats
```

### Test Expectations

From `test_symbol_compilation.rs`:
```rust
#[test]
fn test_symbol_add_compilation() {
    let symbol_node = AstNode::Symbol("add".to_string());
    let result = compile_to_physics_world(&symbol_node, TrustTier::Empirical);
    
    assert!(result.is_ok());
    let (bytecode, _) = result.unwrap();
    
    assert_eq!(bytecode.len(), 1);
    assert_eq!(bytecode[0], OpCode::Add);
}
```

---

## Implementation Options

### Option A: Direct Symbol Mapping in compile_symbol()

**Approach:** Modify `compile_symbol()` to check for known symbol names and return the corresponding opcode.

```rust
pub fn compile_symbol(&mut self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
    // Check for arithmetic operations
    match name {
        "add" => return Ok(vec![OpCode::IAdd]),
        "fadd" => return Ok(vec![OpCode::FAdd]),
        "sub" => return Ok(vec![OpCode::ISub]),
        "fsub" => return Ok(vec![OpCode::FSub]),
        "mul" => return Ok(vec![OpCode::IMul]),
        "fmul" => return Ok(vec![OpCode::FMul]),
        "div" => return Ok(vec![OpCode::IDiv]),
        "fdiv" => return Ok(vec![OpCode::FDiv]),
        "mod" => return Ok(vec![OpCode::Mod]),
        
        // Comparison
        "eq" => return Ok(vec![OpCode::Eq]),
        "lt" => return Ok(vec![OpCode::Lt]),
        "gt" => return Ok(vec![OpCode::Gt]),
        "le" => return Ok(vec![OpCode::Le]),
        "ge" => return Ok(vec![OpCode::Ge]),
        
        // Stack operations
        "swap" => return Ok(vec![OpCode::Swap]),
        "dup" => return Ok(vec![OpCode::Dup]),
        "pop" => return Ok(vec![OpCode::Pop]),
        
        // String operations
        "str-concat" => return Ok(vec![OpCode::StrConcat]),
        "str-len" => return Ok(vec![OpCode::StrLen]),
        "str-index" => return Ok(vec![OpCode::StrIndex]),
        
        // Fall back to symbol
        _ => {
            let symbol_index = self.get_string_index(name);
            Ok(vec![OpCode::Symbol(symbol_index)])
        }
    }
}
```

**Pros:**
- Simple implementation
- Fast lookup (O(1) match)
- Easy to understand

**Cons:**
- Hardcoded mapping
- No extensibility
- Duplicates logic elsewhere

### Option B: Symbol Registry

**Approach:** Create a registry mapping symbol names to opcodes, similar to FFI registry.

```rust
pub struct SymbolRegistry {
    mappings: HashMap<String, OpCode>,
}

impl SymbolRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            mappings: HashMap::new(),
        };
        // Register standard mappings
        registry.register("add", OpCode::IAdd);
        registry.register("fadd", OpCode::FAdd);
        // ...
        registry
    }
    
    pub fn lookup(&self, name: &str) -> Option<OpCode> {
        self.mappings.get(name).copied()
    }
}
```

**Pros:**
- Extensible
- Can be configured
- Cleaner separation

**Cons:**
- More code
- Slightly more complex

### Option C: Hybrid (Default to Symbol, Special Cases Direct)

**Approach:** Keep current behavior for unknown symbols, but detect special cases.

```rust
pub fn compile_symbol(&mut self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
    // Check if this is a known operation
    if let Some(opcode) = self.symbol_registry.lookup(name) {
        return Ok(vec![opcode]);
    }
    
    // Fall back to Symbol opcode
    let symbol_index = self.get_string_index(name);
    Ok(vec![OpCode::Symbol(symbol_index)])
}
```

**Pros:**
- Backward compatible
- Flexible
- Can be extended

**Cons:**
- More complex
- Requires registry maintenance

---

## Implementation Plan (Option A - Simplest)

### Step 1: Define Symbol Mapping Constants

**File:** `jue_world/src/physics_integration/physics_compiler.rs`

Add a mapping structure near the top of the file:

```rust
/// Mapping from symbol names to opcodes for built-in operations
const SYMBOL_TO_OPCODE: &[(&str, OpCode)] = &[
    // Arithmetic (integer)
    ("add", OpCode::IAdd),
    ("sub", OpCode::ISub),
    ("mul", OpCode::IMul),
    ("div", OpCode::IDiv),
    ("mod", OpCode::Mod),
    
    // Arithmetic (float)
    ("fadd", OpCode::FAdd),
    ("fsub", OpCode::FSub),
    ("fmul", OpCode::FMul),
    ("fdiv", OpCode::FDiv),
    
    // Comparison
    ("eq", OpCode::Eq),
    ("lt", OpCode::Lt),
    ("gt", OpCode::Gt),
    ("le", OpCode::Le),
    ("ge", OpCode::Ge),
    
    // Stack operations
    ("swap", OpCode::Swap),
    ("dup", OpCode::Dup),
    ("pop", OpCode::Pop),
    
    // String operations
    ("str-concat", OpCode::StrConcat),
    ("str-len", OpCode::StrLen),
    ("str-index", OpCode::StrIndex),
];
```

### Step 2: Update compile_symbol()

```rust
pub fn compile_symbol(&mut self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
    // Check for known operation symbols
    for (symbol_name, opcode) in SYMBOL_TO_OPCODE {
        if name == *symbol_name {
            return Ok(vec![*opcode]);
        }
    }
    
    // Unknown symbol - emit as Symbol opcode
    let symbol_index = self.get_string_index(name);
    Ok(vec![OpCode::Symbol(symbol_index)])
}
```

### Step 3: Handle Unknown Symbols

Update the `test_unknown_symbol_error` test expectation:

Currently expects error for unknown symbols. With this change:
- Known symbols → direct opcode
- Unknown symbols → Symbol opcode (no error)

**Decision needed:** Should unknown symbols still error, or emit Symbol opcode?

### Step 4: Ensure OpCode Variants Exist

Check that all expected opcodes exist in `physics_world::types::OpCode`:

```rust
// Should have:
IAdd, ISub, IMul, IDiv, Mod
FAdd, FSub, FMul, FDiv
Eq, Lt, Gt, Le, Ge
Swap, Dup, Pop
StrConcat, StrLen, StrIndex
```

**Questions for Expert Feedback:**
1. Should we use `Add`/`FAdd` or `IAdd`/`FAdd` naming convention?
2. Should mixed-type operations (e.g., `add` with int and float) be supported?
3. Should we error on unknown symbols or emit Symbol opcode?

---

## Test Plan

### Tests to Unignore

From `test_symbol_compilation.rs`:
- `test_symbol_add_compilation`
- `test_symbol_arithmetic_operations`
- `test_symbol_comparison_operations`
- `test_symbol_stack_operations`
- `test_symbol_string_operations`
- `test_unknown_symbol_error`
- `test_symbol_within_expression`
- `test_physics_compiler_symbol_case`

### Test Cases to Add

```rust
#[test]
fn test_all_arithmetic_symbols() {
    let ops = [
        ("add", OpCode::IAdd),
        ("sub", OpCode::ISub),
        ("mul", OpCode::IMul),
        ("div", OpCode::IDiv),
        ("mod", OpCode::Mod),
        ("fadd", OpCode::FAdd),
        ("fmul", OpCode::FMul),
    ];
    
    for (name, expected) in ops {
        let ast = AstNode::Symbol(name.to_string());
        let result = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
        assert_eq!(result.0, vec![expected], "Failed for {}", name);
    }
}

#[test]
fn test_unknown_symbol_emits_symbol_opcode() {
    let ast = AstNode::Symbol("my-custom-symbol".to_string());
    let result = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
    
    // Should emit Symbol opcode, not error
    assert!(matches!(result.0[0], OpCode::Symbol(_)));
}

#[test]
fn test_symbol_in_call_expression() {
    // (add 1 2) should compile to: Int(1), Int(2), Add
    let ast = AstNode::Call {
        function: Box::new(AstNode::Symbol("add".to_string())),
        arguments: vec![
            AstNode::Literal(Literal::Int(1)),
            AstNode::Literal(Literal::Int(2)),
        ],
        location: Default::default(),
    };
    
    let result = compile_to_physics_world(&ast, TrustTier::Formal).unwrap();
    assert!(result.0.contains(&OpCode::IAdd));
}
```

---

## Symbol Mapping Table

| Symbol Name  | Opcode    | Type       | Notes                  |
| ------------ | --------- | ---------- | ---------------------- |
| `add`        | IAdd      | int        | Integer addition       |
| `sub`        | ISub      | int        | Integer subtraction    |
| `mul`        | IMul      | int        | Integer multiplication |
| `div`        | IDiv      | int        | Integer division       |
| `mod`        | Mod       | int        | Modulo                 |
| `fadd`       | FAdd      | float      | Float addition         |
| `fsub`       | FSub      | float      | Float subtraction      |
| `fmul`       | FMul      | float      | Float multiplication   |
| `fdiv`       | FDiv      | float      | Float division         |
| `eq`         | Eq        | comparison | Equality               |
| `lt`         | Lt        | comparison | Less than              |
| `gt`         | Gt        | comparison | Greater than           |
| `le`         | Le        | comparison | Less or equal          |
| `ge`         | Ge        | comparison | Greater or equal       |
| `swap`       | Swap      | stack      | Swap top two           |
| `dup`        | Dup       | stack      | Duplicate top          |
| `pop`        | Pop       | stack      | Pop top                |
| `str-concat` | StrConcat | string     | Concatenate            |
| `str-len`    | StrLen    | string     | Length                 |
| `str-index`  | StrIndex  | string     | Index                  |

---

## Open Questions for Expert Feedback

1. **Naming Convention:** Should we use:
   - `add`/`fadd` (separate names)
   - `add`/`addf` (suffix variant)
   - `+`/`+.` (symbolic names)

2. **Mixed Types:** Should `(add 1 2.5)` work?
   - If yes, what's the result type?
   - If no, when is the error detected?

3. **Unknown Symbols:** Should unknown symbols:
   - Emit `Symbol` opcode (current with changes)
   - Error at compile time
   - Warning at compile time

4. **Extensibility:** Should users be able to:
   - Define custom symbols?
   - Override built-in symbols?
   - Define symbols in libraries?

5. **Namespace:** Should we support:
   - `math:add`
   - `str:concat`
   - Simple names only

---

## Dependencies and Risks

### Dependencies
1. All opcodes must exist in `physics_world::types::OpCode`
2. VM must implement all opcodes
3. Tests expect specific behavior

### Risks
1. **Naming Conflicts:** User-defined functions named "add" will conflict
2. **Opcode Gaps:** Some expected opcodes may not exist
3. **Test Expectations:** Tests may need adjustment

---

## Success Criteria

- [ ] `Symbol("add")` compiles to `OpCode::IAdd`
- [ ] `Symbol("mul")` compiles to `OpCode::IMul`
- [ ] All arithmetic, comparison, stack, string symbols work
- [ ] Unknown symbols emit `Symbol` opcode (no error)
- [ ] All 8 symbol tests pass
- [ ] `(add 1 2)` generates correct bytecode

---

## Timeline Estimate

| Task                    | Effort        | Risk |
| ----------------------- | ------------- | ---- |
| Define symbol mapping   | 1 hour        | Low  |
| Update compile_symbol   | 1 hour        | Low  |
| Verify opcode existence | 30 min        | Low  |
| Update tests            | 1 hour        | Low  |
| **Total**               | **3.5 hours** | -    |