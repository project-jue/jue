/// Memory Allocation Tests
use physics_layer::memory_manager::{MemoryError, MemoryManager};

#[test]
fn test_memory_allocation_and_free() {
    // Test memory allocation and deallocation
    let mut manager = MemoryManager::new();

    // Allocate memory
    let ptr1 = manager.allocate(1024).unwrap();
    let ptr2 = manager.allocate(2048).unwrap();

    // Verify pointers are different
    assert_ne!(ptr1, ptr2);

    // Free memory
    manager.free(ptr1.clone()).unwrap();
    manager.free(ptr2.clone()).unwrap();

    // Test error handling
    let result = manager.free(ptr1);
    assert!(matches!(result, Err(MemoryError::DoubleFree)));
}

#[test]
fn test_snapshot_and_rollback() {
    // Test snapshot and rollback functionality
    let mut manager = MemoryManager::new();

    // Allocate some memory
    let ptr1 = manager.allocate(1024).unwrap();
    let ptr2 = manager.allocate(2048).unwrap();

    // Take a snapshot
    manager.snapshot().unwrap();

    // Allocate more memory
    let ptr3 = manager.allocate(4096).unwrap();

    // Rollback to snapshot
    manager.rollback().unwrap();

    // ptr3 should no longer be valid
    let result = manager.free(ptr3);
    assert!(matches!(result, Err(MemoryError::InvalidPointer)));

    // ptr1 and ptr2 should still be valid
    manager.free(ptr1).unwrap();
    manager.free(ptr2).unwrap();
}

#[test]
fn test_memory_operations() {
    // Test memory read/write operations
    let mut manager = MemoryManager::new();

    // Allocate memory
    let ptr = manager.allocate(1024).unwrap();

    // Write to memory
    let data = vec![1u8, 2, 3, 4, 5];
    manager.write_memory(&ptr, 0, &data).unwrap();

    // Read from memory
    let buffer = manager.read_memory(&ptr, 0, 5).unwrap();

    // Verify data matches
    assert_eq!(data, buffer);

    // Test error conditions
    let result = manager.write_memory(&ptr, 1024, &data);
    assert!(matches!(result, Err(MemoryError::OutOfMemory)));

    let result = manager.read_memory(&ptr, 1024, 5);
    assert!(matches!(result, Err(MemoryError::OutOfMemory)));

    // Free memory
    manager.free(ptr).unwrap();
}

#[test]
fn test_memory_stats() {
    // Test memory statistics tracking
    let mut manager = MemoryManager::new();

    // Initial stats
    let (allocated, freed, active) = manager.get_memory_stats();
    assert_eq!(allocated, 0);
    assert_eq!(freed, 0);
    assert_eq!(active, 0);

    // Allocate memory
    let ptr1 = manager.allocate(1024).unwrap();
    let ptr2 = manager.allocate(2048).unwrap();

    // Updated stats
    let (allocated, freed, active) = manager.get_memory_stats();
    assert_eq!(allocated, 3072);
    assert_eq!(freed, 0);
    assert_eq!(active, 2);

    // Free memory
    manager.free(ptr1).unwrap();

    // Final stats
    let (allocated, freed, active) = manager.get_memory_stats();
    assert_eq!(allocated, 3072);
    assert_eq!(freed, 1024);
    assert_eq!(active, 1);

    // Free remaining memory
    manager.free(ptr2).unwrap();
}
