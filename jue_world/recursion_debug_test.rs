use jue_world::compiler::compile;
use jue_world::integration::physics::execute_compiled_program;
use jue_world::trust_tier::TrustTier;

fn main() {
    println!("=== Testing Recursive Function Execution ===\n");

    // Test 1: Simple recursive factorial function
    let factorial_source = r#"
        (let ((fact (lambda (n)
                      (if (<= n 1)
                          1
                          (* n (fact (- n 1)))))))
          (fact 5))
    "#;

    println!("Test 1: Compiling factorial function...");
    match compile(factorial_source, TrustTier::Empirical, 1000, 1024) {
        Ok(compilation_result) => {
            println!("✅ Compilation successful");
            println!("Bytecode length: {}", compilation_result.bytecode.len());
            println!("Constants length: {}", compilation_result.constants.len());

            // Check for MakeClosure opcodes
            let closure_count = compilation_result
                .bytecode
                .iter()
                .filter(|op| matches!(op, physics_world::types::OpCode::MakeClosure(_, _)))
                .count();
            println!("Number of MakeClosure opcodes: {}", closure_count);

            // Try to execute the compiled program
            println!("\nTest 1: Executing factorial function...");
            match execute_compiled_program(
                &compilation_result.bytecode,
                &compilation_result.constants,
                1000,
                1024,
            ) {
                Ok(result) => {
                    println!("✅ Execution successful!");
                    println!("Result: {:?}", result);
                    println!("Expected: Int(120) (5!)");

                    // Check if result is correct
                    if let Some(value) = result.result_value {
                        match value {
                            physics_world::types::Value::Int(n) => {
                                if n == 120 {
                                    println!("✅ Correct result! 5! = 120");
                                } else {
                                    println!("❌ Wrong result! Expected 120, got {}", n);
                                }
                            }
                            _ => println!("❌ Wrong result type! Expected Int, got {:?}", value),
                        }
                    } else {
                        println!("❌ No result value returned");
                    }
                }
                Err(e) => {
                    println!("❌ Execution failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Compilation failed: {:?}", e);
        }
    }

    println!("\n" + "=".repeat(60) + "\n");

    // Test 2: Mutual recursion
    let mutual_recursion_source = r#"
        (let ((is-even? (lambda (n)
                          (if (= n 0)
                              true
                              (is-odd? (- n 1)))))
              (is-odd? (lambda (n)
                         (if (= n 0)
                             false
                             (is-even? (- n 1))))))
          (is-even? 4))
    "#;

    println!("Test 2: Compiling mutual recursion function...");
    match compile(mutual_recursion_source, TrustTier::Empirical, 1000, 1024) {
        Ok(compilation_result) => {
            println!("✅ Compilation successful");
            println!("Bytecode length: {}", compilation_result.bytecode.len());
            println!("Constants length: {}", compilation_result.constants.len());

            let closure_count = compilation_result
                .bytecode
                .iter()
                .filter(|op| matches!(op, physics_world::types::OpCode::MakeClosure(_, _)))
                .count();
            println!("Number of MakeClosure opcodes: {}", closure_count);

            println!("\nTest 2: Executing mutual recursion function...");
            match execute_compiled_program(
                &compilation_result.bytecode,
                &compilation_result.constants,
                1000,
                1024,
            ) {
                Ok(result) => {
                    println!("✅ Execution successful!");
                    println!("Result: {:?}", result);
                    println!("Expected: Bool(true) (4 is even)");

                    if let Some(value) = result.result_value {
                        match value {
                            physics_world::types::Value::Bool(b) => {
                                if b {
                                    println!("✅ Correct result! 4 is even = true");
                                } else {
                                    println!("❌ Wrong result! Expected true, got false");
                                }
                            }
                            _ => println!("❌ Wrong result type! Expected Bool, got {:?}", value),
                        }
                    } else {
                        println!("❌ No result value returned");
                    }
                }
                Err(e) => {
                    println!("❌ Execution failed: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ Compilation failed: {:?}", e);
        }
    }
}
