use juec::frontend::parser;
use juec::middle::mir;
use juec::middle::mir_lower;
/// Basic validation test for parsing and MIR AST generation
/// This demonstrates how the shared samples would be used in actual tests
use std::path::PathBuf;

#[test]
fn test_arithmetic_parsing() {
    let test_file =
        PathBuf::from("tests/shared_samples/phase_1_parsing/01_arithmetic_expressions.jue");
    let source = std::fs::read_to_string(test_file).expect("Failed to read test file");

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
        .filter(|expr| matches!(expr, juec::frontend::ast::Expression::BinaryOp { .. }))
        .collect();

    assert!(binary_ops.len() > 0, "Should have binary operations");
}

#[test]
fn test_variable_declarations_parsing() {
    let test_file =
        PathBuf::from("tests/shared_samples/phase_1_parsing/02_variable_declarations.jue");
    let source = std::fs::read_to_string(test_file).expect("Failed to read test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse variable declarations");

    // Basic validation - should have assignment statements
    assert!(!ast.statements.is_empty(), "Should have parsed statements");

    // Verify we can find assignments
    let assignments: Vec<_> = ast
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, juec::frontend::ast::Statement::Assign { .. }))
        .collect();

    assert!(assignments.len() > 0, "Should have assignment statements");
}

#[test]
fn test_control_flow_parsing() {
    let test_file = PathBuf::from("tests/shared_samples/phase_1_parsing/03_control_flow.jue");
    let source = std::fs::read_to_string(test_file).expect("Failed to read test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse control flow");

    // Basic validation - should have if statements
    assert!(!ast.statements.is_empty(), "Should have parsed statements");

    // Verify we can find if statements
    let if_statements: Vec<_> = ast
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, juec::frontend::ast::Statement::If { .. }))
        .collect();

    assert!(if_statements.len() > 0, "Should have if statements");
}

#[test]
fn test_function_definitions_parsing() {
    let test_file =
        PathBuf::from("tests/shared_samples/phase_1_parsing/04_function_definitions.jue");
    let source = std::fs::read_to_string(test_file).expect("Failed to read test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse function definitions");

    // Basic validation - should have function definitions
    assert!(!ast.statements.is_empty(), "Should have parsed statements");

    // Verify we can find function definitions
    let function_defs: Vec<_> = ast
        .statements
        .iter()
        .filter(|stmt| matches!(stmt, juec::frontend::ast::Statement::FunctionDef { .. }))
        .collect();

    assert!(function_defs.len() > 0, "Should have function definitions");
}

/// Example of MIR AST generation validation
#[test]
fn test_arithmetic_mir_generation() {
    let test_file =
        PathBuf::from("tests/shared_samples/phase_1_parsing/01_arithmetic_expressions.jue");
    let source = std::fs::read_to_string(test_file).expect("Failed to read test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse arithmetic expressions");

    // Lower to MIR
    let mut mir = mir::Mir::new();
    let module_id =
        mir_lower::lower_frontend_to_mir(&ast, &mut mir).expect("Failed to lower to MIR");

    // Basic validation - MIR should have nodes
    assert!(mir.nodes.len() > 0, "MIR should have nodes");

    // Verify we have binary operations in MIR
    let binary_ops: Vec<_> = mir
        .nodes
        .iter()
        .filter(|node| matches!(node.kind, mir::NodeKind::BinaryOp { .. }))
        .collect();

    assert!(binary_ops.len() > 0, "MIR should have binary operations");
}

/// Example of MIR AST validation against expected structure
#[test]
fn test_mir_structure_validation() {
    // This would be a more comprehensive test that validates the MIR structure
    // against the expected JSON files in phase_2_mir_ast/

    // For now, we just demonstrate the concept
    let mir_file =
        PathBuf::from("tests/shared_samples/phase_2_mir_ast/10_arithmetic_mir_expected.json");
    let mir_content =
        std::fs::read_to_string(mir_file).expect("Failed to read MIR expectation file");

    // In a real implementation, this would parse the JSON and compare
    // the actual MIR structure against the expected structure
    assert!(
        !mir_content.is_empty(),
        "MIR expectation file should not be empty"
    );
}
