use physics_layer::memory_manager::{MemoryError, MemoryManager, ThreadSafeMemoryManager};
/// Physics Layer Comprehensive Test Suite
/// This file contains comprehensive unit tests for all Physics Layer components
/// Tests primitive operations, memory management, and atomicity guarantees
use physics_layer::primitives::{add, div_f64, div_i32, mul, sub, ArithmeticError};
use std::sync::{Arc, Mutex};
use std::thread;

#[cfg(test)]
mod physics_layer_tests {
    use super::*;

    /// Test Arithmetic Operations
    #[test]
    fn test_arithmetic_operations() {
        // Test addition
        assert_eq!(add(5, 3), Ok(8));
        assert_eq!(add(-2, 7), Ok(5));
        assert_eq!(add(0, 0), Ok(0));

        // Test subtraction
        assert_eq!(sub(10, 4), Ok(6));
        assert_eq!(sub(5, 10), Ok(-5));
        assert_eq!(sub(0, 0), Ok(0));

        // Test multiplication
        assert_eq!(mul(3, 7), Ok(21));
        assert_eq!(mul(-2, 5), Ok(-10));
        assert_eq!(mul(0, 100), Ok(0));

        // Test division
        assert_eq!(div_i32(15, 3), Ok(5));
        assert_eq!(div_f64(15.0, 3.0), Ok(5.0));
    }

    /// Test Division by Zero Error Handling
    #[test]
    fn test_division_by_zero() {
        assert_eq!(div_i32(10, 0), Err(ArithmeticError::DivisionByZero));
        assert_eq!(div_f64(5.0, 0.0), Err(ArithmeticError::DivisionByZero));
    }

    /// Test Arithmetic Properties
    #[test]
    fn test_arithmetic_properties() {
        // Commutative property of addition
        assert_eq!(add(3, 5), add(5, 3));

        // Distributive property
        let a = 2;
        let b = 3;
        let c = 4;
        let left = mul(a, add(b, c).unwrap()).unwrap();
        let right = add(mul(a, b).unwrap(), mul(a, c).unwrap()).unwrap();
        assert_eq!(left, right);

        // Identity properties
        assert_eq!(add(7, 0), Ok(7));
        assert_eq!(mul(9, 1), Ok(9));
    }

    /// Test Memory Management
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

    /// Test Memory Fragmentation Handling
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

    /// Test Thread Safety of Memory Operations
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

    /// Test Atomic Operations
    #[test]
    fn test_atomic_operations() {
        let memory_manager = Arc::new(Mutex::new(MemoryManager::new()));
        let counter = Arc::new(Mutex::new(0));

        let mut handles = vec![];

        // Spawn threads that perform atomic operations
        for _ in 0..5 {
            let mm = Arc::clone(&memory_manager);
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut mm = mm.lock().unwrap();
                let block = mm.allocate(100).unwrap();

                // Simulate some work
                let mut counter = counter.lock().unwrap();
                *counter += 1;

                mm.free(block).unwrap();
            });
            handles.push(handle);
        }

        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }

        // Verify all operations completed
        let counter = counter.lock().unwrap();
        assert_eq!(*counter, 5);
    }

    /// Test Memory Leak Detection
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

    /// Test Large Memory Allocations
    #[test]
    fn test_large_memory_allocations() {
        let mut memory_manager = MemoryManager::new();

        // Test allocation of large blocks
        let large_block = memory_manager.allocate(1024 * 1024).unwrap(); // 1MB

        // Should be able to free it
        assert!(memory_manager.free(large_block).is_ok());
    }

    /// Test Memory Defragmentation
    #[test]
    fn test_memory_defragmentation() {
        let mut memory_manager = MemoryManager::new();

        // Create fragmentation
        let block1 = memory_manager.allocate(100).unwrap();
        let block2 = memory_manager.allocate(200).unwrap();
        let block3 = memory_manager.allocate(300).unwrap();

        // Free middle block
        memory_manager.free(block2).unwrap();

        // Should be able to allocate a larger contiguous block
        let large_block = memory_manager.allocate(500);
        assert!(large_block.is_ok());

        // Clean up
        memory_manager.free(block1).unwrap();
        memory_manager.free(block3).unwrap();
        memory_manager.free(large_block.unwrap()).unwrap();
    }

    /// Test Error Handling in Memory Operations
    #[test]
    fn test_memory_error_handling() {
        let mut memory_manager = MemoryManager::new();

        // Test allocation with invalid size
        let invalid_allocation = memory_manager.allocate(0);
        assert_eq!(invalid_allocation, Err(MemoryError::OutOfMemory));
    }

    /// Test Memory Statistics
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

    /// Test Snapshot and Rollback
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

    /// Test Memory Read/Write Operations
    #[test]
    fn test_memory_read_write_operations() {
        let mut memory_manager = MemoryManager::new();

        // Allocate memory
        let block = memory_manager.allocate(16).unwrap();

        // Write data
        let test_data = b"Hello, World!";
        assert!(memory_manager.write_memory(&block, 0, test_data).is_ok());

        // Read data back
        let read_data = memory_manager
            .read_memory(&block, 0, test_data.len())
            .unwrap();
        assert_eq!(read_data, test_data);

        // Test partial read
        let partial_data = memory_manager.read_memory(&block, 0, 5).unwrap();
        assert_eq!(partial_data, b"Hello");

        // Test invalid operations
        assert_eq!(
            memory_manager.read_memory(&block, 20, 10),
            Err(MemoryError::OutOfMemory)
        );
    }
}

/// Integration tests for Physics Layer with other components
#[cfg(test)]
mod physics_integration_tests {
    use super::*;

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
}

/// Performance and stress tests
#[cfg(test)]
mod physics_performance_tests {
    use super::*;

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
}
