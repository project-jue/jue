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
            let remaining = &bytes[cursor + func_len..];
            if remaining.is_empty() {
                return Err(ParseError::IncompleteData);
            }
            let arg = deserialize_core_expr(remaining)?;
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
            let remaining = &bytes[cursor + first_len..];
            if remaining.is_empty() {
                return Err(ParseError::IncompleteData);
            }
            let second = deserialize_core_expr(remaining)?;
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
                    write!(f, "({})", second)?
                } else {
                    write!(f, "{}", second)?
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
#[path = "test/core_expr_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "test/core_expr_serialization_tests.rs"]
mod serialization_tests;
