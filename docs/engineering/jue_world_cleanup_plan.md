# Jue-World Code Cleanup Plan

## Executive Summary

This document identifies orphaned, duplicate, and obsolete files/modules in the `jue_world` codebase and provides a prioritized cleanup plan.

## Inventory of Cleanup Candidates

### Category 1: HIGH RISK (Directly Referenced but Unused)

These files are imported/used but may be dead code:

| File                     | References Found              | Status                | Risk Justification                                                                            |
| ------------------------ | ----------------------------- | --------------------- | --------------------------------------------------------------------------------------------- |
| `capability_ffi.rs`      | 2 imports in `physics_ffi.rs` | Orphan implementation | Active code but `physics_ffi.rs` not declared in mod.rs                                       |
| `integration/physics.rs` | 11 imports                    | Placeholder module    | Contains `PhysicsIntegrationContext` used in tests; functions are placeholder implementations |

### Category 2: MEDIUM RISK (No References - Likely Safe to Delete)

These files have no import references and appear to be orphaned:

| File                     | Size      | Created      | Purpose                                    | Risk Justification                     |
| ------------------------ | --------- | ------------ | ------------------------------------------ | -------------------------------------- |
| `ffi_backup.rs`          | 271 lines | Backup       | Duplicate of FFI implementation            | Backup file - no references found      |
| `parser_symbol_fix.rs`   | 33 lines  | Fragment     | Symbol parsing function                    | Fragment file - no references found    |
| `parser_tokenize_fix.rs` | 65 lines  | Fragment     | Tokenize function                          | Fragment file - no references found    |
| `macro_system_old.rs`    | 274 lines | Old version  | Previous macro implementation              | No references found                    |
| `physics_ffi.rs`         | 88 lines  | Partial impl | FFI integration for `PhysicsWorldCompiler` | Not declared in `mod.rs` - orphan file |

### Category 3: LOW RISK (Dead Module Declarations)

Declared but not implemented:

| Module Declaration                     | Location                                     | Comment in Code                    |
| -------------------------------------- | -------------------------------------------- | ---------------------------------- |
| `ast_compilation::analysis`            | `physics_integration/ast_compilation/mod.rs` | "implementation files don't exist" |
| `ast_compilation::calls`               | `physics_integration/ast_compilation/mod.rs` | "implementation files don't exist" |
| `ast_compilation::capabilities`        | `physics_integration/ast_compilation/mod.rs` | "implementation files don't exist" |
| `ast_compilation::compiler`            | `physics_integration/ast_compilation/mod.rs` | "implementation files don't exist" |
| `ast_compilation::control_flow`        | `physics_integration/ast_compilation/mod.rs` | "implementation files don't exist" |
| `ast_compilation::ffi`                 | `physics_integration/ast_compilation/mod.rs` | "implementation files don't exist" |
| `ast_compilation::lambdas`             | `physics_integration/ast_compilation/mod.rs` | "implementation files don't exist" |
| `ast_compilation::lets`                | `physics_integration/ast_compilation/mod.rs` | "implementation files don't exist" |
| `ast_compilation::literals`            | `physics_integration/ast_compilation/mod.rs` | "implementation files don't exist" |
| `ast_compilation::symbols`             | `physics_integration/ast_compilation/mod.rs` | "implementation files don't exist" |
| `ast_compilation::variables`           | `physics_integration/ast_compilation/mod.rs` | "implementation files don't exist" |
| `core_compilation::proof_verifier`     | `core_compilation/mod.rs`                    | "modules don't exist yet"          |
| `core_compilation::trust_tier_handler` | `core_compilation/mod.rs`                    | "modules don't exist yet"          |

### Category 4: LOW RISK (Placeholder Implementations)

These files exist but contain placeholder/stub code:

| File                                        | Status      | Notes                                  |
| ------------------------------------------- | ----------- | -------------------------------------- |
| `physics_integration/bytecode_generator.rs` | Placeholder | "This is a placeholder implementation" |
| `physics_integration/runtime_checks.rs`     | Placeholder | "This is a placeholder implementation" |
| `physics_integration/sandbox_wrapper.rs`    | Partial     | Contains placeholder implementations   |
| `integration/physics.rs`                    | Placeholder | Functions are stubs                    |

