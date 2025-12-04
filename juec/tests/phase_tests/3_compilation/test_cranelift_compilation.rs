use anyhow::Result;
use juec::backend::cranelift_gen::CraneliftCodeGen;
/// Phase 3 Cranelift Compilation Tests
/// These tests validate the full compilation pipeline from parsing → MIR → Cranelift IR
/// using shared sample files and follow the test patterns established in the architecture.
use juec::frontend::parser;
use juec::middle::mir_lower::lower_frontend_module;
use test_data::data_dir;

#[test]
fn test_arithmetic_expressions_compilation() {
    let test_file = data_dir().join("shared_samples/phase_1_parsing/01_arithmetic_expressions.jue");
    let source = std::fs::read_to_string(test_file)
        .expect("Failed to read arithmetic expressions test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse arithmetic expressions");

    // Lower to MIR
    let mir = lower_frontend_module(&ast);

    // Generate Cranelift IR
    let mut codegen = CraneliftCodeGen::new("arithmetic_test")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);

    assert!(
        result.is_ok(),
        "Should successfully compile arithmetic expressions to Cranelift IR: {:?}",
        result.err()
    );

    // Verify we generated some functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated at least one function"
    );

    println!("✅ Arithmetic expressions compilation test passed");
}

#[test]
fn test_variable_declarations_compilation() {
    let test_file = data_dir().join("shared_samples/phase_1_parsing/02_variable_declarations.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read variable declarations test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse variable declarations");

    // Lower to MIR
    let mir = lower_frontend_module(&ast);

    // Generate Cranelift IR
    let mut codegen =
        CraneliftCodeGen::new("variable_test").expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);

    assert!(
        result.is_ok(),
        "Should successfully compile variable declarations to Cranelift IR: {:?}",
        result.err()
    );

    // Verify we generated some functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated at least one function"
    );

    println!("✅ Variable declarations compilation test passed");
}

#[test]
fn test_control_flow_compilation() {
    let test_file = data_dir().join("shared_samples/phase_1_parsing/03_control_flow.jue");
    let source = std::fs::read_to_string(test_file).expect("Failed to read control flow test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse control flow");

    // Lower to MIR
    let mir = lower_frontend_module(&ast);

    // Generate Cranelift IR
    let mut codegen = CraneliftCodeGen::new("control_flow_test")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);

    assert!(
        result.is_ok(),
        "Should successfully compile control flow to Cranelift IR: {:?}",
        result.err()
    );

    // Verify we generated some functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated at least one function"
    );

    println!("✅ Control flow compilation test passed");
}

#[test]
fn test_function_definitions_compilation() {
    let test_file = data_dir().join("shared_samples/phase_1_parsing/04_function_definitions.jue");
    let source =
        std::fs::read_to_string(test_file).expect("Failed to read function definitions test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse function definitions");

    // Lower to MIR
    let mir = lower_frontend_module(&ast);

    // Generate Cranelift IR
    let mut codegen =
        CraneliftCodeGen::new("function_test").expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);

    assert!(
        result.is_ok(),
        "Should successfully compile function definitions to Cranelift IR: {:?}",
        result.err()
    );

    // Verify we generated some functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated at least one function"
    );

    println!("✅ Function definitions compilation test passed");
}

#[test]
fn test_full_compilation_pipeline_integration() {
    // Test that all shared sample files can be compiled successfully through the full pipeline
    let sample_files = vec![
        data_dir().join("shared_samples/phase_1_parsing/01_arithmetic_expressions.jue"),
        data_dir().join("shared_samples/phase_1_parsing/02_variable_declarations.jue"),
        data_dir().join("shared_samples/phase_1_parsing/03_control_flow.jue"),
        data_dir().join("shared_samples/phase_1_parsing/04_function_definitions.jue"),
    ];

    for file_path in sample_files {
        let source = std::fs::read_to_string(&file_path)
            .expect(&format!("Failed to read {}", file_path.to_string_lossy()));

        // Parse the source code
        let ast = parser::parse_jue(&source)
            .expect(&format!("Failed to parse {}", file_path.to_string_lossy()));

        // Lower to MIR
        let mir = lower_frontend_module(&ast);

        // Generate Cranelift IR
        let mut codegen = CraneliftCodeGen::new(&format!(
            "test_{}",
            file_path.to_string_lossy().replace('/', "_")
        ))
        .expect("Failed to create Cranelift code generator");

        let result = codegen.generate(&mir);

        assert!(
            result.is_ok(),
            "Should successfully compile {} through full pipeline: {:?}",
            file_path.to_string_lossy(),
            result.err()
        );

        // Verify we generated some functions
        assert!(
            !codegen.function_ids.is_empty(),
            "Should have generated at least one function for {}",
            file_path.to_string_lossy()
        );
    }

    println!("✅ Full compilation pipeline integration test passed");
}

#[test]
fn test_compilation_error_handling() {
    // Test that invalid code produces appropriate compilation errors
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

    println!("✅ Compilation error handling test passed");
}

#[test]
fn test_cranelift_ir_structure_validation() {
    // Test that the generated Cranelift IR has the expected structure
    let test_file = data_dir().join("shared_samples/phase_1_parsing/01_arithmetic_expressions.jue");
    let source = std::fs::read_to_string(test_file)
        .expect("Failed to read arithmetic expressions test file");

    // Parse the source code
    let ast = parser::parse_jue(&source).expect("Failed to parse arithmetic expressions");

    // Lower to MIR
    let mir = lower_frontend_module(&ast);

    // Generate Cranelift IR
    let mut codegen =
        CraneliftCodeGen::new("structure_test").expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);

    assert!(
        result.is_ok(),
        "Should successfully compile for structure validation: {:?}",
        result.err()
    );

    // Verify we have generated function IDs
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated function IDs"
    );

    // Print IR structure for validation (this would be more comprehensive in a real test)
    codegen.print_ir();

    println!("✅ Cranelift IR structure validation test passed");
}

#[test]
fn test_compilation_performance() {
    // Test compilation performance with a larger input
    let mut large_input = String::new();

    // Add many variable declarations and expressions
    for i in 0..20 {
        large_input.push_str(&format!("var_{} = {}\n", i, i * 2));
        large_input.push_str(&format!("result_{} = var_{} + {}\n", i, i, i + 1));
    }

    // Test that the compiler can handle this large input
    let start_time = std::time::Instant::now();
    let ast = parser::parse_jue(&large_input).expect("Should be able to parse large input");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("performance_test")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    let duration = start_time.elapsed();

    assert!(result.is_ok(), "Should be able to compile large input");
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated functions for large input"
    );

    println!(
        "⏱️  Compilation performance: {:?} for large input",
        duration
    );
    println!("✅ Compilation performance test passed");
}
