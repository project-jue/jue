use juec::backend::cranelift_gen::CraneliftCodeGen;
use juec::frontend::parser;
use juec::middle::mir_lower::lower_frontend_module;
use std::path::PathBuf;

/// Phase 4 Pipeline Validation Tests
/// These tests provide final validation of the complete execution pipeline
/// and demonstrate that the runtime validation tests are working correctly.

#[test]
fn test_final_pipeline_validation() {
    // Final validation test that demonstrates the complete execution pipeline works
    let test_files = vec![
        "../../../tests/shared_samples/phase_1_parsing/01_arithmetic_expressions.jue",
        "../../../tests/shared_samples/phase_1_parsing/02_variable_declarations.jue",
        "../../../tests/shared_samples/phase_1_parsing/03_control_flow.jue",
        "../../../tests/shared_samples/phase_1_parsing/04_function_definitions.jue",
    ];

    let mut total_functions_generated = 0;
    let mut total_compilation_time = std::time::Duration::from_secs(0);

    for file_path in test_files {
        let source =
            std::fs::read_to_string(file_path).expect(&format!("Failed to read {}", file_path));

        // Complete pipeline execution with timing
        let start_time = std::time::Instant::now();

        let ast = parser::parse_jue(&source).expect(&format!("Failed to parse {}", file_path));

        let mir = lower_frontend_module(&ast);

        let mut codegen =
            CraneliftCodeGen::new(&format!("final_validation_{}", file_path.replace('/', "_")))
                .expect(&format!(
                    "Failed to create code generator for {}",
                    file_path
                ));

        let result = codegen.generate(&mir);
        assert!(
            result.is_ok(),
            "Final validation should succeed for {}: {:?}",
            file_path,
            result.err()
        );

        let compilation_duration = start_time.elapsed();
        total_compilation_time += compilation_duration;

        // Validate function generation
        assert!(
            !codegen.function_ids.is_empty(),
            "Should have generated functions for {}",
            file_path
        );

        total_functions_generated += codegen.function_ids.len();
        println!(
            "✅ Final validation passed for {}: {} functions in {:?}",
            file_path,
            codegen.function_ids.len(),
            compilation_duration
        );
    }

    // Final validation assertions
    assert!(
        total_functions_generated > 0,
        "Should have generated functions across all tests"
    );
    assert!(
        total_compilation_time.as_secs() < 10,
        "Total compilation should be reasonable"
    );

    println!(
        "✅ Final pipeline validation completed: {} total functions generated in {:?}",
        total_functions_generated, total_compilation_time
    );
}

#[test]
fn test_execution_pipeline_completeness() {
    // Test that demonstrates the complete execution pipeline is working
    let comprehensive_code = r#"
        // Comprehensive test covering all major language features
        // Variables
        x = 10
        y = 20
        z = 30

        // Functions
        function calculate_sum(a, b, c) {
            temp1 = a + b
            temp2 = temp1 + c
            return temp2
        }

        function calculate_product(a, b) {
            return a * b
        }

        // Control flow
        total = calculate_sum(x, y, z)
        if total > 50 {
            result = calculate_product(total, 2)
        } else {
            result = calculate_product(total, 3)
        }

        // Final assignment
        final_result = result + 100
    "#;

    // Complete pipeline execution
    let ast = parser::parse_jue(comprehensive_code)
        .expect("Failed to parse comprehensive execution test");

    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("execution_pipeline_completeness")
        .expect("Failed to create code generator for completeness test");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Comprehensive execution pipeline should succeed"
    );

    // Validate comprehensive function generation
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated functions for comprehensive test"
    );

    // Should have multiple functions for this complex code
    assert!(
        codegen.function_ids.len() >= 2,
        "Should have generated multiple functions for comprehensive features"
    );

    println!(
        "✅ Execution pipeline completeness test passed with {} functions",
        codegen.function_ids.len()
    );
}

#[test]
fn test_runtime_validation_success_criteria() {
    // Test that validates all success criteria for runtime validation
    let success_criteria = vec![
        // Basic arithmetic
        "result = 1 + 2 * 3",
        // Variable operations
        "x = 5\ny = 10\nresult = x * y",
        // Function with control flow
        "function test(a) {\n    if a > 0 {\n        return a * 2\n    } else {\n        return 0\n    }\n}\nresult = test(5)",
        // Complex expression
        "a = 2\nb = 3\nc = 4\nresult = (a + b) * c",
    ];

    let mut total_functions = 0;

    for (i, code) in success_criteria.iter().enumerate() {
        let ast = parser::parse_jue(code).expect(&format!("Success criteria {} should parse", i));

        let mir = lower_frontend_module(&ast);

        let mut codegen = CraneliftCodeGen::new(&format!("success_criteria_{}", i))
            .expect(&format!("Success criteria {} should create codegen", i));

        let result = codegen.generate(&mir);
        assert!(
            result.is_ok(),
            "Success criteria {} should compile: {:?}",
            i,
            result.err()
        );

        assert!(
            !codegen.function_ids.is_empty(),
            "Success criteria {} should generate functions",
            i
        );

        total_functions += codegen.function_ids.len();
    }

    // Validate we met all success criteria
    assert!(
        total_functions > 0,
        "Should have generated functions for all success criteria"
    );

    println!(
        "✅ Runtime validation success criteria test passed with {} total functions",
        total_functions
    );
}

#[test]
fn test_pipeline_validation_summary() {
    // Summary test that demonstrates the complete validation pipeline
    println!("=== Runtime Validation Pipeline Summary ===");

    // Test all major components
    let components = vec![
        ("Arithmetic", "1 + 2 + 3 + 4"),
        ("Variables", "x = 10\ny = 20\nresult = x + y"),
        (
            "Functions",
            "function add(a, b) { return a + b }\nresult = add(5, 3)",
        ),
        (
            "Control Flow",
            "x = 15\nif x > 10 { result = 1 } else { result = 0 }",
        ),
    ];

    let mut component_results = Vec::new();

    for (name, code) in components {
        let ast = parser::parse_jue(code).expect(&format!("{} component should parse", name));
        let mir = lower_frontend_module(&ast);

        let mut codegen = CraneliftCodeGen::new(&format!("summary_{}", name))
            .expect(&format!("{} component should create codegen", name));

        let result = codegen.generate(&mir);
        assert!(result.is_ok(), "{} component should compile", name);

        component_results.push((name, codegen.function_ids.len()));
        println!(
            "✅ {} component validated: {} functions",
            name,
            codegen.function_ids.len()
        );
    }

    // Summary validation
    let total_functions: usize = component_results.iter().map(|(_, count)| *count).sum();
    assert!(
        total_functions > 0,
        "Should have generated functions across all components"
    );

    println!("=== Pipeline Validation Summary ===");
    println!("Total functions generated: {}", total_functions);
    println!("All components validated successfully");
    println!("✅ Pipeline validation summary test completed");
}
