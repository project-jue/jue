
# **LLM-Assisted Software Engineering Guidelines**

This document defines standards for projects that incorporate Large Language Model (LLM)–driven code generation. The goal is to minimize hallucination, architectural drift, misorganization, and context-limit constraints while maximizing correctness, maintainability, and predictability.

---

## **1. File Size and Code Granularity**

**1.1 Keep files small and focused**
Avoid large monolithic source files. Divide functionality into coherent modules, typically 100–400 lines each.

**1.2 Prefer many small modules over fewer large ones**
Smaller files reduce LLM confusion and improve the accuracy of edits.

**1.3 Mirror module boundaries in tests**
Inline tests belong in the same file as the code they validate.
Integration tests belong under `/tests`.

---

## **2. Documentation Structure for LLM Consumption**

**2.1 Maintain clear, short engineering documents**
Place all design and engineering documents under `docs/`.

**2.2 Use a predictable directory layout**

```
docs/
  overview.md
  design/
  subsystems/
  cheatsheets/
  prompts/
  adr/
```

**2.3 Provide subsystem summaries**
Each major subsystem receives a short, self-contained summary under `docs/subsystems/`.

**2.4 Keep documents concise and explicit**
Use short sections, explicit definitions, and shallow hierarchical structure.
Avoid long uninterrupted prose.

---

## **3. Repository Organization**

**3.1 Keep the root directory clean**
The root directory contains only essential project files: README, license, Cargo manifest, top-level config.

**3.2 Organize all source code under `src/`**
Group modules in subdirectories by domain to avoid crowding.
Example:

```
src/
  core/
  runtime/
  parser/
  vm/
  utils/
```

**3.3 Organize tests appropriately**

* Inline unit tests remain with the module.
* Scenario-driven integration tests go in `tests/`.

**3.4 Maintain documentation hierarchy**
Do not place loose documents at the root.

---

## **4. Cheat Sheets and Environment Reference**

**4.1 Provide environment summaries**
Store OS details, tool versions, and system notes in:
`docs/cheatsheets/environment.md`

**4.2 Provide testing and tooling cheat sheets**
Include instructions for test commands, coverage tools, and debugging under:
`docs/cheatsheets/testing.md`

**4.3 Maintain filesystem and shell command reference**
Capture file operations and common commands for the development environment under:
`docs/cheatsheets/filesystem.md`

**4.4 Supply LLM integration guidance**
Document prompting standards, context packaging, and safety checks in:
`docs/cheatsheets/llm_integration.md`

---

## **5. Naming Conventions**

**5.1 Use normalized filenames**
All files use lowercase with underscores.
Avoid inconsistent naming or redundant suffixes.

**5.2 Use predictable directory names**
Short, clear, lowercase names with single, unambiguous meaning.

**5.3 Follow Rust naming standards**
Modules and crate names follow snake_case.
Avoid surprises in file-to-module mapping.

---

## **6. Testing Strategy**

**6.1 Emphasize unit-level coverage**
Every module receives extensive inline tests covering nominal cases, edge cases, and error states.

**6.2 Prefer many short tests over a few large ones**
Short, focused tests simplify LLM review and regeneration.

**6.3 Keep integration tests scenario-driven**
Integration tests exercise public API usage and multi-module workflows.

**6.4 Use test helpers**
Shared utilities belong in test-only modules behind `#[cfg(test)]`.

**6.5 Avoid leaking internals into public API**
Do not make private items public solely for testing.

---

## **7. Documenting Invariants and Assumptions**

**7.1 Record invariants for each module**
Add invariants either in comments or in `docs/subsystems/<module>.md`.

**7.2 Define error-handling strategy**
Document expected error types, panic boundaries, and recovery behavior.

**7.3 Clarify concurrency rules**
If the system uses threads, async, or shared state, document the rules explicitly.

---

## **8. LLM-Oriented Coding Practices**

**8.1 Apply changes one file at a time**
Avoid large multi-file edits through an LLM unless necessary.

**8.2 Always provide full context**
LLMs produce more accurate patches when given full source files rather than fragments.

**8.3 Limit change scope**
Specify what must be modified and what must remain untouched.

**8.4 Validate output immediately**
Compile, lint, run tests, and check formatting after every LLM-generated patch.

**8.5 Maintain canonical examples**
Store canonical code examples for patterns such as error handling, parsing, logging, and state transitions.
LLMs generalize from these patterns.

**8.6 Maintain reusable prompt templates**
Store prompt templates for common tasks under `docs/prompts/`.

---

## **9. Stability and Architectural Consistency**

**9.1 Keep module boundaries stable**
Frequent reorganization increases the chance of incorrect LLM assumptions.

**9.2 Use Architecture Decision Records (ADRs)**
Record each major decision under `docs/adr/`.

**9.3 Standardize the code-validation pipeline**
Create a script (e.g., `./check.sh`) that runs:

* formatting
* linting
* clippy
* unit tests
* integration tests
* coverage

**9.4 Periodic cleanup of generated tests**
Review test suites and remove redundant or overly specific tests.

**9.5 Keep example code small and precise**
Examples should be easy for LLMs to load into context.

---

## 10. **10. Put inline tests in a separate file, to reduce file sizes  **
Large file sizes cause troubles when being debugged or edited by LLMs.  Most of the file size is taken by the inline tests. Luckily there are ways to keep the tests in a separate file and still access private members if needed.

Separate Test Files with #[path]
Split tests into separate files while maintaining access to private members:

```rust
// src/my_module.rs
pub struct MyStruct {
    private_field: i32,
}

impl MyStruct {
    pub fn new() -> Self { /* ... */ }
    fn private_method(&self) -> i32 { /* ... */ }
}

#[cfg(test)]
#[path = "test/my_module_tests.rs"]  // Path to test file
mod tests;
```

Access Private Members in Tests:
```rust
// src/test/my_module_tests.rs
use super::*;  // Import from parent module
// you may have to import other crates here to resolve symbols

#[test]
fn test_private_method() {
    let s = MyStruct::new();
    assert_eq!(s.private_method(), 42); // Can access private_method
}
```
---

## **11. Summary**

The purpose of these guidelines is to structure development so that LLM-driven code generation becomes predictable and robust. By keeping files small, documentation clear, naming consistent, and architectural decisions explicit, the project remains manageable for both humans and automated systems.

---