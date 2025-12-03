use crate::frontend::internal_ast::{JueAST, NodeKind, Stmt};

// Extensibility:
// - Add new built-in functions
// - Type inference and checking
// - Scope tracking (variables, classes, modules)

#[derive(Debug)]
pub enum SemanticError {
    UndefinedFunction(String),
    InvalidArguments(String),
}

pub type SemanticResult<T> = Result<T, SemanticError>;

pub struct SemanticAnalyzer;

impl SemanticAnalyzer {
    //TODO, don't use the common ast anymore, use the frontend ast
    pub fn analyze(ast: &JueAST) -> SemanticResult<()> {
        match ast {
            JueAST::Module { body, .. } => {
                for stmt in body {
                    Self::analyze(stmt)?; // recursively analyze each statement
                }
            }
            JueAST::FunctionDef {
                name, args, body, ..
            } => {
                // analyze arguments and body
                for stmt in args {
                    Self::analyze(stmt)?;
                }
                Self::analyze(body)?;
            }
            JueAST::Call { func, args } => {
                // check function call, etc.
            }
            JueAST::Block(stmts) => {
                for stmt in stmts {
                    Self::analyze(stmt)?;
                }
            }
            JueAST::Assignment { target, value } => {
                Self::analyze(target)?;
                Self::analyze(value)?;
            }
            JueAST::Return { value } => {
                Self::analyze(value)?;
            }
            JueAST::Literal(_) | JueAST::Identifier(_) => {}
            _ => {
                // other nodes
            }
        }
        Ok(())
    }

    pub fn analyze_stmt(stmt: &Stmt) -> SemanticResult<()> {
        match stmt {
            Stmt::Expr(expr) => Self::analyze_expr(expr)?,
            Stmt::Assign { targets, value } => {
                for target in targets {
                    Self::analyze_expr(target)?;
                }
                Self::analyze_expr(value)?;
            }
            Stmt::AugAssign {
                target,
                op: _,
                value,
            } => {
                Self::analyze_expr(target)?;
                Self::analyze_expr(value)?;
            }
            Stmt::Return(expr_opt) => {
                if let Some(expr) = expr_opt {
                    Self::analyze_expr(expr)?;
                }
            }
            Stmt::Pass | Stmt::Break | Stmt::Continue => {}
            Stmt::Raise(expr_opt) => {
                if let Some(expr) = expr_opt {
                    Self::analyze_expr(expr)?;
                }
            }
            Stmt::FuncDef {
                name: _,
                params: _,
                body,
                decorators: _,
            } => {
                for s in body {
                    Self::analyze_stmt(s)?;
                }
            }
            Stmt::ClassDef {
                name: _,
                body,
                decorators: _,
            } => {
                for s in body {
                    Self::analyze_stmt(s)?;
                }
            }
            Stmt::For {
                target,
                iter,
                body,
                orelse,
            } => {
                Self::analyze_expr(target)?;
                Self::analyze_expr(iter)?;
                for s in body {
                    Self::analyze_stmt(s)?;
                }
                for s in orelse {
                    Self::analyze_stmt(s)?;
                }
            }
            Stmt::While { test, body, orelse } => {
                Self::analyze_expr(test)?;
                for s in body {
                    Self::analyze_stmt(s)?;
                }
                for s in orelse {
                    Self::analyze_stmt(s)?;
                }
            }
            Stmt::If { test, body, orelse } => {
                Self::analyze_expr(test)?;
                for s in body {
                    Self::analyze_stmt(s)?;
                }
                for s in orelse {
                    Self::analyze_stmt(s)?;
                }
            }
            Stmt::With { items, body } => {
                for (expr, _alias) in items {
                    Self::analyze_expr(expr)?;
                }
                for s in body {
                    Self::analyze_stmt(s)?;
                }
            }
            Stmt::Try {
                body,
                handlers,
                orelse,
                finalbody,
            } => {
                for s in body {
                    Self::analyze_stmt(s)?;
                }
                for (exc_type, handler_body) in handlers {
                    if let Some(t) = exc_type {
                        Self::analyze_expr(t)?;
                    }
                    for s in handler_body {
                        Self::analyze_stmt(s)?;
                    }
                }
                for s in orelse {
                    Self::analyze_stmt(s)?;
                }
                for s in finalbody {
                    Self::analyze_stmt(s)?;
                }
            }
        }
        Ok(())
    }

    pub fn analyze_expr(expr: &NodeKind) -> SemanticResult<()> {
        match expr {
            Expr::Name(_) | Expr::Number(_) | Expr::String(_) | Expr::Bool(_) | Expr::None => {}
            Expr::BinOp { left, right, op: _ } => {
                Self::analyze_expr(left)?;
                Self::analyze_expr(right)?;
            }
            Expr::UnaryOp { expr, op: _ } => Self::analyze_expr(expr)?,
            Expr::Call { func, args } => {
                // MVP: only allow built-in "print"
                if let Expr::Name(name) = &**func {
                    if name != "print" {
                        return Err(SemanticError::UndefinedFunction(name.clone()));
                    }
                }
                for arg in args {
                    Self::analyze_expr(arg)?;
                }
            }
            Expr::Lambda { params: _, body } => Self::analyze_expr(body)?,
        }
        Ok(())
    }
}
