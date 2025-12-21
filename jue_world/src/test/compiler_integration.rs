use crate::compiler::{compile, CompilationResult, TrustTier};
use crate::error::CompilationError;
use crate::parser::parse;

#[test]
fn test_compile_hello_world() {
    let source = r#"
        (println "Hello, World!")
    "#;

    let result = compile(source, TrustTier::Empirical, 1000, 1024);

    match result {
        Ok(compilation_result) => {
            // Verify that compilation succeeded
            assert!(
                !compilation_result.bytecode.is_empty() || compilation_result.constants.is_empty()
            );
            assert_eq!(compilation_result.sandboxed, false); // Empirical tier should not be sandboxed
            assert_eq!(compilation_result.step_limit, 1000);
            assert_eq!(compilation_result.memory_limit, 1024);
        }
        Err(e) => {
            panic!("Compilation failed: {:?}", e);
        }
    }
}

#[test]
fn test_compile_arithmetic() {
    let source = r#"
        (+ 1 2 3)
        (- 10 5)
        (* 2 3)
        (/ 10 2)
    "#;

    let result = compile(source, TrustTier::Empirical, 1000, 1024);

    match result {
        Ok(compilation_result) => {
            // Verify that compilation succeeded
            assert!(
                !compilation_result.bytecode.is_empty() || compilation_result.constants.is_empty()
            );
            assert_eq!(compilation_result.sandboxed, false);
        }
        Err(e) => {
            panic!("Compilation failed: {:?}", e);
        }
    }
}

#[test]
fn test_compile_lambda() {
    let source = r#"
        (lambda (x) (+ x 1))
    "#;

    let result = compile(source, TrustTier::Empirical, 1000, 1024);

    match result {
        Ok(compilation_result) => {
            // Verify that compilation succeeded
            assert!(
                !compilation_result.bytecode.is_empty() || compilation_result.constants.is_empty()
            );
            assert_eq!(compilation_result.sandboxed, false);
        }
        Err(e) => {
            panic!("Compilation failed: {:?}", e);
        }
    }
}

#[test]
fn test_compile_lambda_application() {
    let source = r#"
        ((lambda (x) (+ x 1)) 5)
    "#;

    let result = compile(source, TrustTier::Empirical, 1000, 1024);

    match result {
        Ok(compilation_result) => {
            // Verify that compilation succeeded
            assert!(
                !compilation_result.bytecode.is_empty() || compilation_result.constants.is_empty()
            );
            assert_eq!(compilation_result.sandboxed, false);
        }
        Err(e) => {
            panic!("Compilation failed: {:?}", e);
        }
    }
}

#[test]
fn test_compile_let_binding() {
    let source = r#"
        (let ((x 5)
              (y 10))
          (+ x y))
    "#;

    let result = compile(source, TrustTier::Empirical, 1000, 1024);

    match result {
        Ok(compilation_result) => {
            // Verify that compilation succeeded
            assert!(
                !compilation_result.bytecode.is_empty() || compilation_result.constants.is_empty()
            );
            assert_eq!(compilation_result.sandboxed, false);
        }
        Err(e) => {
            panic!("Compilation failed: {:?}", e);
        }
    }
}

#[test]
fn test_compile_conditional() {
    let source = r#"
        (if (> 5 3)
            "greater"
            "less")
    "#;

    let result = compile(source, TrustTier::Empirical, 1000, 1024);

    match result {
        Ok(compilation_result) => {
            // Verify that compilation succeeded
            assert!(
                !compilation_result.bytecode.is_empty() || compilation_result.constants.is_empty()
            );
            assert_eq!(compilation_result.sandboxed, false);
        }
        Err(e) => {
            panic!("Compilation failed: {:?}", e);
        }
    }
}

