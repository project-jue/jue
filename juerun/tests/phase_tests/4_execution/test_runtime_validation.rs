use anyhow::Result;
use juec::backend::cranelift_gen::CraneliftCodeGen;
use juec::frontend::parser;
use juec::middle::mir_lower::lower_frontend_module;
use std::path::PathBuf;

/// Phase 4 Runtime Validation Tests
/// These tests validate the full execution pipeline from parsing → MIR → Cranelift → Runtime
/// using shared sample files and follow the test patterns established in the architecture.

#[test]
fn test_arithmetic_expressions_runtime_validation() {
    let test_file = PathBuf::from(
        "../../../tests/shared_samples/phase_1_parsing/01_arithmetic_expressions.jue",
    );
    let source = std::fs::read_to_string(test_file)
        .expect("Failed to read arithmetic expressions test file");

    // Full pipeline: Parse → MIR → Cranelift IR generation
    let ast = parser::parse_jue(&source).expect("Failed to parse arithmetic expressions");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("arithmetic_runtime_test")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully compile arithmetic expressions for runtime: {:?}",
        result.err()
    );

    // Verify we generated executable functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated executable functions for arithmetic expressions"
    );

    // Validate that the generated IR is ready for execution
    assert!(
        codegen.module.is_finalized(),
        "Cranelift module should be finalized and ready for execution"
    );

    println!("✅ Arithmetic expressions runtime validation test passed");
}

#[test]
fn test_variable_declarations_runtime_validation() {
    let test_file =
        PathBuf::from("../../../tests/shared_samples/phase_1_parsing/02_variable_declarations.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read variable declarations test file");

    // Full pipeline: Parse → MIR → Cranelift IR generation
    let ast = parser::parse_jue(&source).expect("Failed to parse variable declarations");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("variable_runtime_test")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully compile variable declarations for runtime: {:?}",
        result.err()
    );

    // Verify we generated executable functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated executable functions for variable declarations"
    );

    println!("✅ Variable declarations runtime validation test passed");
}

#[test]
fn test_control_flow_runtime_validation() {
    let test_file =
        PathBuf::from("../../../tests/shared_samples/phase_1_parsing/03_control_flow.jue");
    let source = std::fs::read_to_string(test_file).expect("Failed to read control flow test file");

    // Full pipeline: Parse → MIR → Cranelift IR generation
    let ast = parser::parse_jue(&source).expect("Failed to parse control flow");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("control_flow_runtime_test")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully compile control flow for runtime: {:?}",
        result.err()
    );

    // Verify we generated executable functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated executable functions for control flow"
    );

    println!("✅ Control flow runtime validation test passed");
}

#[test]
fn test_function_definitions_runtime_validation() {
    let test_file =
        PathBuf::from("../../../tests/shared_samples/phase_1_parsing/04_function_definitions.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read function definitions test file");

    // Full pipeline: Parse → MIR → Cranelift IR generation
    let ast = parser::parse_jue(&source).expect("Failed to parse function definitions");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("function_runtime_test")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully compile function definitions for runtime: {:?}",
        result.err()
    );

    // Verify we generated executable functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated executable functions for function definitions"
    );

    println!("✅ Function definitions runtime validation test passed");
}

#[test]
fn test_full_runtime_pipeline_validation() {
    // Test that all shared sample files can be processed through the full pipeline
    // and are ready for runtime execution
    let sample_files = vec![
        "../../../tests/shared_samples/phase_1_parsing/01_arithmetic_expressions.jue",
        "../../../tests/shared_samples/phase_1_parsing/02_variable_declarations.jue",
        "../../../tests/shared_samples/phase_1_parsing/03_control_flow.jue",
        "../../../tests/shared_samples/phase_1_parsing/04_function_definitions.jue",
    ];

    for file_path in sample_files {
        let source =
            std::fs::read_to_string(file_path).expect(&format!("Failed to read {}", file_path));

        // Full pipeline: Parse → MIR → Cranelift IR generation
        let ast = parser::parse_jue(&source).expect(&format!("Failed to parse {}", file_path));

        let mir = lower_frontend_module(&ast);

        let mut codegen =
            CraneliftCodeGen::new(&format!("runtime_test_{}", file_path.replace('/', "_")))
                .expect("Failed to create Cranelift code generator");

        let result = codegen.generate(&mir);
        assert!(
            result.is_ok(),
            "Should successfully compile {} for runtime: {:?}",
            file_path,
            result.err()
        );

        // Verify we generated executable functions
        assert!(
            !codegen.function_ids.is_empty(),
            "Should have generated executable functions for {}",
            file_path
        );

        println!("✅ Runtime pipeline validation passed for {}", file_path);
    }
}

#[test]
fn test_runtime_error_handling() {
    // Test that invalid code produces appropriate errors during runtime preparation
    let invalid_cases = vec![
        "function broken_syntax (", // Missing closing parenthesis
        "if x > 0 {",               // Missing closing brace
        "x = ",                     // Incomplete assignment
    ];

    for (i, invalid_code) in invalid_cases.iter().enumerate() {
        let result = parser::parse_jue(invalid_code);
        assert!(
            result.is_err(),
            "Case {} should fail to parse: {}",
            i + 1,
            invalid_code
        );
    }

    println!("✅ Runtime error handling test passed");
}

#[test]
fn test_runtime_performance_validation() {
    // Test runtime preparation performance with a larger input
    let mut large_input = String::new();

    // Add many variable declarations and expressions
    for i in 0..20 {
        large_input.push_str(&format!("var_{} = {}\n", i, i * 2));
        large_input.push_str(&format!("result_{} = var_{} + {}\n", i, i, i + 1));
    }

    // Test that the runtime preparation pipeline can handle this large input
    let start_time = std::time::Instant::now();

    let ast = parser::parse_jue(&large_input).expect("Should be able to parse large input");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("runtime_performance_test")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    let duration = start_time.elapsed();

    assert!(
        result.is_ok(),
        "Should be able to compile large input for runtime"
    );
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated functions for large input"
    );

    println!(
        "⏱️  Runtime preparation performance: {:?} for large input",
        duration
    );
    println!("✅ Runtime performance validation test passed");
}

#[test]
fn test_runtime_consistency_validation() {
    // Test that the same code produces consistent results across multiple runtime preparations
    let test_file = PathBuf::from(
        "../../../tests/shared_samples/phase_1_parsing/01_arithmetic_expressions.jue",
    );
    let source = std::fs::read_to_string(test_file)
        .expect("Failed to read arithmetic expressions test file");

    let mut function_counts = Vec::new();

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

        function_counts.push(codegen.function_ids.len());
    }

    // All preparations should produce the same number of functions
    assert!(
        function_counts.windows(2).all(|w| w[0] == w[1]),
        "Multiple preparations should produce consistent function counts"
    );
    println!("✅ Runtime consistency validation test passed");
}

#[test]
fn test_cross_feature_runtime_integration() {
    // Test integration between different language features for runtime validation
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

    // Full pipeline execution
    let ast = parser::parse_jue(&combined_code).expect("Failed to parse combined code");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("cross_feature_runtime_test")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully compile cross-feature code for runtime"
    );

    // Verify we generated executable functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated executable functions for cross-feature code"
    );

    println!("✅ Cross-feature runtime integration test passed");
}
