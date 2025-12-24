/// Simple recursion tests to debug the recursive function implementation
use jue_world::error::CompilationError;
use jue_world::parser::parse;
use jue_world::physics_compiler::compile_to_physics_world;
use jue_world::trust_tier::TrustTier;

#[test]
fn test_simple_recursive_base_case() {
    // Test a recursive function that only executes the base case
    // This should work without actually recursing
    let source = r#"
        (let ((fact (lambda (n)
                      (if (<= n 1)
                          1
                          (* n (fact (- n 1)))))))
             (fact 1))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let result = compile_to_physics_world(&ast, TrustTier::Formal);

    match result {
        Ok((bytecode, _)) => {
            println!("✅ Simple recursive base case compilation successful");
            println!("Bytecode: {:?}", bytecode);
        }
        Err(e) => {
            panic!("❌ Simple recursive base case compilation failed: {}", e);
        }
    }
}

#[test]
fn test_single_recursion_step() {
    // Test a recursive function that only recurses once
    // This should help isolate the recursion issue
    let source = r#"
        (let ((fact (lambda (n)
                      (if (<= n 1)
                          1
                          (* n (fact (- n 1)))))))
             (fact 2))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let result = compile_to_physics_world(&ast, TrustTier::Formal);

    match result {
        Ok((bytecode, _)) => {
            println!("✅ Single recursion step compilation successful");
            println!("Bytecode: {:?}", bytecode);
        }
        Err(e) => {
            panic!("❌ Single recursion step compilation failed: {}", e);
        }
    }
}

#[test]
fn test_non_recursive_lambda() {
    // Test a lambda that doesn't use recursion at all
    // This should work as a baseline
    let source = r#"
        (let ((add-one (lambda (n)
                         (+ n 1))))
             (add-one 5))
    "#;

    let ast = parse(source).expect("Parsing should succeed");
    let result = compile_to_physics_world(&ast, TrustTier::Formal);

    match result {
        Ok((bytecode, _)) => {
            println!("✅ Non-recursive lambda compilation successful");
            println!("Bytecode: {:?}", bytecode);
        }
        Err(e) => {
            panic!("❌ Non-recursive lambda compilation failed: {}", e);
        }
    }
}
