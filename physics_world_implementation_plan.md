# Physics World Implementation Plan

## Current State Analysis

The Physics World implementation is already quite comprehensive with:
- ✅ Core data types (HeapPtr, OpCode, Value)
- ✅ Memory arena with allocation
- ✅ VM state with instruction execution
- ✅ Round-robin scheduler
- ✅ Public API
- ✅ Comprehensive test coverage

## Gaps Identified

### 1. Missing OpCode Variants
**Spec Requirement**: `MakeClosure` and `CheckStepLimit` opcodes
**Current State**: Not implemented
**Impact**: Limits functionality for closure creation and resource enforcement

### 2. Memory Limit Enforcement
**Spec Requirement**: "Enforcement: All resource limits (computation, memory) are enforced"
**Current State**: Arena doesn't track memory usage against limits
**Impact**: AIKR not fully enforced - actors could exceed memory limits

### 3. Resource Tracking
**Spec Requirement**: "Resource usage metrics for actor execution"
**Current State**: Memory usage tracking is incomplete
**Impact**: Cannot provide accurate resource metrics in ExecutionResult

### 4. Closure Implementation
**Spec Requirement**: Full closure support with captured environments
**Current State**: Basic closure allocation but no proper execution
**Impact**: Limits higher-level language features

## Implementation Plan

### Phase 1: Complete OpCode Implementation
**Files**: `physics_world/src/types.rs`, `physics_world/src/vm/state.rs`

1. **Add missing OpCode variants**:
   ```rust
   MakeClosure(usize /* code_idx */, usize /* capture_count */),
   CheckStepLimit,
   ```

2. **Implement MakeClosure in VM**:
   - Pop `capture_count` values from stack
   - Allocate closure object with code reference and captured environment
   - Push closure pointer to stack

3. **Implement CheckStepLimit in VM**:
   - Check if steps_remaining <= 0
   - Return CpuLimitExceeded error if exceeded

### Phase 2: Memory Management Enhancement
**Files**: `physics_world/src/memory/arena.rs`, `physics_world/src/vm/state.rs`

1. **Add memory tracking to ObjectArena**:
   ```rust
   pub struct ObjectArena {
       storage: Vec<u8>,
       next_free: u32,
       capacity: u32,
       used_memory: u32, // Track actual memory usage
   }
   ```

2. **Update allocate method**:
   - Check `used_memory + total_needed > capacity`
   - Return `ArenaError::ArenaFull` if exceeded
   - Update `used_memory` on successful allocation

3. **Add memory usage reporting**:
   ```rust
   pub fn used_memory(&self) -> u32 {
       self.used_memory
   }
   ```

### Phase 3: Resource Enforcement
**Files**: `physics_world/src/vm/state.rs`

1. **Add memory limit to VmState**:
   ```rust
   pub struct VmState {
       // ... existing fields
       memory_limit: usize,
   }
   ```

2. **Enhance step() method**:
   - Check memory usage before each instruction
   - Return `VmError::MemoryLimitExceeded` if `memory.used_memory() > memory_limit`

3. **Update resource tracking**:
   - Track memory usage in metrics
   - Include in ExecutionResult

### Phase 4: Closure Execution
**Files**: `physics_world/src/vm/state.rs`

1. **Implement proper Call instruction**:
   - Extract function code from closure
   - Set up call frame with return address
   - Jump to closure's code

2. **Add closure data structure**:
   ```rust
   struct ClosureData {
       code_index: usize,
       environment: Vec<Value>,
   }
   ```

3. **Update memory layout**:
   - Proper serialization/deserialization of closures
   - GC-safe closure handling

## Testing Strategy

### Unit Tests
1. **Memory limit enforcement**:
   - Test allocation fails when exceeding limit
   - Test VM halts on memory exhaustion

2. **Step limit enforcement**:
   - Test CheckStepLimit instruction
   - Test CPU limit exceeded error

3. **Closure operations**:
   - Test MakeClosure instruction
   - Test closure creation and execution
   - Test captured environment access

### Integration Tests
1. **End-to-end execution**:
   - Simple program with closures
   - Program that hits resource limits
   - Complex program with multiple actors

2. **Determinism verification**:
   - Same input produces same output
   - Serialization/deserialization preserves state

## Implementation Order

1. **Phase 1**: OpCode variants (1 day)
2. **Phase 2**: Memory management (2 days)
3. **Phase 3**: Resource enforcement (1 day)
4. **Phase 4**: Closure execution (3 days)
5. **Testing**: Comprehensive test coverage (2 days)

## Success Criteria

- ✅ All OpCode variants from spec implemented
- ✅ Memory and CPU limits strictly enforced
- ✅ Accurate resource usage metrics
- ✅ Full closure support with captured environments
- ✅ Comprehensive test coverage (>90%)
- ✅ Deterministic execution verified
- ✅ All existing tests still pass

## Risk Assessment

**High Risk**:
- Closure implementation complexity
- Memory management changes affecting existing code

**Mitigation**:
- Incremental implementation with frequent testing
- Maintain backward compatibility
- Comprehensive property-based testing

## Timeline Estimate

- **Total**: 9 days
- **Phase 1**: 1 day
- **Phase 2**: 2 days
- **Phase 3**: 1 day
- **Phase 4**: 3 days
- **Testing**: 2 days

## Dependencies

- No external dependencies needed
- All changes contained within physics_world module
- No breaking changes to public API