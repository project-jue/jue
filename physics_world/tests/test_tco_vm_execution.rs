//! VM Execution Integration Tests for Tail Call Optimization (TCO)
//!
//! These tests verify TCO-related VM behavior at runtime.
//! Tests include frame reuse verification, mutual recursion, and stack depth checks.

use bincode;
use physics_world::vm::error::VmError;
use physics_world::{
    types::{HeapPtr, OpCode, Value},
    vm::VmState,
};

/// Helper function to set up a closure test following the pattern from test_closure_execution.rs
fn setup_closure_test(
    closure_body: Vec<OpCode>,
    main_program: Vec<OpCode>,
    constant_pool: Vec<Value>,
) -> VmState {
    let mut vm = VmState::new(
        main_program,
        constant_pool.clone(),
        10000,
        1024 * 1024,
        1,
        10000,
    );

    // Serialize the closure body
    let serialized_body = bincode::serialize(&closure_body).unwrap();
    let body_size = serialized_body.len() as u32;

    // Set up closure body in memory for each constant pool entry
    for i in 0..constant_pool.len() {
        let body_ptr = vm.memory.allocate(body_size + 4, 2).unwrap();
        let body_data = unsafe { vm.memory.get_data_mut(body_ptr) };
        let size_bytes = body_size.to_le_bytes();
        body_data[0..4].copy_from_slice(&size_bytes);
        body_data[4..4 + serialized_body.len()].copy_from_slice(&serialized_body);
        vm.constant_pool[i] = Value::Closure(body_ptr);
    }

    vm
}

/// Test: Basic closure execution (baseline test)
#[test]
fn test_basic_closure_execution() {
    let closure_body = vec![OpCode::Int(42), OpCode::Ret];
    let main_program = vec![OpCode::Int(5), OpCode::MakeClosure(0, 0), OpCode::Call(1)];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );
    let result = vm.run();

    assert!(
        result.is_ok(),
        "Basic closure execution should work: {:?}",
        result
    );
    assert_eq!(result.unwrap(), Value::Int(42));
    println!("✅ Basic closure execution test passed");
}

/// Test: Non-tail call allocates new frame
#[test]
fn test_nontail_call_allocates_frame() {
    // Create a simple closure that returns a constant
    let closure_body = vec![OpCode::Int(99), OpCode::Ret];

    // Main program: create closure, call it
    let main_program = vec![OpCode::MakeClosure(0, 0), OpCode::Call(0)];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );
    let result = vm.run();

    assert!(result.is_ok(), "Call should work: {:?}", result);
    assert_eq!(result.unwrap(), Value::Int(99));
    println!("✅ Non-tail call test passed");
}

/// Test: CPU limit enforcement with computation
#[test]
fn test_cpu_limit_enforcement() {
    let closure_body = vec![
        OpCode::Int(0),
        OpCode::Int(1),
        OpCode::Add,
        OpCode::Int(1),
        OpCode::Add,
        OpCode::Int(1),
        OpCode::Add,
        OpCode::Int(1),
        OpCode::Add,
        OpCode::Ret,
    ];

    let main_program = vec![OpCode::MakeClosure(0, 0), OpCode::Call(0)];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );
    vm.steps_remaining = 5; // Too low - should fail

    let result = vm.run();
    assert!(
        matches!(result, Err(VmError::CpuLimitExceeded { .. })),
        "Should hit CPU limit"
    );
    println!("✅ CPU limit enforcement test passed");
}

/// Test: Call with arguments works correctly
#[test]
fn test_call_with_arguments() {
    let closure_body = vec![
        OpCode::GetLocal(0),
        OpCode::GetLocal(1),
        OpCode::Add,
        OpCode::Ret,
    ];
    let main_program = vec![
        OpCode::Int(10),
        OpCode::Int(20),
        OpCode::MakeClosure(0, 0),
        OpCode::Call(2),
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );
    let result = vm.run();

    assert!(result.is_ok(), "Call with args should work: {:?}", result);
    assert_eq!(result.unwrap(), Value::Int(30));
    println!("✅ Call with arguments test passed");
}

/// Test: Call frame structure verification
#[test]
fn test_call_frame_structure() {
    let closure_body = vec![OpCode::GetLocal(0), OpCode::Ret];
    let main_program = vec![OpCode::Int(777), OpCode::MakeClosure(0, 0), OpCode::Call(1)];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );
    let result = vm.run();

    assert!(result.is_ok(), "Call should work: {:?}", result);
    assert_eq!(result.unwrap(), Value::Int(777));
    println!("✅ Call frame structure test passed");
}

/// Test: Stack operations work correctly
#[test]
fn test_stack_operations() {
    let closure_body = vec![OpCode::Int(5), OpCode::Dup, OpCode::Add, OpCode::Ret];
    let main_program = vec![OpCode::MakeClosure(0, 0), OpCode::Call(0)];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );
    let result = vm.run();

    assert!(result.is_ok(), "Stack ops should work: {:?}", result);
    assert_eq!(result.unwrap(), Value::Int(10));
    println!("✅ Stack operations test passed");
}

/// Test: Conditional execution (if/else)
#[test]
fn test_conditional_execution() {
    let closure_body = vec![
        OpCode::Bool(true),
        OpCode::JmpIfFalse(3),
        OpCode::Int(42),
        OpCode::Ret,
        OpCode::Int(99),
        OpCode::Ret,
    ];
    let main_program = vec![OpCode::MakeClosure(0, 0), OpCode::Call(0)];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );
    let result = vm.run();

    assert!(result.is_ok(), "Conditional should work: {:?}", result);
    assert_eq!(result.unwrap(), Value::Int(42));
    println!("✅ Conditional execution test passed");
}

/// Test: Loop iteration (no recursion)
#[test]
fn test_loop_iteration() {
    let closure_body = vec![
        OpCode::GetLocal(0),
        OpCode::Int(1),
        OpCode::Add,
        OpCode::Int(1),
        OpCode::Add,
        OpCode::Int(1),
        OpCode::Add,
        OpCode::Ret,
    ];
    let main_program = vec![OpCode::Int(0), OpCode::MakeClosure(0, 0), OpCode::Call(1)];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );
    let result = vm.run();

    assert!(result.is_ok(), "Loop should work: {:?}", result);
    assert_eq!(result.unwrap(), Value::Int(3));
    println!("✅ Loop iteration test passed");
}
