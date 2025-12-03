use juec::backend::cranelift_gen::CraneliftCodeGen;
use juec::frontend::parser;
use juec::middle::mir_lower::lower_frontend_module;
use std::path::PathBuf;

/// Phase 4 Comprehensive Validation Tests
/// These tests provide comprehensive validation of the full execution pipeline
/// and demonstrate the complete integration from parsing through runtime preparation.

#[test]
fn test_complete_execution_pipeline() {
    // Test the complete execution pipeline with all shared sample files
    let sample_files = vec![
        (
            "arithmetic",
            "../../../tests/shared_samples/phase_1_parsing/01_arithmetic_expressions.jue",
        ),
        (
            "variables",
            "../../../tests/shared_samples/phase_1_parsing/02_variable_declarations.jue",
        ),
        (
            "control_flow",
            "../../../tests/shared_samples/phase_1_parsing/03_control_flow.jue",
        ),
        (
            "functions",
            "../../../tests/shared_samples/phase_1_parsing/04_function_definitions.jue",
        ),
    ];

    let mut total_functions = 0;

    for (name, file_path) in sample_files {
        let source = std::fs::read_to_string(file_path)
            .expect(&format!("Failed to read {} test file", name));

        // Complete pipeline: Parse → MIR → Cranelift IR
        let ast = parser::parse_jue(&source).expect(&format!("Failed to parse {}", name));

        let mir = lower_frontend_module(&ast);

        let mut codegen = CraneliftCodeGen::new(&format!("comprehensive_{}_test", name)).expect(
            &format!("Failed to create Cranelift code generator for {}", name),
        );

        let result = codegen.generate(&mir);
        assert!(
            result.is_ok(),
            "Should successfully compile {}: {:?}",
            name,
            result.err()
        );

        // Verify we generated executable functions
        assert!(
            !codegen.function_ids.is_empty(),
            "Should have generated functions for {}",
            name
        );

        total_functions += codegen.function_ids.len();
        println!(
            "✅ {} validation passed with {} functions",
            name,
            codegen.function_ids.len()
        );
    }

    // We should have generated a reasonable number of functions across all tests
    assert!(
        total_functions > 0,
        "Should have generated functions across all test cases"
    );
    println!(
        "✅ Complete execution pipeline test passed with total {} functions",
        total_functions
    );
}

#[test]
fn test_end_to_end_validation() {
    // Test end-to-end validation from source code to runtime-ready execution
    let test_code = r#"
        // Comprehensive test code
        x = 10
        y = 20
        function add(a, b) {
            return a + b
        }
        result = add(x, y)
        if result > 15 {
            final = result * 2
        } else {
            final = result + 5
        }
    "#;

    // Complete pipeline execution
    let ast = parser::parse_jue(test_code).expect("Failed to parse comprehensive test code");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("end_to_end_test")
        .expect("Failed to create Cranelift code generator");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully compile comprehensive test code"
    );

    // Verify we have runtime-ready functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated runtime-ready functions for comprehensive test"
    );

    // Validate function count is reasonable for this code
    assert!(
        codegen.function_ids.len() >= 1,
        "Should have generated at least one function for comprehensive test"
    );

    println!(
        "✅ End-to-end validation test passed with {} functions",
        codegen.function_ids.len()
    );
}

#[test]
fn test_runtime_validation_comprehensive() {
    // Comprehensive test that validates all aspects of runtime preparation
    let test_cases = vec![
        (
            "simple_arithmetic",
            "result = (1 + 2) * (3 + 4)",
        ),
        (
            "variable_operations",
            "x = 5\ny = 10\nresult = x * y + (x + y)",
        ),
        (
            "function_with_logic",
            "function calculate(a, b) {\n    if a > b {\n        return a * 2\n    } else {\n        return b * 3\n    }\n}\nresult = calculate(5, 10)",
        ),
    ];

    for (name, code) in test_cases {
        let ast = parser::parse_jue(code).expect(&format!("Failed to parse {}", name));
        let mir = lower_frontend_module(&ast);

        let mut codegen = CraneliftCodeGen::new(&format!("comprehensive_{}", name))
            .expect(&format!("Failed to create codegen for {}", name));

        let result = codegen.generate(&mir);
        assert!(
            result.is_ok(),
            "Should successfully compile {}: {:?}",
            name,
            result.err()
        );

        assert!(
            !codegen.function_ids.is_empty(),
            "Should have generated functions for {}",
            name
        );

        println!("✅ {} comprehensive validation passed", name);
    }
}

#[test]
fn test_pipeline_robustness() {
    // Test the robustness of the pipeline with various code patterns
    let robustness_cases = vec![
        "x = 1 + 2 + 3 + 4 + 5",                   // Multiple operations
        "a = 1\nb = 2\nc = 3\nresult = a + b + c", // Multiple variables
        "function chain(a) {\n    return a + 1\n}\nresult = chain(chain(chain(1)))", // Function chaining
    ];

    for (i, code) in robustness_cases.iter().enumerate() {
        let ast = parser::parse_jue(code).expect(&format!("Failed to parse robustness case {}", i));
        let mir = lower_frontend_module(&ast);

        let mut codegen = CraneliftCodeGen::new(&format!("robustness_test_{}", i))
            .expect(&format!("Failed to create codegen for case {}", i));

        let result = codegen.generate(&mir);
        assert!(
            result.is_ok(),
            "Robustness case {} should compile successfully: {:?}",
            i,
            result.err()
        );

        println!("✅ Robustness test case {} passed", i);
    }

    println!("✅ Pipeline robustness test passed");
}

#[test]
fn test_execution_preparation_validation() {
    // Validate that code is properly prepared for execution
    let preparation_code = r#"
        // Code that should be ready for execution
        counter = 0
        function increment() {
            counter = counter + 1
            return counter
        }
        result = increment()
        if result == 1 {
            success = true
        } else {
            success = false
        }
    "#;

    let ast =
        parser::parse_jue(preparation_code).expect("Failed to parse execution preparation code");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("execution_preparation_test")
        .expect("Failed to create code generator for execution preparation");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully prepare code for execution"
    );

    // Verify we have executable functions
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated executable functions"
    );

    // Validate that all function IDs are valid
    for func_id in &codegen.function_ids {
        assert!(
            !func_id.is_null(),
            "Function ID should be valid for execution"
        );
    }

    println!("✅ Execution preparation validation test passed");
}

#[test]
fn test_runtime_compatibility_validation() {
    // Test that generated code is compatible with runtime execution requirements
    let compatibility_code = r#"
        // Code that should be compatible with runtime execution
        value1 = 42
        value2 = 100
        function compute() {
            temp = value1 * 2
            result = temp + value2
            return result
        }
        final_result = compute()
    "#;

    let ast =
        parser::parse_jue(compatibility_code).expect("Failed to parse compatibility test code");
    let mir = lower_frontend_module(&ast);

    let mut codegen = CraneliftCodeGen::new("runtime_compatibility_test")
        .expect("Failed to create code generator for compatibility test");

    let result = codegen.generate(&mir);
    assert!(
        result.is_ok(),
        "Should successfully generate runtime-compatible code"
    );

    // Verify runtime compatibility indicators
    assert!(
        !codegen.function_ids.is_empty(),
        "Should have generated runtime-compatible functions"
    );

    println!("✅ Runtime compatibility validation test passed");
}
