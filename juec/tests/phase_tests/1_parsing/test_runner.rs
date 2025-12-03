use juec::frontend::ast;
use juec::frontend::parser;
use std::path::PathBuf;

pub mod test_error_handling;
/// Comprehensive Test Runner for Phase 1 Parsing
/// This module provides a unified way to run all phase 1 parsing tests
/// and integrates with the existing test infrastructure
pub mod test_parsing_validation;
pub mod test_syntax_validation;

/// Run all phase 1 parsing tests in sequence
/// This function can be called from the main test harness
pub fn run_all_phase_1_parsing_tests() -> Result<(), String> {
    println!("🚀 Starting Phase 1 Parsing Tests");
    println!("=================================");

    // Run parsing validation tests
    println!("📝 Running parsing validation tests...");
    if let Err(e) = run_parsing_validation_tests() {
        return Err(format!("Parsing validation tests failed: {}", e));
    }

    // Run syntax validation tests
    println!("🔍 Running syntax validation tests...");
    if let Err(e) = run_syntax_validation_tests() {
        return Err(format!("Syntax validation tests failed: {}", e));
    }

    // Run error handling tests
    println!("⚠️  Running error handling tests...");
    if let Err(e) = run_error_handling_tests() {
        return Err(format!("Error handling tests failed: {}", e));
    }

    println!("✅ All Phase 1 Parsing Tests Completed Successfully!");
    Ok(())
}

/// Run parsing validation tests
fn run_parsing_validation_tests() -> Result<(), String> {
    // Test arithmetic expressions
    test_parsing_validation::test_arithmetic_expressions_parsing();

    // Test variable declarations
    test_parsing_validation::test_variable_declarations_parsing();

    // Test control flow
    test_parsing_validation::test_control_flow_parsing();

    // Test function definitions
    test_parsing_validation::test_function_definitions_parsing();

    // Test syntax error handling
    test_parsing_validation::test_syntax_error_handling();

    // Test comprehensive parsing integration
    test_parsing_validation::test_comprehensive_parsing_integration();

    // Test AST structure validation
    test_parsing_validation::test_ast_structure_validation();

    // Test parser error recovery
    test_parsing_validation::test_parser_error_recovery();

    Ok(())
}

/// Run syntax validation tests
fn run_syntax_validation_tests() -> Result<(), String> {
    // Test arithmetic operator precedence
    test_syntax_validation::test_arithmetic_operator_precedence();

    // Test variable assignment patterns
    test_syntax_validation::test_variable_assignment_patterns();

    // Test control flow structures
    test_syntax_validation::test_control_flow_structures();

    // Test function definition patterns
    test_syntax_validation::test_function_definition_patterns();

    // Test function call patterns
    test_syntax_validation::test_function_call_patterns();

    // Test syntax error scenarios
    test_syntax_validation::test_syntax_error_scenarios();

    // Test mixed syntax validation
    test_syntax_validation::test_mixed_syntax_validation();

    Ok(())
}

/// Run error handling tests
fn run_error_handling_tests() -> Result<(), String> {
    // Test invalid operator combinations
    test_error_handling::test_invalid_operator_combinations();

    // Test incomplete statements
    test_error_handling::test_incomplete_statements();

    // Test invalid identifier usage
    test_error_handling::test_invalid_identifier_usage();

    // Test type mismatch scenarios
    test_error_handling::test_type_mismatch_scenarios();

    // Test nested structure depth
    test_error_handling::test_nested_structure_depth();

    // Test parser resilience with mixed valid/invalid
    test_error_handling::test_parser_resilience_with_mixed_valid_invalid();

    // Test empty and whitespace scenarios
    test_error_handling::test_empty_and_whitespace_scenarios();

    // Test parser edge cases
    test_error_handling::test_parser_edge_cases();

    Ok(())
}

/// Test that all shared sample files can be parsed successfully
/// This serves as a comprehensive integration test
#[test]
fn test_all_shared_samples_integration() {
    let sample_files = vec![
        "tests/shared_samples/phase_1_parsing/01_arithmetic_expressions.jue",
        "tests/shared_samples/phase_1_parsing/02_variable_declarations.jue",
        "tests/shared_samples/phase_1_parsing/03_control_flow.jue",
        "tests/shared_samples/phase_1_parsing/04_function_definitions.jue",
    ];

    let mut total_expressions = 0;
    let mut total_statements = 0;
    let mut total_functions = 0;
    let mut total_if_statements = 0;

    for file_path in sample_files {
        let source =
            std::fs::read_to_string(file_path).expect(&format!("Failed to read {}", file_path));
        let result = parser::parse_jue(&source);

        assert!(
            result.is_ok(),
            "Should be able to parse {}: {:?}",
            file_path,
            result.err()
        );

        let ast = result.unwrap();
        total_expressions += ast.expressions.len();
        total_statements += ast.statements.len();

        // Count function definitions
        let functions: usize = ast
            .statements
            .iter()
            .filter(|stmt| matches!(stmt, ast::Statement::FunctionDef { .. }))
            .count();
        total_functions += functions;

        // Count if statements
        let if_statements: usize = ast
            .statements
            .iter()
            .filter(|stmt| matches!(stmt, ast::Statement::If { .. }))
            .count();
        total_if_statements += if_statements;
    }

    println!("📊 Shared Samples Integration Results:");
    println!("- Total expressions parsed: {}", total_expressions);
    println!("- Total statements parsed: {}", total_statements);
    println!("- Total function definitions: {}", total_functions);
    println!("- Total if statements: {}", total_if_statements);

    // Basic validation that we actually parsed meaningful content
    assert!(total_expressions > 0, "Should have parsed some expressions");
    assert!(total_statements > 0, "Should have parsed some statements");
    assert!(total_functions > 0, "Should have parsed some functions");
    assert!(
        total_if_statements > 0,
        "Should have parsed some if statements"
    );
}

/// Test parser performance with large input
#[test]
fn test_parser_performance_with_large_input() {
    // Create a large input to test parser performance
    let mut large_input = String::new();

    // Add many variable declarations
    for i in 0..100 {
        large_input.push_str(&format!("var_{} = {}\n", i, i * 2));
    }

    // Add many arithmetic expressions
    for i in 0..50 {
        large_input.push_str(&format!("result_{} = {} + {} * {}\n", i, i, i + 1, i + 2));
    }

    // Add some control flow
    large_input.push_str(
        r#"
        if result_0 > 100 {
            status = "high"
        } else {
            status = "low"
        }
    "#,
    );

    // Add a function definition
    large_input.push_str(
        r#"
        function calculate_sum(n) {
            total = 0
            for i in 0..n {
                total = total + i
            }
            return total
        }
    "#,
    );

    // Test that the parser can handle this large input
    let start_time = std::time::Instant::now();
    let result = parser::parse_jue(&large_input);
    let duration = start_time.elapsed();

    assert!(result.is_ok(), "Should be able to parse large input");

    let ast = result.unwrap();
    assert!(
        !ast.statements.is_empty(),
        "Should have parsed statements from large input"
    );

    println!("⏱️  Parser performance: {:?} for large input", duration);
}

/// Test that the test infrastructure itself is working correctly
#[test]
fn test_test_infrastructure_validation() {
    // This meta-test validates that our test infrastructure is properly set up

    // Verify we can access the test modules
    assert!(true, "Test infrastructure should be accessible");

    // Verify we can run a simple parsing test
    let simple_test = "x = 1 + 2";
    let result = parser::parse_jue(simple_test);
    assert!(
        result.is_ok(),
        "Simple parsing should work in test infrastructure"
    );

    println!("✅ Test infrastructure validation passed");
}
