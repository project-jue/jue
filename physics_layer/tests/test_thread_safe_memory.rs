/// Thread Safe Memory Tests
use physics_layer::memory_manager::{MemoryError, MemoryManager, ThreadSafeMemoryManager};
use std::thread;

#[test]
fn test_thread_safe_memory_manager() {
    // Test thread-safe memory manager
    let manager = ThreadSafeMemoryManager::new();

    // Allocate memory from multiple threads
    let mut handles = vec![];

    for _ in 0..10 {
        let manager_clone = manager.clone();
        let handle = thread::spawn(move || {
            let ptr = manager_clone.allocate(1024).unwrap();
            manager_clone.free(ptr).unwrap();
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify no memory leaks - all memory should be freed
    let (allocated, freed, active) = manager.get_memory_stats();
    assert_eq!(allocated, 10240); // Total allocated
    assert_eq!(freed, 10240); // Total freed
    assert_eq!(active, 0); // No active blocks
}

#[test]
fn test_error_conditions() {
    // Test various error conditions
    let mut manager = MemoryManager::new();

    // Test allocation of zero bytes
    let result = manager.allocate(0);
    assert!(matches!(result, Err(MemoryError::OutOfMemory)));

    // Test snapshot/rollback on empty manager
    manager.snapshot().unwrap();
    manager.rollback().unwrap();
}

#[test]
fn test_persistent_structure_support() {
    // Test support for persistent data structures
    let mut manager = MemoryManager::new();

    // Allocate memory for persistent structure
    let ptr = manager.allocate(1024).unwrap();

    // Simulate writing persistent data
    let data = vec![0xAA; 1024];
    manager.write_memory(&ptr, 0, &data).unwrap();

    // Take snapshot
    manager.snapshot().unwrap();

    // Modify data
    let modified_data = vec![0xBB; 1024];
    manager.write_memory(&ptr, 0, &modified_data).unwrap();

    // Rollback to snapshot
    manager.rollback().unwrap();

    // Verify data is restored
    let buffer = manager.read_memory(&ptr, 0, 1024).unwrap();
    assert_eq!(buffer, data);

    // Free memory
    manager.free(ptr).unwrap();
}

#[test]
fn test_large_allocation() {
    // Test large memory allocation
    let mut manager = MemoryManager::new();

    // Allocate large block
    let large_size = 1024 * 1024; // 1MB
    let ptr = manager.allocate(large_size).unwrap();

    // Write to large allocation
    let data = vec![0xFF; 1024];
    for offset in (0..large_size).step_by(1024) {
        manager.write_memory(&ptr, offset, &data).unwrap();
    }

    // Read from large allocation
    let _buffer = vec![0u8; 1024];
    for offset in (0..large_size).step_by(1024) {
        let read_buffer = manager.read_memory(&ptr, offset, 1024).unwrap();
        assert_eq!(read_buffer, data);
    }

    // Free large allocation
    manager.free(ptr).unwrap();
}

#[test]
fn test_memory_performance() {
    // Test memory operation performance
    use std::time::Instant;

    let mut manager = MemoryManager::new();

    let start = Instant::now();

    // Allocate and free many small blocks
    for _ in 0..1000 {
        let ptr = manager.allocate(1024).unwrap();
        manager.free(ptr).unwrap();
    }

    let duration = start.elapsed();

    // Should complete in reasonable time
    assert!(duration.as_millis() < 1000);

    // Verify all memory is properly tracked
    let (allocated, freed, active) = manager.get_memory_stats();
    assert_eq!(allocated, 1024000); // Total allocated
    assert_eq!(freed, 1024000); // Total freed
    assert_eq!(active, 0); // No active blocks
}
