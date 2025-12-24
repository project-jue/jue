use jue_world::parsing::parser::parse;
use jue_world::physics_integration::physics_compiler::compile_to_physics_world;
use jue_world::shared::trust_tier::TrustTier;
use physics_world::api::core::PhysicsWorld;

fn test_recursive_program(name: &str, source: &str) {
    println!("\n=== Testing {} ===", name);

    // Test parsing
    println!("1. Testing parsing...");
    let ast = match parse(source) {
        Ok(ast) => ast,
        Err(e) => {
            println!("✗ Failed to parse {}: {}", name, e);
            return;
        }
    };
    println!("✓ Successfully parsed {}", name);

    // Test compilation
    println!("2. Testing compilation...");
    match compile_to_physics_world(&ast, TrustTier::Empirical) {
        Ok((bytecode, constants)) => {
            println!("✓ Successfully compiled {}", name);
            println!("   Bytecode length: {}", bytecode.len());
            println!("   Constants: {:?}", constants);
            println!("   Bytecode: {:?}", bytecode);

            // Test VM execution
            println!("3. Testing Physics World VM execution...");
            let mut world = PhysicsWorld::new();
            let vm_result = world.execute_actor(1, bytecode, constants, 1000, 1024);

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
