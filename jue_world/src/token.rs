/// Token type for Jue language
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Opening parenthesis
    OpenParen,
    /// Closing parenthesis
    CloseParen,
    /// Symbol identifier
    Symbol(String),
    /// Quoted symbol
    QuotedSymbol(String),
    /// String literal
    String(String),
    /// Numeric literal
    Number(String),
    /// Boolean literal
    Boolean(bool),
    /// Nil value
    Nil,
}
