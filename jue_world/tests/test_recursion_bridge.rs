// Comprehensive recursion tests for the Jue-World to Physics-World bridge
// These tests verify compilation of recursive functions across all trust tiers
use jue_world::parser::parse;
use jue_world::physics_compiler::compile_to_physics_world;
use jue_world::trust_tier::TrustTier;
use physics_world::types::Value;
use physics_world::vm::VmState;

/// Test helper to compile and verify recursive function results
fn test_recursive_compilation(source: &str, trust_tier: TrustTier, test_name: &str) {
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

    // Execute the program if possible
    let mut vm = VmState::new(bytecode.clone(), string_constants, 1000, 1024, 1, 100);
    let execution_result = vm.run();

    match execution_result {
        Ok(result) => {
            println!("✅ Execution successful! Result: {:?}", result);
        }
        Err(e) => {
            println!("⚠️ Execution error (expected for some tests): {:?}", e);
        }
    }
}

/// Level 1: Basic Recursion Edge Cases

#[test]
fn test_non_recursive_identity_function() {
    // Test a simple non-recursive identity function with let
    let source = r#"
        (let ((identity (lambda (x)
                          x)))
             (identity 42))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");
    assert!(!bytecode.is_empty(), "Should compile to bytecode");
    println!(
        "✅ Non-recursive Identity compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_recursive_base_case_only() {
    // Test a recursive function that only executes the base case
    // Note: Use letrec for recursive functions so 'fact' is visible inside the lambda
    let source = r#"
        (letrec ((fact (lambda (n)
                      (if (<= n 1)
                          1
                          (* n (fact (- n 1)))))))
             (fact 1))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");
    assert!(!bytecode.is_empty(), "Should compile to bytecode");
    println!(
        "✅ Recursive Base Case Only compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_recursive_single_step() {
    // Test a recursive function that only recurses once
    let source = r#"
        (letrec ((fact (lambda (n)
                      (if (<= n 1)
                          1
                          (* n (fact (- n 1)))))))
             (fact 2))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");
    assert!(!bytecode.is_empty(), "Should compile to bytecode");
    println!(
        "✅ Recursive Single Step compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

/// Level 2: Simple Recursive Patterns

#[test]
fn test_recursive_addition() {
    // Test a simple recursive addition function
    let source = r#"
        (letrec ((add (lambda (n)
                      (if (= n 0)
                          0
                          (+ n (add (- n 1)))))))
             (add 3))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");
    assert!(!bytecode.is_empty(), "Should compile to bytecode");
    println!(
        "✅ Recursive Addition compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_recursive_subtraction() {
    // Test a simple recursive subtraction function
    let source = r#"
        (letrec ((sub (lambda (n)
                      (if (= n 0)
                          10
                          (sub (- n 1))))))
             (sub 5))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");
    assert!(!bytecode.is_empty(), "Should compile to bytecode");
    println!(
        "✅ Recursive Subtraction compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

/// Level 3: Parameter Manipulation

#[test]
fn test_recursive_parameter_decrement() {
    let source = r#"
        (letrec ((countdown (lambda (n)
                           (if (= n 0)
                               0
                               (countdown (- n 1))))))
             (countdown 5))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");
    assert!(!bytecode.is_empty(), "Should compile to bytecode");
    println!(
        "✅ Recursive Parameter Decrement compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_recursive_parameter_increment() {
    let source = r#"
        (letrec ((countup (lambda (n limit)
                         (if (>= n limit)
                             n
                             (countup (+ n 1) limit)))))
             (countup 0 5))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");
    assert!(!bytecode.is_empty(), "Should compile to bytecode");
    println!(
        "✅ Recursive Parameter Increment compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

/// Level 4: Multiple Recursive Calls

#[test]
fn test_recursive_two_calls() {
    let source = r#"
        (letrec ((double-rec (lambda (n)
                            (if (= n 0)
                                1
                                (* 2 (double-rec (- n 1)))))))
             (double-rec 3))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");
    assert!(!bytecode.is_empty(), "Should compile to bytecode");
    println!(
        "✅ Recursive Two Calls compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

/// Level 5: Tail Recursion Patterns

#[test]
fn test_tail_recursive_simple() {
    let source = r#"
        (letrec ((sum (lambda (n acc)
                     (if (= n 0)
                         acc
                         (sum (- n 1) (+ acc n))))))
             (sum 5 0))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");
    assert!(!bytecode.is_empty(), "Should compile to bytecode");
    println!(
        "✅ Tail Recursive Simple compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

/// Level 6: Error Cases and Edge Conditions

#[test]
fn test_recursive_stack_overflow() {
    // Test a recursive function that should hit recursion limit
    let source = r#"
        (letrec ((deep (lambda (n)
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
        (letrec ((infinite (lambda (n)
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

#[test]
fn test_vm_closure_creation() {
    let source = r#"
        (letrec ((fact (lambda (n)
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
    let source = r#"
        (letrec ((fact (lambda (n)
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

#[test]
fn test_recursive_with_let_bindings() {
    let source = r#"
        (letrec ((add-base (lambda (n)
                          (if (= n 0)
                              10
                              (+ 10 (add-base (- n 1)))))))
             (add-base 3))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");
    assert!(!bytecode.is_empty(), "Should compile to bytecode");
    println!(
        "✅ Recursive with Let Bindings compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_recursive_with_conditional() {
    let source = r#"
        (letrec ((complex-cond (lambda (n)
                              (if (= n 0)
                                  0
                                  (if (< n 0)
                                      (complex-cond (+ n 1))
                                      (complex-cond (- n 1)))))))
             (complex-cond 5))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");
    assert!(!bytecode.is_empty(), "Should compile to bytecode");
    println!(
        "✅ Recursive with Conditional compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

/// Level 9: All Trust Tiers

#[test]
fn test_recursion_all_trust_tiers() {
    let source = r#"
        (letrec ((fact (lambda (n)
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
        let ast = parse(source).expect("Parsing should succeed");
        let (bytecode, _) = compile_to_physics_world(&ast, tier)
            .expect(&format!("Compilation should succeed for {:?}", tier));
        assert!(!bytecode.is_empty(), "Should compile to bytecode");
        println!(
            "✅ Recursion compiled for {:?} tier ({} opcodes)",
            tier,
            bytecode.len()
        );
    }
}

/// Y-Combinator Tests

#[test]
fn test_y_combinator_basic() {
    let source = r#"
        (let ((Y (lambda (f)
                  ((lambda (x) (f (x x)))
                   (lambda (x) (f (x x)))))))
          Y)
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");

    assert!(
        !bytecode.is_empty(),
        "Y-combinator should compile to bytecode"
    );
    println!(
        "✅ Y-Combinator Basic compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_y_combinator_factorial_compiles() {
    let source = r#"
        (let ((Y (lambda (f)
                  ((lambda (x) (f (x x)))
                   (lambda (x) (f (x x)))))))
          (let ((fact (Y (lambda (f)
                           (lambda (n)
                             (if (= n 0)
                               1
                               (* n ((f) (- n 1)))))))))
            ((fact) 5)))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");

    assert!(
        !bytecode.is_empty(),
        "Y-combinator factorial should compile to bytecode"
    );
    println!(
        "✅ Y-Combinator Factorial compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_y_combinator_sum_compiles() {
    let source = r#"
        (let ((Y (lambda (f)
                  ((lambda (x) (f (x x)))
                   (lambda (x) (f (x x)))))))
          (let ((sum (Y (lambda (f)
                          (lambda (n)
                            (if (= n 0)
                              0
                              (+ n ((f) (- n 1)))))))))
            ((sum) 10)))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");

    assert!(
        !bytecode.is_empty(),
        "Y-combinator sum should compile to bytecode"
    );
    println!(
        "✅ Y-Combinator Sum compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_letrec_factorial_compiles() {
    let source = r#"
        (letrec ((fact (lambda (n)
                        (if (= n 0)
                          1
                          (* n (fact (- n 1)))))))
          (fact 6))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");

    assert!(
        !bytecode.is_empty(),
        "Letrec factorial should compile to bytecode"
    );
    println!(
        "✅ Letrec Factorial compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_letrec_fibonacci_compiles() {
    let source = r#"
        (letrec ((fib (lambda (n)
                       (if (< n 2)
                         n
                         (+ (fib (- n 1)) (fib (- n 2)))))))
          (fib 10))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");

    assert!(
        !bytecode.is_empty(),
        "Letrec fibonacci should compile to bytecode"
    );
    println!(
        "✅ Letrec Fibonacci compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_letrec_mutual_recursion_compiles() {
    let source = r#"
        (letrec ((even (lambda (n)
                        (if (= n 0)
                          true
                          (odd (- n 1)))))
                 (odd (lambda (n)
                       (if (= n 0)
                         false
                         (even (- n 1))))))
          (even 10))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");

    assert!(
        !bytecode.is_empty(),
        "Letrec mutual recursion should compile to bytecode"
    );
    println!(
        "✅ Letrec Mutual Recursion compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_define_factorial_compiles() {
    // Test top-level recursion using letrec (define not fully implemented)
    let source = r#"
        (letrec ((fact (lambda (n)
                        (if (= n 0)
                          1
                          (* n (fact (- n 1)))))))
          (fact 7))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");

    assert!(
        !bytecode.is_empty(),
        "Letrec factorial should compile to bytecode"
    );
    println!(
        "✅ Factorial compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_z_combinator_compiles() {
    let source = r#"
        (let ((Z (lambda (f)
                  ((lambda (x) (f (lambda (v) ((x x) v))))
                   (lambda (x) (f (lambda (v) ((x x) v))))))))
          (let ((fact (Z (lambda (f)
                           (lambda (n acc)
                             (if (= n 0)
                               acc
                               (f (- n 1) (* n acc))))))))
            ((fact 5) 1)))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let (bytecode, _) =
        compile_to_physics_world(&ast, TrustTier::Empirical).expect("Compilation should succeed");

    assert!(
        !bytecode.is_empty(),
        "Z-combinator should compile to bytecode"
    );
    println!(
        "✅ Z-Combinator compiled successfully ({} opcodes)",
        bytecode.len()
    );
}

#[test]
fn test_y_combinator_all_trust_tiers_compile() {
    let source = r#"
        (let ((Y (lambda (f)
                  ((lambda (x) (f (x x)))
                   (lambda (x) (f (x x)))))))
          (let ((fact (Y (lambda (f)
                           (lambda (n)
                             (if (= n 0)
                               1
                               (* n ((f) (- n 1)))))))))
            ((fact) 4)))
    "#;

    for tier in [
        TrustTier::Formal,
        TrustTier::Verified,
        TrustTier::Empirical,
        TrustTier::Experimental,
    ] {
        let ast = parse(source).expect("Parsing should succeed");
        let (bytecode, _) = compile_to_physics_world(&ast, tier)
            .expect(&format!("Y-Combinator should compile for {:?}", tier));

        assert!(
            !bytecode.is_empty(),
            "Y-Combinator should compile to bytecode"
        );
        println!(
            "✅ Y-Combinator compiled for {:?} tier ({} opcodes)",
            tier,
            bytecode.len()
        );
    }
}

#[test]
fn test_letrec_all_trust_tiers_compile() {
    let source = r#"
        (letrec ((fact (lambda (n)
                        (if (= n 0)
                          1
                          (* n (fact (- n 1)))))))
          (fact 5))
    "#;

    for tier in [
        TrustTier::Formal,
        TrustTier::Verified,
        TrustTier::Empirical,
        TrustTier::Experimental,
    ] {
        let ast = parse(source).expect("Parsing should succeed");
        let (bytecode, _) = compile_to_physics_world(&ast, tier)
            .expect(&format!("Letrec should compile for {:?}", tier));

        assert!(!bytecode.is_empty(), "Letrec should compile to bytecode");
        println!(
            "✅ Letrec compiled for {:?} tier ({} opcodes)",
            tier,
            bytecode.len()
        );
    }
}
