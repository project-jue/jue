# Physics-World Integration TODO Implementation Guide

## Executive Summary

This document provides a comprehensive engineering plan for implementing critical TODO features in the Physics-World integration layer of Project Jue. The current implementation has significant gaps that prevent proper execution of Jue programs on the Physics-World virtual machine. These TODO items represent fundamental functionality required for:

- **Float and String Literal Support**: Currently hardcoded to return Nil values
- **Variable Resolution**: Missing environment-based variable lookup
- **Closure Environment Capture**: Improper lexical scoping implementation
- **Trust Tier Enforcement**: Missing capability checking and sandboxing

**Priority Level**: CRITICAL - These features are blocking basic program execution and must be implemented before any meaningful testing can occur.

## Architecture Context

### Physics-World in Project Jue Architecture

Physics-World serves as the foundational execution engine that bridges the gap between high-level Jue code and deterministic, verifiable machine execution. It operates in the context of Project Jue's layered architecture:

```
Dan-World (Cognitive Layer) 
    ↓ Compiles to
Jue-World (Execution Engine)
    ↓ Translates to  
Core-World (Formal Kernel)
    ↓ Executes on
Physics-World (Runtime VM)
```

### Integration Points

The Physics-World integration (`jue_world/src/integration/physics.rs`) serves as the compilation bridge that:
1. Takes AST nodes from Jue-World
2. Generates appropriate OpCodes for Physics-World VM
3. Applies trust tier-specific transformations
4. Enforces capability and security constraints

### Current Implementation Status

The `PhysicsWorldCompiler` struct has partial implementation with placeholder TODOs for critical features. The current state shows:
- ✅ Basic AST traversal structure
- ✅ Simple literal handling (for Int, Bool, Nil)
- ✅ Function call compilation framework
- ❌ **MISSING**: Float/String literal support
- ❌ **MISSING**: Variable environment management
- ❌ **MISSING**: Proper closure creation with lexical scoping
- ❌ **MISSING**: Trust tier enforcement mechanisms

## Current Implementation Analysis

### File: `jue_world/src/integration/physics.rs`

#### Current Compiler Structure
```rust
pub struct PhysicsWorldCompiler {
    pub tier: TrustTier,
    pub location: SourceLocation,
    pub capability_indices: Vec<Capability>,
    pub ffi_registry: FfiCallGenerator,
}
```

#### Critical TODO Analysis

**1. Float Literals (Lines 85-87)**
```rust
crate::ast::Literal::Float(f) => {
    // TODO: Handle float literals properly
    OpCode::Int(*f as i64)  // BUG: Lossy conversion
}
```

**Issues:**
- Forced conversion from `f64` to `i64` causes precision loss
- No `OpCode::Float` variant exists in current OpCode enum
- Physics-World VM expects IEEE 754 float support

**2. String Literals (Lines 89-91)**
```rust
crate::ast::Literal::String(_s) => {
    // TODO: Handle string literals properly  
    OpCode::Nil  // BUG: All strings become nil
}
```

**Issues:**
- Complete absence of string handling
- No string constant pool support
- Missing string-specific OpCodes

**3. Variable Lookup (Lines 99-102)**
```rust
fn compile_variable(&self, _name: &str) -> Result<Vec<OpCode>, CompilationError> {
    // TODO: Implement variable lookup and loading
    Ok(vec![OpCode::Nil])  // BUG: All variables are nil
}
```

**Issues:**
- No environment management
- No lexical scoping support
- Variables cannot be resolved during execution

**4. Closure Creation (Lines 141-142)**
```rust
// TODO: Implement proper closure creation with environment capture
bytecode.push(OpCode::MakeClosure(0, parameters.len()));
```

**Issues:**
- Hardcoded code_idx = 0
- No environment capture mechanism
- Missing lexical variable resolution

**5. Capability Check Insertion (Lines 309-312)**
```rust
pub fn insert_runtime_capability_checks(
    &mut self,
    bytecode: Vec<OpCode>,
) -> Result<Vec<OpCode>, CompilationError> {
    // TODO: Implement capability check insertion
    Ok(bytecode)  // BUG: No capability checks added
}
```

**6. Sandbox Wrapper (Lines 319-322)**
```rust
pub fn add_sandbox_wrapper(
    &mut self,
    bytecode: Vec<OpCode>,
) -> Result<Vec<OpCode>, CompilationError> {
    // TODO: Implement sandbox wrapper  
    Ok(bytecode)  // BUG: No sandboxing applied
}
```

## TODO Features Breakdown

### 1. Float Literal Handling

