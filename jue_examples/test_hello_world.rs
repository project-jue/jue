use jue_world::parsing::parser::parse;
use jue_world::physics_integration::physics_compiler::compile_to_physics_world;
use jue_world::shared::trust_tier::TrustTier;
use physics_world::api::core::PhysicsWorld;
use physics_world::types::OpCode;

fn main() {
    // Test parsing the hello world Jue program
    let hello_world_source = r#"
        ; Simple "Hello World" Jue program
        (:formal 42)
    "#;

    println!("Testing Jue Hello World program...");

    // Parse the Jue source code
    match parse(hello_world_source) {
        Ok(ast) => {
            println!("✓ Successfully parsed Jue program");
            println!("AST: {:?}", ast);

            // Try to compile it (this will show current compiler capabilities)
            match compile_to_physics_world(&ast, TrustTier::Formal) {
                Ok((bytecode, constants)) => {
                    println!("✓ Successfully compiled Jue program");
                    println!("Bytecode: {:?}", bytecode);
                    println!("Constants: {:?}", constants);
                }
                Err(e) => {
                    println!("⚠ Compilation error: {}", e);
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to parse Jue program: {}", e);
        }
    }

    // Test the Physics World VM directly with equivalent bytecode
    println!("\nTesting Physics World VM with equivalent bytecode...");

    let mut world = PhysicsWorld::new();

    // Create bytecode equivalent to (:formal 42) - just push 42
    let bytecode = vec![OpCode::Int(42)];
    let constants = vec![];

    let result = world.execute_actor(1, bytecode, constants, 1000, 1024);

    match result.output {
        Some(value) => {
            println!("✓ Physics World VM executed successfully");
            println!("Output: {:?}", value);
        }
        None => {
            if let Some(error) = result.error {
                println!("✗ Physics World VM execution failed: {:?}", error);
            } else {
                println!("✗ Physics World VM produced no output");
            }
        }
    }
}
