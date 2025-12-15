use physics_world::{
    types::{OpCode, Value},
    vm::{VmError, VmState},
};

// Test 1: Basic Stack Execution
#[test]
fn vm_stack_operations() {
    let code = vec![
        OpCode::Int(5),
        OpCode::Int(3),
        OpCode::Add, // Custom opcode for i64 addition
    ];
    let mut vm = VmState::new(code, vec![], 1000, 1024);
    let result = vm.run().unwrap();
    assert_eq!(result, Value::Int(8));
}

// Test 2: AIKR Enforcement - CPU Limit
#[test]
fn vm_enforces_cpu_limit() {
    // Create a program that will exceed the step limit
    // We'll use a series of Add operations that will consume all steps
    let code = vec![
        OpCode::Int(1),
        OpCode::Int(1),
        OpCode::Add, // Step 1
        OpCode::Int(1),
        OpCode::Add, // Step 2
        OpCode::Int(1),
        OpCode::Add, // Step 3
        OpCode::Int(1),
        OpCode::Add, // Step 4
        OpCode::Int(1),
        OpCode::Add, // Step 5
        OpCode::Int(1),
        OpCode::Add, // Step 6
        OpCode::Int(1),
        OpCode::Add, // Step 7
        OpCode::Int(1),
        OpCode::Add, // Step 8
        OpCode::Int(1),
        OpCode::Add, // Step 9
        OpCode::Int(1),
        OpCode::Add, // Step 10 - this should exceed the limit
    ];
    let mut vm = VmState::new(code, vec![], 10, 1024); // Limit: 10 steps
    let result = vm.run();
    assert!(matches!(result, Err(VmError::CpuLimitExceeded)));
}

// Test 3: Deterministic Replay
#[test]
fn vm_deterministic_replay() {
    let code = vec![OpCode::Int(7), OpCode::Dup, OpCode::Add];
    let mut vm1 = VmState::new(code.clone(), vec![], 100, 1024);
    let mut vm2 = VmState::new(code, vec![], 100, 1024);

    let result1 = vm1.run().unwrap();
    let result2 = vm2.run().unwrap();
    assert_eq!(result1, result2); // Must be identical

    // Also test that serialized states are equal
    let state1 = bincode::serialize(&vm1).unwrap();
    let state2 = bincode::serialize(&vm2).unwrap();
    assert_eq!(state1, state2);
}
