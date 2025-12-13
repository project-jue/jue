/// Test for memory leak detection
use physics_layer::memory_manager::MemoryManager;

#[test]
fn test_memory_leak_detection() {
    let mut memory_manager = MemoryManager::new();

    // Allocate some blocks
    let block1 = memory_manager.allocate(100).unwrap();
    let block2 = memory_manager.allocate(200).unwrap();

    // Check for leaks (should find 2 allocated blocks)
    let (_total, _freed, active) = memory_manager.get_memory_stats();
    assert_eq!(active, 2);

    // Free one block
    memory_manager.free(block1).unwrap();

    // Should still have 1 active block
    let (_total, _freed, active) = memory_manager.get_memory_stats();
    assert_eq!(active, 1);

    // Free the other block
    memory_manager.free(block2).unwrap();

    // Should have no active blocks
    let (_total, _freed, active) = memory_manager.get_memory_stats();
    assert_eq!(active, 0);
}
