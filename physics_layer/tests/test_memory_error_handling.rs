/// Test for memory error handling
use physics_layer::memory_manager::{MemoryError, MemoryManager};

#[test]
fn test_memory_error_handling() {
    let mut memory_manager = MemoryManager::new();

    // Test allocation with invalid size
    let invalid_allocation = memory_manager.allocate(0);
    assert_eq!(invalid_allocation, Err(MemoryError::OutOfMemory));
}
