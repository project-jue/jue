/// Test for memory defragmentation
use physics_layer::memory_manager::MemoryManager;

#[test]
fn test_memory_defragmentation() {
    let mut memory_manager = MemoryManager::new();

    // Create fragmentation
    let block1 = memory_manager.allocate(100).unwrap();
    let block2 = memory_manager.allocate(200).unwrap();
    let block3 = memory_manager.allocate(300).unwrap();

    // Free middle block
    memory_manager.free(block2).unwrap();

    // Should be able to allocate a larger contiguous block
    let large_block = memory_manager.allocate(500);
    assert!(large_block.is_ok());

    // Clean up
    memory_manager.free(block1).unwrap();
    memory_manager.free(block3).unwrap();
    memory_manager.free(large_block.unwrap()).unwrap();
}
