/// Memory management system for the Physics Layer
///
/// This module provides memory allocation, deallocation, and snapshot functionality
/// for persistent data structures. It includes support for immutable memory regions
/// and rollback capabilities for system state management.
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};

/// Memory management error type
#[derive(Debug, PartialEq)]
pub enum MemoryError {
    OutOfMemory,
    InvalidPointer,
    DoubleFree,
    SnapshotFailed,
    RollbackFailed,
}

impl fmt::Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MemoryError::OutOfMemory => write!(f, "Out of memory"),
            MemoryError::InvalidPointer => write!(f, "Invalid memory pointer"),
            MemoryError::DoubleFree => write!(f, "Attempt to free already freed memory"),
            MemoryError::SnapshotFailed => write!(f, "Memory snapshot failed"),
            MemoryError::RollbackFailed => write!(f, "Memory rollback failed"),
        }
    }
}

/// Memory pointer type for tracking allocated memory
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MemoryPointer {
    id: usize,
}

impl MemoryPointer {
    fn new(id: usize) -> Self {
        Self { id }
    }
}

/// Memory block structure
#[derive(Debug, Clone)]
struct MemoryBlock {
    size: usize,
    data: Vec<u8>,
    is_freed: bool,
}

impl MemoryBlock {
    fn new(size: usize) -> Self {
        Self {
            size,
            data: vec![0; size],
            is_freed: false,
        }
    }
}

/// Memory manager structure
#[derive(Debug)]
pub struct MemoryManager {
    blocks: HashMap<MemoryPointer, MemoryBlock>,
    next_id: usize,
    snapshots: Vec<HashMap<MemoryPointer, MemoryBlock>>,
}

impl MemoryManager {
    /// Creates a new MemoryManager instance
    ///
    /// # Returns
    /// A new MemoryManager with empty memory blocks
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            next_id: 1,
            snapshots: Vec::new(),
        }
    }

    /// Allocates memory of the specified size
    ///
    /// # Arguments
    /// * `size` - Size of memory to allocate in bytes
    ///
    /// # Returns
    /// Result containing a MemoryPointer or MemoryError
    ///
    /// # Examples
    /// `
    /// let mut manager = MemoryManager::new();
    /// let ptr = manager.allocate(1024);
    /// assert!(ptr.is_ok());
    /// `
    pub fn allocate(&mut self, size: usize) -> Result<MemoryPointer, MemoryError> {
        if size == 0 {
            return Err(MemoryError::OutOfMemory);
        }

        let pointer = MemoryPointer::new(self.next_id);
        self.next_id += 1;

        let block = MemoryBlock::new(size);

        self.blocks.insert(pointer.clone(), block);
        Ok(pointer)
    }

    /// Frees memory associated with the given pointer
    ///
    /// # Arguments
    /// * `ptr` - MemoryPointer to free
    ///
    /// # Returns
    /// Result indicating success or MemoryError
    ///
    /// # Examples
    /// `
    /// let mut manager = MemoryManager::new();
    /// let ptr = manager.allocate(100).unwrap();
    /// let result = manager.free(ptr);
    /// assert!(result.is_ok());
    /// `
    pub fn free(&mut self, ptr: MemoryPointer) -> Result<(), MemoryError> {
        let block = self
            .blocks
            .get_mut(&ptr)
            .ok_or(MemoryError::InvalidPointer)?;

        if block.is_freed {
            return Err(MemoryError::DoubleFree);
        }

        block.is_freed = true;
        Ok(())
    }

    /// Takes a snapshot of the current memory state
    ///
    /// # Returns
    /// Result indicating success or MemoryError
    ///
    /// # Examples
    /// `
    /// let mut manager = MemoryManager::new();
    /// let result = manager.snapshot();
    /// assert!(result.is_ok());
    /// `
    pub fn snapshot(&mut self) -> Result<(), MemoryError> {
        let snapshot = self.blocks.clone();
        self.snapshots.push(snapshot);
        Ok(())
    }

    /// Rolls back memory state to the most recent snapshot
    ///
    /// # Returns
    /// Result indicating success or MemoryError
    ///
    /// # Examples
    /// `
    /// let mut manager = MemoryManager::new();
    /// manager.snapshot().unwrap();
    /// // ... make some changes ...
    /// let result = manager.rollback();
    /// assert!(result.is_ok());
    /// `
    pub fn rollback(&mut self) -> Result<(), MemoryError> {
        let snapshot = self.snapshots.pop().ok_or(MemoryError::RollbackFailed)?;

        self.blocks = snapshot;
        Ok(())
    }

    /// Gets the current memory usage statistics
    ///
    /// # Returns
    /// Tuple of (total_allocated, total_freed, active_blocks)
    pub fn get_memory_stats(&self) -> (usize, usize, usize) {
        let mut total_allocated = 0;
        let mut total_freed = 0;
        let mut active_blocks = 0;

        for block in self.blocks.values() {
            total_allocated += block.size;
            if block.is_freed {
                total_freed += block.size;
            } else {
                active_blocks += 1;
            }
        }

        (total_allocated, total_freed, active_blocks)
    }

    /// Writes data to the specified memory location
    ///
    /// # Arguments
    /// * `ptr` - MemoryPointer to write to
    /// * `offset` - Offset within the memory block
    /// * `data` - Data to write
    ///
    /// # Returns
    /// Result indicating success or MemoryError
    pub fn write_memory(
        &mut self,
        ptr: &MemoryPointer,
        offset: usize,
        data: &[u8],
    ) -> Result<(), MemoryError> {
        let block = self
            .blocks
            .get_mut(ptr)
            .ok_or(MemoryError::InvalidPointer)?;

        if block.is_freed {
            return Err(MemoryError::InvalidPointer);
        }

        if offset + data.len() > block.size {
            return Err(MemoryError::OutOfMemory);
        }

        block.data[offset..offset + data.len()].copy_from_slice(data);
        Ok(())
    }

    /// Reads data from the specified memory location
    ///
    /// # Arguments
    /// * `ptr` - MemoryPointer to read from
    /// * `offset` - Offset within the memory block
    /// * `length` - Number of bytes to read
    ///
    /// # Returns
    /// Result containing the data or MemoryError
    pub fn read_memory(
        &self,
        ptr: &MemoryPointer,
        offset: usize,
        length: usize,
    ) -> Result<Vec<u8>, MemoryError> {
        let block = self.blocks.get(ptr).ok_or(MemoryError::InvalidPointer)?;

        if block.is_freed {
            return Err(MemoryError::InvalidPointer);
        }

        if offset + length > block.size {
            return Err(MemoryError::OutOfMemory);
        }

        Ok(block.data[offset..offset + length].to_vec())
    }
}

