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
        /// Function expression being called
        function: Box<AstNode>,
        /// Arguments passed to the function
        arguments: Vec<AstNode>,
        /// Source location for error reporting
        location: SourceLocation,
    },

    /// Lambda expression
    Lambda {
        /// Parameter names
        parameters: Vec<String>,
        /// Function body
        body: Box<AstNode>,
        /// Source location for error reporting
        location: SourceLocation,
    },

    /// Let binding
    Let {
        /// Variable bindings (name, value pairs)
        bindings: Vec<(String, AstNode)>,
        /// Body expression
        body: Box<AstNode>,
        /// Source location for error reporting
        location: SourceLocation,
    },

    /// If expression
    If {
        /// Condition expression
        condition: Box<AstNode>,
        /// Expression to evaluate if condition is true
        then_branch: Box<AstNode>,
        /// Expression to evaluate if condition is false
        else_branch: Box<AstNode>,
        /// Source location for error reporting
        location: SourceLocation,
    },

    /// Trust tier annotation
    TrustTier {
        /// Trust tier level
        tier: String,
        /// Expression being annotated
        expression: Box<AstNode>,
        /// Source location for error reporting
        location: SourceLocation,
    },

    /// Capability requirement declaration
    RequireCapability {
        /// Capability being required
        capability: String,
        /// Source location for error reporting
        location: SourceLocation,
    },

    /// Capability check
    HasCapability {
        /// Capability being checked
        capability: String,
        /// Source location for error reporting
        location: SourceLocation,
    },

    /// Type signature declaration
    TypeSignature {
        /// Function name
        name: String,
        /// Parameter types
        parameters: Vec<Type>,
        /// Return type
        return_type: Box<Type>,
        /// Required capabilities
        capabilities: Vec<String>,
        /// Source location for error reporting
        location: SourceLocation,
    },

    /// Macro definition
    MacroDefinition {
        /// Macro name
        name: String,
        /// Parameter names
        parameters: Vec<String>,
        /// Macro body
        body: Box<AstNode>,
        /// Required capabilities
        capabilities: Vec<String>,
        /// Trust tier for the macro
        tier: String,
        /// Source location for error reporting
        location: SourceLocation,
    },

    /// Macro expansion
    MacroExpansion {
        /// Macro name being expanded
        name: String,
        /// Arguments passed to macro
        arguments: Vec<AstNode>,
        /// Source location for error reporting
        location: SourceLocation,
    },

    /// FFI call
    FfiCall {
        /// Foreign function name
        function: String,
        /// Arguments passed to FFI function
        arguments: Vec<AstNode>,
        /// Source location for error reporting
        location: SourceLocation,
    },

    /// List construction
    List {
        /// List elements
        elements: Vec<AstNode>,
        /// Source location for error reporting
        location: SourceLocation,
    },

    /// Pair construction
    Cons {
        /// First element of pair (car)
        car: Box<AstNode>,
        /// Second element of pair (cdr)
        cdr: Box<AstNode>,
        /// Source location for error reporting
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
        /// Parameter types
        parameters: Vec<Type>,
        /// Return type
        return_type: Box<Type>,
    },

    /// Capability-annotated type
    CapabilityType {
        /// Base type being annotated
        base_type: Box<Type>,
        /// Required capabilities
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
