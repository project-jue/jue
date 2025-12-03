use anyhow::{Context, Result};
use juec::frontend::parser::{parse_program, JueParser, Rule};
use juec::frontend::print::print_module;
use pest::Parser;
use std::env;

fn main() -> Result<()> {
    let filename = env::args().nth(1).unwrap_or("program.jue".to_string());
    let src = std::fs::read_to_string(&filename)
        .unwrap_or_else(|_| "print(\"Hello, Jue!\")\n".to_string());

    let parse = JueParser::parse(Rule::program, &src).context("parsing source")?;
    let module = parse_program(parse)?;

    print_module(&module);

    Ok(())
}
