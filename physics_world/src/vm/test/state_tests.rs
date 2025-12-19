use super::super::state::{InstructionResult, VmError, VmState};
use crate::types::OpCode;

#[test]
fn test_new_vm_state() {
    let vm = VmState::new(vec![OpCode::Int(42), OpCode::Int(23)], vec![], 100, 1024);

    assert_eq!(vm.ip, 0);
    assert_eq!(vm.instructions.len(), 2);
    assert_eq!(vm.constant_pool.len(), 0);
    assert_eq!(vm.stack.len(), 0);
    assert_eq!(vm.call_stack.len(), 0);
    assert_eq!(vm.steps_remaining, 100);
    assert_eq!(vm.memory.capacity(), 1024);
}

#[test]
fn test_simple_int_program() {
    let mut vm = VmState::new(vec![OpCode::Int(5), OpCode::Int(3)], vec![], 10, 1024);

    // Execute first instruction (Int 5)
    let result = vm.step();
    assert!(matches!(result, Ok(InstructionResult::Continue)));
    assert_eq!(vm.ip, 1);
    assert_eq!(vm.stack.len(), 1);
    assert_eq!(vm.stack[0], crate::types::Value::Int(5));

    // Execute second instruction (Int 3)
    let result = vm.step();
    assert!(matches!(result, Ok(InstructionResult::Continue)));
    assert_eq!(vm.ip, 2);
    assert_eq!(vm.stack.len(), 2);
    assert_eq!(vm.stack[0], crate::types::Value::Int(5));
    assert_eq!(vm.stack[1], crate::types::Value::Int(3));
}

#[test]
fn test_cpu_limit_exceeded() {
    let mut vm = VmState::new(
        vec![OpCode::Int(1)],
        vec![],
        0, // Zero steps remaining
        1024,
    );

    let result = vm.step();
    assert!(matches!(result, Err(VmError::CpuLimitExceeded)));
}

#[test]
fn test_stack_operations() {
    let mut vm = VmState::new(
        vec![OpCode::Int(1), OpCode::Int(2), OpCode::Dup, OpCode::Pop],
        vec![],
        10,
        1024,
    );

    // Int 1
    vm.step().unwrap();
    assert_eq!(vm.stack, vec![crate::types::Value::Int(1)]);

    // Int 2
    vm.step().unwrap();
    assert_eq!(
        vm.stack,
        vec![crate::types::Value::Int(1), crate::types::Value::Int(2)]
    );

    // Dup
    vm.step().unwrap();
    assert_eq!(
        vm.stack,
        vec![
            crate::types::Value::Int(1),
            crate::types::Value::Int(2),
            crate::types::Value::Int(2)
        ]
    );

    // Pop
    vm.step().unwrap();
    assert_eq!(
        vm.stack,
        vec![crate::types::Value::Int(1), crate::types::Value::Int(2)]
    );
}

#[test]
fn test_stack_underflow() {
    let mut vm = VmState::new(vec![OpCode::Pop], vec![], 10, 1024);

    let result = vm.step();
    assert!(matches!(result, Err(VmError::StackUnderflow)));
}

#[test]
fn test_call_and_return() {
    // Create a simple closure that acts as an identity function
    let mut vm = VmState::new(
        vec![
            OpCode::MakeClosure(0, 0), // Create closure with code index 0, no captures
            OpCode::Int(42),           // Push argument
            OpCode::Call(1),           // Call with 1 argument
            OpCode::Int(99),           // This should not execute
        ],
        vec![],
        20,
        1024,
    );

    // MakeClosure
    vm.step().unwrap();
    assert_eq!(vm.call_stack.len(), 0);
    assert_eq!(vm.stack.len(), 1);
    assert!(matches!(vm.stack[0], crate::types::Value::Closure(_)));

    // Push argument
    vm.step().unwrap();
    assert_eq!(vm.stack.len(), 2);
    assert_eq!(vm.stack[1], crate::types::Value::Int(42));

    // Call - this should execute the identity function
    let result = vm.step();
    assert!(matches!(result, Ok(InstructionResult::Continue)));

    // The call should have created a call frame and executed the identity function
    assert_eq!(vm.call_stack.len(), 1);
    assert_eq!(vm.stack.len(), 1); // Should have the return value (42)
    assert_eq!(vm.stack[0], crate::types::Value::Int(42));

    // Now return
    vm.instructions = vec![OpCode::Ret];
    vm.ip = 0;
    let result = vm.step();
    assert!(matches!(result, Ok(InstructionResult::Continue)));

    // Should have restored stack and IP
    assert_eq!(vm.call_stack.len(), 0);
    assert_eq!(vm.stack.len(), 1); // Return value
    assert_eq!(vm.stack[0], crate::types::Value::Int(42));
}

#[test]
fn test_conditional_jump() {
    let mut vm = VmState::new(
        vec![
            OpCode::Bool(false),
            OpCode::JmpIfFalse(2), // Jump 2 instructions forward if false
            OpCode::Int(1),        // This should be skipped
            OpCode::Int(2),        // This should execute
        ],
        vec![],
        10,
        1024,
    );

    // Push false
    vm.step().unwrap();
    // JmpIfFalse - should jump
    vm.step().unwrap();
    assert_eq!(vm.ip, 3); // Should have jumped to Int(2)

    // Execute Int(2)
    vm.step().unwrap();
    assert_eq!(vm.stack, vec![crate::types::Value::Int(2)]);
}

#[test]
fn test_yield() {
    let mut vm = VmState::new(vec![OpCode::Yield], vec![], 10, 1024);

    let result = vm.step();
    assert!(matches!(result, Ok(InstructionResult::Yield)));
    assert_eq!(vm.ip, 1);
}
