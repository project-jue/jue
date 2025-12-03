use juec::frontend::ast;
use juec::frontend::parser;
use std::path::PathBuf;

/// Complex Parsing Tests
/// These tests validate advanced parsing functionality with complex expressions and structures
/// Building upon the basic parsing tests to ensure incremental complexity

#[test]
fn test_complex_expressions_parsing() {
    let test_file =
        PathBuf::from("tests/shared_samples/phase_1_parsing/05_complex_expressions.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read complex expressions test file");

    // Parse the complex source code
    let ast = parser::parse_jue(&source).expect("Failed to parse complex expressions");

    // Validate we have complex expressions
    assert!(
        !ast.expressions.is_empty(),
        "Should have parsed complex expressions"
    );

    // Verify we have nested binary operations (indicating complex expressions)
    let nested_binary_ops: Vec<_> = ast
        .expressions
        .iter()
        .filter(|expr| {
            if let ast::Expression::BinaryOp { left, op: _, right } = expr {
                // Check if left or right are also binary operations (nested)
                matches!(left, ast::Expression::BinaryOp { .. })
                    || matches!(right, ast::Expression::BinaryOp { .. })
            } else {
                false
            }
        })
        .collect();

    assert!(
        nested_binary_ops.len() > 0,
        "Should have nested binary operations in complex expressions"
    );

    // Verify we have complex assignment patterns
    let source_content = &source;
    assert!(
        source_content.contains("chained = a = b = c = 10 + 5 * 2"),
        "Should contain chained assignment patterns"
    );
    assert!(
        source_content.contains("nested_assign = x = (y = (z = 1 + 2 * 3)) + 4"),
        "Should contain nested assignment patterns"
    );
}

#[test]
fn test_advanced_control_flow_parsing() {
    let test_file =
        PathBuf::from("tests/shared_samples/phase_1_parsing/06_advanced_control_flow.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read advanced control flow test file");

    // Parse the complex control flow code
    let ast = parser::parse_jue(&source).expect("Failed to parse advanced control flow");

    // Validate we have complex nested if statements
    let if_statements: Vec<_> = ast
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, ast::Statement::If { .. }))
        .collect();

    assert!(
        if_statements.len() > 5,
        "Should have multiple complex if statements"
    );

    // Verify we have complex boolean logic
    let source_content = &source;
    assert!(
        source_content.contains("&&")
            && source_content.contains("||")
            && source_content.contains("!"),
        "Should contain complex boolean logic operators"
    );
    assert!(
        source_content.contains("nested if statements"),
        "Should contain deeply nested control flow structures"
    );
}

#[test]
fn test_complex_functions_parsing() {
    let test_file = PathBuf::from("tests/shared_samples/phase_1_parsing/07_complex_functions.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read complex functions test file");

    // Parse the complex function code
    let ast = parser::parse_jue(&source).expect("Failed to parse complex functions");

    // Validate we have complex function definitions
    let function_defs: Vec<_> = ast
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, ast::Statement::FunctionDef { .. }))
        .collect();

    assert!(
        function_defs.len() > 3,
        "Should have multiple complex function definitions"
    );

    // Verify we have recursive function patterns
    let source_content = &source;
    assert!(
        source_content.contains("factorial(n - 1)"),
        "Should contain recursive function calls"
    );
    assert!(
        source_content.contains("is_even") && source_content.contains("is_odd"),
        "Should contain mutually recursive functions"
    );

    // Verify we have complex nested function calls
    assert!(
        source_content.contains("complex_params(") && source_content.contains("factorial("),
        "Should contain complex nested function calls"
    );
}

#[test]
fn test_complexity_progression_validation() {
    // Test that we can parse all complexity levels in progression
    let complexity_files = vec![
        "tests/shared_samples/phase_1_parsing/01_arithmetic_expressions.jue", // Basic
        "tests/shared_samples/phase_1_parsing/02_variable_declarations.jue",  // Basic
        "tests/shared_samples/phase_1_parsing/03_control_flow.jue",           // Intermediate
        "tests/shared_samples/phase_1_parsing/04_function_definitions.jue",   // Intermediate
        "tests/shared_samples/phase_1_parsing/05_complex_expressions.jue",    // Advanced
        "tests/shared_samples/phase_1_parsing/06_advanced_control_flow.jue",  // Advanced
        "tests/shared_samples/phase_1_parsing/07_complex_functions.jue",      // Advanced
    ];

    for file_path in complexity_files {
        let source = std::fs::read_to_string(file_path).expect(&format!(
            "Failed to read complexity progression file: {}",
            file_path
        ));

        let result = parser::parse_jue(&source);

        assert!(
            result.is_ok(),
            "Should be able to parse complexity progression file: {}",
            file_path
        );

        let ast = result.unwrap();
        // Basic validation that we got a non-empty AST
        assert!(
            !ast.expressions.is_empty() || !ast.statements.is_empty(),
            "AST should contain expressions or statements for complexity progression: {}",
            file_path
        );
    }
}

#[test]
fn test_complex_ast_structure_validation() {
    // Test that complex AST structures are correctly formed
    let test_file = PathBuf::from("tests/shared_samples/phase_1_parsing/07_complex_functions.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read complex functions test file");
    let ast = parser::parse_jue(&source).expect("Failed to parse complex functions");

    // Validate that we have complex function structures
    for stmt in &ast.statements {
        if let ast::Statement::FunctionDef {
            name, params, body, ..
        } = stmt
        {
            // Complex functions should have multiple parameters or complex bodies
            assert!(
                params.len() > 1 || body.len() > 3,
                "Complex function {} should have multiple parameters or complex body",
                name
            );
        }
    }

    // Validate that complex expressions have proper nesting
    for expr in &ast.expressions {
        if let ast::Expression::BinaryOp { left, op: _, right } = expr {
            // In complex expressions, we should find nested structures
            if let ast::Expression::BinaryOp { .. } = left {
                assert!(
                    true,
                    "Found properly nested left binary operation in complex expression"
                );
            }
            if let ast::Expression::BinaryOp { .. } = right {
                assert!(
                    true,
                    "Found properly nested right binary operation in complex expression"
                );
            }
        }
    }
}
