/// Core expression types for λ-calculus
/// This module defines the basic expression types: Var, Lam, App
use std::fmt;

/// Core expression enum representing λ-calculus terms
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CoreExpr {
    /// Variable expression with De Bruijn index
    Var(usize),
    /// Lambda abstraction (function)
    Lam(Box<CoreExpr>),
    /// Function application
    App(Box<CoreExpr>, Box<CoreExpr>),
    Nat(u64),
    Pair(Box<CoreExpr>, Box<CoreExpr>),
}

/// Binary serialization format for CoreExpr
/// Format specification:
/// - Little-endian encoding
/// - Var(n): [0x01, n as u64]
/// - Lam(body): [0x02, body_bytes...]
/// - App(f, a): [0x03, f_bytes..., a_bytes...]
/// - Nat(n): [0x04, n as u64]
/// - Pair(f, s): [0x05, f_bytes..., s_bytes...]

pub fn serialize_core_expr(expr: &CoreExpr) -> Vec<u8> {
    let mut bytes = Vec::new();
    match expr {
        CoreExpr::Var(index) => {
            bytes.push(0x01);
            bytes.extend_from_slice(&(*index as u64).to_le_bytes());
        }
        CoreExpr::Lam(body) => {
            bytes.push(0x02);
            bytes.extend_from_slice(&serialize_core_expr(body));
        }
        CoreExpr::App(func, arg) => {
            bytes.push(0x03);
            bytes.extend_from_slice(&serialize_core_expr(func));
            bytes.extend_from_slice(&serialize_core_expr(arg));
        }
        CoreExpr::Nat(n) => {
            bytes.push(0x04);
            bytes.extend_from_slice(&n.to_le_bytes());
        }
        CoreExpr::Pair(first, second) => {
            bytes.push(0x05);
            bytes.extend_from_slice(&serialize_core_expr(first));
            bytes.extend_from_slice(&serialize_core_expr(second));
        }
    }
    bytes
}

