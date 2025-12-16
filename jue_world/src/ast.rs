use crate::error::SourceLocation;
use serde::{Deserialize, Serialize};
/// Abstract Syntax Tree for Jue language
use std::fmt;

/// AST node types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AstNode {
    /// Literal value
    Literal(Literal),

    /// Symbol reference
    Symbol(String),

    /// Variable reference
    Variable(String),

    /// Function call
    Call {
        function: Box<AstNode>,
        arguments: Vec<AstNode>,
        location: SourceLocation,
    },

    /// Lambda expression
    Lambda {
        parameters: Vec<String>,
        body: Box<AstNode>,
        location: SourceLocation,
    },

    /// Let binding
    Let {
        bindings: Vec<(String, AstNode)>,
        body: Box<AstNode>,
        location: SourceLocation,
    },

    /// If expression
    If {
        condition: Box<AstNode>,
        then_branch: Box<AstNode>,
        else_branch: Box<AstNode>,
        location: SourceLocation,
    },

    /// Trust tier annotation
    TrustTier {
        tier: String,
        expression: Box<AstNode>,
        location: SourceLocation,
    },

    /// Capability requirement declaration
    RequireCapability {
        capability: String,
        location: SourceLocation,
    },

    /// Capability check
    HasCapability {
        capability: String,
        location: SourceLocation,
    },

    /// Type signature declaration
    TypeSignature {
        name: String,
        parameters: Vec<Type>,
        return_type: Box<Type>,
        capabilities: Vec<String>,
        location: SourceLocation,
    },

    /// Macro definition
    MacroDefinition {
        name: String,
        parameters: Vec<String>,
        body: Box<AstNode>,
        capabilities: Vec<String>,
        tier: String,
        location: SourceLocation,
    },

    /// Macro expansion
    MacroExpansion {
        name: String,
        arguments: Vec<AstNode>,
        location: SourceLocation,
    },

    /// FFI call
    FfiCall {
        function: String,
        arguments: Vec<AstNode>,
        location: SourceLocation,
    },

    /// List construction
    List {
        elements: Vec<AstNode>,
        location: SourceLocation,
    },

    /// Pair construction
    Cons {
        car: Box<AstNode>,
        cdr: Box<AstNode>,
        location: SourceLocation,
    },
}

/// Literal values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    /// Nil value
    Nil,

    /// Boolean value
    Bool(bool),

    /// Integer value
    Int(i64),

    /// Float value
    Float(f64),

    /// String value
    String(String),
}

/// Type representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    /// Basic type
    Basic(String),

    /// Function type
    Function {
        parameters: Vec<Type>,
        return_type: Box<Type>,
    },

    /// Capability-annotated type
    CapabilityType {
        base_type: Box<Type>,
        capabilities: Vec<String>,
    },

    /// Type variable
    Variable(String),
}

impl fmt::Display for AstNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AstNode::Literal(lit) => write!(f, "{}", lit),
            AstNode::Symbol(s) => write!(f, "'{}", s),
            AstNode::Variable(v) => write!(f, "{}", v),
            AstNode::Call {
                function,
                arguments,
                ..
            } => {
                write!(f, "({}", function)?;
                for arg in arguments {
                    write!(f, " {}", arg)?;
                }
                write!(f, ")")
            }
            AstNode::Lambda {
                parameters, body, ..
            } => {
                write!(f, "(lambda (")?;
                for (i, param) in parameters.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") {})", body)
            }
            AstNode::Let { bindings, body, .. } => {
                write!(f, "(let (")?;
                for (i, (name, value)) in bindings.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "({} {})", name, value)?;
                }
                write!(f, ") {})", body)
            }
            AstNode::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                write!(f, "(if {} {} {})", condition, then_branch, else_branch)
            }
            AstNode::TrustTier {
                tier, expression, ..
            } => {
                write!(f, "({} {})", tier, expression)
            }
            AstNode::RequireCapability { capability, .. } => {
                write!(f, "(require-capability '{} )", capability)
            }
            AstNode::HasCapability { capability, .. } => {
                write!(f, "(has-capability? '{} )", capability)
            }
            AstNode::TypeSignature {
                name,
                parameters,
                return_type,
                ..
            } => {
                write!(f, "(:signature ({}", name)?;
                for param in parameters {
                    write!(f, " {}", param)?;
                }
                write!(f, " -> {})", return_type)
            }
            AstNode::MacroDefinition {
                name, parameters, ..
            } => {
                write!(f, "(defmacro {} (", name)?;
                for (i, param) in parameters.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, ") ...)")
            }
            AstNode::MacroExpansion {
                name, arguments, ..
            } => {
                write!(f, "({}", name)?;
                for arg in arguments {
                    write!(f, " {}", arg)?;
                }
                write!(f, ")")
            }
            AstNode::FfiCall {
                function,
                arguments,
                ..
            } => {
                write!(f, "(ffi-call {} ", function)?;
                for arg in arguments {
                    write!(f, "{} ", arg)?;
                }
                write!(f, ")")
            }
            AstNode::List { elements, .. } => {
                write!(f, "(list")?;
                for elem in elements {
                    write!(f, " {}", elem)?;
                }
                write!(f, ")")
            }
            AstNode::Cons { car, cdr, .. } => {
                write!(f, "(cons {} {})", car, cdr)
            }
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Nil => write!(f, "nil"),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Int(i) => write!(f, "{}", i),
            Literal::Float(fl) => write!(f, "{}", fl),
            Literal::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Basic(name) => write!(f, "{}", name),
            Type::Function {
                parameters,
                return_type,
            } => {
                write!(f, "(")?;
                for (i, param) in parameters.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{}", param)?;
                }
                write!(f, " -> {})", return_type)
            }
            Type::CapabilityType {
                base_type,
                capabilities,
            } => {
                write!(f, "{}", base_type)?;
                if !capabilities.is_empty() {
                    write!(f, " [")?;
                    for (i, cap) in capabilities.iter().enumerate() {
                        if i > 0 {
                            write!(f, " ")?;
                        }
                        write!(f, "{}", cap)?;
                    }
                    write!(f, "]")?;
                }
                Ok(())
            }
            Type::Variable(name) => write!(f, "{}", name),
        }
    }
}

#[cfg(test)]
#[path = "test/ast.rs"]
mod tests;
