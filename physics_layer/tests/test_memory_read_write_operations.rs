/// Test for memory read/write operations
use physics_layer::memory_manager::MemoryManager;

#[test]
fn test_memory_read_write_operations() {
    let mut memory_manager = MemoryManager::new();

    // Allocate memory
    let block = memory_manager.allocate(16).unwrap();

    // Write data
    let test_data = b"Hello, World!";
    assert!(memory_manager.write_memory(&block, 0, test_data).is_ok());

    // Read data back
    let read_data = memory_manager
        .read_memory(&block, 0, test_data.len())
        .unwrap();
    assert_eq!(read_data, test_data);

    // Test partial read
    let partial_data = memory_manager.read_memory(&block, 0, 5).unwrap();
    assert_eq!(partial_data, b"Hello");

    // Test invalid operations
    assert_eq!(
        memory_manager.read_memory(&block, 20, 10),
        Err(physics_layer::memory_manager::MemoryError::OutOfMemory)
    );
}
