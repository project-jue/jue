/// Physics Layer Performance Tests
use physics_layer::memory_manager::MemoryManager;
use physics_layer::primitives::{add, mul, sub};
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_high_volume_arithmetic() {
    // Test performance of arithmetic operations under load
    let start_time = std::time::Instant::now();

    for _ in 0..10000 {
        let _ = add(5, 3);
        let _ = sub(10, 4);
        let _ = mul(3, 7);
    }

    let duration = start_time.elapsed();
    println!("10,000 arithmetic operations completed in {:?}", duration);
}

#[test]
fn test_memory_stress() {
    // Test memory manager under stress
    let mut memory_manager = MemoryManager::new();
    let start_time = std::time::Instant::now();

    // Allocate and free many blocks
    for i in 0..1000 {
        let size = (i % 100) + 1; // Vary size from 1 to 100
        let block = memory_manager.allocate(size);
        if block.is_ok() {
            memory_manager.free(block.unwrap()).unwrap();
        }
    }

    let duration = start_time.elapsed();
    println!("1,000 memory operations completed in {:?}", duration);

    // Verify no memory leaks
    let (_total, _freed, active) = memory_manager.get_memory_stats();
    assert_eq!(active, 0);
}

#[test]
fn test_concurrent_memory_access() {
    // Test concurrent memory access patterns
    let memory_manager = Arc::new(Mutex::new(MemoryManager::new()));
    let mut handles = vec![];

    for thread_id in 0..20 {
        let mm = Arc::clone(&memory_manager);
        let handle = thread::spawn(move || {
            let mut mm = mm.lock().unwrap();

            // Each thread allocates multiple blocks
            let mut blocks = vec![];
            for i in 0..10 {
                let block = mm.allocate((thread_id * 10) + i);
                if block.is_ok() {
                    blocks.push(block.unwrap());
                }
            }

            // Free them in reverse order
            for block in blocks {
                mm.free(block).unwrap();
            }
        });
        handles.push(handle);
    }

    // Wait for completion
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify consistency
    let mm = memory_manager.lock().unwrap();
    let (_total, _freed, active) = mm.get_memory_stats();
    assert_eq!(active, 0);
}

#[test]
fn test_persistent_structure_support() {
    let mut memory_manager = MemoryManager::new();

    // Allocate memory for persistent structure
    let block = memory_manager.allocate(1024).unwrap();

    // Take snapshot of initial state (all zeros)
    assert!(memory_manager.snapshot().is_ok());

    // Modify the memory (simulating structure updates)
    let data1 = vec![42; 100]; // Use a distinct value
    assert!(memory_manager.write_memory(&block, 0, &data1).is_ok());

    // Take another snapshot
    assert!(memory_manager.snapshot().is_ok());

    // Modify again - change the first byte to verify rollback works
    let data2 = vec![99; 1]; // Different value
    assert!(memory_manager.write_memory(&block, 0, &data2).is_ok());

    // Verify current state (first byte should be 99)
    let current_data = memory_manager.read_memory(&block, 0, 1).unwrap();
    assert_eq!(current_data, vec![99]);

    // Rollback to second snapshot
    assert!(memory_manager.rollback().is_ok());

    // Verify we're back to first modification (first byte should be 42)
    let read_data = memory_manager.read_memory(&block, 0, 1).unwrap();
    assert_eq!(read_data, vec![42]);

    // Rollback to first snapshot
    assert!(memory_manager.rollback().is_ok());

    // Verify we're back to initial state (first byte should be 0)
    let read_data = memory_manager.read_memory(&block, 0, 1).unwrap();
    assert_eq!(read_data, vec![0]);
}
