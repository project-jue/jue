/// Expression parser for Jue language
use crate::ast::{AstNode, Literal};
use crate::error::{CompilationError, SourceLocation};
use crate::token::Token;

/// Expression parser
pub struct ExpressionParser<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> ExpressionParser<'a> {
    /// Create a new expression parser
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Safely get current token
    fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    /// Check if at end of tokens
    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }

    /// Parse an expression from tokens
    pub fn parse(&mut self) -> Result<AstNode, CompilationError> {
        if self.is_at_end() {
            return Err(CompilationError::ParseError {
                message: "Unexpected end of input".to_string(),
                location: SourceLocation::default(),
            });
        }

        match self.current_token() {
            Some(Token::OpenParen) => self.parse_list(),
            Some(Token::Symbol(s)) => {
                let result = self.parse_variable(s);
                self.advance();
                result
            }
            Some(Token::QuotedSymbol(s)) => {
                let result = self.parse_symbol(s);
                self.advance();
                result
            }
            Some(Token::String(s)) => {
                let result = self.parse_string(s);
                self.advance();
                result
            }
            Some(Token::Number(n)) => {
                let result = self.parse_number_float(*n);
                self.advance();
                result
            }
            Some(Token::Boolean(b)) => {
                let result = self.parse_boolean(*b);
                self.advance();
                result
            }
            Some(Token::Nil) => {
                let result = self.parse_nil();
                self.advance();
                result
            }
            Some(token) => Err(CompilationError::ParseError {
                message: format!("Unexpected token: {:?}", token),
                location: SourceLocation::default(),
            }),
            None => Err(CompilationError::ParseError {
                message: "Unexpected end of input".to_string(),
                location: SourceLocation::default(),
            }),
        }
    }

    fn parse_list(&mut self) -> Result<AstNode, CompilationError> {
        self.advance(); // Skip '('

        if self.is_at_end() {
            return Err(CompilationError::ParseError {
                message: "Unexpected end of list".to_string(),
                location: SourceLocation::default(),
            });
        }

        match self.current_token() {
            Some(Token::Symbol(s)) if s.starts_with(':') => self.parse_trust_tier(),
            Some(Token::Symbol(s)) if s == "lambda" => self.parse_lambda(),
            Some(Token::Symbol(s)) if s == "let" => self.parse_let(),
            Some(Token::Symbol(s)) if s == "if" => self.parse_if(),
            Some(Token::Symbol(s)) if s == "require-capability" => self.parse_require_capability(),
            Some(Token::Symbol(s)) if s == "has-capability?" => self.parse_has_capability(),
            Some(Token::Symbol(s)) if s == "defmacro" => self.parse_macro_definition(),
            Some(Token::Symbol(s)) if s == "comptime-eval" => self.parse_comptime_eval(),
            Some(Token::Symbol(s)) if s == "ffi-call" => self.parse_ffi_call(),
            Some(Token::OpenParen) => self.parse_function_call_with_expression(),
            _ => self.parse_function_call_with_symbol(),
        }
    }

    fn parse_trust_tier(&mut self) -> Result<AstNode, CompilationError> {
        let tier = match self.current_token() {
            Some(Token::Symbol(s)) => s.clone(),
            _ => {
                return Err(CompilationError::ParseError {
                    message: "Expected trust tier".to_string(),
                    location: SourceLocation::default(),
                })
            }
        };

        self.advance();

        let expression = self.parse()?;

        // Skip closing paren
        if let Some(Token::CloseParen) = self.current_token() {
            self.advance();
        } else {
            return Err(CompilationError::ParseError {
                message: "Expected closing parenthesis".to_string(),
                location: SourceLocation::default(),
            });
        }

        Ok(AstNode::TrustTier {
            tier,
            expression: Box::new(expression),
            location: SourceLocation::default(),
        })
    }

    fn parse_lambda(&mut self) -> Result<AstNode, CompilationError> {
        self.advance(); // Skip 'lambda'

        let parameters = self.parse_parameter_list()?;
        let body = self.parse()?;

        // Skip closing paren
        if let Some(Token::CloseParen) = self.current_token() {
            self.advance();
        } else {
            return Err(CompilationError::ParseError {
                message: "Expected closing parenthesis".to_string(),
                location: SourceLocation::default(),
            });
        }

        Ok(AstNode::Lambda {
            parameters,
            body: Box::new(body),
            location: SourceLocation::default(),
        })
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<String>, CompilationError> {
        if self.is_at_end() {
            return Err(CompilationError::ParseError {
                message: "Expected parameter list".to_string(),
                location: SourceLocation::default(),
            });
        }

        match self.current_token() {
            Some(Token::OpenParen) => {
                self.advance();
                let mut parameters = Vec::new();

                while !self.is_at_end() {
                    match self.current_token() {
                        Some(Token::CloseParen) => {
                            self.advance();
                            break;
                        }
                        Some(Token::Symbol(s)) => {
                            parameters.push(s.clone());
                            self.advance();
                        }
                        _ => {
                            return Err(CompilationError::ParseError {
                                message: "Expected parameter name".to_string(),
                                location: SourceLocation::default(),
                            })
                        }
                    }
                }

                Ok(parameters)
            }
            Some(Token::Symbol(s)) => {
                let param = s.clone();
                self.advance();
                Ok(vec![param])
            }
            _ => Err(CompilationError::ParseError {
                message: "Expected parameter list".to_string(),
                location: SourceLocation::default(),
            }),
        }
    }

    fn parse_let(&mut self) -> Result<AstNode, CompilationError> {
        self.advance(); // Skip 'let'

        let bindings = self.parse_bindings()?;
        let body = self.parse()?;

        // Skip closing paren
        if let Some(Token::CloseParen) = self.current_token() {
            self.advance();
        } else {
            return Err(CompilationError::ParseError {
                message: "Expected closing parenthesis".to_string(),
                location: SourceLocation::default(),
            });
        }

        Ok(AstNode::Let {
            bindings,
            body: Box::new(body),
            location: SourceLocation::default(),
        })
    }

    fn parse_bindings(&mut self) -> Result<Vec<(String, AstNode)>, CompilationError> {
        if self.is_at_end() {
            return Err(CompilationError::ParseError {
                message: "Expected bindings".to_string(),
                location: SourceLocation::default(),
            });
        }

        match self.current_token() {
            Some(Token::OpenParen) => {
                self.advance();
                let mut bindings = Vec::new();

                while !self.is_at_end() {
                    match self.current_token() {
                        Some(Token::CloseParen) => {
                            self.advance();
                            break;
                        }
                        Some(Token::OpenParen) => {
                            self.advance();
                            let name = match self.current_token() {
                                Some(Token::Symbol(s)) => s.clone(),
                                _ => {
                                    return Err(CompilationError::ParseError {
                                        message: "Expected binding name".to_string(),
                                        location: SourceLocation::default(),
                                    })
                                }
                            };
                            self.advance();

                            let value = self.parse()?;

                            bindings.push((name, value));

                            // Skip closing paren
                            if let Some(Token::CloseParen) = self.current_token() {
                                self.advance();
                            } else {
                                return Err(CompilationError::ParseError {
                                    message: "Expected closing parenthesis".to_string(),
                                    location: SourceLocation::default(),
                                });
                            }
                        }
                        _ => {
                            return Err(CompilationError::ParseError {
                                message: "Expected binding".to_string(),
                                location: SourceLocation::default(),
                            })
                        }
                    }
                }

                Ok(bindings)
            }
            _ => Err(CompilationError::ParseError {
                message: "Expected bindings list".to_string(),
                location: SourceLocation::default(),
            }),
        }
    }

    fn parse_if(&mut self) -> Result<AstNode, CompilationError> {
        self.advance(); // Skip 'if'

        let condition = self.parse()?;
        let then_branch = self.parse()?;
        let else_branch = self.parse()?;

        // Skip closing paren
        if let Some(Token::CloseParen) = self.current_token() {
            self.advance();
        } else {
            return Err(CompilationError::ParseError {
                message: "Expected closing parenthesis".to_string(),
                location: SourceLocation::default(),
            });
        }

        Ok(AstNode::If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: Box::new(else_branch),
            location: SourceLocation::default(),
        })
    }

    fn parse_require_capability(&mut self) -> Result<AstNode, CompilationError> {
        self.advance(); // Skip 'require-capability'

        let capability = match self.current_token() {
            Some(Token::Symbol(s)) => s.clone(),
            Some(Token::QuotedSymbol(s)) => s.clone(),
            _ => {
                return Err(CompilationError::ParseError {
                    message: "Expected capability name".to_string(),
                    location: SourceLocation::default(),
                })
            }
        };

        self.advance();

        // Skip closing paren
        if let Some(Token::CloseParen) = self.current_token() {
            self.advance();
        } else {
            return Err(CompilationError::ParseError {
                message: "Expected closing parenthesis".to_string(),
                location: SourceLocation::default(),
            });
        }

        Ok(AstNode::RequireCapability {
            capability,
            location: SourceLocation::default(),
        })
    }

    fn parse_has_capability(&mut self) -> Result<AstNode, CompilationError> {
        self.advance(); // Skip 'has-capability?'

        let capability = match self.current_token() {
            Some(Token::Symbol(s)) => s.clone(),
            Some(Token::QuotedSymbol(s)) => s.clone(),
            _ => {
                return Err(CompilationError::ParseError {
                    message: "Expected capability name".to_string(),
                    location: SourceLocation::default(),
                })
            }
        };

        self.advance();

        // Skip closing paren
        if let Some(Token::CloseParen) = self.current_token() {
            self.advance();
        } else {
            return Err(CompilationError::ParseError {
                message: "Expected closing parenthesis".to_string(),
                location: SourceLocation::default(),
            });
        }

        Ok(AstNode::HasCapability {
            capability,
            location: SourceLocation::default(),
        })
    }

    fn parse_macro_definition(&mut self) -> Result<AstNode, CompilationError> {
        self.advance(); // Skip 'defmacro'

        let name = match self.current_token() {
            Some(Token::Symbol(s)) => s.clone(),
            _ => {
                return Err(CompilationError::ParseError {
                    message: "Expected macro name".to_string(),
                    location: SourceLocation::default(),
                })
            }
        };

        self.advance();

        let parameters = self.parse_parameter_list()?;
        let body = self.parse()?;

        // Skip closing paren
        if let Some(Token::CloseParen) = self.current_token() {
            self.advance();
        } else {
            return Err(CompilationError::ParseError {
                message: "Expected closing parenthesis".to_string(),
                location: SourceLocation::default(),
            });
        }

        // TODO: Parse macro metadata (capabilities, tier, etc.)
        Ok(AstNode::MacroDefinition {
            name,
            parameters,
            body: Box::new(body),
            capabilities: vec![],       // TODO: Parse capabilities
            tier: "formal".to_string(), // TODO: Parse tier
            location: SourceLocation::default(),
        })
    }

    fn parse_comptime_eval(&mut self) -> Result<AstNode, CompilationError> {
        self.advance(); // Skip 'comptime-eval'

        let expression = self.parse()?;

        // Skip closing paren
        if let Some(Token::CloseParen) = self.current_token() {
            self.advance();
        } else {
            return Err(CompilationError::ParseError {
                message: "Expected closing parenthesis".to_string(),
                location: SourceLocation::default(),
            });
        }

        // TODO: Implement comptime evaluation
        Ok(expression)
    }

    fn parse_ffi_call(&mut self) -> Result<AstNode, CompilationError> {
        self.advance(); // Skip 'ffi-call'

        // Parse function name
        let function = match self.current_token() {
            Some(Token::Symbol(s)) => s.clone(),
            Some(Token::QuotedSymbol(s)) => s.clone(),
            _ => {
                return Err(CompilationError::ParseError {
                    message: "Expected FFI function name".to_string(),
                    location: SourceLocation::default(),
                })
            }
        };

        self.advance();

        // Parse arguments
        let mut arguments = Vec::new();
        while !self.is_at_end() {
            match self.current_token() {
                Some(Token::CloseParen) => {
                    self.advance();
                    break;
                }
                _ => {
                    let arg = self.parse()?;
                    arguments.push(arg);
                }
            }
        }

        Ok(AstNode::FfiCall {
            function,
            arguments,
            location: SourceLocation::default(),
        })
    }

    fn parse_function_call_with_symbol(&mut self) -> Result<AstNode, CompilationError> {
        // Parse function name as a symbol
        let function_name = match self.current_token() {
            Some(Token::Symbol(s)) => s.clone(),
            _ => {
                return Err(CompilationError::ParseError {
                    message: "Expected function name".to_string(),
                    location: SourceLocation::default(),
                })
            }
        };
        self.advance();

        let mut arguments = Vec::new();

        while !self.is_at_end() {
            match self.current_token() {
                Some(Token::CloseParen) => {
                    self.advance();

                    // FIXED: Check if this is a variable (valid identifier) or symbol (operator)
                    // Exclude single-character operators from being treated as variables
                    let single_char_ops = ["+", "-", "*", "/", "%", "<", ">", "=", "!"];

                    if single_char_ops.contains(&function_name.as_str()) {
                        // Single-character operators should always be treated as symbols
                        return Ok(AstNode::Call {
                            function: Box::new(AstNode::Symbol(function_name)),
                            arguments,
                            location: SourceLocation::default(),
                        });
                    } else if function_name.chars().all(|c| {
                        c.is_alphanumeric()
                            || c == '-'
                            || c == '?'
                            || c == '!'
                            || c == '_'
                            || c == ':'
                    }) {
                        // Multi-character names with valid identifier characters are variables
                        return Ok(AstNode::Call {
                            function: Box::new(AstNode::Variable(function_name)),
                            arguments,
                            location: SourceLocation::default(),
                        });
                    } else {
                        // Everything else is treated as a symbol (built-in operator)
                        return Ok(AstNode::Call {
                            function: Box::new(AstNode::Symbol(function_name)),
                            arguments,
                            location: SourceLocation::default(),
                        });
                    }
                }
                _ => {
                    let arg = self.parse()?;
                    arguments.push(arg);
                }
            }
        }

        // If we reach here, we didn't find a closing paren
        Err(CompilationError::ParseError {
            message: "Expected closing parenthesis".to_string(),
            location: SourceLocation::default(),
        })
    }

    fn parse_function_call_with_expression(&mut self) -> Result<AstNode, CompilationError> {
        // Parse function as any expression
        let function = self.parse()?;

        let mut arguments = Vec::new();

        while !self.is_at_end() {
            match self.current_token() {
                Some(Token::CloseParen) => {
                    self.advance();
                    return Ok(AstNode::Call {
                        function: Box::new(function),
                        arguments,
                        location: SourceLocation::default(),
                    });
                }
                _ => {
                    let arg = self.parse()?;
                    arguments.push(arg);
                }
            }
        }

        // If we reach here, we didn't find a closing paren
        Err(CompilationError::ParseError {
            message: "Expected closing parenthesis".to_string(),
            location: SourceLocation::default(),
        })
    }

    fn parse_symbol(&self, symbol: &str) -> Result<AstNode, CompilationError> {
        Ok(AstNode::Symbol(symbol.to_string()))
    }

    fn parse_variable(&self, variable: &str) -> Result<AstNode, CompilationError> {
        Ok(AstNode::Variable(variable.to_string()))
    }

    fn parse_string(&self, string: &str) -> Result<AstNode, CompilationError> {
        Ok(AstNode::Literal(Literal::String(string.to_string())))
    }

    fn parse_number(&self, _number: &str) -> Result<AstNode, CompilationError> {
        // This method is no longer used - parse_number_float handles Token::Number(f64)
        Err(CompilationError::ParseError {
            message: "Internal error: parse_number called with string".to_string(),
            location: SourceLocation::default(),
        })
    }

    /// Parse a number from f64 token value
    fn parse_number_float(&self, number: f64) -> Result<AstNode, CompilationError> {
        if number.fract() != 0.0 {
            Ok(AstNode::Literal(Literal::Float(number)))
        } else {
            Ok(AstNode::Literal(Literal::Int(number as i64)))
        }
    }

    fn parse_boolean(&self, value: bool) -> Result<AstNode, CompilationError> {
        Ok(AstNode::Literal(Literal::Bool(value)))
    }

    fn parse_nil(&self) -> Result<AstNode, CompilationError> {
        Ok(AstNode::Literal(Literal::Nil))
    }

    /// Advance to next token safely
    fn advance(&mut self) {
        self.position += 1;
    }
}
