use crate::types::HeapPtr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Error type for arena allocation failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ArenaError {
    #[error("Arena is full (capacity {capacity}, requested {requested})")]
    ArenaFull { capacity: u32, requested: u32 },
}

/// Error type for garbage collection failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum GarbageCollectionError {
    #[error("Garbage collection failed: {0}")]
    CollectionFailed(String),
}

/// Error type for defragmentation failures.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DefragmentationError {
    #[error("Defragmentation failed: {0}")]
    DefragmentationFailed(String),
}

/// Result type for garbage collection operations.
pub type GarbageCollectionResult = Result<(), GarbageCollectionError>;

/// Result type for defragmentation operations.
pub type DefragmentationResult = Result<DefragmentationStats, DefragmentationError>;

/// Statistics from defragmentation operation.
#[derive(Debug, Clone)]
pub struct DefragmentationStats {
    pub objects_moved: u32,
    pub bytes_reclaimed: u32,
    pub fragmentation_before: f32,
    pub fragmentation_after: f32,
    pub time_taken_ms: u64,
}

/// Header prepended to each allocated object.
#[repr(C)]
#[derive(Debug)]
pub struct ObjectHeader {
    pub size: u32,
    pub tag: u8,
    pub marked: bool,  // Mark bit for garbage collection
    _padding: [u8; 2], // Maintain 8-byte alignment
}

