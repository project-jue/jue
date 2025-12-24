//! GC integration helpers for the VM state.
//!
//! This module provides VM-level garbage collection operations,
//! integrating the GarbageCollector with VmState.
//!
//! # Extracted from
//! - `vm/state.rs` (lines 714-740, GC integration methods)

use crate::memory::arena::{ObjectArena, ObjectHeader};
use crate::types::{HeapPtr, Value};
use crate::vm::error::VmError;
use crate::vm::gc::{GarbageCollector, GcPtr, GcRoot, GcStats, HeapObject};

/// GC integration layer for VmState.
///
/// Provides methods for managing heap allocation and garbage collection
/// at the VM state level.
pub struct GcIntegration;

impl GcIntegration {
    /// Allocate a heap object with garbage collection.
    ///
    /// # Arguments
    /// * `state` - Mutable reference to the VM state
    /// * `object` - The heap object to allocate
    ///
    /// # Returns
    /// Result containing either the GC pointer or an error
    pub fn allocate_heap_object(
        state: &mut crate::vm::state::VmState,
        object: HeapObject,
    ) -> Result<Value, VmError> {
        if !state.gc_enabled {
            // Create a minimal context for the error
            let context = crate::vm::error::ErrorContext {
                instruction_pointer: state.ip,
                current_instruction: None,
                stack_state: state.stack.clone(),
                call_stack_depth: state.call_stack.len(),
                steps_remaining: state.steps_remaining,
                actor_id: state.actor_id,
                memory_usage: state.memory.next_free() as usize,
                stack_trace: Vec::new(),
                execution_history: Vec::new(),
                timestamp: 0,
            };
            return Err(VmError::GcDisabled);
        }

        let ptr = state.gc.allocate(object);
        Ok(Value::GcPtr(ptr))
    }

    /// Add a GC root for tracking.
    ///
    /// # Arguments
    /// * `state` - Mutable reference to the VM state
    /// * `ptr` - The GC pointer to root
    /// * `description` - Description of the root for debugging
    pub fn add_gc_root(state: &mut crate::vm::state::VmState, ptr: GcPtr, description: &str) {
        state.gc.roots.push(GcRoot {
            ptr,
            description: description.to_string(),
        });
    }

    /// Remove a GC root.
    ///
    /// # Arguments
    /// * `state` - Mutable reference to the VM state
    /// * `ptr` - The GC pointer to unroot
    pub fn remove_gc_root(state: &mut crate::vm::state::VmState, ptr: GcPtr) {
        state.gc.roots.retain(|root| root.ptr != ptr);
    }

    /// Get GC statistics.
    ///
    /// # Arguments
    /// * `state` - Reference to the VM state
    ///
    /// # Returns
    /// Current GC statistics
    pub fn get_gc_stats(state: &crate::vm::state::VmState) -> GcStats {
        state.gc.gc_stats.clone()
    }

    /// Enable or disable garbage collection.
    ///
    /// # Arguments
    /// * `state` - Mutable reference to the VM state
    /// * `enabled` - Whether GC should be enabled
    pub fn set_gc_enabled(state: &mut crate::vm::state::VmState, enabled: bool) {
        state.gc_enabled = enabled;
    }

    /// Check if GC is enabled.
    ///
    /// # Arguments
    /// * `state` - Reference to the VM state
    ///
    /// # Returns
    /// Whether GC is currently enabled
    pub fn is_gc_enabled(state: &crate::vm::state::VmState) -> bool {
        state.gc_enabled
    }

    /// Get current allocation count.
    ///
    /// # Arguments
    /// * `state` - Reference to the VM state
    ///
    /// # Returns
    /// Number of allocations since last GC
    pub fn allocations_since_last_gc(state: &crate::vm::state::VmState) -> usize {
        state.gc.allocations_since_last_gc
    }

    /// Get the GC allocation threshold.
    ///
    /// # Arguments
    /// * `state` - Reference to the VM state
    ///
    /// # Returns
    /// The allocation threshold that triggers GC
    pub fn gc_threshold(state: &crate::vm::state::VmState) -> usize {
        state.gc_threshold
    }

    /// Set the GC allocation threshold.
    ///
    /// # Arguments
    /// * `state` - Mutable reference to the VM state
    /// * `threshold` - New threshold value
    pub fn set_gc_threshold(state: &mut crate::vm::state::VmState, threshold: usize) {
        state.gc_threshold = threshold;
    }

