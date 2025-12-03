use juec::frontend::ast;
use juec::frontend::parser;
use std::path::PathBuf;

/// Syntax Validation Tests
/// These tests focus on validating specific syntax constructs and error handling

#[test]
fn test_arithmetic_operator_precedence() {
    let test_cases = vec![
        // Basic operator precedence
        "2 + 3 * 4",         // Should parse as 2 + (3 * 4)
        "(2 + 3) * 4",       // Should parse as (2 + 3) * 4
        "10 - 5 + 2",        // Should parse as (10 - 5) + 2
        "8 / 2 * 2",         // Should parse as (8 / 2) * 2
        "(1 + 2) * (3 + 4)", // Complex nested precedence
        "10 / (2 + 3)",      // Division with parenthesized denominator
    ];

    for test_case in test_cases {
        let result = parser::parse_jue(test_case);
        assert!(
            result.is_ok(),
            "Should parse operator precedence correctly: {}",
            test_case
        );

        let ast = result.unwrap();
        assert!(
            !ast.expressions.is_empty(),
            "Should have parsed expressions for: {}",
            test_case
        );
    }
}

#[test]
fn test_variable_assignment_patterns() {
    let test_cases = vec![
        "x = 1",                          // Simple assignment
        "x, y = 1, 2",                    // Multiple assignment
        "x = y = 10",                     // Chained assignment
        "result = x + y",                 // Expression assignment
        "message = \"Value: \" + result", // String concatenation assignment
        "total = (a * b) + (x - y)",      // Complex expression assignment
        "x = x + 1",                      // Self-referential assignment
    ];

    for test_case in test_cases {
        let result = parser::parse_jue(test_case);
        assert!(
            result.is_ok(),
            "Should parse variable assignment: {}",
            test_case
        );

        let ast = result.unwrap();
        let assignments: Vec<_> = ast
            .statements
            .iter()
            .filter(|stmt| matches!(stmt, ast::Statement::Assign { .. }))
            .collect();

        assert!(
            !assignments.is_empty(),
            "Should have assignment statements for: {}",
            test_case
        );
    }
}

#[test]
fn test_control_flow_structures() {
    let test_cases = vec![
        // Simple if
        "if x > 0 { result = \"positive\" }",
        // If-else
        "if temperature > 30 { status = \"hot\" } else { status = \"moderate\" }",
        // If-elif-else chain
        r#"if score >= 90 { grade = "A" } elif score >= 80 { grade = "B" } else { grade = "C" }"#,
        // Nested if
        r#"if x > 0 { if y > 0 { quadrant = "I" } else { quadrant = "IV" } }"#,
    ];

    for test_case in test_cases {
        let result = parser::parse_jue(test_case);
        assert!(result.is_ok(), "Should parse control flow: {}", test_case);

        let ast = result.unwrap();
        let if_statements: Vec<_> = ast
            .statements
            .iter()
            .filter(|stmt| matches!(stmt, ast::Statement::If { .. }))
            .collect();

        assert!(
            !if_statements.is_empty(),
            "Should have if statements for: {}",
            test_case
        );
    }
}

#[test]
fn test_function_definition_patterns() {
    let test_cases = vec![
        // Function without parameters
        "function greet() { return \"Hello\" }",
        // Function with parameters
        "function add(a, b) { return a + b }",
        // Function with multiple parameters
        "function calculate(x, y, z) { return (x + y) * z }",
        // Function with complex body
        r#"function complex() { temp = 1; result = temp + 2; return result }"#,
    ];

    for test_case in test_cases {
        let result = parser::parse_jue(test_case);
        assert!(
            result.is_ok(),
            "Should parse function definition: {}",
            test_case
        );

        let ast = result.unwrap();
        let function_defs: Vec<_> = ast
            .statements
            .iter()
            .filter(|stmt| matches!(stmt, ast::Statement::FunctionDef { .. }))
            .collect();

        assert!(
            !function_defs.is_empty(),
            "Should have function definitions for: {}",
            test_case
        );
    }
}

#[test]
fn test_function_call_patterns() {
    let test_cases = vec![
        "result = greet()",                            // Simple call
        "result = add(5, 3)",                          // Call with arguments
        "final = add(result1, result2)",               // Call with variables
        "nested = add(calculate(1, 2, 3), add(4, 5))", // Nested calls
    ];

    for test_case in test_cases {
        let result = parser::parse_jue(test_case);
        assert!(result.is_ok(), "Should parse function call: {}", test_case);

        let ast = result.unwrap();
        // Basic validation that we got a non-empty AST
        assert!(
            !ast.expressions.is_empty() || !ast.statements.is_empty(),
            "AST should contain expressions or statements for: {}",
            test_case
        );
    }
}

#[test]
fn test_syntax_error_scenarios() {
    // Test various syntax error scenarios to ensure proper error handling
    let error_cases = vec![
        ("function missing_paren {", "Missing closing parenthesis"),
        ("if x > 0 {", "Missing closing brace"),
        ("x = ", "Incomplete assignment"),
        ("1 + * 2", "Invalid operator sequence"),
        ("function test(a, b,", "Unclosed parameter list"),
        ("if condition { result = \"ok\"", "Unclosed if block"),
        ("x = (1 + 2", "Unclosed parentheses"),
        (
            "function test() { return 1 + * 2 }",
            "Invalid operator in expression",
        ),
    ];

    for (invalid_code, description) in error_cases {
        let result = parser::parse_jue(invalid_code);
        assert!(
            result.is_err(),
            "Should fail to parse {}: {}",
            description,
            invalid_code
        );
    }
}

#[test]
fn test_mixed_syntax_validation() {
    // Test complex mixed syntax scenarios
    let complex_code = r#"
        // Variable declarations
        x = 1
        y = 2
        name = "test"

        // Arithmetic expressions
        result = x + y * 3
        total = (x + y) / 2

        // Control flow
        if result > 10 {
            status = "high"
        } else {
            status = "low"
        }

        // Function definition and call
        function calculate(a, b) {
            return a * b + result
        }

        final = calculate(5, 3)
    "#;

    let result = parser::parse_jue(complex_code);
    assert!(result.is_ok(), "Should parse complex mixed syntax");

    let ast = result.unwrap();

    // Validate we have all expected constructs
    let assignments: Vec<_> = ast
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, ast::Statement::Assign { .. }))
        .collect();

    let if_statements: Vec<_> = ast
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, ast::Statement::If { .. }))
        .collect();

    let function_defs: Vec<_> = ast
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, ast::Statement::FunctionDef { .. }))
        .collect();

    assert!(assignments.len() > 0, "Should have assignment statements");
    assert!(if_statements.len() > 0, "Should have if statements");
    assert!(function_defs.len() > 0, "Should have function definitions");
    assert!(!ast.expressions.is_empty(), "Should have expressions");
}
