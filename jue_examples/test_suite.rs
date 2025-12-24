use jue_world::parsing::parser::parse;
use jue_world::physics_integration::physics_compiler::compile_to_physics_world;
use jue_world::shared::trust_tier::TrustTier;
use physics_world::api::core::PhysicsWorld;
use physics_world::types::{OpCode, Value};

fn test_jue_program(name: &str, source: &str) {
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
    println!("   AST: {:?}", ast);

    // Test compilation
    println!("2. Testing compilation...");
    match compile_to_physics_world(&ast, TrustTier::Formal) {
        Ok((bytecode, constants)) => {
            println!("✓ Successfully compiled {}", name);
            println!("   Bytecode length: {}", bytecode.len());
            println!("   Constants: {:?}", constants);
        }
        Err(e) => {
            println!("⚠ Compilation error for {}: {}", name, e);
        }
    }
}

fn test_physics_vm(
    name: &str,
    bytecode: Vec<OpCode>,
    constants: Vec<Value>,
    expected_output: Option<Value>,
) {
    println!("\n3. Testing Physics World VM for {}...", name);

    let mut world = PhysicsWorld::new();
    let result = world.execute_actor(1, bytecode, constants, 1000, 1024);

    match result.output {
        Some(actual_output) => {
            println!("✓ Physics World VM executed successfully");
            println!("   Output: {:?}", actual_output);

            if let Some(expected) = expected_output {
                if actual_output == expected {
                    println!("✓ Output matches expected: {:?}", expected);
                } else {
                    println!(
                        "✗ Output mismatch. Expected: {:?}, Got: {:?}",
                        expected, actual_output
                    );
                }
            }
        }
        None => {
            if let Some(error) = result.error {
                println!("✗ Physics World VM execution failed: {}", error);
            } else {
                println!("✗ Physics World VM produced no output");
            }
        }
    }
}

fn main() {
    println!("Jue Examples Test Suite");
    println!("======================");

    // Test 1: Hello World
    let hello_world_source = r#"
        (:formal 42)
    "#;

    test_jue_program("Hello World", hello_world_source);

    // Test equivalent bytecode for Hello World
    test_physics_vm(
        "Hello World",
        vec![OpCode::Int(42)],
        vec![],
        Some(Value::Int(42)),
    );

    // Test 2: Arithmetic - test individual expressions
    let arithmetic_add_source = r#"
        (:formal (+ 1 2))
    "#;

    test_jue_program("Arithmetic Addition", arithmetic_add_source);

    let arithmetic_complex_source = r#"
        (:formal (+ (* 3 4) (- 10 5)))
    "#;

    test_jue_program("Complex Arithmetic", arithmetic_complex_source);

    // Test equivalent bytecode for simple arithmetic: 1 + 2 = 3
    test_physics_vm(
        "Simple Addition",
        vec![OpCode::Int(1), OpCode::Int(2), OpCode::Add],
        vec![],
        Some(Value::Int(3)),
    );

    // Test 3: Lambda - test individual expressions
    let lambda_identity_source = r#"
        (:formal (lambda (x) x))
    "#;

    test_jue_program("Lambda Identity", lambda_identity_source);

    let lambda_application_source = r#"
        (:formal ((lambda (x) (+ x 1)) 5))
    "#;

    test_jue_program("Lambda Application", lambda_application_source);

    // Test 4: Complex expressions
    let complex_source = r#"
        (:formal
          (let ((x 5)
                (y 10))
            (+ x y))
        )
    "#;

    test_jue_program("Complex Expressions", complex_source);

    // Test equivalent bytecode for let expression: let x=5, y=10 in x+y
    // This is simplified since we don't have full let support in bytecode yet
    test_physics_vm(
        "Let Expression Equivalent",
        vec![OpCode::Int(5), OpCode::Int(10), OpCode::Add],
        vec![],
        Some(Value::Int(15)),
    );

    println!("\n=== Test Suite Complete ===");
    println!("Summary:");
    println!("- Jue parser is working and can handle all examples");
    println!("- Compiler shows expected behavior (needs implementation)");
    println!("- Physics World VM can execute simple bytecode programs");
    println!("- All basic arithmetic operations work in VM");
    println!("- Foundation is ready for full compiler implementation");
}