/// Thread-safe memory manager wrapper
#[derive(Debug, Clone)]
pub struct ThreadSafeMemoryManager {
    pub inner: Arc<Mutex<MemoryManager>>,
}

impl ThreadSafeMemoryManager {
    /// Creates a new thread-safe memory manager
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(MemoryManager::new())),
        }
    }

    /// Allocates memory in a thread-safe manner
    pub fn allocate(&self, size: usize) -> Result<MemoryPointer, MemoryError> {
        let mut manager = self.inner.lock().unwrap();
        manager.allocate(size)
    }

    /// Frees memory in a thread-safe manner
    pub fn free(&self, ptr: MemoryPointer) -> Result<(), MemoryError> {
        let mut manager = self.inner.lock().unwrap();
        manager.free(ptr)
    }

    /// Takes a snapshot in a thread-safe manner
    pub fn snapshot(&self) -> Result<(), MemoryError> {
        let mut manager = self.inner.lock().unwrap();
        manager.snapshot()
    }

    /// Rolls back memory state in a thread-safe manner
    pub fn rollback(&self) -> Result<(), MemoryError> {
        let mut manager = self.inner.lock().unwrap();
        manager.rollback()
    }

    /// Gets memory statistics in a thread-safe manner
    pub fn get_memory_stats(&self) -> (usize, usize, usize) {
        let manager = self.inner.lock().unwrap();
        manager.get_memory_stats()
    }

    /// Writes data to the specified memory location in a thread-safe manner
    ///
    /// # Arguments
    /// * `ptr` - MemoryPointer to write to
    /// * `offset` - Offset within the memory block
    /// * `data` - Data to write
    ///
    /// # Returns
    /// Result indicating success or MemoryError
    pub fn write_memory(
        &self,
        ptr: &MemoryPointer,
        offset: usize,
        data: &[u8],
    ) -> Result<(), MemoryError> {
        let mut manager = self.inner.lock().unwrap();
        manager.write_memory(ptr, offset, data)
    }

    /// Reads data from the specified memory location in a thread-safe manner
    ///
    /// # Arguments
    /// * `ptr` - MemoryPointer to read from
    /// * `offset` - Offset within the memory block
    /// * `length` - Number of bytes to read
    ///
    /// # Returns
    /// Result containing the data or MemoryError
    pub fn read_memory(
        &self,
        ptr: &MemoryPointer,
        offset: usize,
        length: usize,
    ) -> Result<Vec<u8>, MemoryError> {
        let manager = self.inner.lock().unwrap();
        manager.read_memory(ptr, offset, length)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_allocation_and_free() {
        let mut manager = MemoryManager::new();

        // Test allocation
        let ptr1 = manager.allocate(100).unwrap();
        let ptr2 = manager.allocate(200).unwrap();

        // Verify different pointers
        assert_ne!(ptr1, ptr2);

        // Test freeing
        assert!(manager.free(ptr1.clone()).is_ok());
        assert!(manager.free(ptr2.clone()).is_ok());

        // Test double free
        assert_eq!(manager.free(ptr1), Err(MemoryError::DoubleFree));
    }

    #[test]
    fn test_snapshot_and_rollback() {
        let mut manager = MemoryManager::new();

        // Allocate some memory
        let ptr1 = manager.allocate(100).unwrap();
        let ptr2 = manager.allocate(200).unwrap();

        // Take snapshot
        assert!(manager.snapshot().is_ok());

        // Free some memory
        assert!(manager.free(ptr1.clone()).is_ok());

        // Rollback should restore the freed memory
        assert!(manager.rollback().is_ok());

        // The pointer should be valid again
        assert!(manager.free(ptr1).is_ok());
        assert!(manager.free(ptr2).is_ok());
    }

    #[test]
    fn test_memory_operations() {
        let mut manager = MemoryManager::new();

        // Allocate memory
        let ptr = manager.allocate(16).unwrap();

        // Write data
        let test_data = b"Hello, World!";
        assert!(manager.write_memory(&ptr, 0, test_data).is_ok());

        // Read data back
        let read_data = manager.read_memory(&ptr, 0, test_data.len()).unwrap();
        assert_eq!(read_data, test_data);

        // Test partial read
        let partial_data = manager.read_memory(&ptr, 0, 5).unwrap();
        assert_eq!(partial_data, b"Hello");

        // Test invalid operations
        assert_eq!(
            manager.read_memory(&ptr, 20, 10),
            Err(MemoryError::OutOfMemory)
        );
    }

    #[test]
    fn test_memory_stats() {
        let mut manager = MemoryManager::new();

        // Allocate some memory
        let _ptr1 = manager.allocate(100).unwrap();
        let ptr2 = manager.allocate(200).unwrap();
        let _ptr3 = manager.allocate(300).unwrap();

        // Check initial stats
        let (total, freed, active) = manager.get_memory_stats();
        assert_eq!(total, 600);
        assert_eq!(freed, 0);
        assert_eq!(active, 3);

        // Free one block
        assert!(manager.free(ptr2).is_ok());

        // Check updated stats
        let (total, freed, active) = manager.get_memory_stats();
        assert_eq!(total, 600);
        assert_eq!(freed, 200);
        assert_eq!(active, 2);
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
    fn test_error_conditions() {
        let mut manager = MemoryManager::new();

        // Test invalid pointer operations
        let invalid_ptr = MemoryPointer::new(999);
        assert_eq!(manager.free(invalid_ptr), Err(MemoryError::InvalidPointer));

        // Test zero allocation
        assert_eq!(manager.allocate(0), Err(MemoryError::OutOfMemory));

        // Test rollback without snapshot
        assert_eq!(manager.rollback(), Err(MemoryError::RollbackFailed));
    }

    #[test]
    fn test_persistent_structure_support() {
        let mut manager = MemoryManager::new();

        // Allocate memory for persistent structure
        let ptr = manager.allocate(1024).unwrap();

        // Take snapshot of initial state (all zeros)
        assert!(manager.snapshot().is_ok());

        // Modify the memory (simulating structure updates)
        let data1 = vec![42; 100]; // Use a distinct value
        assert!(manager.write_memory(&ptr, 0, &data1).is_ok());

        // Take another snapshot
        assert!(manager.snapshot().is_ok());

        // Modify again - change the first byte to verify rollback works
        let data2 = vec![99; 1]; // Different value
        assert!(manager.write_memory(&ptr, 0, &data2).is_ok());

        // Verify current state (first byte should be 99)
        let current_data = manager.read_memory(&ptr, 0, 1).unwrap();
        assert_eq!(current_data, vec![99]);

        // Rollback to second snapshot
        assert!(manager.rollback().is_ok());

        // Verify we're back to first modification (first byte should be 42)
        let read_data = manager.read_memory(&ptr, 0, 1).unwrap();
        assert_eq!(read_data, vec![42]);

        // Rollback to first snapshot
        assert!(manager.rollback().is_ok());

        // Verify we're back to initial state (first byte should be 0)
        let read_data = manager.read_memory(&ptr, 0, 1).unwrap();
        assert_eq!(read_data, vec![0]);
    }
}
