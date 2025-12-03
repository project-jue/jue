use crate::frontend::ast::*;
use anyhow::{anyhow, Result};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "frontend/jue.pest"]
pub struct PythonParser;

/// Top-level parser
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
            other => return Err(anyhow!("Unexpected top-level token: {:?}", other)),
        }
    }

    Ok(Module { body })
}

/// Distinguish between simple and compound statements
fn parse_statement(pair: Pair<Rule>) -> Result<Option<Stmt>> {
    match pair.as_rule() {
        Rule::simple_stmt => {
            let inner = pair.into_inner().next().unwrap();
            parse_simple_stmt(inner)
        }
        Rule::compound_stmt => {
            let stmt = parse_compound_stmt(pair)?;
            Ok(Some(stmt))
        }
        Rule::NEWLINE => Ok(None),
        _ => Err(anyhow!("Unexpected statement: {:?}", pair.as_rule())),
    }
}

/// Parse simple statements
fn parse_simple_stmt(pair: Pair<Rule>) -> Result<Option<Stmt>> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::exprlist => {
            let mut exprs = inner
                .into_inner()
                .map(parse_expr)
                .collect::<Result<Vec<_>>>()?;

            if exprs.len() == 1 {
                Ok(Some(Stmt::Expr(exprs.pop().unwrap())))
            } else {
                let value = exprs.pop().unwrap();
                let targets = exprs;
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

        _ => Ok(None),
    }
}

/// Parse compound statements
fn parse_compound_stmt(pair: Pair<Rule>) -> Result<Stmt> {
    let inner = pair.into_inner().next().unwrap();
    match inner.as_rule() {
        Rule::if_stmt => parse_if(inner),
        Rule::for_stmt => parse_for(inner),
        Rule::while_stmt => parse_while(inner),
        Rule::funcdef => parse_funcdef(inner),
        Rule::classdef => parse_classdef(inner),
        Rule::with_stmt => parse_with(inner),
        Rule::try_stmt => parse_try(inner),
        _ => Err(anyhow!("Unexpected compound_stmt: {:?}", inner.as_rule())),
    }
}

/// Placeholder expression parser
fn parse_expr(pair: Pair<Rule>) -> Result<Expr> {
    // TODO: implement full expression parsing
    Ok(Expr::Name(pair.as_str().to_string()))
}

/// Stubs for compound statement parsing
fn parse_if(pair: Pair<Rule>) -> Result<Stmt> {
    Ok(Stmt::Pass)
}
fn parse_for(pair: Pair<Rule>) -> Result<Stmt> {
    Ok(Stmt::Pass)
}
fn parse_while(pair: Pair<Rule>) -> Result<Stmt> {
    Ok(Stmt::Pass)
}
fn parse_funcdef(pair: Pair<Rule>) -> Result<Stmt> {
    Ok(Stmt::Pass)
}
fn parse_classdef(pair: Pair<Rule>) -> Result<Stmt> {
    Ok(Stmt::Pass)
}
fn parse_with(pair: Pair<Rule>) -> Result<Stmt> {
    Ok(Stmt::Pass)
}
fn parse_try(pair: Pair<Rule>) -> Result<Stmt> {
    Ok(Stmt::Pass)
}