impl ObjectHeader {
    /// Create a new header.
    pub fn new(size: u32, tag: u8) -> Self {
        Self {
            size,
            tag,
            marked: false,
            _padding: [0; 2],
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
/// The arena supports defragmentation to reduce memory fragmentation.
#[derive(Serialize, Deserialize, Clone)]
pub struct ObjectArena {
    storage: Vec<u8>,
    next_free: u32,
    capacity: u32,
    /// Fragmentation threshold for automatic defragmentation (0.0 to 1.0)
    fragmentation_threshold: f32,
    /// Enable/disable automatic defragmentation
    auto_defragment: bool,
}

impl ObjectArena {
    /// Creates a new arena with the given capacity (in bytes).
    pub fn with_capacity(capacity: u32) -> Self {
        Self {
            storage: vec![0; capacity as usize],
            next_free: 0,
            capacity,
            fragmentation_threshold: 0.3, // 30% fragmentation threshold
            auto_defragment: true,        // Enable automatic defragmentation by default
        }
    }

    /// Creates a new arena with custom defragmentation settings.
    pub fn with_capacity_and_settings(
        capacity: u32,
        fragmentation_threshold: f32,
        auto_defragment: bool,
    ) -> Self {
        Self {
            storage: vec![0; capacity as usize],
            next_free: 0,
            capacity,
            fragmentation_threshold: fragmentation_threshold.clamp(0.0, 1.0),
            auto_defragment,
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

    /// Returns a mutable reference to the header of the object at `ptr`.
    ///
    /// # Safety
    /// The caller must ensure that `ptr` points to a valid object header.
    pub unsafe fn get_header_mut(&mut self, ptr: HeapPtr) -> &mut ObjectHeader {
        let addr = ptr.get() as usize;
        &mut *(self.storage.as_ptr().add(addr) as *mut ObjectHeader)
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

    /// Marks an object as reachable during garbage collection.
    ///
    /// # Safety
    /// The caller must ensure that `ptr` points to a valid object header.
    pub unsafe fn mark_object(&mut self, ptr: HeapPtr) {
        let header = self.get_header_mut(ptr);
        header.marked = true;
    }

    /// Checks if an object is marked as reachable.
    ///
    /// # Safety
    /// The caller must ensure that `ptr` points to a valid object header.
    pub unsafe fn is_marked(&self, ptr: HeapPtr) -> bool {
        let header = self.get_header(ptr);
        header.marked
    }

    /// Performs garbage collection using a mark-and-sweep algorithm.
    ///
    /// # Arguments
    /// * `root_set` - A slice of `HeapPtr` values representing the root set of reachable objects.
    ///
    /// # Returns
    /// A `GarbageCollectionResult` indicating success or failure.
    pub fn collect_garbage(&mut self, root_set: &[HeapPtr]) -> GarbageCollectionResult {
        // Mark phase: Mark all reachable objects starting from the root set
        self.mark_phase(root_set)?;

        // Sweep phase: Collect unmarked objects and compact memory
        self.sweep_phase()?;

        // Check if automatic defragmentation should be triggered
        if self.auto_defragment && self.should_defragment() {
            let _ = self.defragment(); // Ignore result for automatic defrag
        }

        Ok(())
    }

    /// Performs defragmentation to compact memory and reduce fragmentation.
    ///
    /// # Returns
    /// A `DefragmentationResult` with statistics about the operation.
    pub fn defragment(&mut self) -> DefragmentationResult {
        let start_time = std::time::Instant::now();
        let fragmentation_before = self.fragmentation_ratio();

        // Create a mapping from old pointers to new pointers
        let mut pointer_mapping = Vec::new();
        let mut new_next_free = 0;

        // First pass: calculate new positions and build mapping
        let mut current_ptr = 0;
        while current_ptr < self.next_free {
            let header = unsafe { self.get_header(HeapPtr::new(current_ptr)) };
            let object_size = ObjectHeader::size_bytes() as u32 + header.size;

            if header.marked {
                // This object is live, calculate its new position
                pointer_mapping.push((current_ptr, new_next_free));
                new_next_free += object_size;
            }

            current_ptr += object_size;
        }

        // Second pass: move objects to their new positions
        let mut objects_moved = 0;
        for (old_ptr, new_ptr) in &pointer_mapping {
            let object_size = {
                let header = unsafe { self.get_header(HeapPtr::new(*old_ptr)) };
                ObjectHeader::size_bytes() as u32 + header.size
            };

            // Move the object data
            let src_start = *old_ptr as usize;
            let src_end = src_start + object_size as usize;
            let dst_start = *new_ptr as usize;

            self.storage.copy_within(src_start..src_end, dst_start);
            objects_moved += 1;
        }

        // Update next_free to the new compacted position
        let bytes_reclaimed = self.next_free - new_next_free;
        self.next_free = new_next_free;

        let fragmentation_after = self.fragmentation_ratio();
        let time_taken = start_time.elapsed().as_millis() as u64;

        Ok(DefragmentationStats {
            objects_moved,
            bytes_reclaimed,
            fragmentation_before,
            fragmentation_after,
            time_taken_ms: time_taken,
        })
    }

    /// Checks if defragmentation should be performed based on fragmentation level.
    ///
    /// # Returns
    /// `true` if fragmentation exceeds the threshold, `false` otherwise.
    pub fn should_defragment(&self) -> bool {
        if !self.auto_defragment {
            return false;
        }

        let fragmentation = self.fragmentation_ratio();
        fragmentation > self.fragmentation_threshold
    }

    /// Calculates the current fragmentation ratio (0.0 = no fragmentation, 1.0 = fully fragmented).
    ///
    /// # Returns
    /// Fragmentation ratio as a value between 0.0 and 1.0.
    pub fn fragmentation_ratio(&self) -> f32 {
        if self.next_free == 0 || self.capacity == 0 {
            return 0.0;
        }

        // Calculate used space
        let mut used_space = 0;
        let mut current_ptr = 0;

        while current_ptr < self.next_free {
            let header = unsafe { self.get_header(HeapPtr::new(current_ptr)) };
            let object_size = ObjectHeader::size_bytes() as u32 + header.size;

            if header.marked {
                used_space += object_size;
            }

            current_ptr += object_size;
        }

        // Fragmentation = (total allocated space - used space) / total allocated space
        if used_space == 0 {
            0.0
        } else {
            let wasted_space = self.next_free - used_space;
            (wasted_space as f32) / (self.next_free as f32)
        }
    }

    /// Gets the current defragmentation settings.
    pub fn get_defragmentation_settings(&self) -> (f32, bool) {
        (self.fragmentation_threshold, self.auto_defragment)
    }

    /// Sets the defragmentation settings.
    pub fn set_defragmentation_settings(&mut self, threshold: f32, auto_defragment: bool) {
        self.fragmentation_threshold = threshold.clamp(0.0, 1.0);
        self.auto_defragment = auto_defragment;
    }

    /// Marks all reachable objects starting from the root set.
    fn mark_phase(&mut self, root_set: &[HeapPtr]) -> Result<(), GarbageCollectionError> {
        // Mark all objects in the root set
        for &root_ptr in root_set {
            unsafe { self.mark_object(root_ptr) };
        }

        // TODO: Implement recursive marking for objects that contain references to other objects
        // This would require traversing the object's data region to find HeapPtr values
        // and marking those objects as well.

        Ok(())
    }

    /// Collects unmarked objects and compacts memory.
    fn sweep_phase(&mut self) -> Result<(), GarbageCollectionError> {
        let mut new_next_free = 0;
        let mut current_ptr = 0;

        while current_ptr < self.next_free {
            let header = unsafe { self.get_header(HeapPtr::new(current_ptr)) };
            let object_size = ObjectHeader::size_bytes() as u32 + header.size;

            if header.marked {
                // Object is reachable, keep it
                if current_ptr != new_next_free {
                    // Move the object to the new location
                    let src_start = current_ptr as usize;
                    let src_end = src_start + object_size as usize;
                    let dst_start = new_next_free as usize;

                    self.storage.copy_within(src_start..src_end, dst_start);

                    // Update the pointer in the root set if it points to this object
                    // TODO: This is a simplified approach; a more robust solution would
                    // maintain a mapping of old to new pointers for all reachable objects.
                }
                new_next_free += object_size;
            } else {
                // Object is unreachable, skip it
            }

            current_ptr += object_size;
        }

        self.next_free = new_next_free;
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/arena_tests.rs"]
mod tests;
