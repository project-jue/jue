use jue_world::compiler::compile;
use jue_world::trust_tier::TrustTier;
use std::fs;

fn main() {
    println!("Testing Jue compiler with examples...");

    // Test files to compile
    let test_files = vec![
        ("hello_world.jue", "Hello World example"),
        ("arithmetic.jue", "Arithmetic operations"),
        ("lambda.jue", "Lambda functions"),
        ("complex.jue", "Complex expressions"),
    ];

    for (filename, description) in test_files {
        println!("\nTesting {}...", description);

        // Read the Jue source file
        let source = match fs::read_to_string(filename) {
            Ok(content) => content,
            Err(e) => {
                println!("❌ Failed to read {}: {}", filename, e);
                continue;
            }
        };

        println!("Source: {}", source);

        // Compile with different trust tiers
        for tier in vec![
            TrustTier::Formal,
            TrustTier::Verified,
            TrustTier::Empirical,
            TrustTier::Experimental,
        ] {
            println!("\n  Testing with {:?} tier...", tier);

            let result = compile(&source, tier, 1000, 1024);

            match result {
                Ok(compilation_result) => {
                    println!("  ✅ Compilation successful!");

                    println!("  Bytecode length: {}", compilation_result.bytecode.len());
                    println!("  Constants count: {}", compilation_result.constants.len());
                    println!("  Sandboxed: {}", compilation_result.sandboxed);
                    println!("  Step limit: {}", compilation_result.step_limit);
                    println!("  Memory limit: {}", compilation_result.memory_limit);

                    if let Some(core_expr) = compilation_result.core_expr {
                        println!("  CoreExpr: {}", core_expr);
                    } else {
                        println!("  CoreExpr: None (not Formal/Verified tier)");
                    }
                }
                Err(e) => {
                    println!("  ❌ Compilation failed: {:?}", e);
                }
            }
        }
    }

    println!("\n✅ All compiler tests completed!");
}
