# Physics World Refactoring Plan

**Document Version:** 1.0  
**Last Updated:** 2025-12-24  
**Status:** Draft - Ready for Review

## Executive Summary

This document outlines a comprehensive refactoring plan for the Physics World codebase to address code duplication, single responsibility violations, and excessive coupling. The plan is organized into phases with clear migration steps, success criteria, and backward compatibility considerations.

---

## 1. Current State Analysis

### 1.1 Codebase Overview

The Physics World module contains the following key components:

```
physics_world/src/
├── vm/
│   ├── state.rs (1454 lines)
│   ├── error.rs (639 lines)
│   ├── opcodes/
│   │   ├── make_closure.rs
│   │   ├── make_closure_backup.rs
│   │   ├── make_closure_fixed.rs
│   │   ├── call.rs
│   │   └── closure.rs
│   ├── gc.rs
│   ├── debug.rs
│   └── performance.rs
├── scheduler/
│   ├── core.rs (1103 lines)
│   ├── debug.rs
│   ├── capability.rs
│   ├── execution.rs
│   └── resource.rs
├── memory/
│   └── arena.rs (420 lines)
└── types/
    ├── capability.rs
    └── error.rs
```

### 1.2 Key Metrics

| Metric                                | Current Value | Threshold | Status      |
| ------------------------------------- | ------------- | --------- | ----------- |
| Max file size                         | 1454 lines    | 500 lines | ⚠️ Violation |
| Duplicate closure implementations     | 3             | 0         | ⚠️ Violation |
| Duplicate call handling               | 2             | 0         | ⚠️ Violation |
| Duplicate capability request handling | 2             | 0         | ⚠️ Violation |

---

## 2. Identified Issues

### 2.1 Code Duplication Patterns

#### Issue D1: `make_closure` - Three Competing Implementations

**Location:** [`physics_world/src/vm/opcodes/make_closure.rs`](../physics_world/src/vm/opcodes/make_closure.rs)  
**Backup Location:** [`physics_world/src/vm/opcodes/make_closure_backup.rs`](../physics_world/src/vm/opcodes/make_closure_backup.rs)  
**Fixed Location:** [`physics_world/src/vm/opcodes/make_closure_fixed.rs`](../physics_world/src/vm/opcodes/make_closure_fixed.rs)

**Problem:** Three versions of `handle_make_closure` exist:
- `make_closure.rs`: Original implementation with basic closure creation
- `make_closure_backup.rs`: Backup copy (likely during development)
- `make_closure_fixed.rs`: Version with fixes applied

**Impact:**
- Confusion about which implementation to use
- Risk of import conflicts in [`vm/opcodes/mod.rs`](../physics_world/src/vm/opcodes/mod.rs)
- Maintenance burden with three copies to update

**Duplication Analysis:**
- Lines 1-296: Core `handle_make_closure` function
- Lines 97-107: `create_default_identity_closure` helper
- Lines 109-129: `create_closure_body` helper (also in closure.rs)
- Lines 131-296: `parse_bytecode_from_string` debug parser

#### Issue D2: `handle_call` and `handle_tail_call` - Duplicated Logic

**Location 1:** [`physics_world/src/vm/state.rs`](../physics_world/src/vm/state.rs) lines 873-1028  
**Location 2:** [`physics_world/src/vm/opcodes/call.rs`](../physics_world/src/vm/opcodes/call.rs) lines 1-279

**Problem:** Function call handling is implemented in both locations:
- `VmState::handle_call` (state.rs lines 873-1027)
- `VmState::handle_tail_call` (state.rs lines 841-871)
- `call::handle_call` (call.rs lines 27-54)
- `call::handle_tail_call` (call.rs lines 172-192)

**Impact:**
- Inconsistent behavior between implementations
- The step() function in state.rs uses `self.handle_call()` directly (line 1258)
- Call handler in opcodes/call.rs is imported but potentially unused

#### Issue D3: `handle_capability_request` - Duplicated Between Modules

