use bincode;
use physics_world::{
    types::{OpCode, Value},
    vm::VmState,
};

#[test]
fn test_simple_closure_capture() {
    // Simple test to verify closure capture works

    // Create a simple closure that just returns a constant
    let closure_body = vec![OpCode::Int(99), OpCode::Ret];

    // Main program: push value to capture, create closure, call it
    let main_program = vec![
        OpCode::Int(42),           // Value to capture
        OpCode::MakeClosure(0, 0), // Create closure capturing 0 variables (for now)
        OpCode::Call(0),           // Call with 0 arguments
    ];

    // Set up VM with larger memory limit and proper constant pool
    let mut vm = VmState::new(main_program, vec![Value::Nil], 1000, 2048, 1, 100);

    // Manually set up the closure body in memory
    let serialized_body = bincode::serialize(&closure_body).unwrap();
    let body_size = serialized_body.len() as u32;

    // Allocate closure body in memory
    let body_ptr = vm.memory.allocate(body_size + 4, 2).unwrap();

    // Store size and body
    let body_data = unsafe { vm.memory.get_data_mut(body_ptr) };
    let size_bytes = body_size.to_le_bytes();
    body_data[0..4].copy_from_slice(&size_bytes);
    body_data[4..4 + serialized_body.len()].copy_from_slice(&serialized_body);

    // Set up the constant pool properly - index 0 should contain the closure body
    vm.constant_pool[0] = Value::Closure(body_ptr);

    let result = vm.run();

    // Should return 99 (closure works)
    assert_eq!(result.unwrap(), Value::Int(99));
}
