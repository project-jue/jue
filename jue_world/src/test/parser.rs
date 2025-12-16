#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AstNode, Literal};
    use crate::error::CompilationError;
    use crate::parser::{parse, Parser, Token};
    use crate::test_timeout::{run_test_with_guard, TestError};
    use std::time::Duration;

    /// Wrapper for parse function that includes timeout protection
    fn parse_with_timeout(source: &str) -> Result<AstNode, CompilationError> {
        let source_owned = source.to_string();
        let result = run_test_with_guard(
            move |guard| {
                // Check for cancellation periodically during parsing
                if guard.check_cancellation() {
                    panic!("Test timed out or exceeded memory limits");
                }
                parse(&source_owned)
            },
            Duration::from_secs(5), // 5 second timeout
            100_000_000,            // 100MB memory limit
        );

        match result {
            Ok(ast) => ast,
            Err(TestError::Timeout) => {
                panic!("Test timed out while parsing: {}", source);
            }
            Err(TestError::MemoryLimitExceeded) => {
                panic!("Test exceeded memory limits while parsing: {}", source);
            }
            Err(TestError::Panic) => {
                panic!("Test panicked while parsing: {}", source);
            }
        }
    }

    #[test]
    fn test_parser_creation() {
        let parser = Parser::new("test".to_string());
        assert_eq!(parser.source, "test");
        assert_eq!(parser.position, 0);
        assert_eq!(parser.line, 1);
        assert_eq!(parser.column, 1);
    }

    #[test]
    fn test_tokenization_simple() {
        let mut parser = Parser::new("(+ 1 2)".to_string());
        let tokens = parser.tokenize().unwrap();

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], Token::OpenParen));
        assert!(matches!(&tokens[1], Token::Symbol(s) if s == "+"));
        assert!(matches!(&tokens[2], Token::Number(n) if n == "1"));
        assert!(matches!(&tokens[3], Token::Number(n) if n == "2"));
        assert!(matches!(tokens[4], Token::CloseParen));
    }

    #[test]
    fn test_tokenization_with_strings() {
        let mut parser = Parser::new("(print \"hello\")".to_string());
        let tokens = parser.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0], Token::OpenParen));
        assert!(matches!(&tokens[1], Token::Symbol(s) if s == "print"));
        assert!(matches!(&tokens[2], Token::String(s) if s == "hello"));
        assert!(matches!(tokens[3], Token::CloseParen));
    }

    #[test]
    fn test_tokenization_with_booleans() {
        let mut parser = Parser::new("(and true false)".to_string());
        let tokens = parser.tokenize().unwrap();

        assert_eq!(tokens.len(), 5);
        assert!(matches!(tokens[0], Token::OpenParen));
        assert!(matches!(&tokens[1], Token::Symbol(s) if s == "and"));
        assert!(matches!(tokens[2], Token::Boolean(true)));
        assert!(matches!(tokens[3], Token::Boolean(false)));
        assert!(matches!(tokens[4], Token::CloseParen));
    }

    #[test]
    fn test_tokenization_with_nil() {
        let mut parser = Parser::new("(list nil)".to_string());
        let tokens = parser.tokenize().unwrap();

        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0], Token::OpenParen));
        assert!(matches!(&tokens[1], Token::Symbol(s) if s == "list"));
        assert!(matches!(tokens[2], Token::Nil));
        assert!(matches!(tokens[3], Token::CloseParen));
    }

    #[test]
    fn test_simple_expression_parsing() {
        let result = parse_with_timeout("42");
        assert!(result.is_ok());
        assert!(matches!(
            result.unwrap(),
            AstNode::Literal(Literal::Int(42))
        ));
    }

    #[test]
    fn test_symbol_parsing() {
        let result = parse_with_timeout("'test");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), AstNode::Symbol(s) if s == "test"));
    }

    #[test]
    fn test_string_parsing() {
        let result = parse_with_timeout("\"hello\"");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), AstNode::Literal(Literal::String(s)) if s == "hello"));
    }

    #[test]
    fn test_boolean_parsing() {
        let result = parse_with_timeout("true");
        assert!(result.is_ok());
        assert!(matches!(
            result.unwrap(),
            AstNode::Literal(Literal::Bool(true))
        ));
    }

    #[test]
    fn test_nil_parsing() {
        let result = parse_with_timeout("nil");
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), AstNode::Literal(Literal::Nil)));
    }

    #[test]
    fn test_simple_call_parsing() {
        let result = parse_with_timeout("(+ 1 2)");
        assert!(result.is_ok());

        match result.unwrap() {
            AstNode::Call {
                function,
                arguments,
                ..
            } => {
                assert!(matches!(*function, AstNode::Symbol(s) if s == "+"));
                assert_eq!(arguments.len(), 2);
                assert!(matches!(arguments[0], AstNode::Literal(Literal::Int(1))));
                assert!(matches!(arguments[1], AstNode::Literal(Literal::Int(2))));
            }
            _ => panic!("Expected Call node"),
        }
    }

    #[test]
    fn test_lambda_parsing() {
        let result = parse_with_timeout("(lambda (x) (+ x 1))");
        assert!(result.is_ok());

        match result.unwrap() {
            AstNode::Lambda {
                parameters, body, ..
            } => {
                assert_eq!(parameters.len(), 1);
                assert_eq!(parameters[0], "x");

                match *body {
                    AstNode::Call {
                        function,
                        arguments,
                        ..
                    } => {
                        assert!(matches!(*function, AstNode::Symbol(s) if s == "+"));
                        assert_eq!(arguments.len(), 2);
                        assert!(matches!(&arguments[0], AstNode::Variable(v) if v == "x"));
                        assert!(matches!(arguments[1], AstNode::Literal(Literal::Int(1))));
                    }
                    _ => panic!("Expected Call node in lambda body"),
                }
            }
            _ => panic!("Expected Lambda node"),
        }
    }

    #[test]
    fn test_let_parsing() {
        let result = parse_with_timeout("(let ((x 5)) (+ x x))");
        println!("Let parsing result: {:?}", result);
        assert!(result.is_ok());

        match result.unwrap() {
            AstNode::Let { bindings, body, .. } => {
                assert_eq!(bindings.len(), 1);
                assert_eq!(bindings[0].0, "x");
                assert!(matches!(bindings[0].1, AstNode::Literal(Literal::Int(5))));

                match *body {
                    AstNode::Call {
                        function,
                        arguments,
                        ..
                    } => {
                        assert!(matches!(*function, AstNode::Symbol(s) if s == "+"));
                        assert_eq!(arguments.len(), 2);
                        assert!(matches!(&arguments[0], AstNode::Variable(v) if v == "x"));
                        assert!(matches!(&arguments[1], AstNode::Variable(v) if v == "x"));
                    }
                    _ => panic!("Expected Call node in let body"),
                }
            }
            _ => panic!("Expected Let node"),
        }
    }

    #[test]
    fn test_if_parsing() {
        let mut parser = Parser::new("(if true \"yes\" \"no\")".to_string());
        let tokens = parser.tokenize().unwrap();
        println!("If tokens: {:?}", tokens);
        let result = parse_with_timeout("(if true \"yes\" \"no\")");
        println!("If parsing result: {:?}", result);
        assert!(result.is_ok());

        match result.unwrap() {
            AstNode::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                assert!(matches!(*condition, AstNode::Literal(Literal::Bool(true))));
                assert!(matches!(*then_branch, AstNode::Literal(Literal::String(s)) if s == "yes"));
                assert!(matches!(*else_branch, AstNode::Literal(Literal::String(s)) if s == "no"));
            }
            _ => panic!("Expected If node"),
        }
    }

    #[test]
    fn test_trust_tier_parsing() {
        let result = parse_with_timeout("(:empirical (+ 1 1))");
        assert!(result.is_ok());

        match result.unwrap() {
            AstNode::TrustTier {
                tier, expression, ..
            } => {
                assert_eq!(tier, ":empirical");

                match *expression {
                    AstNode::Call {
                        function,
                        arguments,
                        ..
                    } => {
                        assert!(matches!(*function, AstNode::Symbol(s) if s == "+"));
                        assert_eq!(arguments.len(), 2);
                        assert!(matches!(arguments[0], AstNode::Literal(Literal::Int(1))));
                        assert!(matches!(arguments[1], AstNode::Literal(Literal::Int(1))));
                    }
                    _ => panic!("Expected Call node in trust tier expression"),
                }
            }
            _ => panic!("Expected TrustTier node"),
        }
    }

    #[test]
    fn test_require_capability_parsing() {
        let result = parse_with_timeout("(require-capability 'io-read-sensor)");
        assert!(result.is_ok());

        match result.unwrap() {
            AstNode::RequireCapability { capability, .. } => {
                assert_eq!(capability, "io-read-sensor");
            }
            _ => panic!("Expected RequireCapability node"),
        }
    }

    #[test]
    fn test_has_capability_parsing() {
        let result = parse_with_timeout("(has-capability? 'macro-hygienic)");
        assert!(result.is_ok());

        match result.unwrap() {
            AstNode::HasCapability { capability, .. } => {
                assert_eq!(capability, "macro-hygienic");
            }
            _ => panic!("Expected HasCapability node"),
        }
    }

    #[test]
    fn test_parse_error() {
        let result = parse_with_timeout("(incomplete");
        assert!(result.is_err());
        match result {
            Err(CompilationError::ParseError { .. }) => assert!(true),
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_nested_expressions() {
        let result = parse_with_timeout("((lambda (x) (+ x 1)) 5)");
        assert!(result.is_ok());

        match result.unwrap() {
            AstNode::Call {
                function,
                arguments,
                ..
            } => {
                assert_eq!(arguments.len(), 1);
                assert!(matches!(arguments[0], AstNode::Literal(Literal::Int(5))));

                match *function {
                    AstNode::Lambda {
                        parameters, body, ..
                    } => {
                        assert_eq!(parameters.len(), 1);
                        assert_eq!(parameters[0], "x");

                        match *body {
                            AstNode::Call {
                                function,
                                arguments,
                                ..
                            } => {
                                assert!(matches!(*function, AstNode::Symbol(s) if s == "+"));
                                assert_eq!(arguments.len(), 2);
                                assert!(matches!(&arguments[0], AstNode::Variable(v) if v == "x"));
                                assert!(matches!(arguments[1], AstNode::Literal(Literal::Int(1))));
                            }
                            _ => panic!("Expected Call node in lambda body"),
                        }
                    }
                    _ => panic!("Expected Lambda node as function"),
                }
            }
            _ => panic!("Expected Call node"),
        }
    }
}