**Location 1:** [`physics_world/src/scheduler/core.rs`](../physics_world/src/scheduler/core.rs) lines 372-488  
**Location 2:** [`physics_world/src/scheduler/debug.rs`](../physics_world/src/scheduler/debug.rs) lines 23-145

**Problem:** Nearly identical implementations of `handle_capability_request`:
- Core version handles full capability decision logic
- Debug version duplicates the same logic with minor variations

**Duplication Details:**
- Lines 23-67: Request logging and actor lookup
- Lines 68-119: Capability decision logic (identical match arms)
- Lines 121-145: Logging and capability insertion

### 2.2 Single Responsibility Violations

#### Issue S1: `vm/state.rs` (1454 lines)

**Current Responsibilities:**
1. VM execution state management (lines 373-396)
2. Instruction execution/dispatch (lines 1095-1414)
3. Call frame management (lines 402-411)
4. Debugging support (lines 114-310, 505-705)
5. Performance monitoring integration (lines 790-818)
6. GC integration (lines 714-740)
7. Error context creation (lines 467-503)

**Violation:** VmState mixes core VM functionality with debugging, performance, and integration concerns.

**Recommended Split:**
```
vm/state/
├── state.rs (core VM state, ~400 lines)
├── execution.rs (step/run logic, ~300 lines)
├── call_state.rs (CallFrame, frame management, ~200 lines)
└── gc.rs (GC integration, already separate - verify imports)
```

#### Issue S2: `scheduler/core.rs` (1103 lines)

**Current Responsibilities:**
1. Actor scheduling (lines 13-31)
2. Tick execution (lines 155-220)
3. Capability management (lines 358-488)
4. Capability delegation (lines 490-566)
5. Capability revocation (lines 597-656)
6. Consensus voting (lines 685-745)
7. Priority scheduling (lines 240-322)
8. Resource management (lines 874-1103)

**Violation:** Scheduler core mixes scheduling with capability and resource management.

**Recommended Split:**
```
scheduler/
├── core.rs (scheduler, tick, ~400 lines)
├── capability.rs (capability management, ~350 lines)
├── resource.rs (resource quotas, ~300 lines)
└── priority.rs (already exists - verify content)
```

#### Issue S3: `vm/error.rs` (639 lines)

**Current Responsibilities:**
1. `SimpleVmError` enum (lines 9-20)
2. `ErrorContext` struct (lines 24-45)
3. `StackFrame` struct (lines 49-58)
4. `VmError` enum with context (lines 62-157)
5. Error creation methods (lines 159-328)
6. Error formatting (lines 353-497)
7. Recovery logic (lines 499-546)
8. `RecoveryAction` enum (lines 550-561)
9. `WithContext` trait (lines 612-639)

**Violation:** Error module mixes error types, context, and recovery mechanisms.

**Recommended Split:**
```
vm/error/
├── types.rs (SimpleVmError, StackFrame, ~100 lines)
├── context.rs (ErrorContext, VmError, ~350 lines)
└── recovery.rs (RecoveryAction, WithContext, ~150 lines)
```

#### Issue S4: `memory/arena.rs` (420 lines - Approaching Threshold)

**Current Status:** 420 lines is approaching the 500-line threshold. Consider monitoring for future refactoring if the file grows.

### 2.3 Coupling Issues

#### Issue C1: VmState Dependencies

**Current Dependencies:**
```
VmState → memory/arena (ObjectArena)
VmState → vm/debug (Debugger, DebugEvent)
VmState → vm/error (SimpleVmError, VmError)
VmState → vm/gc (GarbageCollector)
VmState → vm/performance (PerformanceMonitor)
VmState → vm/opcodes/* (all opcode modules)
VmState → types (HeapPtr, OpCode, Value)
```

**Problem:** VmState directly depends on 6+ modules, creating tight coupling.

**Recommended Approach:**
- Use trait-based optional features for debug, performance, GC
- Consider a `VmFeatures` config struct to enable/disable features

