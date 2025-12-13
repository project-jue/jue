/// Test for snapshot and rollback
use physics_layer::memory_manager::MemoryManager;

#[test]
fn test_snapshot_and_rollback() {
    let mut memory_manager = MemoryManager::new();

    // Allocate some memory
    let block1 = memory_manager.allocate(100).unwrap();
    let block2 = memory_manager.allocate(200).unwrap();

    // Take snapshot
    assert!(memory_manager.snapshot().is_ok());

    // Free some memory
    assert!(memory_manager.free(block1.clone()).is_ok());

    // Rollback should restore the freed memory
    assert!(memory_manager.rollback().is_ok());

    // The block should be valid again
    assert!(memory_manager.free(block1).is_ok());
    assert!(memory_manager.free(block2).is_ok());
}
