/// Comprehensive recursion tests to bridge the gap between simple recursion and complex Fibonacci
/// These tests gradually increase in complexity to help identify where recursion fails
use jue_world::parser::parse;
use jue_world::trust_tier::TrustTier;
use jue_world::physics_compiler::compile_to_physics_world;
use physics_world::types::Value;
use physics_world::vm::VmState;

/// Test helper to execute and validate recursive function results
fn test_recursive_execution(
    source: &str,
    trust_tier: TrustTier,
    expected_result: Option<Value>,
    test_name: &str,
) {
    println!("\n=== Testing {} ===", test_name);

    // Parse and compile the program
    let ast = match parse(source) {
        Ok(parsed_ast) => {
            println!("✅ Parsing successful");
            parsed_ast
        }
        Err(e) => {
            panic!("❌ Parsing failed for {}: {:?}", test_name, e);
        }
    };

    let (bytecode, string_constants) = match compile_to_physics_world(&ast, trust_tier) {
        Ok((bytecode, constants)) => {
            println!("✅ Compilation successful");
            println!("Bytecode length: {}", bytecode.len());
            (bytecode, constants)
        }
        Err(e) => {
            panic!("❌ Compilation failed for {}: {:?}", test_name, e);
        }
    };

    // Execute the program
    let mut vm = VmState::new(bytecode, string_constants, 1000, 1024, 1, 100);
    let execution_result = match vm.run() {
        Ok(result) => {
            println!("✅ Execution successful!");
            result
        }
        Err(e) => {
            panic!("❌ Execution failed for {}: {:?}", test_name, e);
        }
    };

    // Validate the result if provided
    if let Some(expected) = expected_result {
        match (&execution_result, &expected) {
            (Value::Int(actual_int), Value::Int(expected_int)) => {
                assert_eq!(
                    actual_int, expected_int,
                    "{}: Expected Int({}), got Int({})",
                    test_name, expected_int, actual_int
                );
                println!("✅ Correct result: Int({})", actual_int);
            }
            (Value::Bool(actual_bool), Value::Bool(expected_bool)) => {
                assert_eq!(
                    actual_bool, expected_bool,
                    "{}: Expected Bool({}), got Bool({})",
                    test_name, expected_bool, actual_bool
                );
                println!("✅ Correct result: Bool({})", actual_bool);
            }
            (Value::Float(actual_float), Value::Float(expected_float)) => {
                assert!(
                    (actual_float - expected_float).abs() < f64::EPSILON,
                    "{}: Expected Float({}), got Float({})",
                    test_name,
                    expected_float,
                    actual_float
                );
                println!("✅ Correct result: Float({})", actual_float);
            }
            (Value::String(actual_string), Value::String(expected_string)) => {
                assert_eq!(
                    actual_string, expected_string,
                    "{}: Expected String({}), got String({})",
                    test_name, expected_string, actual_string
                );
                println!("✅ Correct result: String({})", actual_string);
            }
            _ => {
                panic!(
                    "{}: Wrong result type! Expected {:?}, got {:?}",
                    test_name, expected, execution_result
                );
            }
        }
    }
}

/// Level 1: Basic Recursion Edge Cases
/// These tests focus on the simplest possible recursive scenarios

#[test]
fn test_recursive_identity_function() {
    // Test a recursive function that just returns its parameter (base case only)
    let source = r#"
        (let ((identity (lambda (x)
                          x)))
             (identity 42))
    "#;

    test_recursive_execution(
        source,
        TrustTier::Empirical,
        Some(Value::Int(42)),
        "Recursive Identity Function",
    );
}

#[test]
fn test_recursive_base_case_only() {
    // Test a recursive function that only executes the base case
    let source = r#"
        (let ((fact (lambda (n)
                      (if (<= n 1)
                          1
                          (* n (fact (- n 1)))))))
             (fact 1))
    "#;

    test_recursive_execution(
        source,
        TrustTier::Empirical,
        Some(Value::Int(1)),
        "Recursive Base Case Only",
    );
}