**Current Issue**: `OpCode::Int(*f as i64)` causes precision loss and incorrect behavior.

**Required Implementation**:
1. **Add OpCode::Float variant** to Physics-World VM
2. **Implement Float constant pool** for deduplication
3. **Add Float arithmetic operations** to VM
4. **Update literal compilation** to use proper float handling

**Implementation Details**:

#### Step 1: Update OpCode Enum
```rust
// In physics_world/src/types/core.rs
pub enum OpCode {
    // ... existing variants
    Float(f64),  // NEW: Direct float constant
    // ... other variants
}

impl OpCode {
    pub fn size_bytes(&self) -> usize {
        match self {
            OpCode::Float(_) => 9,  // f64 (8 bytes) + opcode tag (1 byte)
            // ... other cases
        }
    }
}
```

#### Step 2: Update Value Enum
```rust
// In physics_world/src/types/core.rs
pub enum Value {
    // ... existing variants
    Float(f64),  // NEW: Float value type
    // ... other variants
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Float(fl) => write!(f, "{}", fl),
            // ... other cases
        }
    }
}
```

#### Step 3: Update Literal Compilation
```rust
// In jue_world/src/integration/physics.rs
fn compile_literal(&self, lit: &crate::ast::Literal) -> Result<Vec<OpCode>, CompilationError> {
    let opcode = match lit {
        crate::ast::Literal::Nil => OpCode::Nil,
        crate::ast::Literal::Bool(b) => OpCode::Bool(*b),
        crate::ast::Literal::Int(i) => OpCode::Int(*i),
        crate::ast::Literal::Float(f) => OpCode::Float(*f),  // FIXED: Proper float handling
        crate::ast::Literal::String(s) => {
            // Handle string literals (see String section)
            self.compile_string_literal(s)?
        }
    };
    Ok(vec![opcode])
}
```

#### Step 4: Add Float Operations to VM
```rust
// Add to VM execution loop in physics_world/src/vm/mod.rs
match opcode {
    OpCode::Float(f) => {
        vm.stack.push(Value::Float(f));
    }
    OpCode::FAdd => {
        let b = vm.pop_number()?;
        let a = vm.pop_number()?;
        vm.stack.push(Value::Float(a + b));
    }
    // Similar for FSub, FMul, FDiv, etc.
    _ => return Err(VmError::UnknownOpcode),
}
```

### 2. String Literal Handling

**Current Issue**: All strings return `OpCode::Nil`, making string operations impossible.

**Required Implementation**:
1. **Add string constant pool** to compiler
2. **Implement string OpCodes** for loading constants
3. **Add string operations** (length, concatenation, indexing)
4. **Handle string escaping** in compilation

**Implementation Details**:

#### Step 1: Add String Support to Compiler
```rust
// In jue_world/src/integration/physics.rs
pub struct PhysicsWorldCompiler {
    // ... existing fields
    pub string_pool: HashMap<String, usize>,  // NEW: String deduplication
    pub string_constants: Vec<String>,        // NEW: String constant storage
}

impl PhysicsWorldCompiler {
    pub fn new(tier: TrustTier) -> Self {
        Self {
            // ... existing fields
            string_pool: HashMap::new(),
            string_constants: Vec::new(),
        }
    }

    fn compile_string_literal(&mut self, s: &str) -> Result<OpCode, CompilationError> {
        // Add string to constant pool if not present
        let index = if let Some(&idx) = self.string_pool.get(s) {
            idx
        } else {
            let idx = self.string_constants.len();
            self.string_constants.push(s.to_string());
            self.string_pool.insert(s.to_string(), idx);
            idx
        };
        Ok(OpCode::LoadString(index))
    }
}
```

#### Step 2: Add String OpCodes
```rust
// In physics_world/src/types/core.rs
pub enum OpCode {
    // ... existing variants
    LoadString(usize),  // NEW: Load string from constant pool
    StrLen,            // NEW: Get string length
    StrConcat,         // NEW: Concatenate strings
    StrIndex,          // NEW: Index into string
    // ... other variants
}
```

#### Step 3: Update Value Enum
```rust
// In physics_world/src/types/core.rs
pub enum Value {
    // ... existing variants
    String(String),  // NEW: String value type
    // ... other variants
}
```

### 3. Variable Lookup and Environment Management

**Current Issue**: All variables resolve to `OpCode::Nil`, making variable usage impossible.

**Required Implementation**:
1. **Environment tracking** during compilation
2. **Variable resolution** with proper scoping
3. **Environment capture** for closures
4. **GetLocal/SetLocal OpCode** generation