#### Issue C2: Circular Dependencies Risk

**Problem Areas:**
- `vm/state.rs` imports from `vm/opcodes/*`
- `vm/opcodes/*` imports from `vm/state.rs` (VmState, CallFrame)
- `scheduler/core.rs` imports `vm::state::InstructionResult`

**Risk:** As the codebase grows, these bidirectional imports could create circular dependency issues.

#### Issue C3: Capability Logic Embedding

**Problem:** Capability checking logic is embedded directly in `scheduler/core.rs` instead of being isolated in a separate module.

**Current State:**
- Capability types in `types/capability.rs`
- Capability audit in `scheduler/capability.rs`
- But core capability logic in `scheduler/core.rs` and `scheduler/debug.rs`

---

## 3. Refactoring Plan

### 3.1 Phase 1: Eliminate Duplication (Low Risk)

**Goal:** Remove duplicate code before structural refactoring to prevent issues from propagating.

#### Phase 1A: Consolidate `make_closure` Implementations

**Steps:**
1. Audit all usages of `make_closure` imports in [`vm/opcodes/mod.rs`](../physics_world/src/vm/opcodes/mod.rs)
2. Determine which implementation is the most complete (likely `make_closure_fixed.rs`)
3. Rename `make_closure_fixed.rs` → `make_closure.rs`
4. Delete `make_closure_backup.rs` and old `make_closure.rs`
5. Update module exports in `opcodes/mod.rs`
6. Run tests to verify no breakage

**Files to Modify:**
- [`physics_world/src/vm/opcodes/mod.rs`](../physics_world/src/vm/opcodes/mod.rs)

**Files to Delete:**
- `physics_world/src/vm/opcodes/make_closure.rs`
- `physics_world/src/vm/opcodes/make_closure_backup.rs`
- `physics_world/src/vm/opcodes/make_closure_fixed.rs`

**Success Criteria:**
- ✅ All tests pass after consolidation
- ✅ No duplicate symbol errors
- ✅ Only one `handle_make_closure` function exists

#### Phase 1B: Consolidate Call Handling

**Steps:**
1. Identify which call handling implementation is used in `state.rs` step() function (line 1258)
2. If `self.handle_call()` is used, remove import from call.rs or vice versa
3. Ensure single source of truth for call handling
4. Remove unused imports

**Decision Point:**
- Option A: Keep call handling in `vm/state.rs`, mark `call.rs` as deprecated
- Option B: Move all call handling to `vm/opcodes/call.rs`, call from `state.rs`

**Recommendation:** Option B - Move to opcode module for consistency with other opcodes.

**Files to Modify:**
- [`physics_world/src/vm/state.rs`](../physics_world/src/vm/state.rs)
- [`physics_world/src/vm/opcodes/call.rs`](../physics_world/src/vm/opcodes/call.rs)

**Success Criteria:**
- ✅ Single `handle_call` implementation exists
- ✅ State.rs step() uses opcode module's handler
- ✅ All tests pass

#### Phase 1C: Consolidate Capability Request Handling

**Steps:**
1. Compare implementations in `core.rs` and `debug.rs`
2. Keep the most complete version (likely in `debug.rs` for extended capability handling)
3. Remove duplicate from `core.rs`, call through to debug.rs implementation
4. Or: Move capability logic to `scheduler/capability.rs` and use from both

**Recommendation:** Move to `scheduler/capability.rs` for proper separation.

**Files to Modify:**
- [`physics_world/src/scheduler/core.rs`](../physics_world/src/scheduler/core.rs)
- [`physics_world/src/scheduler/debug.rs`](../physics_world/src/scheduler/debug.rs)
- [`physics_world/src/scheduler/capability.rs`](../physics_world/src/scheduler/capability.rs)

**Success Criteria:**
- ✅ Single `handle_capability_request` implementation
- ✅ Core scheduler uses unified capability module
- ✅ All tests pass

