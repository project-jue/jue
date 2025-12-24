/// Token definitions for Jue-World V2.0
///
/// This module defines the token types used by the lexer and parser.

/// Source location for tokens
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub file: &'static str,
}

impl SourceLocation {
    /// Create a generated source location (for synthetic tokens)
    pub fn generated() -> Self {
        SourceLocation {
            line: 0,
            column: 0,
            file: "generated",
        }
    }
}

/// Token type enumeration
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Left parenthesis
    OpenParen,
    /// Right parenthesis
    CloseParen,
    /// Left bracket
    LeftBracket,
    /// Right bracket
    RightBracket,
    /// Left brace
    LeftBrace,
    /// Right brace
    RightBrace,
    /// Dot
    Dot,
    /// Comma
    Comma,
    /// Semicolon
    Semicolon,
    /// Colon
    Colon,
    /// Quote
    Quote,
    /// Backtick
    Backtick,
    /// Plus
    Plus,
    /// Minus
    Minus,
    /// Star
    Star,
    /// Slash
    Slash,
    /// Percent
    Percent,
    /// Equals
    Equals,
    /// Bang
    Bang,
    /// Less
    Less,
    /// Greater
    Greater,
    /// Ampersand
    Ampersand,
    /// Pipe
    Pipe,
    /// Caret
    Caret,
    /// Question
    Question,
    /// Identifier
    Identifier(String, SourceLocation),
    /// Symbol token
    Symbol(String),
    /// Quoted symbol token
    QuotedSymbol(String),
    /// String literal
    String(String),
    /// Number literal
    Number(f64),
    /// Boolean literal
    Boolean(bool),
    /// Nil literal
    Nil,
    /// End of file
    Eof,
    /// Unknown token
    Unknown(char),
}

impl Token {
    /// Get the source location of the token
    pub fn location(&self) -> SourceLocation {
        // For now, return a default location since the new token format doesn't include locations
        // This should be updated to include proper location tracking
        SourceLocation::generated()
    }

    /// Check if token is an identifier
    pub fn is_identifier(&self) -> bool {
        matches!(self, Token::Identifier(..))
    }

    /// Get identifier name if token is an identifier
    pub fn identifier_name(&self) -> Option<&str> {
        if let Token::Identifier(name, _) = self {
            Some(name)
        } else {
            None
        }
    }
}
