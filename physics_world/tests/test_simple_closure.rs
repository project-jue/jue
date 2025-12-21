use bincode;
use physics_world::{
    types::{HeapPtr, OpCode, Value},
    vm::{VmError, VmState},
};

/// Simple test to verify basic closure execution works
/// This test uses the actual closure format expected by the VM

// Helper function to set up a closure with proper memory layout
fn setup_closure_test(
    closure_body: Vec<OpCode>,
    main_program: Vec<OpCode>,
    constant_pool_setup: Vec<Value>,
) -> VmState {
    let mut vm = VmState::new(main_program, constant_pool_setup, 1000, 1024, 1, 100);

    // Serialize the closure body
    let serialized_body = bincode::serialize(&closure_body).unwrap();
    let body_size = serialized_body.len() as u32;

    // Set up closure body in memory for each constant pool entry
    for i in 0..vm.constant_pool.len() {
        // Allocate closure body in memory (4 bytes size + serialized body)
        let body_ptr = vm.memory.allocate(body_size + 4, 2).unwrap();

        // Store size and body
        let body_data = unsafe { vm.memory.get_data_mut(body_ptr) };
        let size_bytes = body_size.to_le_bytes();
        body_data[0..4].copy_from_slice(&size_bytes);
        body_data[4..4 + serialized_body.len()].copy_from_slice(&serialized_body);

        // Update constant pool to point directly to closure body
        vm.constant_pool[i] = Value::Closure(body_ptr);
    }

    vm
}

#[test]
fn test_basic_closure_execution() {
    // Create a simple closure body that just returns a constant
    let closure_body = vec![OpCode::Int(42), OpCode::Ret];

    // Main program: create closure and call it
    let main_program = vec![
        OpCode::MakeClosure(0, 0), // Create closure (code_idx=0, capture_count=0)
        OpCode::Call(0),           // Call with 0 arguments
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );

    // Now execute
    let result = vm.run();

    // Should return 42
    assert_eq!(result.unwrap(), Value::Int(42));
}

#[test]
fn test_closure_with_arguments() {
    // Create a closure that adds two arguments
    let closure_body = vec![
        OpCode::GetLocal(0), // Get first argument
        OpCode::GetLocal(1), // Get second argument
        OpCode::Add,         // Add them
        OpCode::Ret,         // Return result
    ];

    // Main program: push arguments, create closure, call it
    let main_program = vec![
        OpCode::Int(5),            // First argument
        OpCode::Int(3),            // Second argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(2),           // Call with 2 arguments
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );

    // Execute
    let result = vm.run();

    // Should return 5 + 3 = 8
    assert_eq!(result.unwrap(), Value::Int(8));
}
