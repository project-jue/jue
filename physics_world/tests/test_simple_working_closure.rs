use physics_world::{
    types::{HeapPtr, OpCode, Value},
    vm::VmState,
};

#[test]
fn test_simple_working_closure() {
    // Create a simple function that adds two numbers
    let closure_body = vec![
        OpCode::GetLocal(1), // Get second argument
        OpCode::GetLocal(0), // Get first argument
        OpCode::Add,         // Add them
        OpCode::Ret,         // Return result
    ];

    // Create closure body in memory first
    let serialized_body = bincode::serialize(&closure_body).unwrap();
    let body_size = serialized_body.len() as u32;

    // Main program: push arguments, create closure body, create closure, call it
    let main_program = vec![
        OpCode::Int(5),            // First argument
        OpCode::Int(3),            // Second argument
        // Here we would normally create the closure body in memory
        // But for this test, we'll use a simpler approach
        OpCode::MakeClosure(0, 0), // Create closure (code_idx=0, capture_count=0)
        OpCode::Call(2),           // Call with 2 arguments
    ];

    // Create proper constant pool with closure body
    let body_ptr = HeapPtr::new(1); // Simulated pointer to body data
    let constant_pool = vec![Value::Closure(body_ptr)];

    let mut vm = VmState::new(main_program, constant_pool, 1000, 1024, 1, 100);

    // Manually set up the closure body in memory to simulate proper setup
    // This is what would normally be done by the compiler
    let body_data_size = 4 + serialized_body.len() as u32;
    let body_ptr = vm.memory.allocate(body_data_size, 2).unwrap();
    let body_data = unsafe { vm.memory.get_data_mut(body_ptr) };
    body_data[0..4].copy_from_slice(&body_size.to_le_bytes());
    body_data[4..4 + serialized_body.len()].copy_from_slice(&serialized_body);

    // Update constant pool to point to actual body
    vm.constant_pool = vec![Value::Closure(body_ptr)];

    let result = vm.run();

    // Should return 5 + 3 = 8
    assert_eq!(result.unwrap(), Value::Int(8));
}