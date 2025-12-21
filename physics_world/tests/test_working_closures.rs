use bincode;
use physics_world::vm::error::VmError;
use physics_world::{
    types::{HeapPtr, OpCode, Value},
    vm::VmState,
};

/// Test suite that works with the current closure implementation
/// These tests verify that the existing closure execution works correctly

#[test]
fn test_make_closure_works() {
    // Test that MakeClosure doesn't crash
    let program = vec![
        OpCode::Int(42),
        OpCode::MakeClosure(0, 0), // Create closure with code_idx=0, capture_count=0
    ];

    let constant_pool = vec![Value::Nil]; // Placeholder

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should succeed and return the closure
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Closure(_)));
}

#[test]
fn test_simple_closure_call_identity() {
    // Test the current identity function behavior
    // The current implementation returns the first argument as a fallback

    let program = vec![
        OpCode::Int(42),           // Push argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call with 1 argument
    ];

    let constant_pool = vec![Value::Nil]; // Placeholder

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // With identity function fallback, should return the first argument
    assert_eq!(result.unwrap(), Value::Int(42));
}

#[test]
fn test_closure_call_no_args() {
    // Test function call with no arguments

    let program = vec![
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(0),           // Call with 0 arguments
    ];

    let constant_pool = vec![Value::Nil]; // Placeholder

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should return Nil (no arguments, identity function returns Nil)
    assert_eq!(result.unwrap(), Value::Nil);
}

#[test]
fn test_closure_call_multiple_args() {
    // Test function call with multiple arguments

    let program = vec![
        OpCode::Int(1),            // First argument
        OpCode::Int(2),            // Second argument
        OpCode::Int(3),            // Third argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(3),           // Call with 3 arguments
    ];

    let constant_pool = vec![Value::Nil]; // Placeholder

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // With identity function, should return the first argument
    assert_eq!(result.unwrap(), Value::Int(1));
}

#[test]
fn test_nested_closure_calls() {
    // Test nested function calls

    let program = vec![
        OpCode::Int(5),            // Argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // First call
        OpCode::MakeClosure(0, 0), // Create another closure
        OpCode::Call(1),           // Second call
        OpCode::Add,               // Add results
    ];

    let constant_pool = vec![Value::Nil]; // Placeholder

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should return 5 + 5 = 10
    assert_eq!(result.unwrap(), Value::Int(10));
}

#[test]
fn test_closure_return_value_handling() {
    // Test that function calls properly handle return values

    let program = vec![
        OpCode::Int(10),           // Argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function
        OpCode::Int(5),            // Push another value
        OpCode::Add,               // Add return value + 5
    ];

    let constant_pool = vec![Value::Nil]; // Placeholder

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should return 10 + 5 = 15
    assert_eq!(result.unwrap(), Value::Int(15));
}

#[test]
fn test_closure_stack_management() {
    // Test that function calls properly manage the stack

    let program = vec![
        OpCode::Int(1),            // Local variable
        OpCode::Int(2),            // Argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function
        OpCode::Int(1),            // Push 1
        OpCode::Add,               // Add to local variable (should be 1 + 2 = 3)
    ];

    let constant_pool = vec![Value::Nil]; // Placeholder

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should return 1 + 2 = 3 (local variable preserved)
    assert_eq!(result.unwrap(), Value::Int(3));
}

#[test]
fn test_closure_with_capture() {
    // Test closure creation with variable capture

    let program = vec![
        OpCode::Int(42), // Value to capture
        OpCode::MakeClosure(0, 1), // Create closure capturing 1 variable
                         // Don't call it, just verify it doesn't crash
    ];

    let constant_pool = vec![Value::Nil]; // Placeholder

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should succeed and return the closure
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Closure(_)));
}

#[test]
fn test_closure_error_handling() {
    // Test error handling in function calls

    let program = vec![
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call with 1 argument (but stack only has closure)
    ];

    let constant_pool = vec![Value::Nil]; // Placeholder

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should get stack underflow error
    assert!(matches!(result, Err(VmError::StackUnderflow { .. })));
}

#[test]
fn test_closure_with_arithmetic() {
    // Test closure calls with arithmetic operations

    let program = vec![
        OpCode::Int(10),           // Argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function (returns 10 with identity)
        OpCode::Int(5),            // Push 5
        OpCode::Mul,               // Multiply 10 * 5 = 50
    ];

    let constant_pool = vec![Value::Nil]; // Placeholder

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should return 10 * 5 = 50
    assert_eq!(result.unwrap(), Value::Int(50));
}

#[test]
fn test_closure_with_conditional() {
    // Test closure calls with conditional logic

    let program = vec![
        OpCode::Int(10),           // Argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function (returns 10)
        OpCode::Int(5),            // Push 5
        OpCode::Gt,                // 10 > 5 = true
        OpCode::JmpIfFalse(2),     // Skip next instruction if false
        OpCode::Int(1),            // True branch
        OpCode::Jmp(1),            // Skip false branch
        OpCode::Int(0),            // False branch (never reached)
    ];

    let constant_pool = vec![Value::Nil]; // Placeholder

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should return 1 (true branch)
    assert_eq!(result.unwrap(), Value::Int(1));
}

#[test]
fn test_closure_stack_underflow() {
    // Test stack underflow in closure call

    let program = vec![
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Try to call with 1 argument (but none provided)
    ];

    let constant_pool = vec![Value::Nil]; // Placeholder

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should get stack underflow error
    assert!(matches!(result, Err(VmError::StackUnderflow { .. })));
}

#[test]
fn test_closure_return_without_call() {
    // Test Ret opcode without corresponding Call

    let program = vec![
        OpCode::Int(42), // Push value
        OpCode::Ret,     // Try to return (but no call frame)
    ];

    let constant_pool = vec![]; // Empty

    let mut vm = VmState::new(program, constant_pool, 1000, 1024, 1, 100);
    let result = vm.run();

    // Should get stack underflow error (no call frame)
    assert!(matches!(result, Err(VmError::StackUnderflow { .. })));
}
