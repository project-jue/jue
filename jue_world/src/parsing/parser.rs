/// Parser for Jue language with capability syntax
use crate::ast::AstNode;
use crate::error::{CompilationError, SourceLocation};
use crate::expression_parser::ExpressionParser;
use crate::test_timeout::ParserGuard;
use crate::token::Token;

/// Jue parser
pub struct Parser {
    /// Source code
    pub source: String,

    /// Current position
    pub position: usize,

    /// Current line
    pub line: usize,

    /// Current column
    pub column: usize,

    /// Resource guard for preventing OOM issues
    resource_guard: ParserGuard,
}

impl Parser {
    /// Create a new parser
    pub fn new(source: String) -> Self {
        Self {
            source,
            position: 0,
            line: 1,
            column: 1,
            resource_guard: ParserGuard::new(1000, 10000), // Max depth 1000, max tokens 10000
        }
    }

    /// Get current source location
    pub fn current_location(&self) -> SourceLocation {
        SourceLocation {
            line: self.line,
            column: self.column,
            offset: self.position,
        }
    }

    /// Parse Jue source code
    pub fn parse(&mut self) -> Result<AstNode, CompilationError> {
        let tokens = self.tokenize()?;
        self.resource_guard.add_tokens(tokens.len()).map_err(|e| {
            CompilationError::ParserResourceLimit(crate::error::ParserError {
                message: format!("Token limit exceeded: {}", e),
                location: self.current_location(),
            })
        })?;
        self.parse_expression(&tokens)
    }

    /// Get current character safely
    fn current_char(&self) -> Option<char> {
        self.source.chars().nth(self.position)
    }

    /// Tokenize source code
    fn tokenize(&mut self) -> Result<Vec<Token>, CompilationError> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current_char() {
            match c {
                '(' => {
                    self.resource_guard.enter_scope().map_err(|e| {
                        CompilationError::ParserResourceLimit(crate::error::ParserError {
                            message: format!("Max depth exceeded: {}", e),
                            location: self.current_location(),
                        })
                    })?;
                    tokens.push(Token::OpenParen);
                    self.advance();
                }
                ')' => {
                    self.resource_guard.exit_scope();
                    tokens.push(Token::CloseParen);
                    self.advance();
                }
                ' ' | '\t' | '\n' | '\r' => {
                    self.skip_whitespace();
                }
                ';' => {
                    self.skip_comment();
                }
                '\'' => {
                    self.advance();
                    let symbol = self.read_symbol()?;
                    tokens.push(Token::QuotedSymbol(symbol));
                }
                '"' => {
                    self.advance();
                    let string = self.read_string()?;
                    tokens.push(Token::String(string));
                }
                _ if c.is_digit(10) => {
                    let number = self.read_number()?;
                    // Parse as f64 for Token::Number
                    if let Ok(num) = number.parse::<f64>() {
                        tokens.push(Token::Number(num));
                    } else {
                        return Err(CompilationError::ParseError {
                            message: format!("Invalid number: {}", number),
                            location: self.current_location(),
                        });
                    }
                }
                _ if c.is_alphabetic()
                    || c == ':'
                    || c == '?'
                    || c == '+'
                    || c == '-'
                    || c == '*'
                    || c == '/'
                    || c == '='
                    || c == '<'
                    || c == '>' =>
                {
                    let symbol = self.read_symbol()?;
                    tokens.push(match symbol.as_str() {
                        "true" => Token::Boolean(true),
                        "false" => Token::Boolean(false),
                        "nil" => Token::Nil,
                        _ => Token::Symbol(symbol),
                    });
                }
                _ => {
                    return Err(CompilationError::ParseError {
                        message: format!("Unexpected character: '{}'", c),
                        location: self.current_location(),
                    });
                }
            }
        }

        Ok(tokens)
    }

    /// Advance to next character
    fn advance(&mut self) {
        if let Some(c) = self.current_char() {
            self.position += 1;
            self.column += 1;

            if c == '\n' {
                self.line += 1;
                self.column = 1;
            }
        }
    }

    /// Skip whitespace
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
                self.advance();
            } else {
                break;
            }
        }
    }

    /// Skip comment
    fn skip_comment(&mut self) {
        while let Some(c) = self.current_char() {
            if c == '\n' {
                self.advance();
                break;
            } else {
                self.advance();
            }
        }
    }

    /// Read symbol
    fn read_symbol(&mut self) -> Result<String, CompilationError> {
        let mut symbol = String::new();

        while let Some(c) = self.current_char() {
            if c.is_alphanumeric()
                || c == '-'
                || c == '?'
                || c == '!'
                || c == ':'
                || c == '+'
                || c == '*'
                || c == '/'
                || c == '='
                || c == '<'
                || c == '>'
            {
                symbol.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if symbol.is_empty() {
            return Err(CompilationError::ParseError {
                message: "Expected symbol".to_string(),
                location: self.current_location(),
            });
        }

        Ok(symbol)
    }

    /// Read string
    fn read_string(&mut self) -> Result<String, CompilationError> {
        let mut string = String::new();

        while let Some(c) = self.current_char() {
            if c == '"' {
                self.advance();
                return Ok(string);
            } else if c == '\\' {
                self.advance();
                if let Some(escape) = self.current_char() {
                    match escape {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '"' => string.push('"'),
                        '\\' => string.push('\\'),
                        _ => {
                            return Err(CompilationError::ParseError {
                                message: format!("Invalid escape sequence: \\{}", escape),
                                location: self.current_location(),
                            })
                        }
                    }
                    self.advance();
                } else {
                    return Err(CompilationError::ParseError {
                        message: "Unterminated string".to_string(),
                        location: self.current_location(),
                    });
                }
            } else {
                string.push(c);
                self.advance();
            }
        }

        Err(CompilationError::ParseError {
            message: "Unterminated string".to_string(),
            location: self.current_location(),
        })
    }

    /// Read number
    fn read_number(&mut self) -> Result<String, CompilationError> {
        let mut number = String::new();

        while let Some(c) = self.current_char() {
            if c.is_digit(10) || c == '.' || c == '-' {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if number.is_empty() {
            return Err(CompilationError::ParseError {
                message: "Expected number".to_string(),
                location: self.current_location(),
            });
        }

        Ok(number)
    }

    /// Parse expression from tokens
    fn parse_expression(&self, tokens: &[Token]) -> Result<AstNode, CompilationError> {
        let mut parser = ExpressionParser::new(tokens);
        parser.parse()
    }
}

/// Parse Jue source code
pub fn parse(source: &str) -> Result<AstNode, CompilationError> {
    let mut parser = Parser::new(source.to_string());
    parser.parse()
}
