/// Test for large memory allocations
use physics_layer::memory_manager::MemoryManager;

#[test]
fn test_large_memory_allocations() {
    let mut memory_manager = MemoryManager::new();

    // Test allocation of large blocks
    let large_block = memory_manager.allocate(1024 * 1024).unwrap(); // 1MB

    // Should be able to free it
    assert!(memory_manager.free(large_block).is_ok());
}
