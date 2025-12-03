use crate::frontend::ast::*;
use anyhow::{anyhow, Result};
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser; // required for #[derive(Parser)]

#[derive(Parser)]
#[grammar = "frontend/jue.pest"]
pub struct JueParser;

// No import needed for Rule — it’s generated automatically by Pest

// Use Rule from Pest-generated module
//use crate::frontend::jue::Rule;

/// Entry point: parse the file/module
pub fn parse_program(pairs: Pairs<Rule>) -> Result<Module> {
    let mut body = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                for stmt_pair in pair.into_inner() {
                    if let Some(stmt) = parse_statement(stmt_pair)? {
                        body.push(stmt);
                    }
                }
            }
            Rule::EOI => {}
            _ => return Err(anyhow!("Unexpected top-level token: {:?}", pair.as_rule())),
        }
    }

    Ok(Module { body })
}

/// Parse a single statement
fn parse_statement(pair: Pair<Rule>) -> Result<Option<Stmt>> {
    match pair.as_rule() {
        Rule::simple_stmt => {
            for inner in pair.into_inner() {
                if let Some(stmt) = parse_simple_stmt(inner)? {
                    return Ok(Some(stmt));
                }
            }
            Ok(None)
        }
        Rule::compound_stmt => parse_compound_stmt(pair).map(Some),
        Rule::NEWLINE => Ok(None),
        _ => Err(anyhow!("Unexpected statement rule: {:?}", pair.as_rule())),
    }
}

/// Parse simple statements
fn parse_simple_stmt(pair: Pair<Rule>) -> Result<Option<Stmt>> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::exprlist => {
            // parse_expr returned Result<Expr>, so collect gives Vec<Expr>
            let mut exprs = inner
                .into_inner()
                .map(parse_expr)
                .collect::<Result<Vec<_>>>()?;

            if exprs.len() == 1 {
                // single expression statement
                let e = exprs.pop().unwrap();
                Ok(Some(Stmt::Expr(e)))
            } else {
                // assignment: a, b = value  (targets are all but last)
                let value = exprs.pop().unwrap(); // owned Expr
                let targets = exprs; // remaining owned Exprs
                Ok(Some(Stmt::Assign { targets, value }))
            }
        }

        Rule::augassign_stmt => {
            let mut inner_pairs = inner.into_inner();
            let target = parse_expr(inner_pairs.next().unwrap())?;
            let op = inner_pairs.next().unwrap().as_str().to_string();
            let value = parse_expr(inner_pairs.next().unwrap())?;
            Ok(Some(Stmt::AugAssign { target, op, value }))
        }
        Rule::return_stmt => {
            let mut inner_pairs = inner.into_inner();
            let expr = inner_pairs.next().map(parse_expr).transpose()?;
            Ok(Some(Stmt::Return(expr)))
        }
        Rule::pass_stmt => Ok(Some(Stmt::Pass)),
        Rule::break_stmt => Ok(Some(Stmt::Break)),
        Rule::continue_stmt => Ok(Some(Stmt::Continue)),
        Rule::raise_stmt => {
            let mut inner_pairs = inner.into_inner();
            let expr = inner_pairs.next().map(parse_expr).transpose()?;
            Ok(Some(Stmt::Raise(expr)))
        }
        _ => Ok(None), // extend for other simple statements
    }
}

/// Parse compound statements
/// No functions with decoratyors yet
fn parse_compound_stmt(pair: Pair<Rule>) -> Result<Stmt> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::if_stmt => parse_if(inner),
        Rule::for_stmt => parse_for(inner),
        Rule::while_stmt => parse_while(inner),
        Rule::funcdef => parse_funcdef(inner),
        Rule::classdef => parse_classdef(inner),
        _ => Err(anyhow!("Unexpected compound_stmt: {:?}", inner.as_rule())),
    }
}

/// Parse if/elif/else statement
fn parse_if(pair: Pair<Rule>) -> Result<Stmt> {
    let mut inner = pair.into_inner();
    let test = parse_expr(inner.next().unwrap())?;
    let body = parse_suite(inner.next().unwrap())?;
    let mut orelse = Vec::new();

    while let Some(next_pair) = inner.next() {
        match next_pair.as_rule() {
            Rule::elif_clause => {
                let mut elif_inner = next_pair.into_inner();
                let elif_test = parse_expr(elif_inner.next().unwrap())?;
                let elif_body = parse_suite(elif_inner.next().unwrap())?;
                orelse.push(Stmt::If {
                    test: elif_test,
                    body: elif_body,
                    orelse: vec![],
                });
            }
            Rule::else_clause => {
                orelse = parse_suite(next_pair.into_inner().next().unwrap())?;
            }
            _ => {}
        }
    }

    Ok(Stmt::If { test, body, orelse })
}

/// Parse for statement
fn parse_for(pair: Pair<Rule>) -> Result<Stmt> {
    let mut inner = pair.into_inner();
    let target = parse_expr(inner.next().unwrap())?;
    let iter = parse_expr(inner.next().unwrap())?;
    let body = parse_suite(inner.next().unwrap())?;
    let orelse = if let Some(else_pair) = inner.next() {
        parse_suite(else_pair)?
    } else {
        vec![]
    };
    Ok(Stmt::For {
        target,
        iter,
        body,
        orelse,
    })
}

