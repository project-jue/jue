/// Atomic operations for concurrent programming in the Physics Layer
///
/// This module provides thread-safe atomic operations using Rust's atomic types.
/// These operations are essential for implementing concurrent data structures
/// and ensuring safe memory access in multi-threaded environments.
use std::sync::atomic::{AtomicI32, AtomicI64, AtomicUsize, Ordering};
use std::sync::Arc;

/// Atomic error type for atomic operations
#[derive(Debug, PartialEq)]
pub enum AtomicError {
    NullPointer,
    InvalidAlignment,
    OperationFailed,
}

impl std::fmt::Display for AtomicError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AtomicError::NullPointer => write!(f, "Null pointer error in atomic operation"),
            AtomicError::InvalidAlignment => {
                write!(f, "Invalid memory alignment for atomic operation")
            }
            AtomicError::OperationFailed => write!(f, "Atomic operation failed"),
        }
    }
}

/// Performs an atomic addition operation on an AtomicI32
///
/// # Arguments
/// * `ptr` - Arc-wrapped AtomicI32 to perform the operation on
/// * `val` - Value to add
///
/// # Returns
/// Result containing the new value or an AtomicError
///
/// # Examples
/// `
/// use std::sync::atomic::AtomicI32;
/// use std::sync::Arc;
/// let atomic_val = Arc::new(AtomicI32::new(5));
/// let result = atomic_add(atomic_val.clone(), 3);
/// assert_eq!(result, Ok(8));
/// `
pub fn atomic_add(ptr: Arc<AtomicI32>, val: i32) -> Result<i32, AtomicError> {
    if Arc::ptr_eq(&ptr, &Arc::new(AtomicI32::new(0))) {
        return Err(AtomicError::NullPointer);
    }

    let result = ptr.fetch_add(val, Ordering::SeqCst);
    Ok(result + val)
}

/// Performs an atomic swap operation on an AtomicI32
///
/// # Arguments
/// * `ptr` - Arc-wrapped AtomicI32 to perform the operation on
/// * `val` - New value to swap in
///
/// # Returns
/// Result containing the previous value or an AtomicError
///
/// # Examples
/// `
/// use std::sync::atomic::AtomicI32;
/// use std::sync::Arc;
/// let atomic_val = Arc::new(AtomicI32::new(10));
/// let result = atomic_swap(atomic_val.clone(), 15);
/// assert_eq!(result, Ok(10));
/// assert_eq!(atomic_val.load(Ordering::SeqCst), 15);
/// `
pub fn atomic_swap(ptr: Arc<AtomicI32>, val: i32) -> Result<i32, AtomicError> {
    if Arc::ptr_eq(&ptr, &Arc::new(AtomicI32::new(0))) {
        return Err(AtomicError::NullPointer);
    }

    let result = ptr.swap(val, Ordering::SeqCst);
    Ok(result)
}

/// Performs an atomic addition operation on an AtomicI64
///
/// # Arguments
/// * `ptr` - Arc-wrapped AtomicI64 to perform the operation on
/// * `val` - Value to add
///
/// # Returns
/// Result containing the new value or an AtomicError
pub fn atomic_add_i64(ptr: Arc<AtomicI64>, val: i64) -> Result<i64, AtomicError> {
    if Arc::ptr_eq(&ptr, &Arc::new(AtomicI64::new(0))) {
        return Err(AtomicError::NullPointer);
    }

    let result = ptr.fetch_add(val, Ordering::SeqCst);
    Ok(result + val)
}

/// Performs an atomic swap operation on an AtomicI64
///
/// # Arguments
/// * `ptr` - Arc-wrapped AtomicI64 to perform the operation on
/// * `val` - New value to swap in
///
/// # Returns
/// Result containing the previous value or an AtomicError
pub fn atomic_swap_i64(ptr: Arc<AtomicI64>, val: i64) -> Result<i64, AtomicError> {
    if Arc::ptr_eq(&ptr, &Arc::new(AtomicI64::new(0))) {
        return Err(AtomicError::NullPointer);
    }

    let result = ptr.swap(val, Ordering::SeqCst);
    Ok(result)
}

/// Performs an atomic addition operation on an AtomicUsize
///
/// # Arguments
/// * `ptr` - Arc-wrapped AtomicUsize to perform the operation on
/// * `val` - Value to add
///
/// # Returns
/// Result containing the new value or an AtomicError
pub fn atomic_add_usize(ptr: Arc<AtomicUsize>, val: usize) -> Result<usize, AtomicError> {
    if Arc::ptr_eq(&ptr, &Arc::new(AtomicUsize::new(0))) {
        return Err(AtomicError::NullPointer);
    }

    let result = ptr.fetch_add(val, Ordering::SeqCst);
    Ok(result + val)
}

