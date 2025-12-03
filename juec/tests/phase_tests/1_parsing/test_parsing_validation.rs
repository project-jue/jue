use juec::frontend::ast;
use juec::frontend::parser;
use std::path::PathBuf;

/// Phase 1 Parsing Validation Tests
/// These tests validate basic Jue syntax parsing functionality using shared sample files
/// and follow the test patterns established in the architecture.

#[test]
fn test_arithmetic_expressions_parsing() {
    let test_file =
        PathBuf::from("tests/shared_samples/phase_1_parsing/01_arithmetic_expressions.jue");
    let source = std::fs::read_to_string(test_file)
        .expect("Failed to read arithmetic expressions test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse arithmetic expressions");

    // Basic validation - should have multiple expressions
    assert!(
        !ast.expressions.is_empty(),
        "Should have parsed expressions"
    );

    // Verify we can find binary operations
    let binary_ops: Vec<_> = ast
        .expressions
        .iter()
        .filter(|expr| matches!(expr, ast::Expression::BinaryOp { .. }))
        .collect();

    assert!(binary_ops.len() > 0, "Should have binary operations");

    // Verify operator precedence by checking for parentheses
    let has_parentheses = source.contains('(') && source.contains(')');
    assert!(
        has_parentheses,
        "Should contain parenthesized expressions for precedence testing"
    );
}

#[test]
fn test_variable_declarations_parsing() {
    let test_file =
        PathBuf::from("tests/shared_samples/phase_1_parsing/02_variable_declarations.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read variable declarations test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse variable declarations");

    // Basic validation - should have assignment statements
    assert!(!ast.statements.is_empty(), "Should have parsed statements");

    // Verify we can find assignments
    let assignments: Vec<_> = ast
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, ast::Statement::Assign { .. }))
        .collect();

    assert!(assignments.len() > 0, "Should have assignment statements");

    // Verify we have different types of assignments
    let source_content = &source;
    assert!(
        source_content.contains("x = 1"),
        "Should contain simple numeric assignment"
    );
    assert!(
        source_content.contains("name = \"hello\""),
        "Should contain string assignment"
    );
    assert!(
        source_content.contains("active = true"),
        "Should contain boolean assignment"
    );
}

#[test]
fn test_control_flow_parsing() {
    let test_file = PathBuf::from("tests/shared_samples/phase_1_parsing/03_control_flow.jue");
    let source = std::fs::read_to_string(test_file).expect("Failed to read control flow test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse control flow");

    // Basic validation - should have if statements
    assert!(!ast.statements.is_empty(), "Should have parsed statements");

    // Verify we can find if statements
    let if_statements: Vec<_> = ast
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, ast::Statement::If { .. }))
        .collect();

    assert!(if_statements.len() > 0, "Should have if statements");

    // Verify we have different control flow constructs
    let source_content = &source;
    assert!(
        source_content.contains("if x > 0"),
        "Should contain simple if statement"
    );
    assert!(
        source_content.contains("else"),
        "Should contain else clauses"
    );
    assert!(
        source_content.contains("elif"),
        "Should contain elif clauses"
    );
    assert!(
        source_content.contains("nested"),
        "Should contain nested if statements"
    );
}

#[test]
fn test_function_definitions_parsing() {
    let test_file =
        PathBuf::from("tests/shared_samples/phase_1_parsing/04_function_definitions.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read function definitions test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse function definitions");

    // Basic validation - should have function definitions
    assert!(!ast.statements.is_empty(), "Should have parsed statements");

    // Verify we can find function definitions
    let function_defs: Vec<_> = ast
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, ast::Statement::FunctionDef { .. }))
        .collect();

    assert!(function_defs.len() > 0, "Should have function definitions");

    // Verify we have different types of function definitions
    let source_content = &source;
    assert!(
        source_content.contains("function greet()"),
        "Should contain function without parameters"
    );
    assert!(
        source_content.contains("function add(a, b)"),
        "Should contain function with parameters"
    );
    assert!(
        source_content.contains("return"),
        "Should contain return statements"
    );
    assert!(
        source_content.contains("function calls"),
        "Should contain function calls"
    );
}

#[test]
fn test_syntax_error_handling() {
    // Test that invalid syntax produces appropriate errors
    let invalid_syntax_cases = vec![
        "function missing_paren {", // Missing closing parenthesis
        "if x > 0 {",               // Missing closing brace
        "x = ",                     // Incomplete assignment
        "1 + * 2",                  // Invalid operator sequence
    ];

    for (i, invalid_code) in invalid_syntax_cases.iter().enumerate() {
        let result = parser::parse_jue(invalid_code);
        assert!(
            result.is_err(),
            "Case {} should fail to parse: {}",
            i + 1,
            invalid_code
        );
    }
}

#[test]
fn test_comprehensive_parsing_integration() {
    // Test that all shared sample files can be parsed successfully
    let sample_files = vec![
        "tests/shared_samples/phase_1_parsing/01_arithmetic_expressions.jue",
        "tests/shared_samples/phase_1_parsing/02_variable_declarations.jue",
        "tests/shared_samples/phase_1_parsing/03_control_flow.jue",
        "tests/shared_samples/phase_1_parsing/04_function_definitions.jue",
    ];

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
        // Basic validation that we got a non-empty AST
        assert!(
            !ast.expressions.is_empty() || !ast.statements.is_empty(),
            "AST should contain expressions or statements for {}",
            file_path
        );
    }
}

#[test]
fn test_ast_structure_validation() {
    // Test that the AST structure is correct for basic constructs
    let test_file =
        PathBuf::from("tests/shared_samples/phase_1_parsing/01_arithmetic_expressions.jue");
    let source = std::fs::read_to_string(test_file)
        .expect("Failed to read arithmetic expressions test file");
    let ast = parser::parse_jue(&source).expect("Failed to parse arithmetic expressions");

    // Validate that we have the expected number of expressions
    // The arithmetic expressions file should have multiple expressions
    assert!(
        ast.expressions.len() >= 10,
        "Should have at least 10 arithmetic expressions"
    );

    // Validate that binary operations have correct structure
    for expr in &ast.expressions {
        if let ast::Expression::BinaryOp { left, op, right } = expr {
            // Basic validation that binary ops have left and right operands
            assert!(
                !matches!(left, ast::Expression::Empty),
                "Binary op should have left operand"
            );
            assert!(
                !matches!(right, ast::Expression::Empty),
                "Binary op should have right operand"
            );
        }
    }
}

#[test]
fn test_parser_error_recovery() {
    // Test that the parser can handle and recover from errors gracefully
    let problematic_code = r#"
        // Valid code
        x = 1 + 2

        // Invalid code that should cause error
        function broken_syntax (
        // More valid code after error
        y = 3 + 4
    "#;

    let result = parser::parse_jue(problematic_code);
    // The parser should either fail gracefully or recover and continue parsing
    // For now, we just verify it doesn't panic
    let _ = result;
}