    /// Force garbage collection.
    ///
    /// # Arguments
    /// * `state` - Mutable reference to the VM state
    pub fn collect_garbage(state: &mut crate::vm::state::VmState) {
        state.gc.collect();
    }

    /// Get heap usage statistics.
    ///
    /// # Arguments
    /// * `state` - Reference to the VM state
    ///
    /// # Returns
    /// Tuple of (bytes used, bytes capacity)
    pub fn heap_usage(state: &crate::vm::state::VmState) -> (usize, usize) {
        (
            state.memory.next_free() as usize,
            state.memory.capacity() as usize,
        )
    }

    /// Get the number of live heap objects.
    ///
    /// # Arguments
    /// * `state` - Reference to the VM state
    ///
    /// # Returns
    /// Number of objects in the heap
    pub fn heap_object_count(state: &crate::vm::state::VmState) -> usize {
        state.gc.heap.len()
    }

    /// Get the number of GC roots.
    ///
    /// # Arguments
    /// * `state` - Reference to the VM state
    ///
    /// # Returns
    /// Number of active GC roots
    pub fn root_count(state: &crate::vm::state::VmState) -> usize {
        state.gc.roots.len()
    }
}

/// Memory analysis for VM heap.
///
/// Provides detailed memory usage analysis for the VM.
pub struct MemoryAnalysis<'a> {
    state: &'a crate::vm::state::VmState,
}

impl<'a> MemoryAnalysis<'a> {
    /// Creates a new memory analysis for the given VM state.
    pub fn new(state: &'a crate::vm::state::VmState) -> Self {
        Self { state }
    }

    /// Get total heap usage in bytes.
    pub fn heap_usage(&self) -> usize {
        self.state.memory.next_free() as usize
    }

    /// Get total heap capacity in bytes.
    pub fn heap_capacity(&self) -> usize {
        self.state.memory.capacity() as usize
    }

    /// Get heap usage percentage.
    pub fn heap_usage_percent(&self) -> f64 {
        if self.state.memory.capacity() == 0 {
            0.0
        } else {
            (self.state.memory.next_free() as f64 / self.state.memory.capacity() as f64) * 100.0
        }
    }

    /// Get the number of heap objects.
    pub fn object_count(&self) -> usize {
        self.state.gc.heap.len()
    }

    /// Get the number of GC roots.
    pub fn root_count(&self) -> usize {
        self.state.gc.roots.len()
    }

    /// Get formatted memory usage string.
    pub fn format_usage(&self) -> String {
        format!(
            "Memory Usage: {} bytes used / {} bytes capacity ({:.1}% used)",
            self.heap_usage(),
            self.heap_capacity(),
            self.heap_usage_percent()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::OpCode;
    use crate::vm::gc::HeapObject;
    use crate::vm::state::VmState;

    #[test]
    fn test_gc_integration_creation() {
        let state = VmState::new(vec![], Vec::new(), 100, 1024, 1, 100);
        let analysis = MemoryAnalysis::new(&state);
        assert_eq!(analysis.heap_usage(), 0);
    }

    #[test]
    fn test_gc_threshold_operations() {
        let mut state = VmState::new(vec![], Vec::new(), 100, 1024, 1, 100);

        assert!(GcIntegration::is_gc_enabled(&state));

        GcIntegration::set_gc_enabled(&mut state, false);
        assert!(!GcIntegration::is_gc_enabled(&state));

        GcIntegration::set_gc_enabled(&mut state, true);
        assert!(GcIntegration::is_gc_enabled(&state));
    }

    #[test]
    fn test_memory_analysis() {
        let state = VmState::new(vec![OpCode::Nil], Vec::new(), 100, 1024, 1, 100);

        let analysis = MemoryAnalysis::new(&state);
        assert!(analysis.heap_capacity() > 0);
        assert!(analysis.format_usage().contains("Memory Usage"));
    }

    #[test]
    fn test_heap_usage() {
        let state = VmState::new(vec![], Vec::new(), 100, 1024, 1, 100);

        let (used, capacity) = GcIntegration::heap_usage(&state);
        assert_eq!(used, 0);
        assert_eq!(capacity, 1024);
    }

    #[test]
    fn test_allocation_tracking() {
        let mut state = VmState::new(vec![], Vec::new(), 100, 1024, 1, 100);

        assert_eq!(GcIntegration::allocations_since_last_gc(&state), 0);
        assert_eq!(GcIntegration::gc_threshold(&state), 512); // mem_limit / 2
    }
}
