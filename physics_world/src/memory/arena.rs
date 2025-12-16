use crate::types::HeapPtr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error type for arena allocation failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ArenaError {
    #[error("Arena is full (capacity {capacity}, requested {requested})")]
    ArenaFull { capacity: u32, requested: u32 },
}

/// Header prepended to each allocated object.
#[repr(C)]
#[derive(Debug)]
pub struct ObjectHeader {
    pub size: u32,
    pub tag: u8,
    _padding: [u8; 3],
}

impl ObjectHeader {
    /// Create a new header.
    pub fn new(size: u32, tag: u8) -> Self {
        Self {
            size,
            tag,
            _padding: [0; 3],
        }
    }

    /// Size of the header in bytes.
    pub const fn size_bytes() -> usize {
        std::mem::size_of::<Self>()
    }
}

/// A simple arena allocator that stores objects in a contiguous byte vector.
///
/// Each allocated object is prefixed by an `ObjectHeader`. The arena does not
/// support individual deallocation; the entire arena can be reset with `reset()`.
#[derive(Serialize, Deserialize)]
pub struct ObjectArena {
    storage: Vec<u8>,
    next_free: u32,
    capacity: u32,
}

impl ObjectArena {
    /// Creates a new arena with the given capacity (in bytes).
    pub fn with_capacity(capacity: u32) -> Self {
        Self {
            storage: vec![0; capacity as usize],
            next_free: 0,
            capacity,
        }
    }

