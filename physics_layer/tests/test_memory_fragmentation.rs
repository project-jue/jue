/// Test for memory fragmentation handling
use physics_layer::memory_manager::MemoryManager;

#[test]
fn test_memory_fragmentation() {
    let mut memory_manager = MemoryManager::new();

    // Allocate and free blocks in non-sequential order
    let block1 = memory_manager.allocate(100).unwrap();
    let block2 = memory_manager.allocate(200).unwrap();
    let block3 = memory_manager.allocate(300).unwrap();

    // Free middle block first
    memory_manager.free(block2.clone()).unwrap();

    // Should still be able to allocate
    let block4 = memory_manager.allocate(150);
    assert!(block4.is_ok());

    // Clean up
    memory_manager.free(block1).unwrap();
    memory_manager.free(block3).unwrap();
    memory_manager.free(block4.unwrap()).unwrap();
}
