use juec::backend::cranelift_gen::CraneliftCodeGen;
use juec::frontend::parser;
use juec::middle::mir_lower::lower_frontend_module;
use test_data::data_dir;

/// Phase 4 Execution Integration Tests
/// These tests validate the integration between compilation and runtime execution preparation
/// using the full pipeline from parsing through to runtime-ready code generation.

#[test]
fn test_arithmetic_integration() {
    let test_file = data_dir().join("shared_samples/phase_1_parsing/01_arithmetic_expressions.jue");
    let source = std::fs::read_to_string(test_file)
        .expect("Failed to read arithmetic expressions test file");

    // Full pipeline: Parse → MIR → Cranelift IR generation for runtime
    let ast = parser::parse_jue(&source).expect("Failed to parse arithmetic expressions");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("arithmetic_integration")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully compile arithmetic expressions for runtime integration"
    );

    // Verify we have runtime-ready functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated runtime-ready functions for arithmetic expressions"
    );

    println!("✅ Arithmetic integration test passed");
}

#[test]
fn test_variable_integration() {
    let test_file = data_dir().join("shared_samples/phase_1_parsing/02_variable_declarations.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read variable declarations test file");

    // Full pipeline: Parse → MIR → Cranelift IR generation for runtime
    let ast = parser::parse_jue(&source).expect("Failed to parse variable declarations");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("variable_integration")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully compile variable declarations for runtime integration"
    );

    // Verify we have runtime-ready functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated runtime-ready functions for variable declarations"
    );

    println!("✅ Variable integration test passed");
}

#[test]
fn test_control_flow_integration() {
    let test_file = data_dir().join("shared_samples/phase_1_parsing/03_control_flow.jue");
    let source = std::fs::read_to_string(test_file).expect("Failed to read control flow test file");

    // Full pipeline: Parse → MIR → Cranelift IR generation for runtime
    let ast = parser::parse_jue(&source).expect("Failed to parse control flow");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("control_flow_integration")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully compile control flow for runtime integration"
    );

    // Verify we have runtime-ready functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated runtime-ready functions for control flow"
    );

    println!("✅ Control flow integration test passed");
}

#[test]
fn test_function_integration() {
    let test_file = data_dir().join("shared_samples/phase_1_parsing/04_function_definitions.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read function definitions test file");

    // Full pipeline: Parse → MIR → Cranelift IR generation for runtime
    let ast = parser::parse_jue(&source).expect("Failed to parse function definitions");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("function_integration")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully compile function definitions for runtime integration"
    );

    // Verify we have runtime-ready functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated runtime-ready functions for function definitions"
    );

    println!("✅ Function integration test passed");
}

#[test]
fn test_pipeline_consistency() {
    // Test that the same code produces consistent results across multiple runtime preparations
    let test_file = data_dir().join("shared_samples/phase_1_parsing/01_arithmetic_expressions.jue");
    let source = std::fs::read_to_string(test_file)
        .expect("Failed to read arithmetic expressions test file");

    let mut results = Vec::new();

    // Run the same code multiple times
    for i in 0..3 {
        let ast = parser::parse_jue(&source).expect("Failed to parse arithmetic expressions");
        let mir = lower_frontend_module(&ast);

        let mut codegen = CraneliftCodeGen::new(&format!("consistency_test_{}", i))
            .expect("Failed to create Cranelift code generator");

        let result = codegen.generate(&mir);
        assert!(
            result.is_ok(),
            "Should successfully compile in iteration {}",
            i
        );

        results.push(codegen.function_ids.len());
    }

    // All executions should produce the same result
    assert!(
        results.windows(2).all(|w| w[0] == w[1]),
        "Multiple preparations should produce consistent results"
    );
    println!("✅ Pipeline consistency test passed");
}

#[test]
fn test_cross_component_integration() {
    // Test integration between different language features for runtime preparation
    let mut combined_code = String::new();

    // Combine arithmetic, variables, control flow, and functions
    combined_code.push_str("x = 10\n");
    combined_code.push_str("y = 20\n");
    combined_code.push_str("function add(a, b) {\n");
    combined_code.push_str("    return a + b\n");
    combined_code.push_str("}\n");
    combined_code.push_str("if x > 5 {\n");
    combined_code.push_str("    result = add(x, y)\n");
    combined_code.push_str("} else {\n");
    combined_code.push_str("    result = 0\n");
    combined_code.push_str("}\n");

    // Full pipeline execution for runtime preparation
    let ast = parser::parse_jue(&combined_code).expect("Failed to parse combined code");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("cross_component_integration")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully compile cross-component code for runtime"
    );

    // Verify we have runtime-ready functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated runtime-ready functions for cross-component code"
    );

    println!("✅ Cross-component integration test passed");
}

#[test]
fn test_runtime_readiness_validation() {
    // Test that generated code is actually ready for runtime execution
    let test_file = data_dir().join("shared_samples/phase_1_parsing/01_arithmetic_expressions.jue");
    let source = std::fs::read_to_string(test_file)
        .expect("Failed to read arithmetic expressions test file");

    let ast = parser::parse_jue(&source).expect("Failed to parse arithmetic expressions");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("runtime_readiness_test")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully compile for runtime readiness"
    );

    // Verify the module is in a state ready for execution
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated functions ready for execution"
    );

    // Check that we have valid function IDs (indicating successful compilation)
    for func_id in &codegen.function_ids {
        assert!(
            !func_id.is_null(),
            "Function ID should be valid for execution"
        );
    }

    println!("✅ Runtime readiness validation test passed");
}
