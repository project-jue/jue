use crate::frontend::ast::*;
use anyhow::{anyhow, Result};
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser; // required for #[derive(Parser)]

#[derive(Parser)]
#[grammar = "frontend/jue.pest"]
pub struct JueParser;

/// Entry point: parse a full program/module
pub fn parse_program(pairs: Pairs<Rule>) -> Result<Module> {
    let mut body = Vec::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                // program -> file_input ~ EOI (or similar). Be permissive:
                for child in pair.into_inner() {
                    match child.as_rule() {
                        Rule::file_input => {
                            // file_input may contain: NEWLINE | COMMENT | statement | simple_stmt | compound_stmt
                            for item in child.into_inner() {
                                let items = flatten_top_level_item(item)?;
                                body.extend(items);
                            }
                        }
                        // Some grammars might emit simple_stmt/compound_stmt directly under program,
                        // so handle them defensively.
                        Rule::simple_stmt | Rule::compound_stmt | Rule::statement => {
                            let items = flatten_top_level_item(child)?;
                            body.extend(items);
                        }
                        Rule::COMMENT | Rule::NEWLINE | Rule::EOI => {
                            // ignore
                        }
                        other => {
                            return Err(anyhow!("Unexpected top-level token: {:?}", other));
                        }
                    }
                }
            }
            Rule::EOI => {}
            other => return Err(anyhow!("Unexpected top-level token: {:?}", other)),
        }
    }

    Ok(Module { body })
}

/// Normalize a top-level item (whatever the grammar emitted) into Vec<Stmt>.
/// Accepts: statement, simple_stmt, compound_stmt, COMMENT, NEWLINE
fn flatten_top_level_item(pair: Pair<Rule>) -> Result<Vec<Stmt>> {
    let mut out = Vec::new();

    match pair.as_rule() {
        Rule::file_input => {
            for inner in pair.into_inner() {
                let mut items = flatten_top_level_item(inner)?;
                out.append(&mut items);
            }
        }

        Rule::statement => {
            // statement may contain simple_stmt or compound_stmt
            for inner in pair.into_inner() {
                match inner.as_rule() {
                    Rule::simple_stmt => {
                        for small in inner.into_inner() {
                            if let Some(stmt) = parse_small_stmt(small)? {
                                out.push(stmt);
                            }
                        }
                    }
                    Rule::compound_stmt => {
                        if let Some(stmt) = parse_compound_stmt(inner)? {
                            out.push(stmt);
                        }
                    }
                    Rule::COMMENT | Rule::NEWLINE => {
                        // skip
                    }
                    other => {
                        return Err(anyhow!("Unexpected token inside statement: {:?}", other));
                    }
                }
            }
        }

        Rule::simple_stmt => {
            // simple_stmt may be sequence of small_stmt separated by ';' (or grammar variant)
            for small in pair.into_inner() {
                if let Some(stmt) = parse_small_stmt(small)? {
                    out.push(stmt);
                }
            }
        }

        Rule::compound_stmt => {
            if let Some(stmt) = parse_compound_stmt(pair)? {
                out.push(stmt);
            }
        }

        Rule::COMMENT | Rule::NEWLINE => {
            // nothing to do
        }

        // Defensive: sometimes file_input may directly contain small_stmt (depending on grammar)
        Rule::exprlist
        | Rule::augassign_stmt
        | Rule::pass_stmt
        | Rule::return_stmt
        | Rule::break_stmt
        | Rule::continue_stmt
        | Rule::raise_stmt => {
            // treat these as small_stmt-like items (wrap via parse_small_stmt)
            if let Some(stmt) = parse_small_stmt(pair)? {
                out.push(stmt);
            }
        }

        other => {
            return Err(anyhow!(
                "Unexpected top-level token inside file_input: {:?}",
                other
            ));
        }
    }

    Ok(out)
}

/// Parse a small statement (expr, assignment, return, pass, etc.)
fn parse_small_stmt(pair: Pair<Rule>) -> Result<Option<Stmt>> {
    // Some grammars wrap small_stmt in another node, so be flexible:
    let rule = pair.as_rule();
    let inner_pair = match rule {
        Rule::simple_stmt => {
            // if someone passed a simple_stmt accidentally, unwrap its contents
            pair.into_inner().next()
        }
        Rule::statement => pair.into_inner().next(),
        _ => Some(pair),
    };

    if inner_pair.is_none() {
        return Ok(None);
    }

    let inner = inner_pair.unwrap();
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

        // Add additional small_stmt kinds here (import, global, assert, del, yield, etc.)
        other => {
            // If it's already an expr (depending on grammar shape) try to parse it as expr
            match other {
                Rule::expr => {
                    let e = parse_expr(inner)?;
                    Ok(Some(Stmt::Expr(e)))
                }
                Rule::NAME | Rule::NUMBER | Rule::STRING | Rule::lambda_expr => {
                    // these may appear as atoms — delegate to parse_expr
                    let e = parse_expr(inner)?;
                    Ok(Some(Stmt::Expr(e)))
                }
                _ => Ok(None),
            }
        }
    }
}

/// Parse a compound statement (if, for, while, funcdef, classdef)
fn parse_compound_stmt(pair: Pair<Rule>) -> Result<Option<Stmt>> {
    // Accept either compound_stmt node or a wrapper
    let mut inner_iter = pair.into_inner();
    let first = inner_iter
        .next()
        .unwrap_or_else(|| panic!("compound_stmt missing inner"));
    match first.as_rule() {
        Rule::if_stmt => parse_if(first).map(Some),
        Rule::for_stmt => parse_for(first).map(Some),
        Rule::while_stmt => parse_while(first).map(Some),
        Rule::funcdef => parse_funcdef(first).map(Some),
        Rule::classdef => parse_classdef(first).map(Some),
        other => Err(anyhow!("Unexpected compound_stmt: {:?}", other)),
    }
}

/// Parse if/elif/else
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

/// Parse decorators list (decorators -> multiple decorator)
fn parse_decorators(pair: Pair<Rule>) -> Result<Vec<String>> {
    let mut decorators = Vec::new();
    for decorator_pair in pair.into_inner() {
        // decorator structure: "@" dotted_name ("(" arglist? ")")? NEWLINE
        // We'll stringify the inner dotted_name for now
        for inner in decorator_pair.into_inner() {
            if inner.as_rule() == Rule::dotted_name {
                decorators.push(inner.as_str().to_string());
            }
        }
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
    // first is NAME
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
                    .unwrap_or_default();
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

/// Parse a suite (block). Reuse flattening to collect statements
fn parse_suite(pair: Pair<Rule>) -> Result<Vec<Stmt>> {
    let mut stmts = Vec::new();
    for stmt_pair in pair.into_inner() {
        // Each item might be statement/simple_stmt/compound_stmt/file_input etc.
        let mut items = flatten_top_level_item(stmt_pair)?;
        stmts.append(&mut items);
    }
    Ok(stmts)
}

/// Parse expressions (kept largely as your prior implementation)
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

/// Convenience function to parse Jue source code directly
pub fn parse_jue(source: &str) -> Result<Module> {
    let pairs =
        JueParser::parse(Rule::program, source).map_err(|e| anyhow!("Parse error: {}", e))?;
    parse_program(pairs)
}
