use super::*;
use crate::types::OpCode;
use crate::vm::error::VmError;

#[test]
fn test_detailed_cpu_limit_error() {
    // Test that CPU limit exceeded error includes detailed context
    let mut vm = VmState::new(
        vec![OpCode::Int(1), OpCode::Int(2), OpCode::Add],
        vec![],
        1, // Very small step limit
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Err(VmError::CpuLimitExceeded { .. })));

    if let Err(VmError::CpuLimitExceeded { context, limit }) = result {
        assert_eq!(limit, 0); // Steps remaining when error occurred
        assert!(context.instruction_pointer <= 3); // Should be at one of the instructions
        assert_eq!(context.actor_id, 1);
        assert!(context.steps_remaining == 0);
        assert_eq!(context.call_stack_depth, 0);

        // Test detailed message
        let message = context.detailed_message();
        assert!(message.contains("CPU Limit Exceeded"));
        assert!(message.contains("IP"));
        assert!(message.contains("actor 1"));
    } else {
        panic!("Expected CpuLimitExceeded error");
    }
}

#[test]
fn test_detailed_stack_underflow_error() {
    // Test that stack underflow error includes detailed context
    let mut vm = VmState::new(
        vec![OpCode::Pop], // Try to pop from empty stack
        vec![],
        10,
        1024,
        2,
    );

    let result = vm.run();
    assert!(matches!(result, Err(VmError::StackUnderflow { .. })));

    if let Err(VmError::StackUnderflow {
        context,
        operation,
        required,
        available,
    }) = result
    {
        assert_eq!(operation, "Pop");
        assert_eq!(required, 1);
        assert_eq!(available, 0);
        assert_eq!(context.instruction_pointer, 0);
        assert_eq!(context.actor_id, 2);
        assert_eq!(context.stack_state.len(), 0);

        // Test detailed message
        let message = context.detailed_message();
        assert!(message.contains("Stack Underflow"));
        assert!(message.contains("Pop"));
        assert!(message.contains("requires 1 values, only 0 available"));
    } else {
        panic!("Expected StackUnderflow error");
    }
}

#[test]
fn test_detailed_arithmetic_overflow_error() {
    // Test that arithmetic overflow error includes detailed context
    let mut vm = VmState::new(
        vec![
            OpCode::Int(i64::MAX),
            OpCode::Int(1),
            OpCode::Add, // Should overflow
        ],
        vec![],
        10,
        1024,
        3,
    );

    let result = vm.run();
    assert!(matches!(result, Err(VmError::ArithmeticOverflow { .. })));

    if let Err(VmError::ArithmeticOverflow {
        context,
        operation,
        operand1,
        operand2,
    }) = result
    {
        assert_eq!(operation, "Add");
        assert_eq!(operand1, Some(i64::MAX));
        assert_eq!(operand2, Some(1));
        assert_eq!(context.instruction_pointer, 2); // Should be at Add instruction
        assert_eq!(context.actor_id, 3);

        // Test detailed message
        let message = context.detailed_message();
        assert!(message.contains("Arithmetic Overflow"));
        assert!(message.contains("Add"));
        assert!(message.contains(&i64::MAX.to_string()));
    } else {
        panic!("Expected ArithmeticOverflow error");
    }
}

#[test]
fn test_detailed_division_by_zero_error() {
    // Test that division by zero error includes detailed context
    let mut vm = VmState::new(
        vec![
            OpCode::Int(10),
            OpCode::Int(0),
            OpCode::Div, // Should cause division by zero
        ],
        vec![],
        10,
        1024,
        4,
    );

    let result = vm.run();
    assert!(matches!(result, Err(VmError::DivisionByZero { .. })));

    if let Err(VmError::DivisionByZero { context, operation }) = result {
        assert_eq!(operation, "Div");
        assert_eq!(context.instruction_pointer, 2); // Should be at Div instruction
        assert_eq!(context.actor_id, 4);
        assert_eq!(context.stack_state.len(), 2); // Should have [10, 0] on stack

        // Test detailed message
        let message = context.detailed_message();
        assert!(message.contains("Division By Zero"));
        assert!(message.contains("Div"));
    } else {
        panic!("Expected DivisionByZero error");
    }
}

#[test]
fn test_detailed_type_mismatch_error() {
    // Test that type mismatch error includes detailed context
    let mut vm = VmState::new(
        vec![
            OpCode::Int(42),
            OpCode::Bool(true),
            OpCode::Add, // Should cause type mismatch (Int + Bool)
        ],
        vec![],
        10,
        1024,
        5,
    );

    let result = vm.run();
    assert!(matches!(result, Err(VmError::TypeMismatch { .. })));

    if let Err(VmError::TypeMismatch {
        context,
        operation,
        expected,
        actual,
    }) = result
    {
        assert_eq!(operation, "Add");
        assert_eq!(expected, "Int");
        assert!(actual.contains("Bool"));
        assert_eq!(context.instruction_pointer, 2); // Should be at Add instruction
        assert_eq!(context.actor_id, 5);

        // Test detailed message
        let message = context.detailed_message();
        assert!(message.contains("Type Mismatch"));
        assert!(message.contains("Add"));
        assert!(message.contains("expected Int"));
    } else {
        panic!("Expected TypeMismatch error");
    }
}

