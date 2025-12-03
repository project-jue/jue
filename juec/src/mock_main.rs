use jue_common::ast::JueAST;
use std::env;
use std::fs;

fn main() {
    // Collect command line args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: juec <source.jue>");
        std::process::exit(1);
    }

    let file_path = &args[1];

    // Read source file
    let source_code = fs::read_to_string(file_path).expect("Failed to read source file");

    println!("Compiling: {}", file_path);

    // TODO: Lexer & Parser
    // For now we just mock a trivial AST
    let ast = JueAST::Literal("Hello, Jue!".to_string());

    println!("Generated AST: {:?}", ast);

    // TODO: Semantic analysis, IR generation, codegen
    // For MVP, just write AST to a file
    let output_path = "out.jbc";
    fs::write(output_path, format!("{:?}", ast)).expect("Failed to write output file");

    println!("Compilation finished: {}", output_path);
}
//cargo run -p juec -- examples/hello.jue
