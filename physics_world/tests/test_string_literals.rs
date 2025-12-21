/// Test string literal compilation and execution in Physics-World
use physics_world::types::{OpCode, Value};
use physics_world::vm::state::VmState;

#[test]
fn test_string_literal_compilation() {
    // Test that string literals compile to LoadString opcodes
    let string_literal = "Hello, World!";
    
    // Create a simple VM state with string constant pool
    let mut vm = VmState::new(
        vec![], // No initial instructions
        vec![Value::String(string_literal.to_string())], // String in constant pool
        100,    // Step limit
        1024,   // Memory limit
        1,      // Actor ID
        100,    // Max recursion depth
    );
    
    // Add LoadString instruction to load from constant pool index 0
    vm.instructions = vec![OpCode::LoadString(0)];
    
    // Execute the instruction
    let result = vm.run();
    
    // Verify the result
    assert!(result.is_ok());
    let value = result.unwrap();
    match value {
        Value::String(s) => assert_eq!(s, string_literal),
        _ => panic!("Expected String value, got {:?}", value),
    }
}

#[test]
fn test_string_length_operation() {
    let test_string = "Hello";
    
    let mut vm = VmState::new(
        vec![
            OpCode::LoadString(0), // Load string
            OpCode::StrLen,        // Get length
        ],
        vec![Value::String(test_string.to_string())],
        100,
        1024,
        1,
        100,
    );
    
    let result = vm.run();
    assert!(result.is_ok());
    
    match result.unwrap() {
        Value::Int(len) => assert_eq!(len, 5), // "Hello" has 5 characters
        _ => panic!("Expected Int value for string length"),
    }
}

#[test]
fn test_string_concatenation() {
    let string1 = "Hello";
    let string2 = "World";
    
    let mut vm = VmState::new(
        vec![
            OpCode::LoadString(0), // Load first string
            OpCode::LoadString(1), // Load second string
            OpCode::StrConcat,     // Concatenate
        ],
        vec![
            Value::String(string1.to_string()),
            Value::String(string2.to_string()),
        ],
        100,
        1024,
        1,
        100,
    );
    
    let result = vm.run();
    assert!(result.is_ok());
    
    match result.unwrap() {
        Value::String(s) => assert_eq!(s, "HelloWorld"),
        _ => panic!("Expected String value for concatenation"),
    }
}

#[test]
fn test_string_indexing() {
    let test_string = "Hello";
    let char_index = 1; // 'e'
    
    let mut vm = VmState::new(
        vec![
            OpCode::LoadString(0), // Load string
            OpCode::Int(char_index), // Load index
            OpCode::StrIndex,        // Get character at index
        ],
        vec![Value::String(test_string.to_string())],
        100,
        1024,
        1,
        100,
    );
    
    let result = vm.run();
    assert!(result.is_ok());
    
    match result.unwrap() {
        Value::String(s) => assert_eq!(s, "e"),
        _ => panic!("Expected String value for character"),
    }
}

#[test]
fn test_string_index_out_of_bounds() {
    let test_string = "Hi";
    let char_index = 5; // Out of bounds
    
    let mut vm = VmState::new(
        vec![
            OpCode::LoadString(0), // Load string
            OpCode::Int(char_index), // Load out-of-bounds index
            OpCode::StrIndex,        // Get character at index
        ],
        vec![Value::String(test_string.to_string())],
        100,
        1024,
        1,
        100,
    );
    
    let result = vm.run();
    assert!(result.is_ok());
    
    match result.unwrap() {
        Value::Nil => {}, // Out of bounds should return Nil
        _ => panic!("Expected Nil value for out-of-bounds index"),
    }
}

#[test]
fn test_multiple_string_operations() {
    let string1 = "Hello";
    let string2 = " ";
    let string3 = "World";
    
    let mut vm = VmState::new(
        vec![
            OpCode::LoadString(0), // Load "Hello"
            OpCode::LoadString(1), // Load " "
            OpCode::StrConcat,     // "Hello "
            OpCode::LoadString(2), // Load "World"
            OpCode::StrConcat,     // "Hello World"
            OpCode::StrLen,        // Get length
        ],
        vec![
            Value::String(string1.to_string()),
            Value::String(string2.to_string()),
            Value::String(string3.to_string()),
        ],
        100,
        1024,
        1,
        100,
    );
    
    let result = vm.run();
    assert!(result.is_ok());
    
    match result.unwrap() {
        Value::Int(len) => assert_eq!(len, 11), // "Hello World" has 11 characters
        _ => panic!("Expected Int value for final length"),
    }
}

#[test]
fn test_empty_string_operations() {
    let empty_string = "";
    
    let mut vm = VmState::new(
        vec![
            OpCode::LoadString(0), // Load empty string
            OpCode::StrLen,        // Get length
        ],
        vec![Value::String(empty_string.to_string())],
        100,
        1024,
        1,
        100,
    );
    
    let result = vm.run();
    assert!(result.is_ok());
    
    match result.unwrap() {
        Value::Int(len) => assert_eq!(len, 0), // Empty string has length 0
        _ => panic!("Expected Int value for empty string length"),
    }
}

#[test]
fn test_unicode_string_operations() {
    let unicode_string = "Hello 🌍";
    
    let mut vm = VmState::new(
        vec![
            OpCode::LoadString(0), // Load unicode string
            OpCode::StrLen,        // Get length (should count Unicode characters)
        ],
        vec![Value::String(unicode_string.to_string())],
        100,
        1024,
        1,
        100,
    );
    
    let result = vm.run();
    assert!(result.is_ok());
    
    match result.unwrap() {
        Value::Int(len) => assert_eq!(len, 10), // "Hello 🌍" has 10 bytes in UTF-8
        _ => panic!("Expected Int value for unicode string length"),
    }
}