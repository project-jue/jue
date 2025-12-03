mod test_complex_parsing;
mod test_error_handling;
/// Phase 1 Parsing Tests Module
/// This module integrates all phase 1 parsing validation tests
/// and provides a unified entry point for test execution
// Include all parsing test modules
mod test_parsing_validation;
mod test_syntax_validation;

/// Main integration test that runs all phase 1 parsing tests
#[test]
fn run_all_phase_1_parsing_tests() {
    // This test serves as an integration point that ensures
    // all phase 1 parsing tests can be executed together

    // The individual test functions will be executed by the Rust test harness
    // This function just serves as documentation and integration point
    println!("Phase 1 Parsing Tests Integration");
    println!("=================================");
    println!("All phase 1 parsing tests are being executed by the Rust test harness");
    println!("This includes:");
    println!("- Arithmetic expressions parsing");
    println!("- Variable declarations parsing");
    println!("- Control flow parsing");
    println!("- Function definitions parsing");
    println!("- Complex expressions parsing");
    println!("- Advanced control flow parsing");
    println!("- Complex functions parsing");
    println!("- Syntax validation");
    println!("- Error handling");
    println!("- Comprehensive integration tests");
    println!("- Complexity progression validation");
}
