use bincode;
use physics_world::vm::error::VmError;
use physics_world::{
    types::{HeapPtr, OpCode, Value},
    vm::VmState,
};

/// Comprehensive tests for closure execution functionality
/// Tests the proper implementation of function calls and returns

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
        // The VM expects Value::Closure(body_ptr) where body_ptr points to the closure body data
        vm.constant_pool[i] = Value::Closure(body_ptr);
    }

    vm
}

// Test 1: Simple function call
#[test]
fn test_simple_function_call() {
    // Create a simple function that adds two numbers
    let closure_body = vec![
        OpCode::GetLocal(1), // Get second argument
        OpCode::GetLocal(0), // Get first argument
        OpCode::Add,         // Add them
        OpCode::Ret,         // Return result
    ];

    // Main program: push arguments, create closure, call it
    let main_program = vec![
        OpCode::Int(5),            // First argument
        OpCode::Int(3),            // Second argument
        OpCode::MakeClosure(0, 0), // Create closure (code_idx=0, capture_count=0)
        OpCode::Call(2),           // Call with 2 arguments
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );

    let result = vm.run();

    // Should return 5 + 3 = 8
    assert_eq!(result.unwrap(), Value::Int(8));
}

// Test 2: Function with local variables
#[test]
fn test_function_with_local_variables() {
    // Function that uses local variables
    let closure_body = vec![
        OpCode::GetLocal(0), // Get argument
        OpCode::Int(10),     // Push constant
        OpCode::Add,         // Add to argument
        OpCode::SetLocal(1), // Store in local variable
        OpCode::GetLocal(1), // Retrieve local variable
        OpCode::Int(2),      // Push another constant
        OpCode::Mul,         // Multiply
        OpCode::Ret,         // Return result
    ];

    // Main program
    let main_program = vec![
        OpCode::Int(7),            // Argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call with 1 argument
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );

    let result = vm.run();

    // Should return ((7 + 10) * 2) = 34
    assert_eq!(result.unwrap(), Value::Int(34));
}

// Test 3: Nested function calls
#[test]
fn test_nested_function_calls() {
    // Inner function (adds 1 to argument)
    let inner_closure_body = vec![
        OpCode::GetLocal(0),
        OpCode::Int(1),
        OpCode::Add,
        OpCode::Ret,
    ];

    // Outer function (calls inner function once to test basic nested call)
    let outer_closure_body = vec![
        OpCode::GetLocal(0),       // Get argument
        OpCode::MakeClosure(1, 0), // Create inner closure (code_idx=1)
        OpCode::Call(1),           // Call inner function
        OpCode::Ret,               // Return the result
    ];

    // Main program
    let main_program = vec![
        OpCode::Int(5),            // Argument
        OpCode::MakeClosure(0, 0), // Create outer closure (code_idx=0)
        OpCode::Call(1),           // Call outer function
    ];

    // Create VM with both closures in constant pool
    let mut vm = VmState::new(
        main_program,
        vec![
            Value::Closure(HeapPtr::new(0)), // Outer closure placeholder
            Value::Closure(HeapPtr::new(1)), // Inner closure placeholder
        ],
        1000,
        1024,
        1,
        100,
    );

    // Set up outer closure body in memory (index 0)
    let outer_serialized = bincode::serialize(&outer_closure_body).unwrap();
    let outer_size = outer_serialized.len() as u32;
    let outer_body_ptr = vm.memory.allocate(outer_size + 4, 2).unwrap();
    let outer_body_data = unsafe { vm.memory.get_data_mut(outer_body_ptr) };
    let outer_size_bytes = outer_size.to_le_bytes();
    outer_body_data[0..4].copy_from_slice(&outer_size_bytes);
    outer_body_data[4..4 + outer_serialized.len()].copy_from_slice(&outer_serialized);
    vm.constant_pool[0] = Value::Closure(outer_body_ptr);

    // Set up inner closure body in memory (index 1)
    let inner_serialized = bincode::serialize(&inner_closure_body).unwrap();
    let inner_size = inner_serialized.len() as u32;
    let inner_body_ptr = vm.memory.allocate(inner_size + 4, 2).unwrap();
    let inner_body_data = unsafe { vm.memory.get_data_mut(inner_body_ptr) };
    let inner_size_bytes = inner_size.to_le_bytes();
    inner_body_data[0..4].copy_from_slice(&inner_size_bytes);
    inner_body_data[4..4 + inner_serialized.len()].copy_from_slice(&inner_serialized);
    vm.constant_pool[1] = Value::Closure(inner_body_ptr);

    let result = vm.run();

    // Should return (5 + 1) = 6 (outer calls inner once)
    assert_eq!(result.unwrap(), Value::Int(6));
}

