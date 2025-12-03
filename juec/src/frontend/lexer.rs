use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub enum Token {
    Indent,
    Dedent,
    Newline,
    Name(String),
    Number(String),
    String(String),
    Symbol(String),
    Other(String),
}

pub struct Lexer {
    lines: Vec<String>,
    indent_stack: Vec<usize>,
    buffer: VecDeque<Token>,
}
//Indentation-aware tokenizer
impl Lexer {
    pub fn new(source: &str) -> Self {
        let lines = source.lines().map(|l| l.to_string()).collect::<Vec<_>>();
        Lexer {
            lines,
            indent_stack: vec![0],
            buffer: VecDeque::new(),
        }
    }

    fn compute_indent(&self, line: &str) -> usize {
        line.chars()
            .take_while(|c| *c == ' ' || *c == '\t')
            .map(|c| if c == '\t' { 4 } else { 1 })
            .sum()
    }

    fn process_line(&mut self, line: &str) {
        let indent = self.compute_indent(line);
        let prev_indent = *self.indent_stack.last().unwrap();

        if indent > prev_indent {
            self.indent_stack.push(indent);
            self.buffer.push_back(Token::Indent);
        } else if indent < prev_indent {
            while indent < *self.indent_stack.last().unwrap() {
                self.indent_stack.pop();
                self.buffer.push_back(Token::Dedent);
            }
        }

        // Skip empty / comment lines
        if line.trim().is_empty() || line.trim_start().starts_with('#') {
            return;
        }

        // Simple tokenization: split by whitespace
        for part in line.trim().split_whitespace() {
            // Detect symbols / operators
            let tok = match part {
                "+" | "-" | "*" | "**" | "/" | "//" | "%" | "==" | "!=" | "<=" | ">=" | "<"
                | ">" | "=" => Token::Symbol(part.to_string()),
                _ if part.chars().all(|c| c.is_numeric()) => Token::Number(part.to_string()),
                _ if part.starts_with('"') || part.starts_with('\'') => {
                    Token::String(part.to_string())
                }
                _ => Token::Name(part.to_string()),
            };
            self.buffer.push_back(tok);
        }

        self.buffer.push_back(Token::Newline);
    }
}

impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(tok) = self.buffer.pop_front() {
            return Some(tok);
        }
        if self.lines.is_empty() {
            // Emit DEDENTs for remaining stack
            while self.indent_stack.len() > 1 {
                self.indent_stack.pop();
                return Some(Token::Dedent);
            }
            return None;
        }
        let line = self.lines.remove(0);
        self.process_line(&line);
        self.buffer.pop_front()
    }
}