**Implementation Details**:

#### Step 1: Add Environment Management
```rust
// In jue_world/src/integration/physics.rs
#[derive(Clone, Debug)]
struct CompilationEnvironment {
    variables: HashMap<String, usize>,  // Variable name -> stack offset
    parent: Option<Box<CompilationEnvironment>>,
}

pub struct PhysicsWorldCompiler {
    // ... existing fields
    pub current_env: CompilationEnvironment,  // NEW: Current environment
    pub env_stack: Vec<CompilationEnvironment>, // NEW: Environment stack
}

impl PhysicsWorldCompiler {
    pub fn new(tier: TrustTier) -> Self {
        Self {
            // ... existing fields
            current_env: CompilationEnvironment {
                variables: HashMap::new(),
                parent: None,
            },
            env_stack: Vec::new(),
        }
    }

    fn push_environment(&mut self) {
        let new_env = CompilationEnvironment {
            variables: HashMap::new(),
            parent: Some(Box::new(self.current_env.clone())),
        };
        self.env_stack.push(self.current_env.clone());
        self.current_env = new_env;
    }

    fn pop_environment(&mut self) {
        if let Some(prev_env) = self.env_stack.pop() {
            self.current_env = prev_env;
        }
    }

    fn resolve_variable(&self, name: &str) -> Option<usize> {
        // Search current environment first, then parent environments
        if let Some(&offset) = self.current_env.variables.get(name) {
            Some(offset)
        } else if let Some(ref parent) = self.current_env.parent {
            parent.variables.get(name).copied()
        } else {
            None
        }
    }
}
```

#### Step 2: Update Variable Compilation
```rust
// In jue_world/src/integration/physics.rs
fn compile_variable(&mut self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
    match self.resolve_variable(name) {
        Some(offset) => Ok(vec![OpCode::GetLocal(offset as u16)]),
        None => Err(CompilationError::InternalError(format!(
            "Undefined variable: {}",
            name
        ))),
    }
}

fn compile_lambda(
    &mut self,
    parameters: &[String],
    body: &AstNode,
) -> Result<Vec<OpCode>, CompilationError> {
    let mut bytecode = Vec::new();

    // Push new environment for lambda parameters
    self.push_environment();

    // Add parameters to environment
    for (i, param) in parameters.iter().enumerate() {
        self.current_env.variables.insert(param.clone(), i);
    }

    // Compile body
    let body_bytecode = self.compile_to_physics(body)?;
    bytecode.extend(body_bytecode);

    // Create closure with environment capture
    bytecode.push(OpCode::MakeClosure(0, parameters.len()));

    // Pop environment
    self.pop_environment();

    Ok(bytecode)
}
```

### 4. Proper Closure Creation with Environment Capture

**Current Issue**: `OpCode::MakeClosure(0, parameters.len())` has hardcoded parameters and no environment capture.

**Required Implementation**:
1. **Environment capture mechanism** during closure creation
2. **Proper code index resolution** for closure bodies
3. **Lexical scoping** with variable resolution
4. **Closure value management** in VM

**Implementation Details**:

#### Step 1: Update MakeClosure Implementation
```rust
// In physics_world/src/vm/opcodes/make_closure.rs
pub fn handle_make_closure(
    vm: &mut VmState,
    code_idx: usize,
    capture_count: usize,
) -> Result<Value, VmError> {
    // 1. Validate capture count
    if vm.stack.len() < capture_count {
        return Err(VmError::StackUnderflow);
    }

    // 2. Get closure body from constant pool
    let closure_body_value = vm.constant_pool.get(code_idx)
        .and_then(|v| match v {
            Value::Closure(body_ptr) => Some(*body_ptr),
            _ => None,
        })
        .ok_or(VmError::InvalidClosure)?;

    // 3. Calculate closure size (4 bytes body ptr + 4 bytes per captured value)
    let size = 4 + (capture_count as u32 * 4);
    let closure_ptr = vm.memory.allocate(size, 2)
        .map_err(|_| VmError::MemoryLimitExceeded)?;

    // 4. Store closure body pointer and captured values
    let data = unsafe { vm.memory.get_data_mut(closure_ptr) };

    // Store closure body pointer (first 4 bytes)
    let body_ptr_bytes = closure_body_value.get().to_le_bytes();
    data[0..4].copy_from_slice(&body_ptr_bytes);

    // Store captured values from stack
    for i in 0..capture_count {
        let stack_idx = vm.stack.len() - (capture_count - i);
        let value = &vm.stack[stack_idx];
        let value_bytes = serialize_value_to_bytes(value)?;
        let start = 4 + (i * 4);
        data[start..start + 4].copy_from_slice(&value_bytes);
    }

    // 5. Remove captured values from stack
    for _ in 0..capture_count {
        vm.stack.pop();
    }

    Ok(Value::Closure(closure_ptr))
}
```