// Test 4: Function with multiple arguments
#[test]
fn test_multiple_arguments() {
    // Function that adds three numbers
    let closure_body = vec![
        OpCode::GetLocal(0), // First arg
        OpCode::GetLocal(1), // Second arg
        OpCode::Add,         // Add first two
        OpCode::GetLocal(2), // Third arg
        OpCode::Add,         // Add result
        OpCode::Ret,
    ];

    // Main program
    let main_program = vec![
        OpCode::Int(1),            // First argument
        OpCode::Int(2),            // Second argument
        OpCode::Int(3),            // Third argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(3),           // Call with 3 arguments
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );

    let result = vm.run();

    // Should return 1 + 2 + 3 = 6
    assert_eq!(result.unwrap(), Value::Int(6));
}

// Test 5: Function return value handling
#[test]
fn test_return_value_handling() {
    // Function that returns different types
    let closure_body = vec![
        OpCode::GetLocal(0),   // Get condition
        OpCode::Int(0),        // Push 0
        OpCode::Eq,            // condition == 0?
        OpCode::JmpIfFalse(3), // If false, jump to else
        OpCode::Int(42),       // Return 42
        OpCode::Ret,
        OpCode::Bool(true), // Return true
        OpCode::Ret,
    ];

    // Test with condition = 0
    let main_program_true = vec![
        OpCode::Int(0),            // Condition = 0
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function
    ];

    let mut vm = setup_closure_test(
        closure_body.clone(),
        main_program_true.clone(),
        vec![Value::Closure(HeapPtr::new(0))],
    );
    let result = vm.run();
    assert_eq!(result.unwrap(), Value::Int(42));

    // Test with condition = 1
    let main_program_false = vec![
        OpCode::Int(1),            // Condition = 1
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program_false,
        vec![Value::Closure(HeapPtr::new(0))],
    );
    let result = vm.run();
    assert_eq!(result.unwrap(), Value::Bool(true));
}

// Test 6: Stack frame isolation
#[test]
fn test_stack_frame_isolation() {
    // Function that manipulates stack
    let closure_body = vec![
        OpCode::GetLocal(0), // Get argument
        OpCode::Dup,         // Duplicate it
        OpCode::Add,         // Add to itself (x + x)
        OpCode::SetLocal(0), // Store back (modifies local)
        OpCode::GetLocal(0), // Get modified value
        OpCode::Ret,
    ];

    // Main program with local variables
    let main_program = vec![
        OpCode::Int(10),           // Local variable (not used in this test)
        OpCode::Int(5),            // Argument for function
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function (should return 5+5=10)
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );

    let result = vm.run();

    // Function should return 5 + 5 = 10
    assert_eq!(result.unwrap(), Value::Int(10));
}

// Test 7: Error handling in function calls
#[test]
fn test_function_call_errors() {
    // Function that causes stack underflow
    let closure_body = vec![
        OpCode::Pop, // Will cause underflow
        OpCode::Ret,
    ];

    // Main program
    let main_program = vec![
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(0),           // Call with 0 arguments
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );

    let result = vm.run();

    // Should get stack underflow error
    assert!(matches!(result, Err(VmError::StackUnderflow { .. })));
}