### 3.2 Phase 2: Split Large Files (Medium Risk)

**Goal:** Split files violating single responsibility principle while maintaining API compatibility.

#### Phase 2A: Split `vm/state.rs`

**Target Structure:**
```
vm/state/
├── mod.rs          (re-exports, ~50 lines)
├── state.rs        (VmState struct, new(), ~300 lines)
├── execution.rs    (step(), run(), ~300 lines)
└── call_state.rs   (CallFrame, call handling, ~250 lines)
```

**Migration Steps:**

1. **Create new files:**
   - Create `vm/state/` directory
   - Create `vm/state/call_state.rs` with CallFrame and related methods
   - Create `vm/state/execution.rs` with step() and run()
   - Move remaining VmState core to `vm/state/state.rs`

2. **Update `vm/mod.rs`:**
   ```rust
   pub mod state;
   pub use state::{VmState, CallFrame, InstructionResult, VmError};
   ```

3. **Backward Compatibility:**
   - Keep `vm/state.rs` as re-export module:
     ```rust
     // vm/state.rs - Backward compatibility shim
     pub use self::state::VmState;
     pub use self::call_state::CallFrame;
     pub use self::execution::{InstructionResult, VmError};
     ```

4. **Update imports in dependent files:**
   - `scheduler/core.rs`
   - `vm/opcodes/*.rs`
   - `api/integration.rs`

**Risk Mitigation:**
- Use feature flags for gradual migration
- Keep old module structure working until Phase 3 completion

**Success Criteria:**
- ✅ No file exceeds 500 lines
- ✅ Each module has single responsibility
- ✅ All tests pass with same test count
- ✅ Backward compatibility maintained

#### Phase 2B: Split `scheduler/core.rs`

**Target Structure:**
```
scheduler/
├── mod.rs          (re-exports, ~50 lines)
├── core.rs         (PhysicsScheduler, tick(), ~350 lines)
├── capability.rs   (capability management, ~350 lines)
├── resource.rs     (resource quotas, ~300 lines)
└── priority.rs     (priority scheduling, already exists - verify)
```

**Migration Steps:**

1. **Create new files:**
   - Move capability management to `scheduler/capability.rs`
   - Move resource management to `scheduler/resource.rs`
   - Keep core scheduler (tick, actor management) in `scheduler/core.rs`

2. **Update `scheduler/mod.rs`:**
   ```rust
   pub mod core;
   pub mod capability;
   pub mod resource;
   pub use core::PhysicsScheduler;
   ```

3. **Backward Compatibility:**
   - Keep `scheduler/core.rs` as re-export for capability/resource
   - Deprecate access through old paths

**Files to Create:**
- `scheduler/capability.rs` (moved from core.rs)
- `scheduler/resource.rs` (moved from core.rs)

**Files to Modify:**
- `scheduler/core.rs` (refactored)
- `scheduler/mod.rs` (updated exports)

**Success Criteria:**
- ✅ Core scheduler under 500 lines
- ✅ Capability logic isolated
- ✅ Resource logic isolated
- ✅ All tests pass

#### Phase 2C: Split `vm/error.rs`

**Target Structure:**
```
vm/error/
├── mod.rs          (re-exports, ~30 lines)
├── types.rs        (SimpleVmError, StackFrame, ~100 lines)
├── context.rs      (ErrorContext, VmError, ~350 lines)
└── recovery.rs     (RecoveryAction, WithContext, ~100 lines)
```

**Migration Steps:**

1. **Create new files:**
   - `vm/error/types.rs`: SimpleVmError, StackFrame
   - `vm/error/context.rs`: ErrorContext, VmError with methods
   - `vm/error/recovery.rs`: RecoveryAction, WithContext trait

2. **Update `vm/mod.rs` and other consumers**

**Success Criteria:**
- ✅ No file exceeds 400 lines
- ✅ Clear separation of error types, context, and recovery
- ✅ All tests pass