#### Step 2: Update Variable Access in Closures
```rust
// Add to VM execution loop
match opcode {
    OpCode::GetLocal(offset) => {
        let frame = vm.call_stack.last()
            .ok_or(VmError::NoCallFrame)?;
        let value = frame.locals.get(offset as usize)
            .cloned()
            .unwrap_or(Value::Nil);
        vm.stack.push(value);
    }
    OpCode::SetLocal(offset) => {
        let value = vm.pop_value()?;
        let frame = vm.call_stack.last_mut()
            .ok_or(VmError::NoCallFrame)?;
        if offset as usize >= frame.locals.len() {
            frame.locals.resize(offset as usize + 1, Value::Nil);
        }
        frame.locals[offset as usize] = value;
    }
    _ => return Err(VmError::UnknownOpcode),
}
```

### 5. Capability Check Insertion for Empirical Tier

**Current Issue**: `insert_runtime_capability_checks` returns bytecode unchanged, bypassing trust tier enforcement.

**Required Implementation**:
1. **Capability analysis** during compilation
2. **Runtime check insertion** for Empirical tier
3. **Capability validation** at execution time
4. **Failure handling** for insufficient capabilities

**Implementation Details**:

#### Step 1: Capability Analysis
```rust
// In jue_world/src/integration/physics.rs
impl PhysicsWorldCompiler {
    pub fn analyze_capabilities(&self, ast: &AstNode) -> HashSet<Capability> {
        let mut capabilities = HashSet::new();
        self.collect_capabilities(ast, &mut capabilities);
        capabilities
    }

    fn collect_capabilities(&self, ast: &AstNode, caps: &mut HashSet<Capability>) {
        match ast {
            AstNode::FfiCall { function, .. } => {
                if let Ok(cap) = self.get_ffi_capability(function) {
                    caps.insert(cap);
                }
            }
            AstNode::RequireCapability { capability, .. } => {
                if let Ok(cap) = self.parse_capability(capability) {
                    caps.insert(cap);
                }
            }
            // Recursively check child nodes
            _ => {
                self.walk_ast(ast, |node| self.collect_capabilities(node, caps));
            }
        }
    }

    pub fn insert_runtime_capability_checks(
        &mut self,
        mut bytecode: Vec<OpCode>,
    ) -> Result<Vec<OpCode>, CompilationError> {
        // Analyze required capabilities
        let required_caps = self.analyze_capabilities_from_bytecode(&bytecode);

        // Insert capability checks at the beginning
        let mut check_bytecode = Vec::new();
        for cap in required_caps {
            let cap_index = self.get_capability_index(&cap);
            check_bytecode.push(OpCode::HasCap(cap_index));
            check_bytecode.push(OpCode::JmpIfFalse(2)); // Skip if no capability
        }

        // Prepend capability checks
        check_bytecode.extend(bytecode);
        Ok(check_bytecode)
    }
}
```

### 6. Sandbox Wrapper for Experimental Tier

**Current Issue**: `add_sandbox_wrapper` returns bytecode unchanged, providing no security isolation.

**Required Implementation**:
1. **Resource limit enforcement** (time, memory, operations)
2. **Capability isolation** for untrusted code
3. **Error boundary** creation for fault containment
4. **Monitoring hooks** for experimental code execution

**Implementation Details**:

#### Step 1: Sandbox Wrapper Implementation
```rust
// In jue_world/src/integration/physics.rs
impl PhysicsWorldCompiler {
    pub fn add_sandbox_wrapper(
        &mut self,
        mut bytecode: Vec<OpCode>,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut wrapper = Vec::new();

        // 1. Add resource monitoring initialization
        wrapper.push(OpCode::InitSandbox);

        // 2. Add capability isolation setup
        wrapper.push(OpCode::IsolateCapabilities);

        // 3. Add error boundary setup
        let error_handler_offset = bytecode.len() + 2; // After wrapper setup
        wrapper.push(OpCode::SetErrorHandler(error_handler_offset as i16));

        // 4. Add main bytecode
        wrapper.extend(bytecode);

        // 5. Add cleanup and result return
        wrapper.push(OpCode::CleanupSandbox);
        wrapper.push(OpCode::Ret);

        // 6. Add error handler
        wrapper.push(OpCode::LogSandboxViolation);
        wrapper.push(OpCode::Ret); // Return nil on sandbox violation

        Ok(wrapper)
    }
}
```

