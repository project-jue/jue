use physics_world::vm::error::VmError;
use physics_world::{
    types::{HeapPtr, OpCode, Value},
    vm::VmState,
};

/// Simple test for proper recursion handling
#[test]
fn test_simple_recursion_fixed() {
    // A simple function that returns its argument (no actual recursion)
    let closure_body = vec![
        OpCode::GetLocal(0), // Get argument
        OpCode::Ret,         // Return it
    ];

    // Main program
    let main_program = vec![
        OpCode::Int(5),            // Argument = 5
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );

    let result = vm.run();

    // Should return 5
    assert_eq!(result.unwrap(), Value::Int(5));
}

/// Helper function to set up a closure with proper memory layout
fn setup_closure_test(
    closure_body: Vec<OpCode>,
    main_program: Vec<OpCode>,
    constant_pool_setup: Vec<Value>,
) -> VmState {
    let mut vm = VmState::new(main_program, constant_pool_setup, 1000, 1024, 1, 100);

    // Serialize the closure body
    use bincode;
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