// Test 8: Simple recursion test
#[test]
fn test_simple_recursion() {
    // Simple recursive function that counts down
    let mut closure_body = vec![
        OpCode::GetLocal(0),   // Get n (0)
        OpCode::Int(0),        // Check if n == 0 (1)
        OpCode::Eq,            // Compare (2)
        OpCode::JmpIfFalse(3), // If not zero, continue (3)
        OpCode::Int(0),        // Base case: return 0 (4)
        OpCode::Ret,           // Return (5)
    ];

    // Recursive case: return 1 + countdown(n-1) (6-12)
    closure_body.extend(vec![
        OpCode::GetLocal(0),       // Get n (6)
        OpCode::Int(1),            // Push 1 (7)
        OpCode::Sub,               // Subtract: n - 1 (8)
        OpCode::MakeClosure(0, 0), // Create same closure (9)
        OpCode::Call(1),           // Recursive call with n-1 (10)
        OpCode::Int(1),            // Push 1 (11)
        OpCode::Add,               // Add 1 + result (12)
        OpCode::Ret,               // Return (13)
    ]);

    // Main program: call with n=2 (should return 2)
    let main_program = vec![
        OpCode::Int(2),            // n = 2
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );

    let result = vm.run();

    // Should return 2 (1 + (1 + 0))
    assert_eq!(result.unwrap(), Value::Int(2));
}

// Test 9: Deep call stack (recursion simulation)
#[test]
fn test_deep_call_stack() {
    // Function that calls itself recursively (simulated)
    // Corrected bytecode structure with proper jump offsets
    let mut closure_body = vec![
        OpCode::GetLocal(0),   // Get depth (0)
        OpCode::Int(0),        // Check if depth == 0 (1)
        OpCode::Eq,            // Compare (2)
        OpCode::JmpIfFalse(3), // If not zero, jump to recursive case (3)
        OpCode::Int(1),        // Base case: return 1 (4)
        OpCode::Ret,           // Return (5)
    ];

    // Recursive case: depth * factorial(depth - 1) (6-13)
    closure_body.extend(vec![
        OpCode::GetLocal(0),       // Get current depth (6)
        OpCode::Int(1),            // Push 1 (7)
        OpCode::Sub,               // Subtract: 3 - 1 = 2 (8)
        OpCode::MakeClosure(0, 0), // Create same closure (9)
        OpCode::Call(1),           // Recursive call with (depth - 1) (10)
        OpCode::GetLocal(0),       // Get original depth (11)
        OpCode::Mul,               // Multiply: 3 * result_of_recursive_call (12)
        OpCode::Ret,               // Return (13)
    ]);

    // Main program: call with depth 3 (should return 3! = 6)
    let main_program = vec![
        OpCode::Int(3),            // Depth = 3
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );

    let result = vm.run();

    // Should return 3 * 2 * 1 = 6
    assert_eq!(result.unwrap(), Value::Int(6));
}

// Test 9: Closure with captured variables
#[test]
fn test_closure_capture() {
    // Function that should capture and use a variable
    let closure_body = vec![
        // In a real implementation, captured variables would be accessible
        // For now, we'll just return a constant to verify the call works
        OpCode::Int(99),
        OpCode::Ret,
    ];

    // Main program
    let main_program = vec![
        OpCode::Int(42),           // Value to capture (not actually used yet)
        OpCode::MakeClosure(0, 1), // Create closure capturing 1 variable
        OpCode::Call(0),           // Call with 0 arguments
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );

    let result = vm.run();

    // Should return 99 (closure works)
    assert_eq!(result.unwrap(), Value::Int(99));
}

// Test 10: Function call with no return value
#[test]
fn test_no_return_value() {
    // Function that doesn't explicitly return
    let closure_body = vec![
        OpCode::GetLocal(0), // Get argument
        OpCode::Pop,         // Pop it (no return value)
                             // No Ret instruction - should return Nil
    ];

    // Main program
    let main_program = vec![
        OpCode::Int(42),           // Argument
        OpCode::MakeClosure(0, 0), // Create closure
        OpCode::Call(1),           // Call function
    ];

    let mut vm = setup_closure_test(
        closure_body,
        main_program,
        vec![Value::Closure(HeapPtr::new(0))],
    );

    let result = vm.run();

    // Should return Nil (no explicit return)
    assert_eq!(result.unwrap(), Value::Nil);
}
