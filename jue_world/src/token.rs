/// Token type for Jue language
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Symbol(String),
    QuotedSymbol(String),
    String(String),
    Number(String),
    Boolean(bool),
    Nil,
}
