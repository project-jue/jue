use juec::frontend::nom_parser;
use nom_locate::LocatedSpan;

type Span<'a> = LocatedSpan<&'a str>;

#[test]
fn test_basic_parsing() {
    // Test simple expression - need to wrap in a statement context
    let code = "x = 42";
    let span = Span::new(code);
    let result = nom_parser::parse_program(span);
    assert!(result.is_ok());

    // Test assignment
    let code = "y = 100";
    let span = Span::new(code);
    let result = nom_parser::parse_program(span);
    assert!(result.is_ok());

    // Test function definition
    let code = r#"
def hello():
    return 1 + 2 * 3
"#;
    let span = Span::new(code);
    let result = nom_parser::parse_program(span);
    assert!(result.is_ok());

    // Test return statement
    let code = "return 42";
    let span = Span::new(code);
    let result = nom_parser::parse_program(span);
    assert!(result.is_ok());
}

#[test]
fn test_expression_parsing() {
    // Test arithmetic expressions within assignment context
    let code = "result = 1 + 2 * 3";
    let span = Span::new(code);
    let result = nom_parser::parse_program(span);
    assert!(result.is_ok());

    let code = "result = (1 + 2) * 3";
    let span = Span::new(code);
    let result = nom_parser::parse_program(span);
    assert!(result.is_ok());
}

#[test]
fn test_error_handling() {
    // Test invalid syntax
    let code = "invalid syntax !@#";
    let span = Span::new(code);
    let result = nom_parser::parse_program(span);
    assert!(result.is_err());

    // Test incomplete expression
    let code = "1 +";
    let span = Span::new(code);
    let result = nom_parser::parse_program(span);
    assert!(result.is_err());
}