pub fn deserialize_core_expr(bytes: &[u8]) -> Result<CoreExpr, ParseError> {
    if bytes.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let mut cursor = 0;
    let tag = bytes[cursor];
    cursor += 1;

    match tag {
        0x01 => {
            // Var
            if cursor + 8 > bytes.len() {
                return Err(ParseError::IncompleteData);
            }
            let index_bytes = &bytes[cursor..cursor + 8];
            let index = u64::from_le_bytes(index_bytes.try_into().unwrap());
            Ok(CoreExpr::Var(index as usize))
        }
        0x02 => {
            // Lam
            let body = deserialize_core_expr(&bytes[cursor..])?;
            Ok(CoreExpr::Lam(Box::new(body)))
        }
        0x03 => {
            // App
            let func = deserialize_core_expr(&bytes[cursor..])?;
            let func_serialized = serialize_core_expr(&func);
            let func_len = func_serialized.len();
            let arg = deserialize_core_expr(&bytes[cursor + func_len..])?;
            Ok(CoreExpr::App(Box::new(func), Box::new(arg)))
        }
        0x04 => {
            // Nat
            if cursor + 8 > bytes.len() {
                return Err(ParseError::IncompleteData);
            }
            let n_bytes = &bytes[cursor..cursor + 8];
            let n = u64::from_le_bytes(n_bytes.try_into().unwrap());
            Ok(CoreExpr::Nat(n))
        }
        0x05 => {
            // Pair
            let first = deserialize_core_expr(&bytes[cursor..])?;
            let first_serialized = serialize_core_expr(&first);
            let first_len = first_serialized.len();
            let second = deserialize_core_expr(&bytes[cursor + first_len..])?;
            Ok(CoreExpr::Pair(Box::new(first), Box::new(second)))
        }
        _ => Err(ParseError::InvalidTag(tag)),
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    IncompleteData,
    InvalidTag(u8),
    Overflow,
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
                let arg_needs_parens = matches!(**arg, CoreExpr::App(..));

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
            CoreExpr::Nat(n) => write!(f, "{}", n),
            CoreExpr::Pair(first, second) => {
                // Add parentheses only for applications to avoid over-parenthesizing
                let first_needs_parens = matches!(**first, CoreExpr::App(..));
                let second_needs_parens = matches!(**second, CoreExpr::App(..));

                write!(f, "(")?;

                if first_needs_parens {
                    write!(f, "({})", first)?;
                } else {
                    write!(f, "{}", first)?;
                }

                write!(f, ", ")?;

                if second_needs_parens {
                    write!(f, "({})", second)?;
                } else {
                    write!(f, "{}", second)?;
                }

                write!(f, ")")
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

/// Helper function to create a natural number expression
pub fn nat(value: u64) -> CoreExpr {
    CoreExpr::Nat(value)
}

/// Helper function to create a pair expression
pub fn pair(first: CoreExpr, second: CoreExpr) -> CoreExpr {
    CoreExpr::Pair(Box::new(first), Box::new(second))
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
        // Updated expectation to match the new display logic
        assert_eq!(format!("{}", nested), "(λx.(1 0)) λx.0");
    }

    #[test]
    fn test_nat_creation() {
        let n = nat(42);
        assert!(matches!(n, CoreExpr::Nat(42)));
    }

    #[test]
    fn test_nat_display() {
        let n = nat(42);
        assert_eq!(format!("{}", n), "42");
    }

    #[test]
    fn test_pair_creation() {
        let p = pair(var(0), var(1));
        assert!(matches!(p, CoreExpr::Pair(..)));
        if let CoreExpr::Pair(first, second) = p {
            assert!(matches!(*first, CoreExpr::Var(0)));
            assert!(matches!(*second, CoreExpr::Var(1)));
        }
    }

    #[test]
    fn test_pair_display() {
        let p = pair(var(0), var(1));
        assert_eq!(format!("{}", p), "(0, 1)");
    }

    #[test]
    fn test_nested_pair_display() {
        let inner = pair(var(0), var(1));
        let outer = pair(inner, var(2));
        assert_eq!(format!("{}", outer), "((0, 1), 2)");
    }

    #[test]
    fn test_complex_expression_with_nat_and_pair() {
        let expr = app(lam(pair(var(0), nat(5))), nat(10));
        assert_eq!(format!("{}", expr), "(λx.(0, 5)) 10");
    }
}

#[cfg(test)]
mod serialization_tests {
    use super::*;

    #[test]
    fn test_var_serialization() {
        let expr = var(42);
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_lam_serialization() {
        let expr = lam(var(0));
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_app_serialization() {
        let expr = app(lam(var(0)), var(1));
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_nat_serialization() {
        let expr = nat(12345);
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_pair_serialization() {
        let expr = pair(var(0), var(1));
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_complex_serialization() {
        let expr = app(lam(pair(var(0), nat(5))), nat(10));
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let expr = app(lam(var(0)), var(42));
        let serialized = serialize_core_expr(&expr);
        let deserialized = deserialize_core_expr(&serialized).unwrap();
        assert_eq!(expr, deserialized);
    }

    #[test]
    fn test_empty_input_error() {
        let result = deserialize_core_expr(&[]);
        assert!(matches!(result, Err(ParseError::EmptyInput)));
    }

    #[test]
    fn test_incomplete_data_error() {
        // Incomplete Var - only tag, no data
        let incomplete_var = vec![0x01];
        let result = deserialize_core_expr(&incomplete_var);
        assert!(matches!(result, Err(ParseError::IncompleteData)));

        // Incomplete Nat - only tag, no data
        let incomplete_nat = vec![0x04];
        let result = deserialize_core_expr(&incomplete_nat);
        assert!(matches!(result, Err(ParseError::IncompleteData)));
    }

    #[test]
    fn test_invalid_tag_error() {
        let invalid_tag = vec![0xFF];
        let result = deserialize_core_expr(&invalid_tag);
        assert!(matches!(result, Err(ParseError::InvalidTag(0xFF))));
    }
}