#[test]
fn test_error_context_capture() {
    // Test that error context captures VM state correctly
    let mut vm = VmState::new(
        vec![
            OpCode::Int(1),
            OpCode::Int(2),
            OpCode::Int(3),
            OpCode::Pop, // This should work
            OpCode::Pop, // This should work
            OpCode::Pop, // This should work
            OpCode::Pop, // This should cause stack underflow
        ],
        vec![],
        10,
        1024,
        6,
    );

    // Execute until we hit the error
    let mut steps = 0;
    loop {
        match vm.step() {
            Ok(_) => steps += 1,
            Err(e) => {
                if let Err(VmError::StackUnderflow { context, .. }) = vm.run() {
                    // Verify context captures the state at error time
                    assert_eq!(context.instruction_pointer, 6);
                    assert_eq!(context.stack_state.len(), 0); // Stack should be empty
                    assert_eq!(context.call_stack_depth, 0);
                    assert_eq!(context.actor_id, 6);
                    assert!(context.steps_remaining < 10); // Some steps should have been consumed

                    // Verify the context includes the current instruction
                    if let Some(OpCode::Pop) = context.current_instruction {
                        assert!(true); // Correct instruction captured
                    } else {
                        panic!("Expected Pop instruction in context");
                    }
                }
                break;
            }
        }
    }
}

#[test]
fn test_error_recovery_mechanisms() {
    // Test that error recovery mechanisms work correctly
    let mut vm = VmState::new(
        vec![OpCode::Int(1), OpCode::Int(2), OpCode::Add],
        vec![],
        1, // Very small step limit
        1024,
        7,
    );

    let result = vm.run();
    assert!(matches!(result, Err(VmError::CpuLimitExceeded { .. })));

    if let Err(error) = result {
        // Test that the error is recoverable
        assert!(error.is_recoverable());

        // Test that recovery action is suggested
        if let Some(recovery_action) = error.attempt_recovery() {
            use crate::vm::error::RecoveryAction;
            assert!(matches!(
                recovery_action,
                RecoveryAction::IncreaseCpuLimit(_)
            ));
        } else {
            panic!("Expected recovery action for CPU limit error");
        }
    } else {
        panic!("Expected error for recovery test");
    }
}

#[test]
fn test_non_recoverable_errors() {
    // Test that some errors are non-recoverable
    let mut vm = VmState::new(
        vec![OpCode::Pop], // Stack underflow
        vec![],
        10,
        1024,
        8,
    );

    let result = vm.run();
    assert!(matches!(result, Err(VmError::StackUnderflow { .. })));

    if let Err(error) = result {
        // Stack underflow should not be recoverable
        assert!(!error.is_recoverable());

        // Should not suggest recovery action
        assert!(error.attempt_recovery().is_none());
    } else {
        panic!("Expected error for non-recoverable test");
    }
}

#[test]
fn test_error_display_formatting() {
    // Test that error display formatting works correctly
    let mut vm = VmState::new(
        vec![
            OpCode::Int(i64::MAX),
            OpCode::Int(1),
            OpCode::Add, // Overflow
        ],
        vec![],
        10,
        1024,
        9,
    );

    let result = vm.run();
    assert!(matches!(result, Err(VmError::ArithmeticOverflow { .. })));

    if let Err(error) = result {
        let display_message = format!("{}", error);
        assert!(display_message.contains("Arithmetic Overflow"));
        assert!(display_message.contains("Add"));
        assert!(display_message.contains("IP"));
        assert!(display_message.contains("actor 9"));
        assert!(display_message.contains("Stack"));

        println!("Error display message: {}", display_message);
    } else {
        panic!("Expected error for display test");
    }
}

#[test]
fn test_backward_compatibility() {
    // Test backward compatibility with old error types
    use crate::vm::state::VmError as OldVmError;

    // Create an old-style error
    let old_error = OldVmError::StackUnderflow;

    // Convert to new error type
    let new_error: VmError = old_error.into();

    // Verify it converts correctly
    if let VmError::StackUnderflow {
        context,
        operation,
        required,
        available,
    } = new_error
    {
        assert_eq!(operation, "operation");
        assert_eq!(required, 1);
        assert_eq!(available, 0);
        // Context should have default values for backward compatibility
        assert_eq!(context.instruction_pointer, 0);
        assert_eq!(context.actor_id, 0);
    } else {
        panic!("Backward compatibility conversion failed");
    }
}
