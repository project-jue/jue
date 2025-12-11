# Core World Comprehensive Testing Plan

## Current Test Coverage Analysis

### Existing Tests
- `tests/core_tests.rs` - Comprehensive tests covering CoreExpr, CoreKernel, EvalRelation, and ProofChecker
- `tests/simple_normalization_tests.rs` - Focused tests for normalization edge cases
- `tests/core_world_comprehensive_tests.rs` - More comprehensive integration tests

### Coverage Gaps Identified
1. **Core Expression System** - Needs more edge case testing for complex nested expressions
2. **Core Kernel** - Needs additional tests for substitution edge cases and performance
3. **Evaluation Relation** - Needs more comprehensive closure and environment testing
4. **Proof Checker** - Needs additional tests for composite proofs and proof validation edge cases

## Testing Strategy

### Test Organization
- Follow TDD approach: basic functionality first, then complex scenarios
- Each test file should be 300-400 lines max
- Test files should be named by what they test (e.g., `core_expr_tests.rs`, `beta_reduction_tests.rs`)
- Place tests in appropriate directories:
  - Unit tests in `tests/` directory
  - Integration tests in `tests/` with clear naming
  - Performance tests in `tests/` with `_performance` suffix

### Test Categories

#### 1. Core Expression Tests (`tests/core_expr_tests.rs`)
- Variable creation and manipulation
- Lambda expression creation and structure
- Application expression creation and structure
- Display formatting for all expression types
- Complex nested expression construction
- Expression equality and comparison

#### 2. Beta Reduction Tests (`tests/beta_reduction_tests.rs`)
- Basic identity function reduction
- Complex nested reductions
- Variable capture avoidance
- Multiple reduction steps
- Non-reducible expressions
- Performance benchmarks

#### 3. Normalization Tests (`tests/normalization_tests.rs`)
- Simple expression normalization
- Complex nested normalization
- Idempotent normalization verification
- Normal form detection
- Edge cases with deeply nested expressions
- Performance benchmarks

#### 4. Evaluation Relation Tests (`tests/eval_relation_tests.rs`)
- Variable lookup in environments
- Lambda introduction (closure creation)
- Application elimination (beta reduction)
- Closure application and substitution
- Normal form detection
- Empty environment evaluation
- Complex environment scenarios

#### 5. Proof System Tests (`tests/proof_system_tests.rs`)
- Beta reduction proof generation and verification
- Alpha equivalence proof generation and verification
- Normalization proof generation and verification
- Evaluation proof generation and verification
- Composite proof construction and verification
- Invalid proof detection
- Proof consistency checking
- Proven expression handling

#### 6. Integration Tests (`tests/core_integration_tests.rs`)
- Core kernel and evaluation relation consistency
- Proof system integration with core operations
- End-to-end expression processing
- Cross-component interaction verification

#### 7. Performance Tests (`tests/core_performance_tests.rs`)
- Large expression processing
- Deeply nested reduction performance
- Memory usage benchmarks
- Stress testing with complex scenarios

## Test Implementation Plan

### Phase 1: Basic Functionality Tests
1. Create `tests/core_expr_tests.rs` with basic expression tests
2. Create `tests/beta_reduction_tests.rs` with basic reduction tests
3. Create `tests/normalization_tests.rs` with basic normalization tests
4. Create `tests/eval_relation_tests.rs` with basic evaluation tests

### Phase 2: Advanced Functionality Tests
1. Add complex expression tests to `core_expr_tests.rs`
2. Add advanced reduction scenarios to `beta_reduction_tests.rs`
3. Add edge case normalization tests to `normalization_tests.rs`
4. Add complex evaluation scenarios to `eval_relation_tests.rs`

### Phase 3: Proof System Tests
1. Create `tests/proof_system_tests.rs` with comprehensive proof testing
2. Add integration tests between proof system and core components

### Phase 4: Integration and Performance Tests
1. Create `tests/core_integration_tests.rs` for cross-component testing
2. Create `tests/core_performance_tests.rs` for performance benchmarks

## Test Quality Standards

- Each test should be focused on a single functionality
- Tests should be independent and not rely on shared state
- Use descriptive test names that explain what is being tested
- Include setup, execution, and verification in each test
- Add comments explaining complex test scenarios
- Follow Rust testing best practices and conventions

## Test Execution Strategy

- Run tests incrementally as they are created
- Fix any issues immediately (either in tests or main code)
- Ensure all tests pass before proceeding to next phase
- Use `cargo test` for test execution
- Consider adding test coverage reporting

## Expected Outcomes

- Full unit test coverage of all core system components
- Comprehensive integration testing
- Performance benchmarks for critical operations
- Clear documentation of test coverage
- Improved code quality through test-driven development