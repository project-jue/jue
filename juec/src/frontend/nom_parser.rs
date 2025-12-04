// Cargo.toml dependencies:
// [dependencies]
// nom = "8"
// nom_locate = "4"

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::map_res,
    multi::many0,
    sequence::delimited,
    IResult, Parser,
};
use nom_locate::LocatedSpan;
use std::str::FromStr;

type Span<'a> = LocatedSpan<&'a str>;

// ----------------------
// AST Definitions
// ----------------------
#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Ident(String),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Assign(String, Expr),
    Return(Expr),
    FunctionDef(String, Vec<String>, Vec<Stmt>),
}

// ----------------------
// Token parsers
// ----------------------

fn parse_ident(input: Span) -> IResult<Span, String> {
    let mut parser = take_while1(|c: char| c.is_ascii_alphabetic() || c == '_')
        .map(|s: Span| s.fragment().to_string());
    parser.parse(input)
}

fn parse_number(input: Span) -> IResult<Span, Expr> {
    let mut parser = digit1.map_res(|s: Span| i64::from_str(s.fragment()).map(Expr::Number));
    parser.parse(input)
}

// ----------------------
// Expression parser
// ----------------------

fn parse_factor(input: Span) -> IResult<Span, Expr> {
    alt((
        parse_number,
        parse_ident.map(Expr::Ident),
        delimited(char('('), parse_expr, char(')')),
    ))
    .parse(input)
}

fn parse_term(input: Span) -> IResult<Span, Expr> {
    let (mut input, mut acc) = parse_factor(input)?;
    loop {
        let (i2, _) = multispace0(input)?;
        let op_res = alt((
            char::<Span, nom::error::Error<Span>>('*'),
            char::<Span, nom::error::Error<Span>>('/'),
        ))
        .parse(i2);
        if let Ok((i3, op_ch)) = op_res {
            let (i4, rhs) = parse_factor(i3)?;
            let op = match op_ch {
                '*' => BinOp::Mul,
                '/' => BinOp::Div,
                _ => unreachable!(),
            };
            acc = Expr::BinOp(Box::new(acc), op, Box::new(rhs));
            input = i4;
            continue;
        }
        break;
    }
    Ok((input, acc))
}

fn parse_expr(input: Span) -> IResult<Span, Expr> {
    let (mut input, mut acc) = parse_term(input)?;
    loop {
        let (i2, _) = multispace0(input)?;
        let op_res = alt((
            char::<Span, nom::error::Error<Span>>('+'),
            char::<Span, nom::error::Error<Span>>('-'),
        ))
        .parse(i2);
        if let Ok((i3, op_ch)) = op_res {
            let (i4, rhs) = parse_term(i3)?;
            let op = match op_ch {
                '+' => BinOp::Add,
                '-' => BinOp::Sub,
                _ => unreachable!(),
            };
            acc = Expr::BinOp(Box::new(acc), op, Box::new(rhs));
            input = i4;
            continue;
        }
        break;
    }
    Ok((input, acc))
}

// ----------------------
// Statement parsers
// ----------------------

fn parse_return(input: Span) -> IResult<Span, Stmt> {
    let (input, _) = tag("return").parse(input)?;
    let (input, _) = multispace1(input)?;
    let (input, expr) = parse_expr(input)?;
    Ok((input, Stmt::Return(expr)))
}

fn parse_assign(input: Span) -> IResult<Span, Stmt> {
    let (input, name) = parse_ident(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('=').parse(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = parse_expr(input)?;
    Ok((input, Stmt::Assign(name, expr)))
}

fn parse_stmt(input: Span) -> IResult<Span, Stmt> {
    alt((parse_return, parse_assign)).parse(input)
}

// ----------------------
// Function parser
// ----------------------

fn parse_function(input: Span) -> IResult<Span, Stmt> {
    let (input, _) = tag("def").parse(input)?;
    let (input, _) = multispace1(input)?;
    let (input, name) = parse_ident(input)?;
    let (input, _) = char('(').parse(input)?;
    let (input, _) = char(')').parse(input)?; // No parameters for simplicity
    let (input, _) = char(':').parse(input)?;
    let (input, _) = multispace0(input)?;
    let (input, body) = many0(parse_stmt).parse(input)?;
    Ok((input, Stmt::FunctionDef(name, vec![], body)))
}

// ----------------------
// Top-level parser
// ----------------------

fn parse_program(input: Span) -> IResult<Span, Vec<Stmt>> {
    many0(alt((parse_function, parse_stmt))).parse(input)
}

// ----------------------
// Example usage
// ----------------------

fn main() {
    let code = r#"
def add():
    return 1 + 2 * 3

x = 42
"#;

    let span = Span::new(code);
    match parse_program(span) {
        Ok((_rest, stmts)) => {
            println!("Parsed AST: {:#?}", stmts);
        }
        Err(err) => {
            eprintln!("Parse error: {:?}", err);
        }
    }
}
