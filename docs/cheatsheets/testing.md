# Testing Cheat Sheet

## Test Organization

### Test File Structure
- **Unit tests**: Inline in source files with `#[cfg(test)]` modules
- **Integration tests**: In `tests/` directory
- **Performance tests**: In `benches/` directory

### Test File Naming
- Unit test modules: `mod tests { ... }`
- Integration test files: `tests/<feature>_test.rs`
- Performance test files: `benches/<feature>_bench.rs`

## Test Commands

### Running Tests

**Windows (default):**
```cmd
:: Run all tests
cargo test

:: Run specific test
cargo test test_name

:: Run tests in specific module
cargo test module_name::test_name

:: Run tests with logging
set RUST_LOG=debug && cargo test

:: Run tests with backtrace
set RUST_BACKTRACE=1 && cargo test
```

**Linux:**
```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests in specific module
cargo test module_name::test_name

# Run tests with logging
RUST_LOG=debug cargo test

# Run tests with backtrace
RUST_BACKTRACE=1 cargo test
```

## Test Attributes

### Common Test Annotations
```rust
#[test]
fn test_name() {
    // Test implementation
}

#[test]
#[ignore]
fn ignored_test() {
    // Test that won't run by default
}

#[test]
#[should_panic(expected = "specific error message")]
fn test_panic() {
    // Test that should panic
}
```

### Test Setup and Teardown
```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> TestContext {
        // Setup code
    }

    fn teardown(context: TestContext) {
        // Teardown code
    }

    #[test]
    fn test_with_setup() {
        let context = setup();
        // Test code
        teardown(context);
    }
}
```

## Test Utilities

### Common Test Helpers
```rust
// Create test helpers in separate modules
#[cfg(test)]
mod test_helpers {
    pub fn create_test_expression() -> CoreExpr {
        // Helper implementation
    }

    pub fn assert_normal_form(expr: &CoreExpr) {
        // Assertion helper
    }
}
```

### Property-Based Testing with proptest
```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_property(input in any::<u32>()) {
        // Property-based test
    }
}
```

## Test Coverage

### Coverage Tools
```bash
# Install cargo-tarpaulin (Linux)
cargo install cargo-tarpaulin

# Generate coverage report
cargo tarpaulin --out Html
```

### Coverage Analysis
- Aim for 80%+ coverage on critical modules
- Focus on edge cases and error conditions
- Review uncovered code for missing tests

## Debugging Tests

### Debugging Techniques
```rust
#[test]
fn debug_test() {
    // Use println! for debugging
    println!("Debug value: {:?}", value);

    // Use env_logger for structured logging
    log::debug!("Debug message: {}", value);

    // Use assert_eq! with custom messages
    assert_eq!(actual, expected, "Custom failure message");
}
```

### Common Debug Commands
```bash
# Run single test with verbose output
cargo test test_name -- --nocapture

# Show test output even on success
cargo test -- --nocapture

# Run tests with specific features
cargo test --features "test_feature"
```

## Performance Testing

### Benchmark Setup
```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("function_name", |b| {
        b.iter(|| {
            // Code to benchmark
        })
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

### Running Benchmarks
```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench benchmark_function
```

## Test Best Practices

### Test Organization
- Keep tests small and focused
- Use descriptive test names
- Group related tests together
- Separate unit and integration tests

### Test Quality
- Test both happy paths and error cases
- Avoid testing implementation details
- Use test doubles (mocks/stubs) sparingly
- Keep tests deterministic

### Test Maintenance
- Update tests when requirements change
- Remove obsolete tests
- Review test coverage regularly
- Document complex test scenarios