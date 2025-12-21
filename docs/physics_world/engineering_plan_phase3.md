# Engineering Plan: Phase 3 - Advanced Features
**Duration**: 4 weeks
**Objective**: Implement advanced VM features including garbage collection, debugging support, and performance monitoring

## 📋 Overview
This phase focuses on implementing advanced features that enhance the robustness, debuggability, and performance of the Physics World VM. These features build on the solid foundation established in Phases 1 and 2.

## 🗂️ File Modifications

### 1. **Garbage Collection System**

#### `physics_world/src/vm/gc.rs`
**New File**:
```rust
// Mark-and-sweep garbage collector
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
        self.gc_stats.objects_collected += collected;
        self.gc_stats.total_time += duration;
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
                    }
                    // Other heap object types...
                }
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GcStats {
    pub collections: u32,
    pub objects_collected: u32,
    pub total_time: Duration,
    pub max_pause_time: Duration,
}

#[derive(Debug, Clone, Copy)]
pub struct GcPtr(usize);

#[derive(Debug, Clone)]
pub struct GcRoot {
    pub ptr: GcPtr,
    pub description: String,
}
```

#### `physics_world/src/vm/state.rs`
**Enhanced Modifications**:
```rust
// Add GC integration to VM state
pub struct VmState {
    // ... existing fields ...
    pub gc: GarbageCollector,
    pub gc_enabled: bool,
    pub gc_threshold: usize,
}

impl VmState {
    pub fn new(
        bytecode: Vec<u8>,
        functions: Vec<Function>,
        max_recursion_depth: usize,
        stack_size: usize,
        heap_size: usize,
    ) -> Self {
        let gc = GarbageCollector::new(heap_size, heap_size / 2);

        Self {
            // ... existing initialization ...
            gc,
            gc_enabled: true,
            gc_threshold: heap_size / 2,
        }
    }

    pub fn allocate_heap_object(&mut self, object: HeapObject) -> Result<Value, VmError> {
        if !self.gc_enabled {
            return Err(VmError::gc_disabled());
        }

        let ptr = self.gc.allocate(object);
        Ok(Value::GcPtr(ptr))
    }

    pub fn add_gc_root(&mut self, ptr: GcPtr, description: &str) {
        self.gc.roots.push(GcRoot {
            ptr,
            description: description.to_string(),
        });
    }

    pub fn remove_gc_root(&mut self, ptr: GcPtr) {
        self.gc.roots.retain(|root| root.ptr != ptr);
    }

    pub fn get_gc_stats(&self) -> GcStats {
        self.gc.gc_stats.clone()
    }

    pub fn set_gc_enabled(&mut self, enabled: bool) {
        self.gc_enabled = enabled;
    }
}
```

### 2. **Debugging Support**

#### `physics_world/src/vm/debug.rs`
**Enhanced File**:
```rust
// Enhanced debugging support
pub struct Debugger {
    pub breakpoints: HashSet<usize>,
    pub watchpoints: HashMap<String, Watchpoint>,
    pub step_mode: bool,
    pub call_stack_depth: usize,
    pub debug_log: Vec<DebugEvent>,
    pub vm_state_history: Vec<VmStateSnapshot>,
}

impl Debugger {
    pub fn new() -> Self {
        Self {
            breakpoints: HashSet::new(),
            watchpoints: HashMap::new(),
            step_mode: false,
            call_stack_depth: 0,
            debug_log: Vec::new(),
            vm_state_history: Vec::new(),
        }
    }

    pub fn add_breakpoint(&mut self, address: usize) {
        self.breakpoints.insert(address);
    }

    pub fn remove_breakpoint(&mut self, address: usize) {
        self.breakpoints.remove(&address);
    }

    pub fn add_watchpoint(&mut self, name: &str, expression: &str) {
        self.watchpoints.insert(
            name.to_string(),
            Watchpoint {
                expression: expression.to_string(),
                last_value: None,
            }
        );
    }

    pub fn check_breakpoints(&self, vm: &VmState) -> bool {
        self.breakpoints.contains(&vm.ip)
    }

    pub fn check_watchpoints(&mut self, vm: &VmState) -> Vec<WatchpointTrigger> {
        let mut triggers = Vec::new();

        for (name, watchpoint) in &mut self.watchpoints {
            if let Some(current_value) = self.evaluate_watchpoint(&watchpoint.expression, vm) {
                if let Some(last_value) = &watchpoint.last_value {
                    if current_value != *last_value {
                        triggers.push(WatchpointTrigger {
                            name: name.clone(),
                            old_value: last_value.clone(),
                            new_value: current_value,
                        });
                    }
                }
                watchpoint.last_value = Some(current_value);
            }
        }

        triggers
    }

    pub fn log_debug_event(&mut self, event: DebugEvent) {
        self.debug_log.push(event);
    }

    pub fn capture_vm_state(&mut self, vm: &VmState) {
        self.vm_state_history.push(VmStateSnapshot::from_vm(vm));
    }

    pub fn get_debug_info(&self) -> DebugInfo {
        DebugInfo {
            breakpoints: self.breakpoints.clone(),
            watchpoints: self.watchpoints.clone(),
            events: self.debug_log.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DebugEvent {
    pub timestamp: Instant,
    pub event_type: DebugEventType,
    pub data: DebugEventData,
}

#[derive(Debug, Clone)]
pub enum DebugEventType {
    BreakpointHit,
    WatchpointTriggered,
    StepCompleted,
    FunctionEntry,
    FunctionExit,
    ExceptionThrown,
    ExceptionCaught,
}

#[derive(Debug, Clone)]
pub enum DebugEventData {
    Breakpoint(usize),
    Watchpoint(WatchpointTrigger),
    Function(String),
    Exception(String),
    // Other data types...
}
```

