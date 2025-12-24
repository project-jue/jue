/// Test Symbol compilation to Physics-World
use jue_world::ast::{AstNode, Literal};
use jue_world::physics_compiler::{compile_to_physics_world, PhysicsWorldCompiler};
use jue_world::trust_tier::TrustTier;
use physics_world::types::OpCode;

#[test]
fn test_symbol_add_compilation() {
    // Create a Symbol node for "add"
    let symbol_node = AstNode::Symbol("add".to_string());

    // Compile to Physics-World bytecode
    let result = compile_to_physics_world(&symbol_node, TrustTier::Empirical);

    // Should succeed and generate Add opcode
    assert!(result.is_ok());
    let (bytecode, _) = result.unwrap();

    // Should contain exactly one Add opcode
    assert_eq!(bytecode.len(), 1);
    assert_eq!(bytecode[0], OpCode::Add);
}

#[test]
fn test_symbol_arithmetic_operations() {
    // Test various arithmetic symbols
    let test_cases = vec![
        ("add", OpCode::Add),
        ("sub", OpCode::Sub),
        ("mul", OpCode::Mul),
        ("div", OpCode::Div),
        ("mod", OpCode::Mod),
        ("fadd", OpCode::FAdd),
        ("fsub", OpCode::FSub),
        ("fmul", OpCode::FMul),
        ("fdiv", OpCode::FDiv),
    ];

    for (symbol_name, expected_opcode) in test_cases {
        let symbol_node = AstNode::Symbol(symbol_name.to_string());
        let result = compile_to_physics_world(&symbol_node, TrustTier::Empirical);

        assert!(result.is_ok(), "Failed to compile symbol '{}'", symbol_name);
        let (bytecode, _) = result.unwrap();

        assert_eq!(
            bytecode.len(),
            1,
            "Symbol '{}' should generate exactly one opcode",
            symbol_name
        );
        assert_eq!(
            bytecode[0], expected_opcode,
            "Symbol '{}' should compile to {:?}",
            symbol_name, expected_opcode
        );
    }
}

#[test]
fn test_symbol_comparison_operations() {
    // Test comparison symbols
    let test_cases = vec![("eq", OpCode::Eq), ("lt", OpCode::Lt), ("gt", OpCode::Gt)];

    for (symbol_name, expected_opcode) in test_cases {
        let symbol_node = AstNode::Symbol(symbol_name.to_string());
        let result = compile_to_physics_world(&symbol_node, TrustTier::Empirical);

        assert!(result.is_ok(), "Failed to compile symbol '{}'", symbol_name);
        let (bytecode, _) = result.unwrap();

        assert_eq!(
            bytecode.len(),
            1,
            "Symbol '{}' should generate exactly one opcode",
            symbol_name
        );
        assert_eq!(
            bytecode[0], expected_opcode,
            "Symbol '{}' should compile to {:?}",
            symbol_name, expected_opcode
        );
    }
}

#[test]
fn test_symbol_stack_operations() {
    // Test stack manipulation symbols
    let test_cases = vec![
        ("swap", OpCode::Swap),
        ("dup", OpCode::Dup),
        ("pop", OpCode::Pop),
    ];

    for (symbol_name, expected_opcode) in test_cases {
        let symbol_node = AstNode::Symbol(symbol_name.to_string());
        let result = compile_to_physics_world(&symbol_node, TrustTier::Empirical);

        assert!(result.is_ok(), "Failed to compile symbol '{}'", symbol_name);
        let (bytecode, _) = result.unwrap();

        assert_eq!(
            bytecode.len(),
            1,
            "Symbol '{}' should generate exactly one opcode",
            symbol_name
        );
        assert_eq!(
            bytecode[0], expected_opcode,
            "Symbol '{}' should compile to {:?}",
            symbol_name, expected_opcode
        );
    }
}

#[test]
fn test_symbol_string_operations() {
    // Test string operation symbols
    let test_cases = vec![
        ("str-concat", OpCode::StrConcat),
        ("str-len", OpCode::StrLen),
        ("str-index", OpCode::StrIndex),
    ];

    for (symbol_name, expected_opcode) in test_cases {
        let symbol_node = AstNode::Symbol(symbol_name.to_string());
        let result = compile_to_physics_world(&symbol_node, TrustTier::Empirical);

        assert!(result.is_ok(), "Failed to compile symbol '{}'", symbol_name);
        let (bytecode, _) = result.unwrap();

        assert_eq!(
            bytecode.len(),
            1,
            "Symbol '{}' should generate exactly one opcode",
            symbol_name
        );
        assert_eq!(
            bytecode[0], expected_opcode,
            "Symbol '{}' should compile to {:?}",
            symbol_name, expected_opcode
        );
    }
}

#[test]
fn test_unknown_symbol_error() {
    // Test that unknown symbols produce proper error
    let symbol_node = AstNode::Symbol("unknown_symbol".to_string());
    let result = compile_to_physics_world(&symbol_node, TrustTier::Empirical);

    // Should fail with an internal error
    assert!(result.is_err());
    let error = result.unwrap_err();

    // Check that the error message mentions the unknown symbol
    assert!(error.to_string().contains("unknown_symbol"));
    assert!(error.to_string().contains("Unknown symbol"));
}

#[test]
fn test_symbol_within_expression() {
    // Test Symbol node within a larger expression
    // Create: (add 1 2) which would be represented as a Call with Symbol "add"
    let add_symbol = AstNode::Symbol("add".to_string());
    let literal_1 = AstNode::Literal(Literal::Int(1));
    let literal_2 = AstNode::Literal(Literal::Int(2));

    let call_node = AstNode::Call {
        function: Box::new(add_symbol),
        arguments: vec![literal_1, literal_2],
        location: Default::default(),
    };

    let result = compile_to_physics_world(&call_node, TrustTier::Empirical);

    // Should succeed and contain Add opcode
    assert!(result.is_ok());
    let (bytecode, _) = result.unwrap();

    // Should contain the Add opcode among other instructions for the call
    assert!(bytecode.contains(&OpCode::Add));
    assert_eq!(bytecode.iter().filter(|op| **op == OpCode::Add).count(), 1);
}

#[test]
fn test_physics_compiler_symbol_case() {
    // Test that the compile_to_physics method handles Symbol nodes
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Empirical);

    let symbol_node = AstNode::Symbol("mul".to_string());

    let result = compiler.compile_to_physics(&symbol_node);
    assert!(result.is_ok());
    let bytecode = result.unwrap();

    assert_eq!(bytecode.len(), 1);
    assert_eq!(bytecode[0], OpCode::Mul);
}
