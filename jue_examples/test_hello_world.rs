use jue_world::compiler::compile;
use jue_world::parser::parse;
use jue_world::trust_tier::TrustTier;

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
            match compile(hello_world_source, TrustTier::Formal, 1000, 1024) {
                Ok(result) => {
                    println!("✓ Successfully compiled Jue program");
                    println!("Compilation result: {:?}", result);
                }
                Err(e) => {
                    println!("⚠ Compiler not fully implemented yet: {}", e);
                    println!("This is expected - compiler needs to be implemented");
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to parse Jue program: {}", e);
        }
    }

    // Test the Physics World VM directly with equivalent bytecode
    println!("\nTesting Physics World VM with equivalent bytecode...");

    use physics_world::types::OpCode;
    use physics_world::PhysicsWorld;

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
