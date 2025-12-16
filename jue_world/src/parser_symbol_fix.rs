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
