use jue_world::compiler::compile;
use jue_world::parser::parse;
use jue_world::trust_tier::TrustTier;
use physics_world::types::{OpCode, Value};
use physics_world::api::PhysicsWorld;

fn test_recursive_program(name: &str, source: &str) {
    println!("\n=== Testing {} ===", name);

    // Test parsing
    println!("1. Testing parsing...");
    match parse(source) {
        Ok(ast) => {
            println!("✓ Successfully parsed {}", name);
        }
        Err(e) => {
            println!("✗ Failed to parse {}: {}", name, e);
            return;
        }
    }

    // Test compilation
    println!("2. Testing compilation...");
    match compile(source, TrustTier::Empirical, 1000, 1024) {
        Ok(result) => {
            println!("✓ Successfully compiled {}", name);
            println!("   Bytecode length: {}", result.bytecode.len());
            println!("   Constants: {:?}", result.constants);
            println!("   Bytecode: {:?}", result.bytecode);

            // Test VM execution
            println!("3. Testing Physics World VM execution...");
            let mut world = PhysicsWorld::new();
            let vm_result = world.execute_actor(1, result.bytecode, result.constants, 1000, 1024);

            match vm_result.output {
                Some(output) => {
                    println!("✓ Physics World VM executed successfully");
                    println!("   Output: {:?}", output);
                }
                None => {
                    if let Some(error) = vm_result.error {
                        println!("✗ Physics World VM execution failed: {:?}", error);
                    } else {
                        println!("✗ Physics World VM produced no output");
                    }
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to compile {}: {}", name, e);
        }
    }
}

fn main() {
    println!("Recursive Jue Programs Test Suite");
    println!("================================");

    // Test 1: Simple recursive function
    let simple_recursion_source = r#"
        (let ((fact (lambda (n)
                      (if (<= n 1)
                          1
                          (* n (fact (- n 1)))))))
          (fact 3))
    "#;

    test_recursive_program("Simple Recursion (factorial 3)", simple_recursion_source);

    // Test 2: Simple lambda in let
    let lambda_in_let_source = r#"
        (let ((f (lambda (x) (+ x 1))))
          (f 5))
    "#;

    test_recursive_program("Lambda in Let", lambda_in_let_source);

    println!("\n=== Recursive Test Suite Complete ===");
}
