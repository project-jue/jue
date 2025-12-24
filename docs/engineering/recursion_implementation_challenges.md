# Project Jue: Recursion Implementation Challenges

## Executive Summary

Project Jue is experiencing critical issues with recursive function implementation in the Physics-World VM. This document outlines the current problems, attempted solutions, and architectural insights to guide resolution.

## Current Problem State

### Failing Tests
- **4 factorial recursion tests** (empirical, experimental, formal, verified tiers)
- **10 other recursive tests** (fibonacci, power, mutual recursion, etc.)
- **Total: 14/14 recursive tests failing** (100% failure rate)

### Core Issue
The VM cannot properly execute recursive lambda functions. The factorial function `fact(n) = if n <= 1 then 1 else n * fact(n-1)` returns `Int(5)` instead of `Int(120)` for input 5.

## Technical Analysis

### Bytecode Generation Problem

The compiler generates incorrect bytecode for recursive calls in multiplication expressions:

**Current Bytecode (WRONG):**
```
[GetLocal(0), Int(1), Lte, JmpIfFalse(2), Int(1), Jmp(9),
 GetLocal(0), Int(1), GetLocal(0), Sub, Call(2), GetLocal(0), Call(1), GetLocal(0), Mul]
```

**Issues Identified:**
1. **Double Call Instructions**: Two `Call(1)` instructions instead of one
2. **Wrong Argument Count**: `Call(2)` instead of `Call(1)` for factorial
3. **Stack Order Mismatch**: `Call(1)` expects `[closure, arg]` but gets wrong layout

### VM Execution Flow Problem

**Current Execution Trace:**
1. `GetLocal(0)` → pushes `n` (5)
2. `Int(1)` → pushes `1`
3. `GetLocal(0)` → pushes `n` (5)
4. `Sub` → computes `n-1` (4)
5. `Call(2)` → **FAILS** - expects 2 args, only 1 available
6. `GetLocal(0)` → pushes `n` (5)
7. `Call(1)` → **FAILS** - stack empty due to previous error
8. `GetLocal(0)` → pushes `n` (5)
9. `Mul` → **FAILS** - TypeMismatch (stack empty)

### Root Causes

1. **Compiler Issue**: Multiplication with recursive calls generates wrong bytecode sequence
2. **VM Issue**: `Call` instruction expects specific stack layout that isn't being provided
3. **Missing Opcodes**: `Swap` opcode doesn't exist for stack manipulation
4. **Closure Handling**: Recursive self-references not properly resolved

## Attempted Solutions

### Attempt 1: Fix Jump Offsets
- **Problem**: `Jmp(9)` jumping out of bounds
- **Solution**: Changed offset calculation from `else_bytecode_len` to `else_bytecode_len + 2`
- **Result**: Fixed bounds error but revealed deeper stack issues

### Attempt 2: Manual Bytecode Generation
- **Problem**: Compiler generating double `Call` instructions
- **Solution**: Manual bytecode generation for multiplication with recursion
- **Result**: Eliminated double calls but introduced stack order problems

### Attempt 3: Stack Order Correction
- **Problem**: `Call(1)` expects `[closure, arg]` but gets `[n, closure, n-1]`
- **Solution**: Tried using `Swap` opcode to reorder stack
- **Result**: `Swap` opcode doesn't exist in VM

## Current Working Hypothesis

The fundamental issue is a **stack layout mismatch** between what the compiler generates and what the VM expects:

```
Compiler generates:  [n, closure, n-1]
VM Call(1) expects: [closure, arg]
```

## Proposed Solutions

### Short-Term Fix (Immediate)

**Option A: Implement Stack Manipulation Opcodes**
```rust
// Add Swap opcode to physics_world/types.rs
OpCode::Swap,

// Implement in VM
OpCode::Swap => {
    if stack.len() < 2 {
        return Err(VmError::StackUnderflow);
    }
    let len = stack.len();
    stack.swap(len-1, len-2);
}
```

