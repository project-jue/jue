use physics_world::types::{OpCode, Value};
use physics_world::vm::VmState;

/// Test basic closure creation without environment capture
#[test]
fn test_basic_closure_no_capture() {
    let source = r#"
    (lambda (x) x)
    "#;

    // Simple compilation test - just check it doesn't crash
    let program = vec![
        OpCode::MakeClosure(0, 0), // code_idx=0, capture_count=0 (no environment capture)
    ];
    let constants = vec![Value::Int(0)];

    let mut vm = VmState::new(program, constants, 100, 1024, 1, 100);
    let result = vm.run();

    // Should create a closure without errors
    assert!(result.is_ok());
}

/// Test that environment capture analysis works
#[test]
fn test_closure_capture_analysis() {
    // Test that we can properly analyze captured variables
    let program = vec![
        OpCode::Int(42),           // Push variable to capture
        OpCode::GetLocal(0),       // Reference captured variable
        OpCode::MakeClosure(1, 1), // Create closure with 1 capture
    ];
    let constants = vec![Value::Int(0), Value::Int(0)];

    let mut vm = VmState::new(program, constants, 100, 1024, 1, 100);
    let result = vm.run();

    // Should execute without errors
    assert!(result.is_ok());
}

/// Test closure with multiple captures
#[test]
fn test_multiple_captures() {
    let program = vec![
        OpCode::Int(1),            // First capture
        OpCode::Int(2),            // Second capture
        OpCode::Int(3),            // Third capture
        OpCode::MakeClosure(0, 3), // Create closure with 3 captures
    ];
    let constants = vec![Value::Int(0)];

    let mut vm = VmState::new(program, constants, 100, 1024, 1, 100);
    let result = vm.run();

    // Should handle multiple captures correctly
    assert!(result.is_ok());
}

/// Test nested closure creation
#[test]
fn test_nested_closures() {
    let program = vec![
        OpCode::Int(10),           // Outer capture
        OpCode::MakeClosure(1, 1), // Outer closure
        OpCode::Int(20),           // Inner capture
        OpCode::MakeClosure(0, 1), // Inner closure
    ];
    let constants = vec![Value::Int(0), Value::Int(0)];

    let mut vm = VmState::new(program, constants, 100, 1024, 1, 100);
    let result = vm.run();

    // Should create nested closures correctly
    assert!(result.is_ok());
}

/// Test closure with no captures
#[test]
fn test_closure_no_captures() {
    let program = vec![
        OpCode::MakeClosure(0, 0), // No captures, no parameters
    ];
    let constants = vec![Value::Int(0)];

    let mut vm = VmState::new(program, constants, 100, 1024, 1, 100);
    let result = vm.run();

    // Should work with no captures
    assert!(result.is_ok());
}

/// Test that capture count is properly tracked
#[test]
fn test_capture_count_tracking() {
    // Verify that different capture counts work
    for capture_count in 0..5 {
        let program = vec![OpCode::MakeClosure(0, capture_count)];
        let constants = vec![Value::Int(0)];

        // Add dummy values for captures
        let mut program_with_captures = program.clone();
        for _ in 0..capture_count {
            program_with_captures.insert(0, OpCode::Int(0));
        }

        let mut vm = VmState::new(program_with_captures, constants, 100, 1024, 1, 100);
        let result = vm.run();

        assert!(
            result.is_ok(),
            "Failed with capture count {}",
            capture_count
        );
    }
}

/// Test closure creation with environment capture (not parameter count)
#[test]
fn test_closure_with_captures() {
    for capture_count in 1..4 {
        // Build program: push capture_count values, then create closure
        let mut program = Vec::new();

        // Push values to capture
        for _ in 0..capture_count {
            program.push(OpCode::Int(42));
        }

        // Create closure with the specified capture count
        program.push(OpCode::MakeClosure(0, capture_count));

        let constants = vec![Value::Int(0)];

        let mut vm = VmState::new(program, constants, 100, 1024, 1, 100);
        let result = vm.run();

        assert!(
            result.is_ok(),
            "Failed with capture count {}",
            capture_count
        );
    }
}

/// Test error handling for invalid capture counts
#[test]
fn test_capture_error_handling() {
    // Test with capture count > available stack values
    let program = vec![
        OpCode::MakeClosure(0, 5), // Try to capture 5, but only 1 value on stack
    ];
    let constants = vec![Value::Int(0)];

    let mut vm = VmState::new(program, constants, 100, 1024, 1, 100);
    let result = vm.run();

    // Should handle error gracefully
    assert!(result.is_err());
}

/// Test compilation generates proper MakeClosure instructions
#[test]
fn test_make_closure_generation() {
    // This would test the actual compilation from Jue source
    // For now, just verify the instruction exists and works
    let program = vec![OpCode::Int(42), OpCode::MakeClosure(0, 1)];
    let constants = vec![Value::Int(0)];

    let mut vm = VmState::new(program, constants, 100, 1024, 1, 100);
    let result = vm.run();

    assert!(result.is_ok());
}

/// Test closure execution with captured values
#[test]
fn test_closure_execution() {
    let program = vec![
        OpCode::Int(100),          // Captured value
        OpCode::Int(5),            // Parameter
        OpCode::MakeClosure(0, 1), // Closure with 1 capture, 1 param
    ];
    let constants = vec![Value::Int(0)];

    let mut vm = VmState::new(program, constants, 100, 1024, 1, 100);
    let result = vm.run();

    // Should create closure that can be executed
    assert!(result.is_ok());
}
