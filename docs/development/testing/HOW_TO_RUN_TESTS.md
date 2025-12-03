# How to Run Tests - Jue Compiler Project

This document provides comprehensive instructions for running all types of tests in the Jue compiler project. The test suite is organized into multiple categories to ensure thorough coverage of all components and features.

## Table of Contents

- [Basic Test Commands](#basic-test-commands)
- [Test Types and Organization](#test-types-and-organization)
- [Running Specific Test Categories](#running-specific-test-categories)
- [Advanced Test Options](#advanced-test-options)
- [Test Coverage and Reporting](#test-coverage-and-reporting)
- [CI/CD Integration](#cicd-integration)

## Basic Test Commands

### Run All Tests

To run the complete test suite for the entire workspace:

```bash
cargo test --workspace
```

This command executes all tests across both the compiler (`juec`) and runtime (`juerun`) components.

### Quick Test via Makefile

The project includes a Makefile with convenient test commands:

```bash
make test
```

This is equivalent to running `cargo test --workspace`.

## Test Types and Organization

The Jue test suite is organized into the following categories:

### 1. Unit Tests
- Test individual functions and components in isolation
- Located in individual source files alongside the code
- Focus on specific functionality without external dependencies

### 2. Integration Tests
- Test interactions between multiple components
- Verify that combined components work together correctly
- Located in dedicated test directories

### 3. Phase Tests
- Organized by development phases (1-4)
- Test progressively more complex features
- Located in `tests/phase_tests/` and component-specific phase directories

### 4. Performance Tests
- Benchmark performance-critical code paths
- Measure execution time and resource usage
- Help identify performance regressions

### 5. Regression Tests
- Prevent reintroducing previously fixed bugs
- Ensure existing functionality continues to work
- Run automatically in CI/CD pipelines

### 6. Edge Case Tests
- Test boundary conditions and error scenarios
- Validate robust error handling
- Ensure graceful failure modes

## Running Specific Test Categories

### Run Compiler Tests Only

```bash
cd juec && cargo test
```

### Run Runtime Tests Only

```bash
cd juerun && cargo test
```

### Run Tests for Specific Components

#### Compiler Component Tests

```bash
# Run all juec tests
cargo test -p juec

# Run specific test file
cargo test -p juec test_compilation_simple

# Run tests matching a pattern
cargo test -p juec parsing
```

#### Runtime Component Tests

```bash
# Run all juerun tests
cargo test -p juerun

# Run specific test file
cargo test -p juerun vm_tests

# Run tests matching a pattern
cargo test -p juerun execution
```

### Run Phase-Specific Tests

```bash
# Run Phase 1 (Parsing) tests
cargo test -p juec phase_1

# Run Phase 2 (MIR/AST) tests
cargo test -p juec phase_2

# Run Phase 3 (Compilation) tests
cargo test -p juec phase_3

# Run Phase 4 (Execution/Homoiconic) tests
cargo test -p juerun phase_4
```

## Advanced Test Options

### Run Tests with Verbose Output

```bash
cargo test --workspace -- --nocapture
```

### Run Tests in Release Mode

```bash
cargo test --workspace --release
```

### Run Specific Test by Name

```bash
cargo test --workspace test_name
```

### Run Tests with Filter

```bash
cargo test --workspace -- test_filter_pattern
```

### Run Tests and Show Output for Passing Tests

```bash
cargo test --workspace -- --show-output
```

## Test Coverage and Reporting

### Generate Test Coverage Report

```bash
cargo tarpaulin --workspace --out Html
```

This generates an HTML coverage report in the `tarpaulin-report` directory.

### View Coverage Summary

```bash
cargo tarpaulin --workspace --out Summary
```

### Generate Detailed Coverage Report

```bash
cargo tarpaulin --workspace --out Xml --output-dir coverage-reports
```

## CI/CD Integration

### Standard CI Test Command

```bash
cargo test --workspace
```

### CI Test with Coverage

```bash
cargo tarpaulin --workspace --out Xml
```

### CI Build and Test Sequence

```bash
# Build all components
cargo build --workspace

# Run all tests
cargo test --workspace

# Generate coverage report
cargo tarpaulin --workspace --out Xml
```

## Test Execution Best Practices

### Recommended Test Workflow

1. **Local Development**: Run specific component tests during development
2. **Pre-Commit**: Run full test suite before committing
3. **CI Pipeline**: Full test suite with coverage reporting
4. **Performance Validation**: Run performance tests periodically

### Common Test Patterns

```bash
# Run a quick sanity check
make test

# Run tests with detailed output
cargo test --workspace -- --nocapture

# Run tests for a specific feature
cargo test -p juec parsing

# Check coverage before major changes
cargo tarpaulin --workspace
```

## Troubleshooting

### Test Failure Analysis

```bash
# Run failed tests only
cargo test --workspace -- --test-threads=1

# Run with backtrace for debugging
RUST_BACKTRACE=1 cargo test --workspace
```

### Performance Testing

```bash
# Run performance tests
cargo bench

# Run specific benchmark
cargo bench benchmark_name
```

## Test Organization Structure

```
tests/
├── shared_samples/          # Shared .jue files used across tests
├── phase_tests/             # Phase-organized tests (1-4)
│   ├── 1_parsing/           # Phase 1: Parsing tests
│   ├── 2_mir_ast/           # Phase 2: MIR/AST tests
│   ├── 3_compilation/       # Phase 3: Compilation tests
│   └── 4_execution/         # Phase 4: Execution tests
├── component_tests/         # Component-specific tests
├── integration_tests/       # Cross-component tests
├── performance_tests/       # Benchmark tests
└── regression_tests/        # Regression prevention tests

juec/tests/                   # Compiler-specific tests
juerun/tests/                 # Runtime-specific tests
```

## Command Summary Cheat Sheet

| Test Type           | Command                       | Description              |
| ------------------- | ----------------------------- | ------------------------ |
| **All Tests**       | `cargo test --workspace`      | Run complete test suite  |
| **Quick Test**      | `make test`                   | Run tests via Makefile   |
| **Compiler Only**   | `cargo test -p juec`          | Run compiler tests       |
| **Runtime Only**    | `cargo test -p juerun`        | Run runtime tests        |
| **Specific Test**   | `cargo test test_name`        | Run named test           |
| **Coverage Report** | `cargo tarpaulin --workspace` | Generate coverage        |
| **Verbose Output**  | `cargo test -- --nocapture`   | Show all test output     |
| **Release Mode**    | `cargo test --release`        | Test optimized build     |
| **Phase Tests**     | `cargo test phase_1`          | Run phase-specific tests |

This comprehensive testing guide provides all the commands needed to run, analyze, and maintain the Jue compiler test suite effectively.