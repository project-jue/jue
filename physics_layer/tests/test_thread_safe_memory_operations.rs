/// Test for thread safe memory operations
use physics_layer::memory_manager::MemoryManager;
use std::sync::{Arc, Mutex};
use std::thread;

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
