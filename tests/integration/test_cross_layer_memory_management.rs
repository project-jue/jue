
use physics_world::memory_manager::MemoryManager;

/// Test Memory Management Across Layers
#[test]
pub(crate) fn test_cross_layer_memory_management() {
    let mut memory_manager = MemoryManager::new();

    // Allocate memory for different layer components
    let core_memory = memory_manager.allocate(1024).unwrap(); // Core expressions
    let jue_memory = memory_manager.allocate(2048).unwrap(); // Jue AST and compiler
    let dan_memory = memory_manager.allocate(4096).unwrap(); // Dan modules and events

    // Take snapshot of initial state
    assert!(memory_manager.snapshot().is_ok());

    // Write data to each memory region
    let core_data = vec![1; 100];
    let jue_data = vec![2; 200];
    let dan_data = vec![3; 300];

    assert!(memory_manager
        .write_memory(&core_memory, 0, &core_data)
        .is_ok());
    assert!(memory_manager
        .write_memory(&jue_memory, 0, &jue_data)
        .is_ok());
    assert!(memory_manager
        .write_memory(&dan_memory, 0, &dan_data)
        .is_ok());

    // Verify data can be read back correctly
    let read_core = memory_manager
        .read_memory(&core_memory, 0, core_data.len())
        .unwrap();
    let read_jue = memory_manager
        .read_memory(&jue_memory, 0, jue_data.len())
        .unwrap();
    let read_dan = memory_manager
        .read_memory(&dan_memory, 0, dan_data.len())
        .unwrap();

    assert_eq!(read_core, core_data);
    assert_eq!(read_jue, jue_data);
    assert_eq!(read_dan, dan_data);

    // Take another snapshot
    assert!(memory_manager.snapshot().is_ok());

    // Modify data (simulate system evolution)
    let new_core_data = vec![4; 50];
    assert!(memory_manager
        .write_memory(&core_memory, 0, &new_core_data)
        .is_ok());

    // Test rollback to previous state
    assert!(memory_manager.rollback().is_ok());

    // Verify we're back to previous state
    let read_core_after_rollback = memory_manager
        .read_memory(&core_memory, 0, core_data.len())
        .unwrap();
    assert_eq!(read_core_after_rollback, core_data);

    // Test final rollback to initial state
    assert!(memory_manager.rollback().is_ok());

    // Verify all memory is back to initial (zero) state
    let initial_core = memory_manager
        .read_memory(&core_memory, 0, core_data.len())
        .unwrap();
    assert_eq!(initial_core, vec![0; 100]);

    // Clean up
    assert!(memory_manager.free(core_memory).is_ok());
    assert!(memory_manager.free(jue_memory).is_ok());
    assert!(memory_manager.free(dan_memory).is_ok());
}