## Detailed Analysis of Orphan Files

### `ffi_backup.rs` (HIGH CONFIDENCE - SAFE TO DELETE)

**Analysis:**
- Contains complete FFI implementation with `FfiRegistry`, `FfiCallGenerator`, `FfiFunction`
- No imports found across the codebase
- Named with `_backup` suffix indicating intentional archival
- Duplicates functionality in `ffi_system/global_ffi_registry.rs` and `ffi_system/ffi_call_generator.rs`

**Verdict:** Safe to delete - backup file with no active references.

### `parser_symbol_fix.rs` (HIGH CONFIDENCE - SAFE TO DELETE)

**Analysis:**
- Contains single function `read_symbol()` with no module context
- No imports found across the codebase
- Appears to be a code fragment from tokenizer refactoring
- Functionality already exists in `parsing/tokenizer.rs`

**Verdict:** Safe to delete - orphaned fragment with no references.

### `parser_tokenize_fix.rs` (HIGH CONFIDENCE - SAFE TO DELETE)

**Analysis:**
- Contains single function `tokenize()` with no module context
- No imports found across the codebase
- Appears to be a code fragment from tokenizer refactoring
- Functionality already exists in `parsing/tokenizer.rs`

**Verdict:** Safe to delete - orphaned fragment with no references.

### `macro_system_old.rs` (MEDIUM CONFIDENCE - REVIEW BEFORE DELETE)

**Analysis:**
- Contains complete macro system implementation with `MacroExpander`, `MacroContext`, `HygieneScope`
- No imports found across the codebase
- Named with `_old` suffix indicating previous implementation
- Current implementation is in `macro_system/macro_expander.rs`

**Verdict:** Review for any valuable patterns before deletion. Current macro system may have lost features from this implementation.

### `physics_ffi.rs` (MEDIUM CONFIDENCE - INVESTIGATE BEFORE DELETE)

**Analysis:**
- Contains `impl PhysicsWorldCompiler` with `compile_ffi_call`, `get_ffi_capability`, `get_ffi_host_function`
- Imports `crate::capability_ffi::CapabilityMediatedFfiGenerator`
- **Not declared in any `mod.rs`** - this is the critical issue
- Uses `PhysicsWorldCompiler` which is defined in `physics_compiler.rs`
- File exists but Rust won't include it unless declared in a module

**Verdict:** This file is currently UNUSED because it's not declared in `physics_integration/mod.rs`. The `PhysicsWorldCompiler` methods are likely duplicated in `physics_compiler.rs`. Need to verify if functionality is needed before deletion.

## Recommended Cleanup Actions

### Phase 1: Immediate Deletion (Low Risk)

| File                     | Action | Justification                  |
| ------------------------ | ------ | ------------------------------ |
| `ffi_backup.rs`          | Delete | Backup file, no references     |
| `parser_symbol_fix.rs`   | Delete | Orphan fragment, no references |
| `parser_tokenize_fix.rs` | Delete | Orphan fragment, no references |

### Phase 2: Investigation Required

| File                                         | Action | Required Investigation                                        |
| -------------------------------------------- | ------ | ------------------------------------------------------------- |
| `macro_system_old.rs`                        | Review | Compare with current macro system for any lost features       |
| `physics_ffi.rs`                             | Verify | Check if functionality is duplicated in `physics_compiler.rs` |
| `physics_integration/ast_compilation/mod.rs` | Clean  | Remove commented-out module declarations                      |

### Phase 3: Technical Debt (Lower Priority)

| Item                          | Action                         | Justification                     |
| ----------------------------- | ------------------------------ | --------------------------------- |
| Commented module declarations | Remove comments                | Dead code clutter                 |
| Placeholder implementations   | Add TODO comments or implement | Documentation of missing features |

## Verification Steps

After each deletion:
1. Run `cargo test --package jue_world` to verify no breakage
2. Run `cargo build --package jue_world` to check compilation
3. Check for any new warnings about unused modules

## Rollback Plan

If issues arise after deletion:
1. Restore from git: `git checkout HEAD -- <deleted-file>`
2. Re-run tests to verify recovery