**Option B: Fix Bytecode Generation Logic**
```rust
// Correct sequence for n * fact(n-1):
1. GetLocal(0)  // Push n
2. GetLocal(0)  // Push n for subtraction
3. Int(1)       // Push 1
4. Sub          // n-1
5. GetLocal(0)  // Push closure
6. Call(1)      // Call fact(n-1), leaves [n, result]
7. Mul          // n * result
```

### Long-Term Solution (Architectural)

**Proper Recursive Lambda Support:**
1. **Closure Self-Reference**: Store closure in call frame locals at position 0
2. **Parameter Access**: Parameters start at position 1 (after closure)
3. **Stack Discipline**: Ensure all operations maintain consistent stack layout
4. **Recursion Detection**: Compiler flag for recursive functions to generate special bytecode

## Implementation Recommendations

### Phase 1: Immediate Fix
1. **Add Swap opcode** to enable stack manipulation
2. **Fix multiplication bytecode** to use correct sequence
3. **Update jump offsets** to handle variable bytecode lengths

### Phase 2: Robust Solution
1. **Enhance compiler** to detect recursive patterns
2. **Implement proper closure handling** in VM
3. **Add comprehensive stack manipulation** opcodes (Swap, Rot, Dup2, etc.)
4. **Create recursion test suite** with edge cases

### Phase 3: Architectural Improvement
1. **Design formal recursion protocol** for Jue-World
2. **Implement tail call optimization** for performance
3. **Add recursion depth tracking** and limits
4. **Document recursion semantics** for all trust tiers

## Test Coverage Requirements

### Minimum Viable Tests
- [ ] Basic factorial recursion (n=5 → 120)
- [ ] Fibonacci sequence (fib(6) → 8)
- [ ] Mutual recursion (even/odd functions)
- [ ] Deep recursion (n=100, stack depth test)

### Comprehensive Test Suite
- [ ] Recursion with multiple arguments
- [ ] Recursion with closures
- [ ] Tail recursion optimization
- [ ] Recursion across trust tiers
- [ ] Recursion with FFI calls

## Success Criteria

1. **100% Test Pass Rate**: All 14 recursive tests passing
2. **Correct Results**: Factorial(5) = 120, Fibonacci(6) = 8
3. **No Regressions**: Existing 193 tests remain passing
4. **Performance**: Recursion depth ≥ 100 without stack overflow
5. **Documentation**: Complete recursion implementation guide

## Request for External Expertise

**Specific Questions:**
1. What's the best practice for implementing recursion in stack-based VMs?
2. How should closure self-references be handled in recursive lambdas?
3. What stack manipulation opcodes are essential for recursion support?
4. Are there known patterns for compiling recursive function calls efficiently?

**Architectural Review Needed:**
- Current VM call frame design
- Closure representation and access
- Stack discipline and error handling
- Recursion depth management

## Next Steps

1. ✅ Document current problem (this document)
2. ⏳ Implement Swap opcode and test
3. ⏳ Fix bytecode generation for recursive multiplication
4. ⏳ Verify all recursive tests pass
5. ⏳ Update architecture documentation
6. ⏳ Create recursion implementation guide

## Appendix: Key Code Locations

- **Compiler**: `jue_world/src/integration/physics.rs` (lines 451-495)
- **VM Call Handler**: `physics_world/src/vm/opcodes/call.rs`
- **Bytecode Types**: `physics_world/src/types.rs`
- **Tests**: `jue_world/tests/test_recursive_function_execution.rs`

## Appendix: Example Test Case

```rust
// Factorial function that should work
let source = r#"
(let ((fact (lambda (n)
              (if (<= n 1)
                  1
                  (* n (fact (- n 1)))))))
  (fact 5))
"#;

// Expected: Int(120)
// Actual: Int(5) with TypeMismatch error
```

This document provides a comprehensive overview of the recursion implementation challenges in Project Jue, suitable for external review and expert consultation.