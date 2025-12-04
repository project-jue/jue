/// Main Test Harness for Jue Compiler
/// This file serves as the primary test entry point and integrates all test phases

/// Main test function that runs all compiler tests
#[test]
fn test_compiler_full_suite() {
    println!("🚀 Running Full Compiler Test Suite");
    println!("===================================");

    // Run phase 1 parsing tests by executing individual test functions
    println!("📝 Running Phase 1 Parsing Tests...");

    // Test arithmetic expressions parsing
    test_arithmetic_expressions_parsing();

    // Test variable declarations parsing
    test_variable_declarations_parsing();

    // Test control flow parsing
    test_control_flow_parsing();

    // Test function definitions parsing
    test_function_definitions_parsing();

    println!("✅ All compiler tests completed successfully!");

    // Additional test phases would be added here in the future
    // println!("🔧 Running Phase 2 MIR AST Tests...");
    // println!("🏗️ Running Phase 3 Compilation Tests...");
    // println!("🚀 Running Phase 4 Execution Tests...");
}

/// Test that demonstrates integration with shared samples
#[test]
fn test_shared_samples_integration() {
    // This test demonstrates how shared samples are used across the test suite
    use juec::frontend::parser;

    let sample_files = vec![
        "shared_samples/phase_1_parsing/01_arithmetic_expressions.jue",
        "shared_samples/phase_1_parsing/02_variable_declarations.jue",
        "shared_samples/phase_1_parsing/03_control_flow.jue",
        "shared_samples/phase_1_parsing/04_function_definitions.jue",
    ];

    for file_path in sample_files {
        let full_path = test_data::data_dir().join(file_path);
        let source =
            std::fs::read_to_string(full_path).expect(&format!("Failed to read {}", file_path));
        let result = parser::parse_jue(&source);

        assert!(
            result.is_ok(),
            "Should be able to parse {}: {:?}",
            file_path,
            result.err()
        );
    }
}

/// Test that validates the test infrastructure is working
#[test]
fn test_infrastructure_validation() {
    // Simple test to validate infrastructure
    use juec::frontend::parser;

    let simple_test = "x = 1 + 2";
    let result = parser::parse_jue(simple_test);
    assert!(
        result.is_ok(),
        "Simple parsing should work in test infrastructure"
    );
}

/// Test parser performance with large inputs
#[test]
fn test_parser_performance() {
    use juec::frontend::parser;

    // Create a large input to test parser performance
    let mut large_input = String::new();

    // Add many variable declarations
    for i in 0..50 {
        large_input.push_str(&format!("var_{} = {}\n", i, i * 2));
    }

    // Test that the parser can handle this large input
    let start_time = std::time::Instant::now();
    let result = parser::parse_jue(&large_input);
    let duration = start_time.elapsed();

    assert!(result.is_ok(), "Should be able to parse large input");
    println!("⏱️  Parser performance: {:?} for large input", duration);
}

/// Test arithmetic expressions parsing
fn test_arithmetic_expressions_parsing() {
    use juec::frontend::parser;

    let test_file =
        test_data::data_dir().join("shared_samples/phase_1_parsing/01_arithmetic_expressions.jue");
    let source = std::fs::read_to_string(test_file)
        .expect("Failed to read arithmetic expressions test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse arithmetic expressions");

    // Basic validation - should have multiple expressions
    assert!(!ast.body.is_empty(), "Should have parsed statements");

    println!("✅ Arithmetic expressions parsing test passed");
}

/// Test variable declarations parsing
fn test_variable_declarations_parsing() {
    use juec::frontend::parser;

    let test_file =
        test_data::data_dir().join("shared_samples/phase_1_parsing/02_variable_declarations.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read variable declarations test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse variable declarations");

    // Basic validation - should have assignment statements
    assert!(!ast.body.is_empty(), "Should have parsed statements");

    println!("✅ Variable declarations parsing test passed");
}

/// Test control flow parsing
fn test_control_flow_parsing() {
    use juec::frontend::parser;

    let test_file =
        test_data::data_dir().join("shared_samples/phase_1_parsing/03_control_flow.jue");
    let source = std::fs::read_to_string(test_file).expect("Failed to read control flow test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse control flow");

    // Basic validation - should have if statements
    assert!(!ast.body.is_empty(), "Should have parsed statements");

    println!("✅ Control flow parsing test passed");
}

/// Test function definitions parsing
fn test_function_definitions_parsing() {
    use juec::frontend::parser;

    let test_file =
        test_data::data_dir().join("shared_samples/phase_1_parsing/04_function_definitions.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read function definitions test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse function definitions");

    // Basic validation - should have function definitions
    assert!(!ast.body.is_empty(), "Should have parsed statements");

    println!("✅ Function definitions parsing test passed");
}
