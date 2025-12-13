/// Test for memory statistics
use physics_layer::memory_manager::MemoryManager;

#[test]
fn test_memory_statistics() {
    let mut memory_manager = MemoryManager::new();

    // Get initial stats
    let (total, freed, active) = memory_manager.get_memory_stats();
    assert_eq!(total, 0);
    assert_eq!(freed, 0);
    assert_eq!(active, 0);

    // Allocate some memory
    let block1 = memory_manager.allocate(100).unwrap();
    let _block2 = memory_manager.allocate(200).unwrap();

    // Check updated stats
    let (total, freed, active) = memory_manager.get_memory_stats();
    assert_eq!(total, 300);
    assert_eq!(freed, 0);
    assert_eq!(active, 2);

    // Free one block
    memory_manager.free(block1).unwrap();

    // Check final stats
    let (total, freed, active) = memory_manager.get_memory_stats();
    assert_eq!(total, 300);
    assert_eq!(freed, 100);
    assert_eq!(active, 1);
}