#### Step 2: Add Sandbox OpCodes
```rust
// In physics_world/src/types/core.rs
pub enum OpCode {
    // ... existing variants
    InitSandbox,              // NEW: Initialize sandbox environment
    IsolateCapabilities,      // NEW: Isolate capability access
    SetErrorHandler(i16),     // NEW: Set error handler jump target
    LogSandboxViolation,      // NEW: Log sandbox violation
    CleanupSandbox,           // NEW: Cleanup sandbox resources
    // ... other variants
}
```

## Implementation Roadmap

### Phase 1: Core Data Types (Priority: P0 - Critical)
**Timeline**: 1-2 weeks

**Tasks**:
1. **Float Literal Support**
   - Add `OpCode::Float` and `Value::Float` variants
   - Implement float literal compilation
   - Add basic float arithmetic operations

2. **String Literal Support**
   - Implement string constant pool
   - Add string OpCodes (`LoadString`, `StrLen`, etc.)
   - Handle string escaping in compilation

**Success Criteria**:
- Float literals compile and execute correctly
- String literals compile and execute correctly
- Basic arithmetic operations work with floats

**Dependencies**: None (foundational)

### Phase 2: Variable Environment (Priority: P0 - Critical)
**Timeline**: 2-3 weeks

**Tasks**:
1. **Environment Management**
   - Implement compilation environment tracking
   - Add lexical scoping support
   - Implement variable resolution

2. **Variable Operations**
   - Update `compile_variable` to generate proper GetLocal/SetLocal
   - Handle variable shadowing
   - Support nested scope environments

**Success Criteria**:
- Variables can be defined and accessed correctly
- Lexical scoping works as expected
- Variable shadowing behaves correctly

**Dependencies**: Phase 1 complete

### Phase 3: Closure Implementation (Priority: P1 - High)
**Timeline**: 2-3 weeks

**Tasks**:
1. **Environment Capture**
   - Implement proper closure creation with captured variables
   - Update MakeClosure opcode to handle environment capture
   - Support lexical variable access in closures

2. **Call Frame Management**
   - Enhance call frame handling for closures
   - Implement proper return value handling
   - Support recursive closure calls

**Success Criteria**:
- Closures capture their environment correctly
- Variable access within closures works properly
- Recursive functions work correctly

**Dependencies**: Phase 2 complete

### Phase 4: Trust Tier Enforcement (Priority: P1 - High)
**Timeline**: 2-3 weeks

**Tasks**:
1. **Capability Analysis**
   - Implement capability analysis during compilation
   - Add runtime capability checking for Empirical tier
   - Generate appropriate capability check bytecode

2. **Sandbox Implementation**
   - Add sandbox wrapper for Experimental tier
   - Implement resource limit enforcement
   - Add error boundary handling

**Success Criteria**:
- Empirical tier enforces capability requirements
- Experimental tier runs in sandboxed environment
- Capability violations are properly detected and handled

**Dependencies**: Phases 1-3 complete

## Code Examples

### Example 1: Float Literal Compilation

```rust
// Input Jue code:
// (add 3.14 2.71)

let ast = AstNode::Call {
    function: Box::new(AstNode::Symbol("add".to_string())),
    arguments: vec![
        AstNode::Literal(Literal::Float(3.14)),
        AstNode::Literal(Literal::Float(2.71)),
    ],
    location: SourceLocation::default(),
};

// Expected bytecode:
let expected = vec![
    OpCode::Float(3.14),
    OpCode::Float(2.71),
    OpCode::FAdd,
    OpCode::Ret,
];
```

### Example 2: Variable Environment Management

```rust
// Input Jue code:
// (let ((x 10)) (add x 5))

let ast = AstNode::Let {
    bindings: vec![("x".to_string(), AstNode::Literal(Literal::Int(10)))],
    body: Box::new(AstNode::Call {
        function: Box::new(AstNode::Symbol("add".to_string())),
        arguments: vec![
            AstNode::Variable("x".to_string()),
            AstNode::Literal(Literal::Int(5)),
        ],
        location: SourceLocation::default(),
    }),
    location: SourceLocation::default(),
};

// Expected bytecode:
let expected = vec![
    OpCode::Int(10),
    OpCode::SetLocal(0),  // x = 10
    OpCode::GetLocal(0),  // load x
    OpCode::Int(5),
    OpCode::Add,
    OpCode::Ret,
];
```