### 3.3 Phase 3: Reduce Coupling (Higher Risk)

**Goal:** Use trait-based optional features to reduce VmState dependencies.

#### Phase 3A: Trait-Based Feature Flags

**Approach:**
```rust
// In vm/state.rs

// Core VM state - always present
pub struct VmState {
    // ... core fields
}

// Optional features via traits
#[cfg(feature = "debugging")]
pub trait DebugSupport {
    fn add_breakpoint(&mut self, address: usize);
    fn check_breakpoints(&self) -> bool;
}

#[cfg(feature = "performance")]
pub trait PerformanceMonitoring {
    fn start_timer(&mut self, name: &str);
    fn stop_timer(&mut self, name: &str);
    fn get_metrics(&self) -> PerformanceMetrics;
}
```

**Benefits:**
- Reduces compile dependencies
- Allows users to opt-in to features
- Improves compilation times

**Files to Modify:**
- [`physics_world/src/vm/state.rs`](../physics_world/src/vm/state.rs)
- [`physics_world/Cargo.toml`](../physics_world/Cargo.toml)

#### Phase 3B: Extract Opcode Module Interface

**Problem:** VmState depends on all opcode modules.

**Solution:** Create a unified `OpcodeHandler` trait:
```rust
pub trait OpcodeHandler {
    fn handle(&self, vm: &mut VmState, opcode: &OpCode) -> Result<InstructionResult, SimpleVmError>;
}
```

**Files to Create:**
- `vm/opcodes/handler.rs` (trait definition)

**Files to Modify:**
- `vm/opcodes/*.rs` (implement trait)
- `vm/state.rs` (use trait)

---

## 4. Implementation Order

### Recommended Execution Order

```
Phase 1: Eliminate Duplication (Week 1)
├── 1A: Consolidate make_closure
├── 1B: Consolidate call handling  
└── 1C: Consolidate capability handling

Phase 2: Split Large Files (Week 2)
├── 2A: Split vm/state.rs
├── 2B: Split scheduler/core.rs
└── 2C: Split vm/error.rs

Phase 3: Reduce Coupling (Week 3+)
├── 3A: Trait-based feature flags
└── 3B: Opcode handler interface
```

### Dependencies Between Phases

- **Phase 1 must complete before Phase 2** - eliminates duplication that would otherwise be replicated
- **Phase 2 should complete before Phase 3** - smaller files are easier to decouple
- **Phase 3 can be done incrementally** - each feature flag is independent

---

## 5. Migration Strategy

### 5.1 API Compatibility

**Commitment:** Maintain API compatibility throughout refactoring.

**Breaking Change Policy:**
- No changes to public method signatures without deprecation cycle
- No changes to type layouts affecting serialization
- Deprecation warnings for 1 release cycle before removal

### 5.2 Testing Strategy

**Before Refactoring:**
1. Record current test suite (all tests should pass)
2. Measure code coverage
3. Document integration points

**During Refactoring:**
1. Run full test suite after each sub-phase
2. Add integration tests for refactored boundaries
3. Use git branches for each phase

**After Refactoring:**
1. Verify all tests pass with same coverage
2. Compare performance benchmarks
3. Update documentation

### 5.3 Rollback Plan

**Git Workflow:**
```bash
# Create branch for each phase
git checkout -b refactor/phase1-duplication
# ... do work ...
git commit -m "Phase 1: Eliminate duplication"

# If issues found, can rollback to previous commit
git checkout main
git checkout -b fix-phase1-issues
# ... fix issues ...
```

**Feature Flags for Rollback:**
```rust
#[cfg(feature = "refactor-v2")]
use new_module_structure;

#[cfg(not(feature = "refactor-v2"))]
use old_module_structure; // Fallback for rollback
```

---

## 6. Risk Assessment

### 6.1 Risk Matrix