### 3. **Performance Monitoring**

#### `physics_world/src/vm/performance.rs`
**New File**:
```rust
// Performance monitoring system
pub struct PerformanceMonitor {
    pub metrics: PerformanceMetrics,
    pub counters: HashMap<String, u64>,
    pub timers: HashMap<String, Duration>,
    pub samples: Vec<PerformanceSample>,
    pub sample_interval: Duration,
    pub last_sample_time: Instant,
}

impl PerformanceMonitor {
    pub fn new(sample_interval: Duration) -> Self {
        Self {
            metrics: PerformanceMetrics::default(),
            counters: HashMap::new(),
            timers: HashMap::new(),
            samples: Vec::new(),
            sample_interval,
            last_sample_time: Instant::now(),
        }
    }

    pub fn increment_counter(&mut self, name: &str, value: u64) {
        *self.counters.entry(name.to_string()).or_insert(0) += value;
    }

    pub fn start_timer(&mut self, name: &str) {
        self.timers.insert(name.to_string(), Instant::now().elapsed());
    }

    pub fn stop_timer(&mut self, name: &str) {
        if let Some(start_time) = self.timers.remove(name) {
            let duration = Instant::now().duration_since(start_time);
            self.metrics.total_time += duration;
            self.metrics.counter_increments += 1;
        }
    }

    pub fn take_sample(&mut self, vm: &VmState) {
        let now = Instant::now();
        if now.duration_since(self.last_sample_time) >= self.sample_interval {
            let sample = PerformanceSample {
                timestamp: now,
                instructions_executed: vm.instructions_executed,
                heap_usage: vm.gc.heap.len(),
                call_stack_depth: vm.call_stack.len(),
                counters: self.counters.clone(),
            };

            self.samples.push(sample);
            self.last_sample_time = now;
        }
    }

    pub fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.clone()
    }

    pub fn get_analysis(&self) -> PerformanceAnalysis {
        let mut analysis = PerformanceAnalysis::default();

        if !self.samples.is_empty() {
            let first_sample = &self.samples[0];
            let last_sample = self.samples.last().unwrap();

            analysis.instructions_per_second = (last_sample.instructions_executed - first_sample.instructions_executed) as f64 /
                last_sample.timestamp.duration_since(first_sample.timestamp).as_secs_f64();

            analysis.heap_growth_rate = (last_sample.heap_usage - first_sample.heap_usage) as f64 /
                self.samples.len() as f64;
        }

        analysis
    }
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceMetrics {
    pub instructions_executed: u64,
    pub heap_allocations: u64,
    pub gc_collections: u64,
    pub total_time: Duration,
    pub counter_increments: u64,
    pub timer_operations: u64,
}

#[derive(Debug, Clone, Default)]
pub struct PerformanceAnalysis {
    pub instructions_per_second: f64,
    pub heap_growth_rate: f64,
    pub gc_efficiency: f64,
    pub counter_trends: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct PerformanceSample {
    pub timestamp: Instant,
    pub instructions_executed: u64,
    pub heap_usage: usize,
    pub call_stack_depth: usize,
    pub counters: HashMap<String, u64>,
}
```