### Example 3: Closure with Environment Capture

```rust
// Input Jue code:
// ((lambda (x) (add x y)) 5)  // where y = 3 in outer scope

let ast = AstNode::Call {
    function: Box::new(AstNode::Lambda {
        parameters: vec!["x".to_string()],
        body: Box::new(AstNode::Call {
            function: Box::new(AstNode::Symbol("add".to_string())),
            arguments: vec![
                AstNode::Variable("x".to_string()),
                AstNode::Variable("y".to_string()),
            ],
            location: SourceLocation::default(),
        }),
        location: SourceLocation::default(),
    }),
    arguments: vec![AstNode::Literal(Literal::Int(5))],
    location: SourceLocation::default(),
};

// Expected bytecode:
let expected = vec![
    OpCode::Int(3),        // y = 3
    OpCode::Int(5),        // argument x = 5
    OpCode::MakeClosure(1, 1),  // Capture y, 1 parameter
    OpCode::Call(1),
    OpCode::Ret,
];
```

## Testing Strategy

### Unit Testing

#### Float Literal Tests
```rust
#[test]
fn test_float_literal_compilation() {
    let ast = AstNode::Literal(Literal::Float(3.14));
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
    let bytecode = compiler.compile_to_physics(&ast).unwrap();
    
    assert_eq!(bytecode, vec![OpCode::Float(3.14)]);
}

#[test]
fn test_float_arithmetic() {
    let ast = AstNode::Call {
        function: Box::new(AstNode::Symbol("add".to_string())),
        arguments: vec![
            AstNode::Literal(Literal::Float(1.5)),
            AstNode::Literal(Literal::Float(2.5)),
        ],
        location: SourceLocation::default(),
    };
    
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
    let bytecode = compiler.compile_to_physics(&ast).unwrap();
    
    let mut vm = create_test_vm();
    execute_bytecode(&mut vm, &bytecode);
    
    assert_eq!(vm.stack.pop(), Some(Value::Float(4.0)));
}
```

#### Variable Environment Tests
```rust
#[test]
fn test_variable_resolution() {
    let ast = AstNode::Variable("x".to_string());
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
    compiler.current_env.variables.insert("x".to_string(), 0);
    
    let bytecode = compiler.compile_to_physics(&ast).unwrap();
    assert_eq!(bytecode, vec![OpCode::GetLocal(0)]);
}

#[test]
fn test_lexical_scoping() {
    let ast = AstNode::Let {
        bindings: vec![("x".to_string(), AstNode::Literal(Literal::Int(10)))],
        body: Box::new(AstNode::Variable("x".to_string())),
        location: SourceLocation::default(),
    };
    
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
    let bytecode = compiler.compile_to_physics(&ast).unwrap();
    
    let mut vm = create_test_vm();
    execute_bytecode(&mut vm, &bytecode);
    
    assert_eq!(vm.stack.pop(), Some(Value::Int(10)));
}
```

#### Closure Tests
```rust
#[test]
fn test_closure_environment_capture() {
    let ast = AstNode::Lambda {
        parameters: vec!["x".to_string()],
        body: Box::new(AstNode::Variable("y".to_string())),
        location: SourceLocation::default(),
    };
    
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
    compiler.current_env.variables.insert("y".to_string(), 0);
    
    let bytecode = compiler.compile_to_physics(&ast).unwrap();
    
    assert!(bytecode.contains(&OpCode::MakeClosure(0, 1)));
    
    let mut vm = create_test_vm();
    vm.stack.push(Value::Int(42)); // captured y
    execute_bytecode(&mut vm, &bytecode);
    
    let closure = vm.stack.pop();
    assert!(matches!(closure, Some(Value::Closure(_))));
}
```

### Integration Testing

#### End-to-End Compilation Tests
```rust
#[test]
fn test_full_program_execution() {
    let source = r#"
        (let ((add (lambda (x y) (+ x y))))
            (add 3 4))
    "#;
    
    let mut parser = JueParser::new(source);
    let ast = parser.parse().unwrap();
    
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
    let bytecode = compiler.compile_to_physics(&ast).unwrap();
    
    let mut vm = create_test_vm();
    execute_bytecode(&mut vm, &bytecode);
    
    assert_eq!(vm.stack.pop(), Some(Value::Int(7)));
}
```

