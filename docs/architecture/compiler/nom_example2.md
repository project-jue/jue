// Cargo.toml dependencies (Nom 8 compatible):
// nom = "8"
// nom_locate = "4"
// thiserror = "1"

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, multispace0, space1},
    combinator::{map, map_res},
    multi::many0,
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult,
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
    map(
        take_while1(|c: char| c.is_ascii_alphabetic() || c == '_'),
        |s: Span| s.fragment().to_string(),
    )(input)
}

fn parse_number(input: Span) -> IResult<Span, Expr> {
    map_res(digit1, |s: Span| {
        i64::from_str(s.fragment()).map(Expr::Number)
    })(input)
}

// ----------------------
// Expression parser
// ----------------------

// Very simple left-to-right binary expressions for demo purposes
fn parse_expr(input: Span) -> IResult<Span, Expr> {
    let (input, left) = parse_term(input)?;
    parse_expr_tail(left, input)
}

fn parse_expr_tail(left: Expr, input: Span) -> IResult<Span, Expr> {
    let op_parser = alt((char('+'), char('-')));
    if let Ok((input, op)) = op_parser(input) {
        let (input, right) = parse_term(input)?;
        let bin_op = match op {
            '+' => BinOp::Add,
            '-' => BinOp::Sub,
            _ => unreachable!(),
        };
        let new_left = Expr::BinOp(Box::new(left), bin_op, Box::new(right));
        parse_expr_tail(new_left, input)
    } else {
        Ok((input, left))
    }
}

fn parse_term(input: Span) -> IResult<Span, Expr> {
    let (input, left) = parse_factor(input)?;
    parse_term_tail(left, input)
}

fn parse_term_tail(left: Expr, input: Span) -> IResult<Span, Expr> {
    let op_parser = alt((char('*'), char('/')));
    if let Ok((input, op)) = op_parser(input) {
        let (input, right) = parse_factor(input)?;
        let bin_op = match op {
            '*' => BinOp::Mul,
            '/' => BinOp::Div,
            _ => unreachable!(),
        };
        let new_left = Expr::BinOp(Box::new(left), bin_op, Box::new(right));
        parse_term_tail(new_left, input)
    } else {
        Ok((input, left))
    }
}

fn parse_factor(input: Span) -> IResult<Span, Expr> {
    alt((
        parse_number,
        map(parse_ident, Expr::Ident),
        delimited(char('('), parse_expr, char(')')),
    ))(input)
}

// ----------------------
// Statement parsers
// ----------------------

fn parse_return(input: Span) -> IResult<Span, Stmt> {
    let (input, _) = tag("return")(input)?;
    let (input, _) = space1(input)?;
    let (input, expr) = parse_expr(input)?;
    Ok((input, Stmt::Return(expr)))
}

fn parse_assign(input: Span) -> IResult<Span, Stmt> {
    let (input, ident) = parse_ident(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = char('=')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, expr) = parse_expr(input)?;
    Ok((input, Stmt::Assign(ident, expr)))
}

fn parse_stmt(input: Span) -> IResult<Span, Stmt> {
    alt((parse_return, parse_assign))(input)
}

// ----------------------
// Function parser
// ----------------------

fn parse_function(input: Span) -> IResult<Span, Stmt> {
    let (input, _) = tag("def")(input)?;
    let (input, _) = space1(input)?;
    let (input, name) = parse_ident(input)?;
    let (input, _) = char('(')(input)?;
    let (input, params) = parse_param_list(input)?;
    let (input, _) = char(')')(input)?;
    let (input, _) = char(':')(input)?;
    let (input, _) = multispace0(input)?;
    let (input, body) = many0(parse_stmt)(input)?;
    Ok((input, Stmt::FunctionDef(name, params, body)))
}

fn parse_param_list(input: Span) -> IResult<Span, Vec<String>> {
    let mut params = Vec::new();
    let mut input = input;
    loop {
        match parse_ident(input) {
            Ok((next_input, ident)) => {
                params.push(ident);
                input = next_input;
                if let Ok((next_input, _)) = char::<Span, nom::error::Error<Span>>(',')(input) {
                    input = next_input;
                } else {
                    break;
                }
            }
            Err(_) => break,
        }
    }
    Ok((input, params))
}

// ----------------------
// Top-level program parser
// ----------------------

fn parse_program(input: Span) -> IResult<Span, Vec<Stmt>> {
    many0(alt((parse_function, parse_stmt)))(input)
}

// ----------------------
// Example usage
// ----------------------

fn main() {
    let code = r#"
def add(x, y):
    return x + y

a = 1
b = 2
c = a * b
"#;

    let span = Span::new(code);
    let result = parse_program(span);

    match result {
        Ok((_remaining, stmts)) => {
            println!("Parsed AST: {:#?}", stmts);
        }
        Err(err) => {
            println!("Parse error: {:?}", err);
        }
    }
}
