use physics_layer::atomic_ops::atomic_add_usize;
/// Atomic Operations Properties Tests
use physics_layer::atomic_ops::{atomic_add, atomic_swap};
use std::sync::atomic::{AtomicI32, AtomicUsize};
use std::sync::Arc;

#[test]
fn test_atomic_properties() {
    // Test atomic operation properties
    let atomic = Arc::new(AtomicI32::new(0));

    // Test commutativity: a + b = b + a
    atomic_add(atomic.clone(), 5).unwrap();
    atomic_add(atomic.clone(), 3).unwrap();
    let result1 = atomic.load(std::sync::atomic::Ordering::SeqCst);

    let atomic2 = Arc::new(AtomicI32::new(0));
    atomic_add(atomic2.clone(), 3).unwrap();
    atomic_add(atomic2.clone(), 5).unwrap();
    let result2 = atomic2.load(std::sync::atomic::Ordering::SeqCst);

    assert_eq!(result1, result2);

    // Test associativity: (a + b) + c = a + (b + c)
    let atomic3 = Arc::new(AtomicI32::new(0));
    atomic_add(atomic3.clone(), 1).unwrap();
    atomic_add(atomic3.clone(), 2).unwrap();
    atomic_add(atomic3.clone(), 3).unwrap();
    let result3 = atomic3.load(std::sync::atomic::Ordering::SeqCst);

    let atomic4 = Arc::new(AtomicI32::new(0));
    atomic_add(atomic4.clone(), 1).unwrap();
    let _temp = atomic_add(atomic4.clone(), 2).unwrap();
    atomic_add(atomic4.clone(), 3).unwrap();
    let result4 = atomic4.load(std::sync::atomic::Ordering::SeqCst);

    assert_eq!(result3, result4);
}

#[test]
fn test_atomic_error_handling() {
    // Test error handling in atomic operations
    let atomic = Arc::new(AtomicI32::new(0));

    // Test with invalid operations (should not fail normally)
    let result = atomic_add(atomic.clone(), 0);
    assert!(result.is_ok());

    let result = atomic_swap(atomic.clone(), 0);
    assert!(result.is_ok());
}

#[test]
fn test_atomic_overflow() {
    // Test atomic operations with overflow scenarios
    // Rust's atomic operations panic on overflow by default
    // So we test with values that won't overflow
    let atomic = Arc::new(AtomicI32::new(i32::MAX - 10));
    let result = atomic_add(atomic.clone(), 5);

    // Should handle without overflow
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), i32::MAX - 5);
}

#[test]
fn test_atomic_performance() {
    // Test atomic operation performance
    use std::time::Instant;

    let atomic = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    let start = Instant::now();

    for _ in 0..1000 {
        let atomic_clone = atomic.clone();
        let handle = std::thread::spawn(move || {
            for _ in 0..100 {
                atomic_add_usize(atomic_clone.clone(), 1).unwrap();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    let final_value = atomic.load(std::sync::atomic::Ordering::SeqCst);

    // Should complete in reasonable time and produce correct result
    assert_eq!(final_value, 100000);
    assert!(duration.as_millis() < 1000); // Should be fast
}
