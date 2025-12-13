/// Test for memory management
use physics_layer::memory_manager::{MemoryError, MemoryManager};

#[test]
fn test_memory_management() {
    let mut memory_manager = MemoryManager::new();

    // Test allocation
    let block1 = memory_manager.allocate(100).unwrap();
    let block2 = memory_manager.allocate(200).unwrap();

    // Test that blocks are different
    assert_ne!(block1, block2);

    // Test freeing memory
    assert!(memory_manager.free(block1.clone()).is_ok());
    assert!(memory_manager.free(block2.clone()).is_ok());

    // Test double free prevention
    assert_eq!(memory_manager.free(block1), Err(MemoryError::DoubleFree));
}
