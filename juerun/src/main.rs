// REPL / program runner
//use jue_common::ast::JueAST;
use std::env;
use std::fs;
use std::io::{self, Write};

fn run_file(path: &str) {
    let content = fs::read_to_string(path).expect("Failed to read compiled Jue file");

    // TODO: Deserialize AST or bytecode, then execute
    println!("Running Jue program: {}", path);
    println!("Content: {}", content);
}

fn repl() {
    println!("Welcome to Jue REPL! quit to exit.");
    let mut input = String::new();

    loop {
        print!("jue> ");
        io::stdout().flush().unwrap();

        input.clear();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let trimmed = input.trim();
        if trimmed == "exit" || trimmed == "quit" {
            break;
        }

        // TODO: Parse input into AST and evaluate
        println!("You typed: {}", trimmed);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let cmd = &args[1];
        if cmd == "repl" {
            repl();
        } else {
            run_file(cmd);
        }
    } else {
        println!("Usage: juerun <compiled_file.jbc> | repl");
        repl();
    }
}

//cargo run -p juerun -- repl
//cargo run -p juerun -- out.jbc
