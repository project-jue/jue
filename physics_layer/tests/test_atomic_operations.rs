/// Atomic Operations Unit Tests
/// Comprehensive tests for atomic operations functionality
use physics_layer::atomic_ops::{
    atomic_add, atomic_add_i64, atomic_add_usize, atomic_swap, atomic_swap_i64, atomic_swap_usize,
};
use std::sync::atomic::{AtomicI32, AtomicI64, AtomicUsize};
use std::sync::Arc;

#[test]
fn test_atomic_add_i32() {
    // Test atomic add operation for i32
    let atomic = Arc::new(AtomicI32::new(10));
    let result = atomic_add(atomic.clone(), 5);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 15);
}

#[test]
fn test_atomic_swap_i32() {
    // Test atomic swap operation for i32
    let atomic = Arc::new(AtomicI32::new(10));
    let result = atomic_swap(atomic.clone(), 20);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 10); // Old value returned
}

#[test]
fn test_atomic_add_i64() {
    // Test atomic add operation for i64
    let atomic = Arc::new(AtomicI64::new(100));
    let result = atomic_add_i64(atomic.clone(), 50);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 150);
}

#[test]
fn test_atomic_swap_i64() {
    // Test atomic swap operation for i64
    let atomic = Arc::new(AtomicI64::new(100));
    let result = atomic_swap_i64(atomic.clone(), 200);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 100); // Old value returned
}

#[test]
fn test_atomic_add_usize() {
    // Test atomic add operation for usize
    let atomic = Arc::new(AtomicUsize::new(10));
    let result = atomic_add_usize(atomic.clone(), 5);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 15);
}

#[test]
fn test_atomic_swap_usize() {
    // Test atomic swap operation for usize
    let atomic = Arc::new(AtomicUsize::new(10));
    let result = atomic_swap_usize(atomic.clone(), 20);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 10); // Old value returned
}