#### Trust Tier Tests
```rust
#[test]
fn test_empirical_capability_enforcement() {
    let ast = AstNode::FfiCall {
        function: "read-sensor".to_string(),
        arguments: vec![],
        location: SourceLocation::default(),
    };
    
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);
    let bytecode = compiler.compile_to_physics(&ast).unwrap();
    
    // Should contain capability check
    assert!(bytecode.contains(&OpCode::HasCap(0)));
}

#[test]
fn test_experimental_sandbox_wrapper() {
    let ast = AstNode::Call {
        function: Box::new(AstNode::Symbol("add".to_string())),
        arguments: vec![
            AstNode::Literal(Literal::Int(1)),
            AstNode::Literal(Literal::Int(2)),
        ],
        location: SourceLocation::default(),
    };
    
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Experimental);
    let bytecode = compiler.compile_to_physics(&ast).unwrap();
    
    // Should contain sandbox wrapper
    assert!(bytecode.contains(&OpCode::InitSandbox));
    assert!(bytecode.contains(&OpCode::CleanupSandbox));
}
```

### Property-Based Testing

#### Float Arithmetic Properties
```rust
proptest! {
    #[test]
    fn test_float_arithmetic_commutativity(a in prop::num::f64::ANY, b in prop::num::f64::ANY) {
        // Test that a + b = b + a
        let mut vm1 = create_test_vm();
        let mut vm2 = create_test_vm();
        
        let ast1 = AstNode::Call {
            function: Box::new(AstNode::Symbol("add".to_string())),
            arguments: vec![
                AstNode::Literal(Literal::Float(a)),
                AstNode::Literal(Literal::Float(b)),
            ],
            location: SourceLocation::default(),
        };
        
        let ast2 = AstNode::Call {
            function: Box::new(AstNode::Symbol("add".to_string())),
            arguments: vec![
                AstNode::Literal(Literal::Float(b)),
                AstNode::Literal(Literal::Float(a)),
            ],
            location: SourceLocation::default(),
        };
        
        let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
        let bytecode1 = compiler.compile_to_physics(&ast1).unwrap();
        let bytecode2 = compiler.compile_to_physics(&ast2).unwrap();
        
        execute_bytecode(&mut vm1, &bytecode1);
        execute_bytecode(&mut vm2, &bytecode2);
        
        let result1 = vm1.stack.pop();
        let result2 = vm2.stack.pop();
        
        prop_assert_eq!(result1, result2);
    }
}
```

## Integration Points

### Core-World Integration

**Interface**: The Physics-World compiler receives AST nodes from Jue-World compilation that have already been verified against Core-World formal semantics.

**Requirements**:
- Preserve semantic meaning during compilation
- Maintain proof obligations for trust tiers
- Support Core-World's βη-reduction semantics

**Integration Points**:
```rust
// In jue_world/src/compiler/compiler.rs
pub fn compile_to_physics_world(
    &self,
    ast: &AstNode,
    core_expr: &CoreExpr,
    tier: TrustTier,
) -> Result<Vec<OpCode>, CompilationError> {
    // Verify semantic preservation
    self.verify_semantic_preservation(ast, core_expr)?;
    
    // Compile to Physics-World
    physics::compile_to_physics_world(ast, tier)
}
```

### Physics-World VM Integration

**Interface**: Generated OpCodes must be executable by the Physics-World VM.

**Requirements**:
- Use only defined OpCode variants
- Follow VM execution semantics
- Support garbage collection and memory management

**Integration Points**:
```rust
// In physics_world/src/vm/mod.rs
pub fn execute_instruction(&mut self, opcode: OpCode) -> Result<(), VmError> {
    match opcode {
        OpCode::Float(f) => self.stack.push(Value::Float(f)),
        OpCode::LoadString(idx) => {
            let string = self.string_constants.get(idx)
                .ok_or(VmError::InvalidStringIndex)?;
            self.stack.push(Value::String(string.clone()));
        }
        // ... other opcode handlers
    }
    Ok(())
}
```

### Trust Tier System Integration

**Interface**: Compilation respects trust tier constraints and generates appropriate bytecode.

**Requirements**:
- Check capability availability per tier
- Generate runtime checks for Empirical tier
- Add sandbox wrappers for Experimental tier

**Integration Points**:
```rust
// In jue_world/src/trust_tier.rs
impl TrustTier {
    pub fn requires_capability_check(&self) -> bool {
        match self {
            TrustTier::Empirical | TrustTier::Experimental => true,
            TrustTier::Formal | TrustTier::Verified => false,
        }
    }
}
```

## Validation Criteria

### Functional Requirements

#### Float Support
- [ ] Float literals compile to `OpCode::Float`
- [ ] Float arithmetic operations execute correctly
- [ ] Precision is preserved during operations
- [ ] Float constants are deduplicated in constant pool

