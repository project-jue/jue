use jue_world::parsing::parser::parse;

fn main() {
    let source = r#"
        (let ((fact (lambda (n)
                      (if (<= n 1)
                          1
                          (* n (fact (- n 1)))))))
          (fact 3))
    "#;

    match parse(source) {
        Ok(ast) => {
            println!("AST: {:?}", ast);
        }
        Err(e) => {
            println!("Parse error: {}", e);
        }
    }
}
