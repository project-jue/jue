use crate::types::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

/// Heap object types that can be managed by the garbage collector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HeapObject {
    Closure(Closure),
    Array(Array),
    // Other heap object types can be added here
}

/// Closure representation for GC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Closure {
    pub code_ptr: usize,
    pub environment: HashMap<usize, Value>,
}

/// Array representation for GC
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Array {
    pub elements: Vec<Value>,
}

impl Array {
    pub fn elements(&self) -> &[Value] {
        &self.elements
    }
}

/// Mark-and-sweep garbage collector
#[derive(Clone, Serialize, Deserialize)]
pub struct GarbageCollector {
    pub heap: Vec<HeapObject>,
    pub roots: Vec<GcRoot>,
    pub allocation_threshold: usize,
    pub allocations_since_last_gc: usize,
    pub gc_stats: GcStats,
}

impl GarbageCollector {
    pub fn new(heap_size: usize, threshold: usize) -> Self {
        Self {
            heap: Vec::with_capacity(heap_size),
            roots: Vec::new(),
            allocation_threshold: threshold,
            allocations_since_last_gc: 0,
            gc_stats: GcStats::default(),
        }
    }

    pub fn allocate(&mut self, object: HeapObject) -> GcPtr {
        self.allocations_since_last_gc += 1;

        if self.allocations_since_last_gc >= self.allocation_threshold {
            self.collect();
        }

        let ptr = self.heap.len();
        self.heap.push(object);
        GcPtr(ptr)
    }

    pub fn collect(&mut self) {
        let start_time = Instant::now();
        let mut marked = vec![false; self.heap.len()];

        // Mark phase
        self.mark_roots(&mut marked);

        // Sweep phase
        let mut new_heap = Vec::new();
        let mut new_index_map = Vec::new();

        for (old_index, object) in self.heap.iter().enumerate() {
            if marked[old_index] {
                new_index_map.push(Some(new_heap.len()));
                new_heap.push(object.clone());
            } else {
                new_index_map.push(None);
            }
        }

        // Update roots
        for root in &mut self.roots {
            if let Some(new_index) = new_index_map[root.ptr.0] {
                root.ptr = GcPtr(new_index);
            }
        }

        // Update stats
        let collected = self.heap.len() - new_heap.len();
        let duration = start_time.elapsed();

        self.gc_stats.collections += 1;
        self.gc_stats.objects_collected += collected as u32;
        self.gc_stats.total_time_millis += duration.as_millis() as u64;

        // Update max pause time
        if duration.as_millis() as u64 > self.gc_stats.max_pause_time_millis {
            self.gc_stats.max_pause_time_millis = duration.as_millis() as u64;
        }

        self.allocations_since_last_gc = 0;
        self.heap = new_heap;
    }

    fn mark_roots(&self, marked: &mut [bool]) {
        let mut worklist = Vec::new();

        // Add all roots to worklist
        for root in &self.roots {
            if !marked[root.ptr.0] {
                marked[root.ptr.0] = true;
                worklist.push(root.ptr.0);
            }
        }

        // Process worklist
        while let Some(index) = worklist.pop() {
            if let Some(object) = self.heap.get(index) {
                match object {
                    HeapObject::Closure(closure) => {
                        // Mark closure environment
                        for value in closure.environment.values() {
                            if let Value::GcPtr(ptr) = value {
                                if !marked[ptr.0] {
                                    marked[ptr.0] = true;
                                    worklist.push(ptr.0);
                                }
                            }
                        }
                    }
                    HeapObject::Array(array) => {
                        // Mark array elements
                        for value in array.elements() {
                            if let Value::GcPtr(ptr) = value {
                                if !marked[ptr.0] {
                                    marked[ptr.0] = true;
                                    worklist.push(ptr.0);
                                }
                            }
                        }
                    } // Other heap object types...
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GcStats {
    pub collections: u32,
    pub objects_collected: u32,
    pub total_time_millis: u64,
    pub max_pause_time_millis: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GcPtr(pub usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GcRoot {
    pub ptr: GcPtr,
    pub description: String,
}
