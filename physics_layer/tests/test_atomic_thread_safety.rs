/// Atomic Operations Thread Safety Tests
use physics_layer::atomic_ops::{atomic_add_usize, atomic_swap_usize};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::thread;

#[test]
fn test_thread_safety_atomic_add() {
    // Test thread safety of atomic add operations
    let atomic = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let atomic_clone = atomic.clone();
        let handle = thread::spawn(move || {
            atomic_add_usize(atomic_clone, 1).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final value should be 10
    let final_value = atomic.load(std::sync::atomic::Ordering::SeqCst);
    assert_eq!(final_value, 10);
}

#[test]
fn test_thread_safety_atomic_swap() {
    // Test thread safety of atomic swap operations
    let atomic = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let atomic_clone = atomic.clone();
        let handle = thread::spawn(move || {
            atomic_swap_usize(atomic_clone, i).unwrap();
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Final value should be one of the thread values (0-9)
    let final_value = atomic.load(std::sync::atomic::Ordering::SeqCst);
    assert!(final_value >= 0 && final_value < 10);
}