#### String Support  
- [ ] String literals compile to string constants
- [ ] String operations (length, concatenation) work correctly
- [ ] String escaping is handled properly
- [ ] String constants are deduplicated

#### Variable Environment
- [ ] Variables resolve to correct stack offsets
- [ ] Lexical scoping works as expected
- [ ] Variable shadowing behaves correctly
- [ ] Undefined variables produce compilation errors

#### Closure Environment Capture
- [ ] Closures capture environment variables correctly
- [ ] Captured variables are accessible within closure body
- [ ] Recursive closure calls work properly
- [ ] Environment capture is efficient

#### Trust Tier Enforcement
- [ ] Empirical tier generates capability checks
- [ ] Experimental tier adds sandbox wrappers
- [ ] Capability violations are detected and reported
- [ ] Sandbox violations are contained

### Performance Requirements

#### Compilation Performance
- [ ] Simple programs compile in < 100ms
- [ ] Complex programs compile in < 1s
- [ ] Memory usage during compilation is bounded
- [ ] No quadratic complexity in environment lookup

#### Execution Performance
- [ ] Variable access is O(1)
- [ ] Environment capture is efficient
- [ ] Closure creation has minimal overhead
- [ ] Capability checks have minimal overhead

### Quality Requirements

#### Code Quality
- [ ] All new code follows Rust best practices
- [ ] Comprehensive test coverage (>90%)
- [ ] Documentation coverage for all public APIs
- [ ] No compiler warnings

#### Safety Requirements
- [ ] No unsafe code without justification
- [ ] Memory safety guaranteed
- [ ] Type safety maintained throughout
- [ ] Capability isolation enforced

## Risk Assessment

### High-Risk Items

#### 1. Environment Capture Complexity
**Risk**: Incorrect environment capture can lead to memory leaks or incorrect variable resolution.

**Impact**: CRITICAL - Core functionality failure

**Mitigation Strategies**:
- Implement comprehensive tests for environment capture
- Use property-based testing to verify scoping rules
- Add runtime validation for captured environments
- Implement environment escape analysis

#### 2. Float Precision Issues
**Risk**: IEEE 754 precision differences between compilation and execution environments.

**Impact**: HIGH - Incorrect arithmetic results

**Mitigation Strategies**:
- Use consistent floating-point representation throughout
- Add precision validation tests
- Implement floating-point comparison with tolerance
- Document precision guarantees and limitations

#### 3. Trust Tier Security Gaps
**Risk**: Insufficient capability enforcement could allow privilege escalation.

**Impact**: CRITICAL - Security vulnerability

**Mitigation Strategies**:
- Implement comprehensive security testing
- Add runtime validation for all capability checks
- Perform security audit of trust tier implementation
- Implement sandbox escape detection

### Medium-Risk Items

#### 4. String Memory Management
**Risk**: String constant pool could lead to memory bloat or fragmentation.

**Impact**: MEDIUM - Performance degradation

**Mitigation Strategies**:
- Implement string pool size limits
- Add memory usage monitoring
- Implement string pool garbage collection
- Add string deduplication verification

#### 5. Closure Performance Overhead
**Risk**: Environment capture could introduce significant performance overhead.

**Impact**: MEDIUM - Performance degradation

**Mitigation Strategies**:
- Profile closure creation and access
- Optimize environment representation
- Implement closure optimization passes
- Add performance benchmarks

### Low-Risk Items

#### 6. Error Message Quality
**Risk**: Poor error messages could hinder debugging and development.

**Impact**: LOW - Developer experience

**Mitigation Strategies**:
- Add comprehensive error message tests
- Implement source location tracking
- Add contextual error information
- Create error message style guide

## Conclusion

This implementation guide provides a comprehensive roadmap for addressing the critical TODO features in Physics-World integration. The phased approach ensures foundational features are implemented before building more complex functionality, while the comprehensive testing strategy ensures reliability and correctness.

The implementation of these features will transform Physics-World from a placeholder system into a fully functional execution environment capable of running real Jue programs with proper variable handling, closure support, and trust tier enforcement.

**Next Steps**:
1. Begin Phase 1 implementation (Float and String literals)
2. Set up comprehensive testing infrastructure
3. Establish performance benchmarks
4. Create security testing protocols
5. Begin stakeholder review process

**Success Metrics**:
- All TODO items implemented and tested
- Complete test coverage for new functionality
- Performance benchmarks meet requirements
- Security review passes
- Integration with existing codebase verified

This implementation will provide the foundation for advanced Physics-World features and enable the full potential of Project Jue's layered cognitive architecture.