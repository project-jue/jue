/// Tokenizer implementation for Jue-World V2.0
///
/// This module converts source code into tokens for parsing.
use crate::token::{SourceLocation, Token};

/// Tokenizer state
pub struct Tokenizer {
    /// Input source code
    input: String,
    /// Current position in input
    position: usize,
    /// Current line number
    line: usize,
    /// Current column number
    column: usize,
    /// Current file name
    file: &'static str,
}

impl Tokenizer {
    /// Create a new tokenizer
    pub fn new(input: String, file: &'static str) -> Self {
        Tokenizer {
            input,
            position: 0,
            line: 1,
            column: 1,
            file,
        }
    }

    /// Peek at the next character without consuming it
    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    /// Consume the next character
    fn consume(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += ch.len_utf8();
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }

    /// Skip whitespace
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }

    /// Tokenize the input
    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            match ch {
                // Skip whitespace
                ch if ch.is_whitespace() => {
                    self.skip_whitespace();
                    continue;
                }

                // Single character tokens
                '(' => tokens.push(Token::OpenParen),
                ')' => tokens.push(Token::CloseParen),
                '[' => tokens.push(Token::LeftBracket),
                ']' => tokens.push(Token::RightBracket),
                '{' => tokens.push(Token::LeftBrace),
                '}' => tokens.push(Token::RightBrace),
                '.' => tokens.push(Token::Dot),
                ',' => tokens.push(Token::Comma),
                ';' => tokens.push(Token::Semicolon),
                ':' => tokens.push(Token::Colon),
                '\'' => tokens.push(Token::Quote),
                '`' => tokens.push(Token::Backtick),
                '+' => tokens.push(Token::Plus),
                '-' => tokens.push(Token::Minus),
                '*' => tokens.push(Token::Star),
                '/' => tokens.push(Token::Slash),
                '%' => tokens.push(Token::Percent),
                '=' => tokens.push(Token::Equals),
                '!' => tokens.push(Token::Bang),
                '<' => tokens.push(Token::Less),
                '>' => tokens.push(Token::Greater),
                '&' => tokens.push(Token::Ampersand),
                '|' => tokens.push(Token::Pipe),
                '^' => tokens.push(Token::Caret),
                '?' => tokens.push(Token::Question),

                // Identifiers and keywords
                ch if ch.is_alphabetic() || ch == '_' => {
                    let start = self.position;
                    while let Some(ch) = self.peek() {
                        if ch.is_alphanumeric() || ch == '_' || ch == '-' || ch == '?' || ch == '!'
                        {
                            self.consume();
                        } else {
                            break;
                        }
                    }
                    let end = self.position;
                    let ident = self.input[start..end].to_string();
                    tokens.push(Token::Symbol(ident));
                }

                // Numbers
                ch if ch.is_ascii_digit() => {
                    let start = self.position;
                    let mut is_float = false;
                    while let Some(ch) = self.peek() {
                        if ch.is_ascii_digit() {
                            self.consume();
                        } else if ch == '.' && !is_float {
                            self.consume();
                            is_float = true;
                        } else {
                            break;
                        }
                    }
                    let end = self.position;
                    let num_str = &self.input[start..end];
                    if is_float {
                        if let Ok(num) = num_str.parse::<f64>() {
                            tokens.push(Token::Number(num));
                        } else {
                            tokens.push(Token::Unknown(ch));
                        }
                    } else {
                        if let Ok(num) = num_str.parse::<i64>() {
                            tokens.push(Token::Number(num as f64));
                        } else {
                            tokens.push(Token::Unknown(ch));
                        }
                    }
                }

                // Strings
                '"' => {
                    self.consume(); // Consume opening quote
                    let start = self.position;
                    while let Some(ch) = self.peek() {
                        if ch == '"' {
                            break;
                        }
                        self.consume();
                    }
                    let end = self.position;
                    let str_content = self.input[start..end].to_string();
                    self.consume(); // Consume closing quote
                    tokens.push(Token::String(str_content));
                }

                // Unknown characters
                _ => {
                    let ch = self.consume().unwrap();
                    tokens.push(Token::Unknown(ch));
                }
            }
        }

        tokens.push(Token::Eof);
        tokens
    }
}