| Risk                              | Likelihood | Impact | Mitigation                                          |
| --------------------------------- | ---------- | ------ | --------------------------------------------------- |
| Test breakage during refactoring  | Medium     | High   | Comprehensive test suite, rollback plan             |
| API incompatibility for consumers | Low        | High   | Backward compatibility shims, deprecation cycle     |
| Circular dependencies             | Medium     | High   | Careful module boundary design, dependency analysis |
| Performance regression            | Low        | Medium | Benchmark before/after, optimize if needed          |

### 6.2 Contingency Plans

**If Tests Fail:**
1. Pause refactoring
2. Identify which specific test failed
3. Fix the issue in the original code first
4. Then retry the refactoring step

**If Circular Dependencies Emerge:**
1. Use the "mid-layer" pattern - create intermediate module
2. Extract shared types to common module
3. Consider using traits to break cycles

**If Performance Degrades:**
1. Profile to identify bottleneck
2. Optimize the new structure
3. Consider keeping hot paths inlined

---

## 7. Success Criteria

### 7.1 Quantitative Metrics

| Metric                      | Current    | Target     | Measurement         |
| --------------------------- | ---------- | ---------- | ------------------- |
| Max file size               | 1454 lines | ≤500 lines | `wc -l`             |
| Duplicate code instances    | 3          | 0          | Manual audit        |
| VmState direct dependencies | 6+         | 4 or fewer | Dependency analysis |
| Test pass rate              | 100%       | 100%       | `cargo test`        |

### 7.2 Qualitative Metrics

- [ ] Clear module boundaries with single responsibility
- [ ] No circular dependencies between modules
- [ ] Easy to navigate codebase structure
- [ ] New developers can understand module organization
- [ ] Minimal compilation dependencies (faster builds)

### 7.3 Exit Criteria

Refactoring is complete when:
1. All phases executed successfully
2. All success criteria met
3. Documentation updated
4. No regression in functionality
5. Performance within 5% of original

---

## 8. Appendices

### 8A. File Size Targets

| File     | Current Lines | Target Lines | New File                                                  |
| -------- | ------------- | ------------ | --------------------------------------------------------- |
| state.rs | 1454          | 400          | state/state.rs + state/execution.rs + state/call_state.rs |
| core.rs  | 1103          | 350          | core.rs + capability.rs + resource.rs                     |
| error.rs | 639           | 300          | types.rs + context.rs + recovery.rs                       |
| arena.rs | 420           | 420          | Monitor only                                              |

### 8B. Dependency Graph (Current)

```
VmState
├── memory::ObjectArena
├── vm::debug::Debugger
├── vm::error::SimpleVmError
├── vm::gc::GarbageCollector
├── vm::performance::PerformanceMonitor
├── vm::opcodes::*
└── types::{HeapPtr, OpCode, Value}

PhysicsScheduler
├── scheduler::Actor
├── vm::state::InstructionResult
├── vm::error::VmError
└── types::Capability
```

### 8C. Dependency Graph (Target)

```
VmState (core)
├── memory::ObjectArena
├── types::{HeapPtr, OpCode, Value}
└── [optional features via traits]

VmFeatures (optional)
├── DebugSupport (trait)
├── PerformanceMonitoring (trait)
└── GcIntegration (trait)

PhysicsScheduler (core)
├── scheduler::Actor
└── scheduler::capability::CapabilityManager

CapabilityManager
├── types::Capability
└── scheduler::Actor capabilities
```

### 8D. Glossary

| Term    | Definition                        |
| ------- | --------------------------------- |
| SRP     | Single Responsibility Principle   |
| API     | Application Programming Interface |
| TCO     | Tail Call Optimization            |
| GC      | Garbage Collection                |
| VmState | Virtual Machine execution state   |

---

## 9. Revision History

| Version | Date       | Author             | Changes       |
| ------- | ---------- | ------------------ | ------------- |
| 1.0     | 2025-12-24 | Physics World Team | Initial draft |

---

**Document Owner:** Physics World Team  
**Reviewers:** Architecture Team  
**Approval Status:** Pending Review