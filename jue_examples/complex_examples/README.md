# Complex Jue Examples

This directory contains progressively more complex Jue programs to test the compiler's capabilities and identify areas for improvement.

## Complexity Levels

### Level 1: Basic Arithmetic and Functions
- Simple arithmetic operations
- Basic lambda functions
- Function application

### Level 2: Intermediate Complexity
- Recursive functions ✅ (IMPLEMENTED)
- Data structures (lists, pairs)
- Conditional expressions
- Let bindings

### Level 3: Advanced Features
- Higher-order functions
- Function composition
- Pattern matching
- Complex data structures
- Error handling

### Level 4: Real-world Applications
- Mathematical algorithms
- List processing
- State management
- I/O operations (when available)
- Concurrent processing

## Testing Strategy

Each example should:
1. Compile without errors
2. Execute correctly
3. Produce expected output
4. Handle edge cases appropriately

## Current Status

### Working Features
- ✅ Simple literals (integers)
- ✅ Basic arithmetic operations (+, -, *)
- ✅ Lambda function abstraction
- ✅ Function application
- ✅ Closure creation
- ✅ Trust tier annotations
- ✅ Conditional expressions (if statements)
- ✅ Let bindings (variable bindings)
- ✅ Function composition (higher-order functions)
- ✅ Recursive functions (factorial, mutual recursion, nested recursion)

### Recent Improvements
- ✅ Fixed recursive function compilation with proper environment tracking
- ✅ Implemented Environment struct for variable binding management
- ✅ Added comprehensive unit tests for recursion functionality
- ✅ Fixed lambda compilation with proper capture_count handling
- ✅ Resolved stack underflow errors in recursive function calls

### Known Issues
- ⚠️ VM execution still has "Type mismatch" errors for recursive function calls
- ⚠️ Multiple expression evaluation in trust tier blocks only processes first expression
- ⚠️ Some advanced features still need implementation

## Test Coverage

The recursion functionality is now covered by comprehensive unit tests:
- Simple recursive functions (factorial)
- Mutual recursion (is-even?/is-odd?)
- Nested recursive functions
- Simple lambda in let bindings

All tests are passing and demonstrate that the compiler can successfully handle various recursion patterns at the compilation level.