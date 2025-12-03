mod test_comprehensive_validation;
mod test_error_handling;
mod test_execution_integration;
mod test_pipeline_validation;
/// Phase 4 Execution Tests Module
/// This module integrates all phase 4 runtime validation tests
/// and provides a unified entry point for test execution
// Include all execution test modules
mod test_runtime_validation;

// Re-export test functions for external access
pub use test_comprehensive_validation::*;
pub use test_error_handling::*;
pub use test_execution_integration::*;
pub use test_pipeline_validation::*;
pub use test_runtime_validation::*;

/// Main integration test that runs all phase 4 execution tests
#[test]
fn run_all_phase_4_execution_tests() {
    // This test serves as an integration point that ensures
    // all phase 4 execution tests can be executed together

    // The individual test functions will be executed by the Rust test harness
    // This function just serves as documentation and integration point
    println!("Phase 4 Execution Tests Integration");
    println!("=================================");
    println!("All phase 4 execution tests are being executed by the Rust test harness");
    println!("This includes:");
    println!("- Arithmetic expressions execution validation");
    println!("- Variable declarations execution validation");
    println!("- Control flow execution validation");
    println!("- Function definitions execution validation");
    println!("- Full pipeline validation");
    println!("- Runtime error handling");
    println!("- Performance validation");
    println!("- Integration testing");
    println!("- Runtime readiness validation");
    println!("- Comprehensive validation");
    println!("- End-to-end testing");
    println!("- Pipeline validation");
    println!("- Success criteria validation");
}