#[test]
fn test_compile_list_operations() {
    let source = r#"
        (cons 1 2)
        (list 1 2 3 4 5)
    "#;

    let result = compile(source, TrustTier::Empirical, 1000, 1024);

    match result {
        Ok(compilation_result) => {
            // Verify that compilation succeeded
            assert!(
                !compilation_result.bytecode.is_empty() || compilation_result.constants.is_empty()
            );
            assert_eq!(compilation_result.sandboxed, false);
        }
        Err(e) => {
            panic!("Compilation failed: {:?}", e);
        }
    }
}

#[test]
fn test_compile_with_different_trust_tiers() {
    let source = r#"
        (+ 1 2)
    "#;

    // Test with Formal tier
    let formal_result = compile(source, TrustTier::Formal, 1000, 1024);
    assert!(formal_result.is_ok());
    let formal_compilation = formal_result.unwrap();
    assert_eq!(formal_compilation.sandboxed, false);

    // Test with Verified tier
    let verified_result = compile(source, TrustTier::Verified, 1000, 1024);
    assert!(verified_result.is_ok());
    let verified_compilation = verified_result.unwrap();
    assert_eq!(verified_compilation.sandboxed, false);

    // Test with Empirical tier
    let empirical_result = compile(source, TrustTier::Empirical, 1000, 1024);
    assert!(empirical_result.is_ok());
    let empirical_compilation = empirical_result.unwrap();
    assert_eq!(empirical_compilation.sandboxed, false);

    // Test with Experimental tier
    let experimental_result = compile(source, TrustTier::Experimental, 1000, 1024);
    assert!(experimental_result.is_ok());
    let experimental_compilation = experimental_result.unwrap();
    assert_eq!(experimental_compilation.sandboxed, true); // Experimental should be sandboxed
}

#[test]
fn test_compile_with_capability_requirements() {
    let source = r#"
        (require-capability "macro-hygienic")
        (defmacro test-macro () 42)
    "#;

    let result = compile(source, TrustTier::Empirical, 1000, 1024);

    match result {
        Ok(compilation_result) => {
            // Verify that compilation succeeded
            assert!(
                !compilation_result.bytecode.is_empty() || compilation_result.constants.is_empty()
            );
            assert_eq!(compilation_result.sandboxed, false);
        }
        Err(e) => {
            panic!("Compilation failed: {:?}", e);
        }
    }
}

#[test]
fn test_compile_complex_expression() {
    let source = r#"
        (let ((identity (lambda (x) x))
              (add-one (lambda (x) (+ x 1))))
          (let ((result ((compose add-one identity) 5)))
            (if (> result 5)
                "success"
                "failure")))
    "#;

    let result = compile(source, TrustTier::Empirical, 1000, 1024);

    match result {
        Ok(compilation_result) => {
            // Verify that compilation succeeded
            assert!(
                !compilation_result.bytecode.is_empty() || compilation_result.constants.is_empty()
            );
            assert_eq!(compilation_result.sandboxed, false);
        }
        Err(e) => {
            panic!("Compilation failed: {:?}", e);
        }
    }
}

#[test]
fn test_compile_error_handling() {
    // Test with invalid capability
    let source = r#"
        (require-capability "invalid-capability")
        (+ 1 2)
    "#;

    let result = compile(source, TrustTier::Empirical, 1000, 1024);

    // This should fail because of unknown capability
    assert!(result.is_err());
    if let Err(CompilationError::ParseError { message, .. }) = result {
        assert!(message.contains("Unknown capability"));
    } else {
        panic!("Expected ParseError for unknown capability");
    }
}

#[test]
fn test_compile_resource_limits() {
    let source = r#"
        (+ 1 2)
    "#;

    let result = compile(source, TrustTier::Empirical, 5000, 2048);

    match result {
        Ok(compilation_result) => {
            // Verify resource limits are set correctly
            assert_eq!(compilation_result.step_limit, 5000);
            assert_eq!(compilation_result.memory_limit, 2048);
        }
        Err(e) => {
            panic!("Compilation failed: {:?}", e);
        }
    }
}