    /// Allocates a region of `size` bytes and returns a `HeapPtr` to it.
    ///
    /// The allocated region is guaranteed to be aligned to 8 bytes (the size of
    /// the header). The header is written at the start of the region.
    ///
    /// # Errors
    /// Returns `ArenaError::ArenaFull` if there is insufficient space.
    pub fn allocate(&mut self, size: u32, tag: u8) -> Result<HeapPtr, ArenaError> {
        let aligned_size = (size + 7) & !7; // align up to 8 bytes
        let total_needed = ObjectHeader::size_bytes() as u32 + aligned_size;

        if self.next_free + total_needed > self.capacity {
            return Err(ArenaError::ArenaFull {
                capacity: self.capacity,
                requested: total_needed,
            });
        }

        let ptr = self.next_free;
        self.next_free += total_needed;

        // Write header
        let header = ObjectHeader::new(size, tag);
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                &header as *const ObjectHeader as *const u8,
                ObjectHeader::size_bytes(),
            )
        };
        let start = ptr as usize;
        self.storage[start..start + ObjectHeader::size_bytes()].copy_from_slice(header_bytes);

        // Zero the data region for safety
        let data_start = start + ObjectHeader::size_bytes();
        let data_end = data_start + aligned_size as usize;
        self.storage[data_start..data_end].fill(0);

        Ok(HeapPtr::new(ptr))
    }

    /// Resets the arena, discarding all allocated objects.
    pub fn reset(&mut self) {
        self.next_free = 0;
        // Optionally zero the storage; not required for correctness but helps debugging.
        self.storage.fill(0);
    }

    /// Returns a reference to the header of the object at `ptr`.
    ///
    /// # Safety
    /// The caller must ensure that `ptr` points to a valid object header.
    pub unsafe fn get_header(&self, ptr: HeapPtr) -> &ObjectHeader {
        let addr = ptr.get() as usize;
        &*(self.storage.as_ptr().add(addr) as *const ObjectHeader)
    }

    /// Returns a mutable slice to the data region of the object at `ptr`.
    ///
    /// # Safety
    /// The caller must ensure that `ptr` points to a valid allocated object.
    pub unsafe fn get_data(&self, ptr: HeapPtr) -> &[u8] {
        let addr = ptr.get() as usize;
        let header = self.get_header(ptr);
        let data_start = addr + ObjectHeader::size_bytes();
        let data_end = data_start + header.size as usize;
        &self.storage[data_start..data_end]
    }

    /// Returns a mutable slice to the data region of the object at `ptr`.
    ///
    /// # Safety
    /// The caller must ensure that `ptr` points to a valid allocated object.
    pub unsafe fn get_data_mut(&mut self, ptr: HeapPtr) -> &mut [u8] {
        let addr = ptr.get() as usize;
        let header = self.get_header(ptr);
        let data_start = addr + ObjectHeader::size_bytes();
        let data_end = data_start + header.size as usize;
        &mut self.storage[data_start..data_end]
    }

    /// Returns the current allocation offset (next free byte). Useful for debugging.
    pub fn next_free(&self) -> u32 {
        self.next_free
    }

    /// Returns the total capacity in bytes.
    pub fn capacity(&self) -> u32 {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_header_size() {
        // Ensure the header is 8 bytes (size 4 + tag 1 + padding 3)
        assert_eq!(ObjectHeader::size_bytes(), 8);
    }

    #[test]
    fn test_arena_allocate_and_retrieve() {
        let mut arena = ObjectArena::with_capacity(1024);
        let ptr = arena.allocate(16, 1).unwrap();
        assert_eq!(ptr.get(), 0);

        // Check header
        let header = unsafe { arena.get_header(ptr) };
        assert_eq!(header.size, 16);
        assert_eq!(header.tag, 1);

        // Check data region is zeroed
        let data = unsafe { arena.get_data(ptr) };
        assert_eq!(data.len(), 16);
        assert!(data.iter().all(|&b| b == 0));

        // Write something to data
        let data_mut = unsafe { arena.get_data_mut(ptr) };
        data_mut[0] = 42;
        assert_eq!(data_mut[0], 42);
    }

    #[test]
    fn test_arena_alignment() {
        let mut arena = ObjectArena::with_capacity(1024);
        // Allocate 1 byte, should be aligned to 8 bytes after header
        let _ptr1 = arena.allocate(1, 0).unwrap();
        // Header is 8 bytes, data size aligned to 8 -> 8 + 8 = 16
        assert_eq!(arena.next_free(), 16);

        // Next allocation should start at offset 16
        let ptr2 = arena.allocate(1, 0).unwrap();
        assert_eq!(ptr2.get(), 16);
    }

    #[test]
    fn test_arena_full() {
        let mut arena = ObjectArena::with_capacity(32);
        // Allocate 8 bytes header + 8 bytes data = 16 bytes
        let ptr1 = arena.allocate(8, 0).unwrap();
        assert_eq!(ptr1.get(), 0);
        assert_eq!(arena.next_free(), 16);

        // Next allocation of 8 bytes header + 8 bytes data = 16 bytes, total 32 -> fits
        let ptr2 = arena.allocate(8, 0).unwrap();
        assert_eq!(ptr2.get(), 16);
        assert_eq!(arena.next_free(), 32);

        // Next allocation should fail
        match arena.allocate(1, 0) {
            Err(ArenaError::ArenaFull {
                capacity,
                requested,
            }) => {
                assert_eq!(capacity, 32);
                assert_eq!(requested, 16); // header 8 + aligned size 8
            }
            Ok(_) => panic!("Expected ArenaFull error"),
        }
    }

    #[test]
    fn test_arena_reset() {
        let mut arena = ObjectArena::with_capacity(1024);
        arena.allocate(32, 0).unwrap();
        assert_eq!(arena.next_free(), 40); // header 8 + aligned 32 = 40
        arena.reset();
        assert_eq!(arena.next_free(), 0);
        // Should be able to allocate again from start
        let ptr = arena.allocate(32, 0).unwrap();
        assert_eq!(ptr.get(), 0);
    }

    #[test]
    fn test_pair_allocation() {
        // Simulate allocating a Pair (two HeapPtrs)
        let mut arena = ObjectArena::with_capacity(1024);
        let pair_ptr = arena.allocate(8, 2).unwrap(); // 8 bytes for two u32s
        let data = unsafe { arena.get_data_mut(pair_ptr) };
        // Write two HeapPtr values
        let ptr1 = HeapPtr::new(42);
        let ptr2 = HeapPtr::new(99);
        let bytes1 = ptr1.get().to_le_bytes();
        let bytes2 = ptr2.get().to_le_bytes();
        data[0..4].copy_from_slice(&bytes1);
        data[4..8].copy_from_slice(&bytes2);

        // Read back
        let data_read = unsafe { arena.get_data(pair_ptr) };
        let read_ptr1 = u32::from_le_bytes(data_read[0..4].try_into().unwrap());
        let read_ptr2 = u32::from_le_bytes(data_read[4..8].try_into().unwrap());
        assert_eq!(read_ptr1, 42);
        assert_eq!(read_ptr2, 99);
    }
}