/// Parse while statement
fn parse_while(pair: Pair<Rule>) -> Result<Stmt> {
    let mut inner = pair.into_inner();
    let test = parse_expr(inner.next().unwrap())?;
    let body = parse_suite(inner.next().unwrap())?;
    let orelse = if let Some(else_pair) = inner.next() {
        parse_suite(else_pair)?
    } else {
        vec![]
    };
    Ok(Stmt::While { test, body, orelse })
}

/// Parse decorators
fn parse_decorators(pair: Pair<Rule>) -> Result<Vec<String>> {
    let mut decorators = Vec::new();
    for decorator_pair in pair.into_inner() {
        let inner = decorator_pair.into_inner().next().unwrap();
        decorators.push(inner.as_str().to_string());
    }
    Ok(decorators)
}

/// Parse function definition
fn parse_funcdef(pair: Pair<Rule>) -> Result<Stmt> {
    let mut inner = pair.into_inner();
    let mut decorators = Vec::new();
    let mut first = inner.next().unwrap();
    if first.as_rule() == Rule::decorators {
        decorators = parse_decorators(first)?;
        first = inner.next().unwrap();
    }
    let name = first.as_str().to_string();
    let params = parse_parameters(inner.next().unwrap())?;
    let body = parse_suite(inner.next().unwrap())?;
    Ok(Stmt::FuncDef {
        name,
        params,
        body,
        decorators,
    })
}

/// Parse class definition
fn parse_classdef(pair: Pair<Rule>) -> Result<Stmt> {
    let mut inner = pair.into_inner();
    let mut decorators = Vec::new();
    let mut first = inner.next().unwrap();
    if first.as_rule() == Rule::decorators {
        decorators = parse_decorators(first)?;
        first = inner.next().unwrap();
    }
    let name = first.as_str().to_string();
    let body = parse_suite(inner.next().unwrap())?;
    Ok(Stmt::ClassDef {
        name,
        body,
        decorators,
    })
}

/// Parse parameters
fn parse_parameters(pair: Pair<Rule>) -> Result<Vec<Param>> {
    let mut params = Vec::new();
    for p in pair.into_inner() {
        match p.as_rule() {
            Rule::tfpdef => {
                let mut inner = p.into_inner();
                let name = inner.next().unwrap().as_str().to_string();
                let default = inner.next().map(parse_expr).transpose()?;
                params.push(Param {
                    name,
                    default,
                    kind: ParamKind::Positional,
                });
            }
            Rule::star_param => {
                let mut inner = p.into_inner();
                let name = inner
                    .next()
                    .map(|x| x.as_str().to_string())
                    .unwrap_or(String::new());
                params.push(Param {
                    name,
                    default: None,
                    kind: ParamKind::Star,
                });
            }
            Rule::double_star_param => {
                let name = p.into_inner().next().unwrap().as_str().to_string();
                params.push(Param {
                    name,
                    default: None,
                    kind: ParamKind::DoubleStar,
                });
            }
            _ => {}
        }
    }
    Ok(params)
}

/// Parse a suite (block)
fn parse_suite(pair: Pair<Rule>) -> Result<Vec<Stmt>> {
    let mut stmts = Vec::new();
    for stmt_pair in pair.into_inner() {
        if let Some(stmt) = parse_statement(stmt_pair)? {
            stmts.push(stmt);
        }
    }
    Ok(stmts)
}

/// Parse expressions
fn parse_expr(pair: Pair<Rule>) -> Result<Expr> {
    match pair.as_rule() {
        Rule::NAME => Ok(Expr::Name(pair.as_str().to_string())),
        Rule::NUMBER => Ok(Expr::Number(pair.as_str().to_string())),
        Rule::STRING => Ok(Expr::String(pair.as_str().to_string())),
        Rule::TRUE => Ok(Expr::Bool(true)),
        Rule::FALSE => Ok(Expr::Bool(false)),
        Rule::NONE => Ok(Expr::None),
        Rule::lambda_expr => {
            let mut inner = pair.into_inner();
            let mut params = Vec::new();
            let first = inner.next().unwrap();
            if first.as_rule() == Rule::expr {
                return Ok(Expr::Lambda {
                    params,
                    body: Box::new(parse_expr(first)?),
                });
            } else {
                params.push(first.as_str().to_string());
                let body_expr = inner.next().unwrap();
                Ok(Expr::Lambda {
                    params,
                    body: Box::new(parse_expr(body_expr)?),
                })
            }
        }
        Rule::binary_expr => {
            let mut inner = pair.into_inner();
            let left = parse_expr(inner.next().unwrap())?;
            let op = inner.next().unwrap().as_str().to_string();
            let right = parse_expr(inner.next().unwrap())?;
            Ok(Expr::BinOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        }
        Rule::unary_expr => {
            let mut inner = pair.into_inner();
            let op = inner.next().unwrap().as_str().to_string();
            let expr = parse_expr(inner.next().unwrap())?;
            Ok(Expr::UnaryOp {
                op,
                expr: Box::new(expr),
            })
        }
        _ => Err(anyhow!(
            "Expression not yet implemented: {:?}",
            pair.as_rule()
        )),
    }
}