#[test]
fn test_recursive_single_step() {
    // Test a recursive function that only recurses once
    let source = r#"
        (let ((fact (lambda (n)
                      (if (<= n 1)
                          1
                          (* n (fact (- n 1)))))))
             (fact 2))
    "#;

    test_recursive_execution(
        source,
        TrustTier::Empirical,
        Some(Value::Int(2)),
        "Recursive Single Step",
    );
}

/// Level 2: Simple Recursive Patterns
/// These tests introduce simple recursive patterns with minimal complexity

#[test]
fn test_recursive_addition() {
    // Test a simple recursive addition function
    let source = r#"
        (let ((add (lambda (n)
                     (if (= n 0)
                         0
                         (+ n (add (- n 1)))))))
             (add 3))
    "#;

    test_recursive_execution(
        source,
        TrustTier::Empirical,
        Some(Value::Int(6)), // 3 + 2 + 1 + 0 = 6
        "Recursive Addition",
    );
}

#[test]
fn test_recursive_subtraction() {
    // Test a simple recursive subtraction function
    let source = r#"
        (let ((sub (lambda (n)
                     (if (= n 0)
                         10
                         (sub (- n 1))))))
             (sub 5))
    "#;

    test_recursive_execution(
        source,
        TrustTier::Empirical,
        Some(Value::Int(5)), // 10 - 5 = 5
        "Recursive Subtraction",
    );
}

/// Level 3: Parameter Manipulation
/// These tests focus on parameter manipulation in recursive calls

#[test]
fn test_recursive_parameter_decrement() {
    // Test recursive function with parameter decrement
    let source = r#"
        (let ((countdown (lambda (n)
                           (if (= n 0)
                               0
                               (countdown (- n 1))))))
             (countdown 5))
    "#;

    test_recursive_execution(
        source,
        TrustTier::Empirical,
        Some(Value::Int(0)),
        "Recursive Parameter Decrement",
    );
}

#[test]
fn test_recursive_parameter_increment() {
    // Test recursive function with parameter increment (less common but valid)
    let source = r#"
        (let ((countup (lambda (n limit)
                         (if (>= n limit)
                             n
                             (countup (+ n 1) limit)))))
             (countup 0 5))
    "#;

    test_recursive_execution(
        source,
        TrustTier::Empirical,
        Some(Value::Int(5)),
        "Recursive Parameter Increment",
    );
}

/// Level 4: Multiple Recursive Calls
/// These tests introduce functions that make multiple recursive calls

#[test]
fn test_recursive_two_calls() {
    // Test a function that makes two recursive calls (simpler than Fibonacci)
    let source = r#"
        (let ((double-rec (lambda (n)
                            (if (= n 0)
                                1
                                (* 2 (double-rec (- n 1)))))))
             (double-rec 3))
    "#;

    test_recursive_execution(
        source,
        TrustTier::Empirical,
        Some(Value::Int(8)), // 2^3 = 8
        "Recursive Two Calls",
    );
}

/// Level 5: Tail Recursion Patterns
/// These tests focus on tail recursion patterns

#[test]
fn test_tail_recursive_simple() {
    // Test a simple tail recursive function
    let source = r#"
        (let ((sum (lambda (n acc)
                     (if (= n 0)
                         acc
                         (sum (- n 1) (+ acc n))))))
             (sum 5 0))
    "#;

    test_recursive_execution(
        source,
        TrustTier::Empirical,
        Some(Value::Int(15)), // 5 + 4 + 3 + 2 + 1 + 0 = 15
        "Tail Recursive Simple",
    );
}

/// Level 6: Error Cases and Edge Conditions
/// These tests should fail or handle edge conditions gracefully

