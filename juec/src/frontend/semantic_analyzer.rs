// frontend/semantic_analyzer.rs
// Works with current frontend::ast and can traverse full modules. Later you can extend it for type inference, decorator validation, scope nesting, etc.

use crate::frontend::ast::{Expr, Module, Stmt};
use anyhow::Result;

#[derive(Debug)]
pub enum SemanticError {
    UndefinedFunction(String),
    InvalidArguments(String),
}

pub type SemanticResult<T> = Result<T, SemanticError>;

pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    /// Analyze a full frontend AST Module
    pub fn analyze_module(module: &Module) -> SemanticResult<()> {
        let mut env = Environment::new();
        for stmt in &module.body {
            Self::analyze_stmt(stmt, &mut env)?;
        }
        Ok(())
    }

    fn analyze_stmt(stmt: &Stmt, env: &mut Environment) -> SemanticResult<()> {
        match stmt {
            Stmt::Expr(e) => Self::analyze_expr(e, env),
            Stmt::Assign { targets, value } => {
                for t in targets {
                    Self::analyze_expr(t, env)?;
                }
                Self::analyze_expr(value, env)
            }
            Stmt::FuncDef {
                name,
                params: _,
                body,
                decorators: _,
            } => {
                env.define_function(name.clone());
                for stmt in body {
                    Self::analyze_stmt(stmt, env)?;
                }
                Ok(())
            }
            Stmt::ClassDef {
                name,
                body,
                decorators: _,
            } => {
                env.define_class(name.clone());
                for stmt in body {
                    Self::analyze_stmt(stmt, env)?;
                }
                Ok(())
            }
            Stmt::Return(opt) => {
                if let Some(e) = opt {
                    Self::analyze_expr(e, env)?;
                }
                Ok(())
            }
            Stmt::If { test, body, orelse } => {
                Self::analyze_expr(test, env)?;
                for s in body {
                    Self::analyze_stmt(s, env)?;
                }
                for s in orelse {
                    Self::analyze_stmt(s, env)?;
                }
                Ok(())
            }
            Stmt::For {
                target,
                iter,
                body,
                orelse,
            } => {
                Self::analyze_expr(target, env)?;
                Self::analyze_expr(iter, env)?;
                for s in body {
                    Self::analyze_stmt(s, env)?;
                }
                for s in orelse {
                    Self::analyze_stmt(s, env)?;
                }
                Ok(())
            }
            Stmt::While { test, body, orelse } => {
                Self::analyze_expr(test, env)?;
                for s in body {
                    Self::analyze_stmt(s, env)?;
                }
                for s in orelse {
                    Self::analyze_stmt(s, env)?;
                }
                Ok(())
            }
            Stmt::With { items, body } => {
                for (ctx, _alias) in items {
                    Self::analyze_expr(ctx, env)?;
                }
                for s in body {
                    Self::analyze_stmt(s, env)?;
                }
                Ok(())
            }
            Stmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                for s in body {
                    Self::analyze_stmt(s, env)?;
                }
                for (_exc, hbody) in handlers {
                    for s in hbody {
                        Self::analyze_stmt(s, env)?;
                    }
                }
                for s in orelse {
                    Self::analyze_stmt(s, env)?;
                }
                for s in finalbody {
                    Self::analyze_stmt(s, env)?;
                }
                Ok(())
            }
            Stmt::Pass | Stmt::Break | Stmt::Continue => Ok(()),
            Stmt::Raise(opt) => {
                if let Some(e) = opt {
                    Self::analyze_expr(e, env)?;
                }
                Ok(())
            }
            Stmt::AugAssign {
                target,
                op: _,
                value,
            } => {
                Self::analyze_expr(target, env)?;
                Self::analyze_expr(value, env)
            }
        }
    }

    fn analyze_expr(expr: &Expr, env: &mut Environment) -> SemanticResult<()> {
        match expr {
            Expr::Name(n) => {
                if !env.is_defined(n) {
                    Err(SemanticError::UndefinedFunction(n.clone()))
                } else {
                    Ok(())
                }
            }
            Expr::Call { func, args } => {
                Self::analyze_expr(func, env)?;
                for a in args {
                    Self::analyze_expr(a, env)?;
                }
                Ok(())
            }
            Expr::BinOp { left, right, op: _ } => {
                Self::analyze_expr(left, env)?;
                Self::analyze_expr(right, env)
            }
            Expr::UnaryOp { op: _, expr } => Self::analyze_expr(expr, env),
            Expr::Lambda { params, body } => {
                for p in params {
                    env.define_variable(p.clone());
                }
                Self::analyze_expr(body, env)
            }
            Expr::Number(_) | Expr::String(_) | Expr::Bool(_) | Expr::None => Ok(()),
        }
    }
}

/// Very simple environment tracking
struct Environment {
    variables: Vec<String>,
    functions: Vec<String>,
    classes: Vec<String>,
}

impl Environment {
    fn new() -> Self {
        Self {
            variables: vec![],
            functions: vec![],
            classes: vec![],
        }
    }

    fn define_variable(&mut self, name: String) {
        self.variables.push(name);
    }

    fn define_function(&mut self, name: String) {
        self.functions.push(name);
    }

    fn define_class(&mut self, name: String) {
        self.classes.push(name);
    }

    fn is_defined(&self, name: &str) -> bool {
        self.variables.contains(&name.to_string())
            || self.functions.contains(&name.to_string())
            || self.classes.contains(&name.to_string())
    }
}
