#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{AstNode, Literal};
    use crate::error::CompilationError;
    use crate::parser::{parse, Parser};
    use crate::token::Token;

    #[test]
    fn test_expression_parser_simple() {
        let mut parser = Parser::new("42".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
        assert!(result.is_ok());
        assert!(matches!(
            result.unwrap(),
            AstNode::Literal(Literal::Int(42))
        ));
    }

    #[test]
    fn test_expression_parser_symbol() {
        let mut parser = Parser::new("'test".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), AstNode::Symbol(s) if s == "test"));
    }

    #[test]
    fn test_expression_parser_string() {
        let mut parser = Parser::new("\"hello\"".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), AstNode::Literal(Literal::String(s)) if s == "hello"));
    }

    #[test]
    fn test_expression_parser_boolean() {
        let mut parser = Parser::new("true".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
        assert!(result.is_ok());
        assert!(matches!(
            result.unwrap(),
            AstNode::Literal(Literal::Bool(true))
        ));
    }

    #[test]
    fn test_expression_parser_nil() {
        let mut parser = Parser::new("nil".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
        assert!(result.is_ok());
        assert!(matches!(result.unwrap(), AstNode::Literal(Literal::Nil)));
    }

    #[test]
    fn test_expression_parser_simple_call() {
        let mut parser = Parser::new("(+ 1 2)".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
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
    fn test_expression_parser_lambda() {
        let mut parser = Parser::new("(lambda (x) (+ x 1))".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
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
    fn test_expression_parser_let() {
        let mut parser = Parser::new("(let ((x 5)) (+ x x))".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
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
    fn test_expression_parser_if() {
        let mut parser = Parser::new("(if true \"yes\" \"no\")".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
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
    fn test_expression_parser_trust_tier() {
        let mut parser = Parser::new("(:empirical (+ 1 1))".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
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
    fn test_expression_parser_require_capability() {
        let mut parser = Parser::new("(require-capability 'io-read-sensor)".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
        assert!(result.is_ok());

        match result.unwrap() {
            AstNode::RequireCapability { capability, .. } => {
                assert_eq!(capability, "io-read-sensor");
            }
            _ => panic!("Expected RequireCapability node"),
        }
    }

    #[test]
    fn test_expression_parser_has_capability() {
        let mut parser = Parser::new("(has-capability? 'macro-hygienic)".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
        assert!(result.is_ok());

        match result.unwrap() {
            AstNode::HasCapability { capability, .. } => {
                assert_eq!(capability, "macro-hygienic");
            }
            _ => panic!("Expected HasCapability node"),
        }
    }

    #[test]
    fn test_expression_parser_error() {
        let tokens = vec![];
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
        assert!(result.is_err());
        match result {
            Err(CompilationError::ParseError { .. }) => assert!(true),
            _ => panic!("Expected ParseError"),
        }
    }

    #[test]
    fn test_expression_parser_nested_expressions() {
        let mut parser = Parser::new("((lambda (x) (+ x 1)) 5)".to_string());
        let tokens = parser.tokenize().unwrap();
        let mut expr_parser = ExpressionParser::new(&tokens);
        let result = expr_parser.parse();
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
