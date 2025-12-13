/// Physics Layer Integration Tests
use physics_layer::memory_manager::{MemoryError, MemoryManager, ThreadSafeMemoryManager};
use physics_layer::primitives::{add, div_i32, mul};
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_physics_core_integration() {
    // Test that physics layer operations can be used by core world
    let result = add(5, 3).unwrap();
    assert_eq!(result, 8);

    // Test that memory operations work in the context of the full system
    let mut memory_manager = MemoryManager::new();
    let block = memory_manager.allocate(1024).unwrap();
    assert!(memory_manager.free(block).is_ok());
}

#[test]
fn test_physics_jue_integration() {
    // Test that physics layer can support Jue world operations
    assert_eq!(mul(2, 3), Ok(6));
    assert_eq!(div_i32(10, 2), Ok(5));

    // Test memory operations for Jue data structures
    let mut memory_manager = MemoryManager::new();
    let block = memory_manager.allocate(2048).unwrap();
    assert!(memory_manager.free(block).is_ok());
}

#[test]
fn test_thread_safe_memory_manager() {
    let manager = ThreadSafeMemoryManager::new();

    // Test allocation
    let ptr1 = manager.allocate(100).unwrap();
    let ptr2 = manager.allocate(200).unwrap();

    // Test freeing
    assert!(manager.free(ptr1.clone()).is_ok());
    assert!(manager.free(ptr2.clone()).is_ok());

    // Test snapshot and rollback
    let ptr3 = manager.allocate(50).unwrap();
    assert!(manager.snapshot().is_ok());
    assert!(manager.free(ptr3.clone()).is_ok());
    assert!(manager.rollback().is_ok());
    assert!(manager.free(ptr3).is_ok());
}

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

#[test]
fn test_thread_safe_memory_operations() {
    let memory_manager = Arc::new(Mutex::new(MemoryManager::new()));
    let mut handles = vec![];

    // Spawn multiple threads to allocate and free memory
    for i in 0..10 {
        let mm = Arc::clone(&memory_manager);
        let handle = thread::spawn(move || {
            let mut mm = mm.lock().unwrap();
            let block = mm.allocate((i + 1) * 100);
            assert!(block.is_ok());
            let block = block.unwrap();
            assert!(mm.free(block).is_ok());
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify memory manager is still in consistent state
    let mm = memory_manager.lock().unwrap();
    let (_total, _freed, active) = mm.get_memory_stats();
    assert_eq!(active, 0);
}