### 4. **Enhanced Error Handling**

#### `physics_world/src/vm/error.rs`
**Enhanced File**:
```rust
// Enhanced error handling with context
#[derive(Debug, Clone, thiserror::Error)]
pub enum VmError {
    #[error("Stack overflow at {context}")]
    StackOverflow {
        context: ErrorContext,
        max_depth: usize,
        attempted_depth: usize,
    },

    #[error("Stack underflow at {context}")]
    StackUnderflow {
        context: ErrorContext,
        expected: usize,
        actual: usize,
    },

    #[error("Recursion limit exceeded at {context}")]
    RecursionLimitExceeded {
        context: ErrorContext,
        limit: usize,
        depth: usize,
    },

    #[error("Invalid opcode {opcode:02X} at {context}")]
    InvalidOpcode {
        context: ErrorContext,
        opcode: u8,
    },

    #[error("Type error at {context}: {message}")]
    TypeError {
        context: ErrorContext,
        message: String,
    },

    #[error("Memory error at {context}: {message}")]
    MemoryError {
        context: ErrorContext,
        message: String,
    },

    #[error("GC disabled")]
    GcDisabled,

    #[error("Heap exhausted")]
    HeapExhausted,

    #[error("Debugger error: {message}")]
    DebuggerError {
        message: String,
    },
}

#[derive(Debug, Clone)]
pub struct ErrorContext {
    pub ip: usize,
    pub function: Option<String>,
    pub call_stack: Vec<CallFrameInfo>,
    pub vm_state: VmStateSummary,
}

impl ErrorContext {
    pub fn new(vm: &VmState) -> Self {
        let call_stack = vm.call_stack.iter()
            .map(|frame| CallFrameInfo {
                function: vm.get_function_name(frame.return_address),
                depth: frame.recursion_depth,
            })
            .collect();

        Self {
            ip: vm.ip,
            function: vm.get_current_function_name(),
            call_stack,
            vm_state: VmStateSummary::from_vm(vm),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VmStateSummary {
    pub stack_size: usize,
    pub heap_size: usize,
    pub call_stack_depth: usize,
    pub recursion_depth: usize,
}

impl VmStateSummary {
    pub fn from_vm(vm: &VmState) -> Self {
        Self {
            stack_size: vm.stack.len(),
            heap_size: vm.gc.heap.len(),
            call_stack_depth: vm.call_stack.len(),
            recursion_depth: vm.call_stack.last().map_or(0, |f| f.recursion_depth),
        }
    }
}
```

## 📋 Implementation Plan

### Week 1: Garbage Collection System
```markdown
[ ] Implement mark-and-sweep garbage collector
[ ] Integrate GC with VM state
[ ] Add heap allocation tracking
[ ] Implement GC root management
[ ] Create GC test suite
[ ] Benchmark GC performance
[ ] Analyze memory usage patterns
```

### Week 2: Debugging Support
```markdown
[ ] Implement breakpoint system
[ ] Add watchpoint support
[ ] Create step-by-step debugging
[ ] Implement call stack inspection
[ ] Add debug logging
[ ] Create debugging test cases
[ ] Integrate with existing error system
```

### Week 3: Performance Monitoring
```markdown
[ ] Implement performance counters
[ ] Add timing measurements
[ ] Create sampling system
[ ] Implement performance analysis
[ ] Add memory usage tracking
[ ] Create performance test suite
[ ] Benchmark monitoring overhead
```

### Week 4: Integration and Testing
```markdown
[ ] Integrate all advanced features
[ ] Create comprehensive test suite
[ ] Performance benchmarking
[ ] Memory usage analysis
[ ] Error handling verification
[ ] Documentation updates
[ ] Final integration testing
```

## 🎯 Success Criteria
- ✅ Complete garbage collection system
- ✅ Comprehensive debugging support
- ✅ Performance monitoring system
- ✅ Enhanced error handling
- ✅ All Phase 1 and 2 tests still passing
- ✅ Memory leak detection
- ✅ Performance analysis capabilities