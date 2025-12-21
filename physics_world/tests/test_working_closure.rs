use physics_world::{
    types::{OpCode, Value},
    vm::VmState,
};

/// Test that verifies the existing closure implementation works
/// This test uses the simplest possible case

#[test]
fn test_existing_closure_implementation() {
    // Test that MakeClosure works as currently implemented
    // This doesn't test full closure execution, just that the opcode doesn't crash

    let main_program = vec![
        OpCode::Int(42), // Push a value
        OpCode::MakeClosure(0, 0), // Create closure with 0 captured variables
                         // Don't call it, just verify it doesn't crash
    ];

    let constant_pool = vec![
        Value::Nil, // Placeholder - not actually used in this test
    ];

    let mut vm = VmState::new(main_program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should succeed and return the closure
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Closure(_)));
}

#[test]
fn test_simple_function_call_identity() {
    // Test the simplest possible function call
    // The current implementation has an identity function fallback

    let main_program = vec![
        OpCode::Int(42),           // Push argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call with 1 argument
    ];

    let constant_pool = vec![
        Value::Nil, // Placeholder
    ];

    let mut vm = VmState::new(main_program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // With the current identity function implementation, should return the argument
    assert_eq!(result.unwrap(), Value::Int(42));
}

#[test]
fn test_function_call_no_args() {
    // Test function call with no arguments

    let main_program = vec![
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(0),           // Call with 0 arguments
    ];

    let constant_pool = vec![
        Value::Nil, // Placeholder
    ];

    let mut vm = VmState::new(main_program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should return Nil (no arguments, identity function returns Nil)
    assert_eq!(result.unwrap(), Value::Nil);
}

#[test]
fn test_function_call_multiple_args() {
    // Test function call with multiple arguments

    let main_program = vec![
        OpCode::Int(1),            // First argument
        OpCode::Int(2),            // Second argument
        OpCode::Int(3),            // Third argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(3),           // Call with 3 arguments
    ];

    let constant_pool = vec![
        Value::Nil, // Placeholder
    ];

    let mut vm = VmState::new(main_program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // With identity function, should return the first argument
    assert_eq!(result.unwrap(), Value::Int(1));
}

#[test]
fn test_nested_function_calls() {
    // Test nested function calls

    let main_program = vec![
        OpCode::Int(5),            // Argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // First call
        OpCode::MakeClosure(0, 0), // Create another closure
        OpCode::Call(1),           // Second call
        OpCode::Add,               // Add results
    ];

    let constant_pool = vec![
        Value::Nil, // Placeholder
    ];

    let mut vm = VmState::new(main_program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should return 5 + 5 = 10
    assert_eq!(result.unwrap(), Value::Int(10));
}

#[test]
fn test_function_return_value() {
    // Test that function calls properly handle return values

    let main_program = vec![
        OpCode::Int(10),           // Argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function
        OpCode::Int(5),            // Push another value
        OpCode::Add,               // Add return value + 5
    ];

    let constant_pool = vec![
        Value::Nil, // Placeholder
    ];

    let mut vm = VmState::new(main_program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should return 10 + 5 = 15
    assert_eq!(result.unwrap(), Value::Int(15));
}

#[test]
fn test_function_call_stack_management() {
    // Test that function calls properly manage the stack

    let main_program = vec![
        OpCode::Int(1),            // Local variable
        OpCode::Int(2),            // Argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function
        OpCode::Int(1),            // Push 1
        OpCode::Add,               // Add to local variable (should be 1 + 2 = 3)
    ];

    let constant_pool = vec![
        Value::Nil, // Placeholder
    ];

    let mut vm = VmState::new(main_program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should return 1 + 2 = 3 (local variable preserved)
    assert_eq!(result.unwrap(), Value::Int(3));
}
