#[cfg(test)]
mod tests {
    use crate::compiler::{compile, CompilationResult};
    use crate::error::CompilationError;
    use crate::TrustTier;
    use physics_world::types::OpCode;

    #[test]
    fn test_recursive_function_compilation() {
        let source = r#"
            (let ((fact (lambda (n)
                          (if (<= n 1)
                              1
                              (* n (fact (- n 1)))))))
              (fact 5))
        "#;

        let result = compile(source, TrustTier::Empirical, 1000, 1024);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded
                assert!(
                    !compilation_result.bytecode.is_empty()
                        || compilation_result.constants.is_empty()
                );
                assert_eq!(compilation_result.sandboxed, false);

                // Check that we have closure creation bytecode
                let has_closure = compilation_result
                    .bytecode
                    .iter()
                    .any(|op| matches!(op, OpCode::MakeClosure(_, _)));

                assert!(
                    has_closure,
                    "Expected MakeClosure opcode in recursive function compilation"
                );
            }
            Err(e) => {
                panic!("Recursive function compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_mutual_recursion_compilation() {
        let source = r#"
            (let ((is-even? (lambda (n)
                              (if (= n 0)
                                  true
                                  (is-odd? (- n 1)))))
                  (is-odd? (lambda (n)
                             (if (= n 0)
                                 false
                                 (is-even? (- n 1))))))
              (is-even? 4))
        "#;

        let result = compile(source, TrustTier::Empirical, 1000, 1024);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded
                assert!(
                    !compilation_result.bytecode.is_empty()
                        || compilation_result.constants.is_empty()
                );

                // Check that we have multiple closures for mutual recursion
                let closure_count = compilation_result
                    .bytecode
                    .iter()
                    .filter(|op| matches!(op, OpCode::MakeClosure(_, _)))
                    .count();

                assert!(
                    closure_count >= 2,
                    "Expected at least 2 closures for mutual recursion"
                );
            }
            Err(e) => {
                panic!("Mutual recursion compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_nested_recursive_functions() {
        let source = r#"
            (let ((outer (lambda (x)
                           (let ((inner (lambda (y)
                                         (if (= y 0)
                                             x
                                             (inner (- y 1))))))
                             (inner x)))))
              (outer 5))
        "#;

        let result = compile(source, TrustTier::Empirical, 1000, 1024);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded
                assert!(
                    !compilation_result.bytecode.is_empty()
                        || compilation_result.constants.is_empty()
                );

                // Check that we have nested closures
                let has_closure = compilation_result
                    .bytecode
                    .iter()
                    .any(|op| matches!(op, OpCode::MakeClosure(_, _)));

                assert!(
                    has_closure,
                    "Expected MakeClosure opcode in nested recursive functions"
                );
            }
            Err(e) => {
                panic!("Nested recursive functions compilation failed: {:?}", e);
            }
        }
    }

    #[test]
    fn test_simple_lambda_in_let() {
        let source = r#"
            (let ((f (lambda (x) (+ x 1))))
              (f 5))
        "#;

        let result = compile(source, TrustTier::Empirical, 1000, 1024);

        match result {
            Ok(compilation_result) => {
                // Verify that compilation succeeded
                assert!(
                    !compilation_result.bytecode.is_empty()
                        || compilation_result.constants.is_empty()
                );

                // Check that we have a closure
                let has_closure = compilation_result
                    .bytecode
                    .iter()
                    .any(|op| matches!(op, OpCode::MakeClosure(_, _)));

                assert!(
                    has_closure,
                    "Expected MakeClosure opcode in lambda compilation"
                );
            }
            Err(e) => {
                panic!("Simple lambda in let compilation failed: {:?}", e);
            }
        }
    }
}
