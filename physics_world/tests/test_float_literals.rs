use physics_world::{
    types::{OpCode, Value},
    vm::{VmError, VmState},
};

/// Test basic float literal creation and stack operations
#[test]
fn test_float_literal_creation() {
    // Test basic float literal creation - let VM finish naturally
    let program = vec![
        OpCode::Float(3.14159), // Push pi onto stack
        OpCode::Float(2.71828), // Push e onto stack
        OpCode::Pop,            // Pop e, leaving pi
                                // No Ret needed - VM will finish when out of instructions
    ];

    let mut vm = VmState::new(program, vec![], 100, 1024, 1, 100);

    let result = vm.run();

    match result {
        Ok(Value::Float(f)) => {
            // Should be approximately pi (3.14159)
            assert!((f - 3.14159).abs() < 1e-10, "Expected pi, got {}", f);
        }
        _ => panic!("Expected Float result, got {:?}", result),
    }
}

/// Test float arithmetic operations
#[test]
fn test_float_arithmetic() {
    // Test float addition
    let program = vec![
        OpCode::Float(10.5),
        OpCode::Float(20.25),
        OpCode::FAdd,
        // No Ret needed - VM will finish when out of instructions
    ];

    let mut vm = VmState::new(program, vec![], 100, 1024, 1, 100);

    let result = vm.run();

    match result {
        Ok(Value::Float(f)) => {
            assert!((f - 30.75).abs() < 1e-10, "Expected 30.75, got {}", f);
        }
        _ => panic!("Expected Float result, got {:?}", result),
    }

    // Test float subtraction
    let program = vec![
        OpCode::Float(100.0),
        OpCode::Float(25.5),
        OpCode::FSub,
        // No Ret needed - VM will finish when out of instructions
    ];

    let mut vm = VmState::new(program, vec![], 100, 1024, 1, 100);

    let result = vm.run();

    match result {
        Ok(Value::Float(f)) => {
            assert!((f - 74.5).abs() < 1e-10, "Expected 74.5, got {}", f);
        }
        _ => panic!("Expected Float result, got {:?}", result),
    }

    // Test float multiplication
    let program = vec![
        OpCode::Float(6.0),
        OpCode::Float(7.0),
        OpCode::FMul,
        // No Ret needed - VM will finish when out of instructions
    ];

    let mut vm = VmState::new(program, vec![], 100, 1024, 1, 100);

    let result = vm.run();

    match result {
        Ok(Value::Float(f)) => {
            assert!((f - 42.0).abs() < 1e-10, "Expected 42.0, got {}", f);
        }
        _ => panic!("Expected Float result, got {:?}", result),
    }

    // Test float division
    let program = vec![
        OpCode::Float(15.0),
        OpCode::Float(3.0),
        OpCode::FDiv,
        // No Ret needed - VM will finish when out of instructions
    ];

    let mut vm = VmState::new(program, vec![], 100, 1024, 1, 100);

    let result = vm.run();

    match result {
        Ok(Value::Float(f)) => {
            assert!((f - 5.0).abs() < 1e-10, "Expected 5.0, got {}", f);
        }
        _ => panic!("Expected Float result, got {:?}", result),
    }
}

/// Test complex float arithmetic expression
#[test]
fn test_complex_float_expression() {
    // Test: (3.5 * 2.0) + (10.0 / 4.0) = 7.0 + 2.5 = 9.5
    let program = vec![
        OpCode::Float(3.5),
        OpCode::Float(2.0),
        OpCode::FMul, // 7.0
        OpCode::Float(10.0),
        OpCode::Float(4.0),
        OpCode::FDiv, // 2.5
        OpCode::FAdd, // 9.5
                      // No Ret needed - VM will finish when out of instructions
    ];

    let mut vm = VmState::new(program, vec![], 100, 1024, 1, 100);

    let result = vm.run();

    match result {
        Ok(Value::Float(f)) => {
            assert!((f - 9.5).abs() < 1e-10, "Expected 9.5, got {}", f);
        }
        _ => panic!("Expected Float result, got {:?}", result),
    }
}

/// Test float arithmetic with negative numbers
#[test]
fn test_float_negative_arithmetic() {
    // Test: -5.5 + 3.2 = -2.3
    let program = vec![
        OpCode::Float(-5.5),
        OpCode::Float(3.2),
        OpCode::FAdd,
        // No Ret needed - VM will finish when out of instructions
    ];

    let mut vm = VmState::new(program, vec![], 100, 1024, 1, 100);

    let result = vm.run();

    match result {
        Ok(Value::Float(f)) => {
            assert!((f - (-2.3)).abs() < 1e-10, "Expected -2.3, got {}", f);
        }
        _ => panic!("Expected Float result, got {:?}", result),
    }
}

/// Test mixed float and integer operations
#[test]
fn test_mixed_float_integer_arithmetic() {
    // This test verifies that float operations work independently of integer operations
    // Test: 10.5 (float) + 5 (integer) should be treated as separate operations

    // First test float operations
    let float_program = vec![
        OpCode::Float(10.5),
        OpCode::Float(5.0),
        OpCode::FAdd,
        // No Ret needed - VM will finish when out of instructions
    ];

    let mut float_vm = VmState::new(float_program, vec![], 100, 1024, 1, 100);
    let float_result = float_vm.run();

    match float_result {
        Ok(Value::Float(f)) => {
            assert!((f - 15.5).abs() < 1e-10, "Expected 15.5, got {}", f);
        }
        _ => panic!("Expected Float result, got {:?}", float_result),
    }

    // Then test integer operations separately
    let int_program = vec![
        OpCode::Int(10),
        OpCode::Int(5),
        OpCode::Add,
        // No Ret needed - VM will finish when out of instructions
    ];

    let mut int_vm = VmState::new(int_program, vec![], 100, 1024, 1, 100);
    let int_result = int_vm.run();

    match int_result {
        Ok(Value::Int(i)) => {
            assert_eq!(i, 15, "Expected 15, got {}", i);
        }
        _ => panic!("Expected Int result, got {:?}", int_result),
    }
}

/// Test special float values
#[test]
fn test_special_float_values() {
    // Test zero
    let program = vec![
        OpCode::Float(0.0),
        // No Ret needed - VM will finish when out of instructions
    ];

    let mut vm = VmState::new(program, vec![], 100, 1024, 1, 100);
    let result = vm.run();

    match result {
        Ok(Value::Float(f)) => {
            assert_eq!(f, 0.0, "Expected 0.0, got {}", f);
        }
        _ => panic!("Expected Float result, got {:?}", result),
    }

    // Test very small number
    let program = vec![
        OpCode::Float(1e-10),
        // No Ret needed - VM will finish when out of instructions
    ];

    let mut vm = VmState::new(program, vec![], 100, 1024, 1, 100);
    let result = vm.run();

    match result {
        Ok(Value::Float(f)) => {
            assert!((f - 1e-10).abs() < 1e-15, "Expected 1e-10, got {}", f);
        }
        _ => panic!("Expected Float result, got {:?}", result),
    }

    // Test very large number
    let program = vec![
        OpCode::Float(1e10),
        // No Ret needed - VM will finish when out of instructions
    ];

    let mut vm = VmState::new(program, vec![], 100, 1024, 1, 100);
    let result = vm.run();

    match result {
        Ok(Value::Float(f)) => {
            assert!((f - 1e10).abs() < 1e5, "Expected 1e10, got {}", f);
        }
        _ => panic!("Expected Float result, got {:?}", result),
    }
}
