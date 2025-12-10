/// Core expression types for λ-calculus
/// This module defines the basic expression types: Var, Lam, App
use std::fmt;

/// Core expression enum representing λ-calculus terms
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreExpr {
    /// Variable expression with De Bruijn index
    Var(usize),
    /// Lambda abstraction (function)
    Lam(Box<CoreExpr>),
    /// Function application
    App(Box<CoreExpr>, Box<CoreExpr>),
}

impl fmt::Display for CoreExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreExpr::Var(index) => write!(f, "{}", index),
            CoreExpr::Lam(body) => {
                // For display purposes, we'll use 'x' as the variable name
                // In a real implementation, we might want to track variable names
                // but De Bruijn indices don't require them
                match **body {
                    CoreExpr::App(..) => write!(f, "λx.({})", body),
                    _ => write!(f, "λx.{}", body),
                }
            }
            CoreExpr::App(func, arg) => {
                // Add parentheses for clarity in nested applications
                let func_needs_parens = matches!(**func, CoreExpr::App(..) | CoreExpr::Lam(..));
                let arg_needs_parens = matches!(**arg, CoreExpr::App(..) | CoreExpr::Lam(..));

                if func_needs_parens {
                    write!(f, "({})", func)?;
                } else {
                    write!(f, "{}", func)?;
                }

                write!(f, " ")?;

                if arg_needs_parens {
                    write!(f, "({})", arg)
                } else {
                    write!(f, "{}", arg)
                }
            }
        }
    }
}

/// Helper function to create a variable expression
pub fn var(index: usize) -> CoreExpr {
    CoreExpr::Var(index)
}

/// Helper function to create a lambda abstraction
pub fn lam(body: CoreExpr) -> CoreExpr {
    CoreExpr::Lam(Box::new(body))
}

/// Helper function to create a function application
pub fn app(func: CoreExpr, arg: CoreExpr) -> CoreExpr {
    CoreExpr::App(Box::new(func), Box::new(arg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_creation() {
        let v = var(0);
        assert!(matches!(v, CoreExpr::Var(0)));
    }

    #[test]
    fn test_lam_creation() {
        let l = lam(var(0));
        assert!(matches!(l, CoreExpr::Lam(_)));
        if let CoreExpr::Lam(body) = l {
            assert!(matches!(*body, CoreExpr::Var(0)));
        }
    }

    #[test]
    fn test_app_creation() {
        let identity = lam(var(0));
        let v = var(1);
        let app_expr = app(identity, v);
        assert!(matches!(app_expr, CoreExpr::App(..)));
    }

    #[test]
    fn test_display_var() {
        let v = var(5);
        assert_eq!(format!("{}", v), "5");
    }

    #[test]
    fn test_display_lam() {
        let l = lam(var(0));
        assert_eq!(format!("{}", l), "λx.0");
    }

    #[test]
    fn test_display_app() {
        let identity = lam(var(0));
        let v = var(1);
        let app_expr = app(identity, v);
        assert_eq!(format!("{}", app_expr), "(λx.0) 1");
    }

    #[test]
    fn test_nested_display() {
        let nested = app(lam(app(var(1), var(0))), lam(var(0)));
        // The inner application (1 0) should be in parentheses
        // The lambda body that contains an application should have parentheses
        assert_eq!(format!("{}", nested), "(λx.(1 0)) (λx.0)");
    }
}
