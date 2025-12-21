use jue_world::parser::parse;

fn main() {
    // Test the arithmetic source directly
    let arithmetic_source = r#"
        (:formal (+ 1 2))
    "#;

    println!("Testing arithmetic source:");
    println!("Source: {:?}", arithmetic_source);

    match parse(arithmetic_source) {
        Ok(ast) => {
            println!("✓ Successfully parsed arithmetic");
            println!("AST: {:?}", ast);
        }
        Err(e) => {
            println!("✗ Failed to parse arithmetic: {}", e);
        }
    }

    // Test lambda source
    let lambda_source = r#"
        (:formal (lambda (x) x))
    "#;

    println!("\nTesting lambda source:");
    println!("Source: {:?}", lambda_source);

    match parse(lambda_source) {
        Ok(ast) => {
            println!("✓ Successfully parsed lambda");
            println!("AST: {:?}", ast);
        }
        Err(e) => {
            println!("✗ Failed to parse lambda: {}", e);
        }
    }
}