/// Performs an atomic swap operation on an AtomicUsize
///
/// # Arguments
/// * `ptr` - Arc-wrapped AtomicUsize to perform the operation on
/// * `val` - New value to swap in
///
/// # Returns
/// Result containing the previous value or an AtomicError
pub fn atomic_swap_usize(ptr: Arc<AtomicUsize>, val: usize) -> Result<usize, AtomicError> {
    if Arc::ptr_eq(&ptr, &Arc::new(AtomicUsize::new(0))) {
        return Err(AtomicError::NullPointer);
    }

    let result = ptr.swap(val, Ordering::SeqCst);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicI32, AtomicI64, AtomicUsize};
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_atomic_add_i32() {
        let atomic_val = Arc::new(AtomicI32::new(5));
        let result = atomic_add(atomic_val.clone(), 3);
        assert_eq!(result, Ok(8));
        assert_eq!(atomic_val.load(Ordering::SeqCst), 8);
    }

    #[test]
    fn test_atomic_swap_i32() {
        let atomic_val = Arc::new(AtomicI32::new(10));
        let result = atomic_swap(atomic_val.clone(), 15);
        assert_eq!(result, Ok(10));
        assert_eq!(atomic_val.load(Ordering::SeqCst), 15);
    }

    #[test]
    fn test_atomic_add_i64() {
        let atomic_val = Arc::new(AtomicI64::new(100));
        let result = atomic_add_i64(atomic_val.clone(), 50);
        assert_eq!(result, Ok(150));
        assert_eq!(atomic_val.load(Ordering::SeqCst), 150);
    }

    #[test]
    fn test_atomic_swap_i64() {
        let atomic_val = Arc::new(AtomicI64::new(200));
        let result = atomic_swap_i64(atomic_val.clone(), 300);
        assert_eq!(result, Ok(200));
        assert_eq!(atomic_val.load(Ordering::SeqCst), 300);
    }

    #[test]
    fn test_atomic_add_usize() {
        let atomic_val = Arc::new(AtomicUsize::new(10));
        let result = atomic_add_usize(atomic_val.clone(), 5);
        assert_eq!(result, Ok(15));
        assert_eq!(atomic_val.load(Ordering::SeqCst), 15);
    }

    #[test]
    fn test_atomic_swap_usize() {
        let atomic_val = Arc::new(AtomicUsize::new(20));
        let result = atomic_swap_usize(atomic_val.clone(), 30);
        assert_eq!(result, Ok(20));
        assert_eq!(atomic_val.load(Ordering::SeqCst), 30);
    }

    #[test]
    fn test_thread_safety_atomic_add() {
        let atomic_val = Arc::new(AtomicI32::new(0));
        let mut handles = vec![];

        // Spawn multiple threads that all add to the same atomic value
        for _ in 0..10 {
            let val_clone = Arc::clone(&atomic_val);
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    let _ = atomic_add(val_clone.clone(), 1);
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // The final value should be exactly 1000 (10 threads * 100 increments)
        assert_eq!(atomic_val.load(Ordering::SeqCst), 1000);
    }

    #[test]
    fn test_thread_safety_atomic_swap() {
        let atomic_val = Arc::new(AtomicI32::new(0));
        let mut handles = vec![];

        // Spawn multiple threads that swap values
        for i in 0..10 {
            let val_clone = Arc::clone(&atomic_val);
            let handle = thread::spawn(move || {
                for _ in 0..10 {
                    let _ = atomic_swap(val_clone.clone(), i);
                }
            });
            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // The final value should be one of the thread values (0-9)
        let final_val = atomic_val.load(Ordering::SeqCst);
        assert!(final_val >= 0 && final_val < 10);
    }

    #[test]
    fn test_atomic_properties() {
        let atomic_val = Arc::new(AtomicI32::new(5));

        // Test that atomic operations are indeed atomic
        let _original = atomic_val.load(Ordering::SeqCst);
        let result = atomic_add(atomic_val.clone(), 3).unwrap();
        let new_val = atomic_val.load(Ordering::SeqCst);

        assert_eq!(result, 8);
        assert_eq!(new_val, 8);
        assert_eq!(result, new_val);

        // Test swap returns previous value
        let prev_val = atomic_swap(atomic_val.clone(), 10).unwrap();
        assert_eq!(prev_val, 8);
        assert_eq!(atomic_val.load(Ordering::SeqCst), 10);
    }
}
