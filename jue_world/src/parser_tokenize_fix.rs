/// Tokenize source code
fn tokenize(&mut self) -> Result<Vec<Token>, CompilationError> {
    let mut tokens = Vec::new();

    while let Some(c) = self.current_char() {
        match c {
            '(' => {
                tokens.push(Token::OpenParen);
                self.advance();
            }
            ')' => {
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
                tokens.push(Token::Symbol(symbol));
            }
            '"' => {
                self.advance();
                let string = self.read_string()?;
                tokens.push(Token::String(string));
            }
            _ if c.is_digit(10) => {
                let number = self.read_number()?;
                tokens.push(Token::Number(number));
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
                || c == '>'
                || c == '!' =>
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