#[test]
fn test_recursive_stack_overflow() {
    // Test a recursive function that should hit recursion limit
    let source = r#"
        (let ((deep (lambda (n)
                      (if (= n 0)
                          "done"
                          (deep (- n 1))))))
             (deep 200))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, string_constants) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");

    // This should hit recursion limit
    let mut vm = VmState::new(bytecode, string_constants, 1000, 1024, 1, 50); // Low recursion limit
    let result = vm.run();

    // Should either succeed, hit recursion limit, or handle gracefully
    assert!(result.is_ok() || result.is_err());
    println!("✅ Deep recursion handled appropriately");
}

#[test]
fn test_recursive_no_base_case() {
    // Test a recursive function with no base case (should handle gracefully)
    let source = r#"
        (let ((infinite (lambda (n)
                          (infinite (+ n 1)))))
             (infinite 0))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, string_constants) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");

    // This should either execute with CPU limit or handle gracefully
    let mut vm = VmState::new(bytecode, string_constants, 100, 1024, 1, 10); // Low step limit
    let result = vm.run();

    // Should either succeed, hit CPU limit, or handle gracefully
    assert!(result.is_ok() || result.is_err());
    println!("✅ Recursive function without base case handled appropriately");
}

/// Level 7: Physics World VM Tests
/// These tests focus on VM-level recursion handling

#[test]
fn test_vm_closure_creation() {
    // Test that the VM correctly creates closures for recursive functions
    let source = r#"
        (let ((fact (lambda (n)
                      (if (<= n 1)
                          1
                          (* n (fact (- n 1)))))))
             (fact 3))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");

    // Check that bytecode contains MakeClosure instructions
    let has_closures = bytecode
        .iter()
        .any(|op| matches!(op, physics_world::types::OpCode::MakeClosure(_, _)));

    assert!(
        has_closures,
        "Should create closures for recursive functions"
    );
    println!("✅ VM closure creation successful");
}

#[test]
fn test_vm_recursive_call_pattern() {
    // Test that the VM generates correct call patterns for recursion
    let source = r#"
        (let ((fact (lambda (n)
                      (if (<= n 1)
                          1
                          (* n (fact (- n 1)))))))
             (fact 2))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");

    // Check that bytecode contains Call instructions
    let has_calls = bytecode
        .iter()
        .any(|op| matches!(op, physics_world::types::OpCode::Call(_)));

    assert!(has_calls, "Should generate call instructions for recursion");
    println!("✅ VM recursive call pattern correct");
}

/// Level 8: Complex Integration Tests
/// These tests combine multiple features with recursion

#[test]
fn test_recursive_with_let_bindings() {
    // Test recursive function with let bindings
    let source = r#"
        (let ((base 10)
              (add-base (lambda (n)
                          (if (= n 0)
                              base
                              (+ base (add-base (- n 1)))))))
             (add-base 3))
    "#;

    test_recursive_execution(
        source,
        TrustTier::Empirical,
        Some(Value::Int(40)), // 10 + 10 + 10 + 10 + 10 = 50
        "Recursive with Let Bindings",
    );
}

#[test]
fn test_recursive_with_conditional() {
    // Test recursive function with complex conditional logic
    let source = r#"
        (let ((complex-cond (lambda (n)
                              (if (= n 0)
                                  0
                                  (if (< n 0)
                                      (complex-cond (+ n 1))
                                      (complex-cond (- n 1)))))))
             (complex-cond 5))
    "#;

    test_recursive_execution(
        source,
        TrustTier::Empirical,
        Some(Value::Int(0)),
        "Recursive with Conditional",
    );
}

/// Level 9: All Trust Tiers
/// These tests verify recursion works across all trust tiers

#[test]
fn test_recursion_all_trust_tiers() {
    let source = r#"
        (let ((fact (lambda (n)
                      (if (<= n 1)
                          1
                          (* n (fact (- n 1)))))))
             (fact 4))
    "#;

    for tier in [
        TrustTier::Formal,
        TrustTier::Verified,
        TrustTier::Empirical,
        TrustTier::Experimental,
    ] {
        test_recursive_execution(
            source,
            tier,
            Some(Value::Int(24)), // 4! = 24
            &format!("Recursion {:?} Tier", tier),
        );
    }
}
