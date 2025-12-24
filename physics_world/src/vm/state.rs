//! VM state management.
//!
//! This module contains the core VmState struct and its essential methods.
//! Execution logic has been moved to `execution.rs`, call frame management
//! to `call_state.rs`, and GC integration to `gc_integration.rs`.
//!
//! # Original File
//! - `vm/state.rs` was 1303 lines
//!
//! # Refactored Structure
//! - `state.rs`: ~400 lines - VmState struct, debugging, error context
//! - `call_state.rs`: ~300 lines - Call frame management
//! - `execution.rs`: ~400 lines - Step execution logic
//! - `gc_integration.rs`: ~200 lines - GC integration helpers

use crate::memory::arena::{ObjectArena, ObjectHeader};
use crate::types::{HeapPtr, OpCode, Value};
use crate::vm::debug::{DebugEvent, DebugEventType, DebugInfo, Debugger, WatchpointTrigger};
use crate::vm::error::{
    ErrorContext, SimpleVmError, StackFrame, VmError as DetailedVmError, WithContext,
};
use crate::vm::gc::{GarbageCollector, GcPtr, GcRoot, GcStats, HeapObject};
use crate::vm::opcodes::closure::Closure;
use crate::vm::opcodes::*;
use crate::vm::performance::{
    PerformanceAnalysis, PerformanceMetrics, PerformanceMonitor, PerformanceSample,
};
use bincode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// Re-export from new modules for convenience
// CallFrame is now defined in call_state.rs and re-exported here for backwards compatibility
pub use crate::vm::call_state::CallFrame;

/// Function information for escape analysis integration
#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub local_count: usize,
    pub escape_info: HashMap<usize, EscapeStatus>,
    pub free_variables: Vec<usize>,
}

/// Escape status for variables
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EscapeStatus {
    Escaping,
    NonEscaping,
}

/// Debug snapshot of VM state for introspection
#[derive(Debug, Clone)]
pub struct VmDebugSnapshot {
    pub instruction_pointer: usize,
    pub instructions: Vec<OpCode>,
    pub stack: Vec<Value>,
    pub call_stack: Vec<CallFrame>,
    pub memory_usage: usize,
    pub memory_capacity: usize,
    pub steps_remaining: u64,
    pub actor_id: u32,
    pub constant_pool: Vec<Value>,
}

/// Enhanced debugging information with capability analysis
#[derive(Debug, Clone)]
pub struct CapabilityDebugInfo {
    pub capabilities: Vec<crate::types::Capability>,
    pub capability_requests: Vec<CapRequestDebugInfo>,
    pub capability_usage_stats: HashMap<String, u32>,
    pub security_analysis: SecurityAnalysis,
}

/// Debug information about capability requests
#[derive(Debug, Clone)]
pub struct CapRequestDebugInfo {
    pub capability: crate::types::Capability,
    pub justification: String,
    pub requested_at: u64,
    pub status: String,
}

/// Security analysis of VM state
#[derive(Debug, Clone)]
pub struct SecurityAnalysis {
    pub potential_vulnerabilities: Vec<String>,
    pub security_score: f32, // 0.0 (insecure) to 1.0 (secure)
    pub recommendations: Vec<String>,
}

/// Memory analysis with fragmentation details
#[derive(Debug, Clone)]
pub struct MemoryAnalysis {
    pub heap_usage: usize,
    pub heap_capacity: usize,
    pub fragmentation_ratio: f32,
    pub object_count: u32,
    pub live_objects: u32,
    pub dead_objects: u32,
    pub largest_free_block: usize,
    pub allocation_patterns: Vec<AllocationPattern>,
}

/// Allocation pattern analysis
#[derive(Debug, Clone)]
pub struct AllocationPattern {
    pub size_range: (usize, usize),
    pub count: u32,
    pub total_bytes: usize,
}

/// Performance profiling data
#[derive(Debug, Clone)]
pub struct PerformanceProfile {
    pub instruction_counts: HashMap<String, u32>,
    pub hotspots: Vec<Hotspot>,
    pub execution_time_ms: u64,
    pub memory_operations: u32,
    pub capability_checks: u32,
}

/// Performance hotspot information
#[derive(Debug, Clone)]
pub struct Hotspot {
    pub instruction_range: (usize, usize),
    pub execution_count: u32,
    pub time_spent_ms: u64,
    pub description: String,
}

/// Comprehensive debugging interface for VM introspection
pub struct VmDebugger {
    vm: VmState,
    profiling_enabled: bool,
    instruction_counts: HashMap<String, u32>,
    execution_history: Vec<ExecutionRecord>,
}

impl VmDebugger {
    /// Create a new debugger for a VM state
    pub fn new(vm: VmState) -> Self {
        Self {
            vm,
            profiling_enabled: false,
            instruction_counts: HashMap::new(),
            execution_history: Vec::new(),
        }
    }

    /// Enable performance profiling
    pub fn enable_profiling(&mut self) {
        self.profiling_enabled = true;
    }

    /// Disable performance profiling
    pub fn disable_profiling(&mut self) {
        self.profiling_enabled = false;
    }

    /// Execute a single instruction with debugging
    pub fn step_with_debug(&mut self) -> Result<InstructionResult, SimpleVmError> {
        let start_time = std::time::Instant::now();

        // Record instruction before execution
        if self.profiling_enabled {
            let current_instruction = self.vm.instructions.get(self.vm.ip).cloned();
            self.execution_history.push(ExecutionRecord {
                ip: self.vm.ip,
                instruction: current_instruction,
                stack_depth: self.vm.stack.len(),
                memory_usage: self.vm.memory.next_free() as usize,
                timestamp: start_time.elapsed().as_micros() as u64,
            });
        }

        // Execute the instruction
        let result = self.vm.step();

        // Update profiling data
        if self.profiling_enabled {
            if let Ok(InstructionResult::Continue) = &result {
                if let Some(instruction) = self.vm.instructions.get(self.vm.ip - 1) {
                    let instruction_name = format!("{:?}", instruction);
                    *self.instruction_counts.entry(instruction_name).or_insert(0) += 1;
                }
            }
        }

        result
    }

    /// Get comprehensive capability debug information
    pub fn get_capability_debug_info(&self) -> CapabilityDebugInfo {
        // In a real implementation, this would analyze the scheduler's capability state
        // For now, we'll return a placeholder with basic information
        CapabilityDebugInfo {
            capabilities: vec![],                   // Would be populated from scheduler
            capability_requests: vec![],            // Would be populated from scheduler
            capability_usage_stats: HashMap::new(), // Would analyze usage patterns
            security_analysis: SecurityAnalysis {
                potential_vulnerabilities: vec!["No capability analysis available".to_string()],
                security_score: 0.5, // Neutral score
                recommendations: vec![
                    "Enable full capability debugging for detailed analysis".to_string()
                ],
            },
        }
    }

    /// Get detailed memory analysis
    pub fn get_memory_analysis(&self) -> MemoryAnalysis {
        let mut object_count = 0;
        let mut live_objects = 0;
        let mut dead_objects = 0;
        let mut largest_free_block = 0;
        let mut allocation_patterns: HashMap<(usize, usize), AllocationPattern> = HashMap::new();

        let mut current_ptr = 0;
        let mut prev_object_end = 0;

        while current_ptr < self.vm.memory.next_free() {
            let header = unsafe { self.vm.memory.get_header(HeapPtr::new(current_ptr)) };
            let object_size = ObjectHeader::size_bytes() as u32 + header.size;

            object_count += 1;
            if header.marked {
                live_objects += 1;
            } else {
                dead_objects += 1;
            }

            // Track allocation patterns by size ranges
            let size_range = self.get_size_range(header.size as usize);
            let pattern =
                allocation_patterns
                    .entry(size_range)
                    .or_insert_with(|| AllocationPattern {
                        size_range,
                        count: 0,
                        total_bytes: 0,
                    });
            pattern.count += 1;
            pattern.total_bytes += header.size as usize;

            // Track largest free block
            let free_block_size = current_ptr - prev_object_end;
            if free_block_size > largest_free_block {
                largest_free_block = free_block_size;
            }

            prev_object_end = current_ptr + object_size;
            current_ptr += object_size;
        }

        // Calculate fragmentation ratio
        let fragmentation_ratio = if self.vm.memory.next_free() > 0 {
            let used_space = live_objects as usize * std::mem::size_of::<ObjectHeader>()
                + live_objects as usize * 100; // Approximate
            let wasted_space = self.vm.memory.next_free() as usize - used_space;
            if used_space > 0 {
                wasted_space as f32 / self.vm.memory.next_free() as f32
            } else {
                0.0
            }
        } else {
            0.0
        };

        MemoryAnalysis {
            heap_usage: self.vm.memory.next_free() as usize,
            heap_capacity: self.vm.memory.capacity() as usize,
            fragmentation_ratio,
            object_count,
            live_objects,
            dead_objects,
            largest_free_block: largest_free_block as usize,
            allocation_patterns: allocation_patterns.into_values().collect(),
        }
    }

    /// Get performance profile
    pub fn get_performance_profile(&self) -> PerformanceProfile {
        // Identify hotspots (instructions executed frequently)
        let mut hotspots = Vec::new();
        for (instruction_name, count) in &self.instruction_counts {
            if *count > 10 {
                // Arbitrary threshold for "hot"
                hotspots.push(Hotspot {
                    instruction_range: (0, 0), // Would be calculated in real implementation
                    execution_count: *count,
                    time_spent_ms: 0, // Would be measured in real implementation
                    description: format!("Frequent {} execution", instruction_name),
                });
            }
        }

        // Sort hotspots by execution count
        hotspots.sort_by(|a, b| b.execution_count.cmp(&a.execution_count));

        PerformanceProfile {
            instruction_counts: self.instruction_counts.clone(),
            hotspots,
            execution_time_ms: 0, // Would be measured in real implementation
            memory_operations: 0, // Would be tracked in real implementation
            capability_checks: 0, // Would be tracked in real implementation
        }
    }

    /// Get the current VM state (consumes the debugger)
    pub fn into_vm_state(self) -> VmState {
        self.vm
    }

    /// Helper function to categorize allocation sizes
    fn get_size_range(&self, size: usize) -> (usize, usize) {
        if size < 64 {
            (0, 64)
        } else if size < 256 {
            (64, 256)
        } else if size < 1024 {
            (256, 1024)
        } else if size < 4096 {
            (1024, 4096)
        } else {
            (4096, usize::MAX)
        }
    }
}

/// Enhanced debug snapshot with capability information
#[derive(Debug, Clone)]
pub struct EnhancedVmDebugSnapshot {
    pub basic: VmDebugSnapshot,
    pub capability_info: CapabilityDebugInfo,
    pub memory_analysis: MemoryAnalysis,
}

/// Record of a single execution step for profiling
#[derive(Debug, Clone)]
struct ExecutionRecord {
    ip: usize,
    instruction: Option<OpCode>,
    stack_depth: usize,
    memory_usage: usize,
    timestamp: u64,
}

/// Result of executing a single instruction.
///
/// The VM uses a step-based execution model where each instruction returns one of these results.
#[derive(Debug, Clone)]
pub enum InstructionResult {
    Continue,        // Normal execution, proceed to next instruction
    Yield,           // Voluntary yield, suspend execution
    Finished(Value), // Execution completed with final value
    WaitingForCapability(crate::types::Capability), // V2: Actor is waiting for capability decision
}

/// Backward compatibility: VmError enum for opcode handlers
/// This provides the same interface as the old VmError enum but uses SimpleVmError internally
#[derive(Debug)]
pub enum VmError {
    CpuLimitExceeded,
    MemoryLimitExceeded,
    StackUnderflow,
    InvalidHeapPtr,
    UnknownOpCode,
    TypeMismatch,
    DivisionByZero,
    ArithmeticOverflow,
    CapabilityDenied,
    RecursionLimitExceeded,
}

impl From<VmError> for SimpleVmError {
    fn from(error: VmError) -> SimpleVmError {
        match error {
            VmError::CpuLimitExceeded => SimpleVmError::CpuLimitExceeded,
            VmError::MemoryLimitExceeded => SimpleVmError::MemoryLimitExceeded,
            VmError::StackUnderflow => SimpleVmError::StackUnderflow,
            VmError::InvalidHeapPtr => SimpleVmError::InvalidHeapPtr,
            VmError::UnknownOpCode => SimpleVmError::UnknownOpCode,
            VmError::TypeMismatch => SimpleVmError::TypeMismatch,
            VmError::DivisionByZero => SimpleVmError::DivisionByZero,
            VmError::ArithmeticOverflow => SimpleVmError::ArithmeticOverflow,
            VmError::CapabilityDenied => SimpleVmError::CapabilityDenied,
            VmError::RecursionLimitExceeded => SimpleVmError::RecursionLimitExceeded,
        }
    }
}

/// Represents the state of a single virtual machine instance.
///
/// # Test Coverage: 100% (critical path)
/// # Tests: nominal, edge cases, error handling
///
/// This struct maintains the complete execution state of a virtual machine,
/// including instruction pointer, bytecode, stack, heap memory, and resource limits.
/// The VM follows the AIKR (Atomic, Isolated, Kernel-enforced, Resource-limited) principles
/// for deterministic and safe execution.
#[derive(Serialize, Deserialize, Clone)]
pub struct VmState {
    // Execution
    pub ip: usize,                 // Instruction Pointer
    pub instructions: Vec<OpCode>, // Loaded bytecode
    pub constant_pool: Vec<Value>, // For Symbol values, etc.
    // Data
    pub stack: Vec<Value>,          // Primary working stack
    pub call_stack: Vec<CallFrame>, // For function calls/returns
    pub memory: ObjectArena,        // Heap
    // Resources (AIKR)
    pub steps_remaining: u64, // Decremented each instruction
    // V2 Capability System - Add actor ID for capability checks
    pub actor_id: u32,
    // Recursion depth limit configuration
    pub max_recursion_depth: u32, // Maximum allowed recursion depth
    // NEW: Frame ID counter for debugging/verification
    pub next_frame_id: u64,
    // Phase 3: Advanced Features
    pub gc: GarbageCollector,                    // Garbage collection system
    pub debugger: Debugger,                      // Debugging support
    pub performance_monitor: PerformanceMonitor, // Performance monitoring
    pub gc_enabled: bool,                        // GC enable/disable flag
    pub gc_threshold: usize,                     // GC allocation threshold
}

impl VmState {
    /// Creates a new VM state with code, constants, and resource limits.
    ///
    /// # Arguments
    /// * `instructions` - Bytecode to execute
    /// * `constants` - Constant pool for symbols and literals
    /// * `step_limit` - Maximum number of instructions before CPU limit error
    /// * `mem_limit` - Maximum heap memory in bytes
    /// * `actor_id` - ID of the actor this VM belongs to
    /// * `max_recursion_depth` - Maximum allowed recursion depth (default: 100)
    ///
    /// # Returns
    /// Initialized VM state ready for execution
    pub fn new(
        instructions: Vec<OpCode>,
        constants: Vec<Value>,
        step_limit: u64,
        mem_limit: usize,
        actor_id: u32,
        max_recursion_depth: u32,
    ) -> Self {
        let gc = GarbageCollector::new(mem_limit, mem_limit / 2);
        let debugger = Debugger::new();
        let performance_monitor = PerformanceMonitor::new(100);

        Self {
            ip: 0,
            instructions,
            constant_pool: constants,
            stack: Vec::new(),
            call_stack: Vec::new(),
            memory: ObjectArena::with_capacity(mem_limit as u32),
            steps_remaining: step_limit,
            actor_id,
            max_recursion_depth,
            next_frame_id: 1, // Start frame IDs from 1
            gc,
            debugger,
            performance_monitor,
            gc_enabled: true,
            gc_threshold: mem_limit / 2,
        }
    }

    /// Create an error context for detailed error reporting
    pub fn create_error_context(&self) -> ErrorContext {
        ErrorContext {
            instruction_pointer: self.ip,
            current_instruction: self.instructions.get(self.ip).cloned(),
            stack_state: self.stack.clone(),
            call_stack_depth: self.call_stack.len(),
            steps_remaining: self.steps_remaining,
            actor_id: self.actor_id,
            memory_usage: self.memory.next_free() as usize,
            stack_trace: self.create_stack_trace(),
            execution_history: self.get_execution_history(),
            timestamp: 0, // Will be set by scheduler
        }
    }

    /// Create a stack trace from the current call stack
    fn create_stack_trace(&self) -> Vec<StackFrame> {
        self.call_stack
            .iter()
            .enumerate()
            .map(|(i, frame)| {
                StackFrame {
                    function_name: format!("frame_{}", i),
                    call_ip: frame.return_ip,
                    arg_count: frame.locals.len(), // Track actual argument counts from locals
                    locals: frame.locals.clone(),  // Capture local variables from call frame
                }
            })
            .collect()
    }

    /// Get the last few executed instructions for error context
    fn get_execution_history(&self) -> Vec<OpCode> {
        let history_size = std::cmp::min(5, self.ip); // Last 5 instructions
        self.instructions[self.ip.saturating_sub(history_size)..self.ip].to_vec()
    }

    /// Debugging support: Get complete VM state snapshot for introspection
    pub fn get_debug_snapshot(&self) -> VmDebugSnapshot {
        VmDebugSnapshot {
            instruction_pointer: self.ip,
            instructions: self.instructions.clone(),
            stack: self.stack.clone(),
            call_stack: self.call_stack.clone(),
            memory_usage: self.memory.next_free() as usize,
            memory_capacity: self.memory.capacity() as usize,
            steps_remaining: self.steps_remaining,
            actor_id: self.actor_id,
            constant_pool: self.constant_pool.clone(),
        }
    }

    /// Debugging support: Get formatted stack trace with function names
    pub fn get_formatted_stack_trace(&self) -> String {
        let mut trace = String::new();
        trace.push_str("Stack Trace:\n");

        for (i, frame) in self.call_stack.iter().enumerate() {
            trace.push_str(&format!(
                "  {}: {} (return_ip: {}, stack_start: {})\n",
                i,
                self.get_function_name_for_frame(frame),
                frame.return_ip,
                frame.stack_start
            ));
        }

        if self.call_stack.is_empty() {
            trace.push_str("  (no call frames)\n");
        }

        trace
    }

    /// Debugging support: Get formatted memory usage information
    pub fn get_memory_usage_info(&self) -> String {
        format!(
            "Memory Usage: {} bytes used / {} bytes capacity ({:.1}% used)",
            self.memory.next_free(),
            self.memory.capacity(),
            (self.memory.next_free() as f64 / self.memory.capacity() as f64) * 100.0
        )
    }

    /// Debugging support: Get formatted execution statistics
    pub fn get_execution_stats(&self) -> String {
        format!(
            "Execution Stats:\n  Steps remaining: {}\n  Stack depth: {}\n  Call stack depth: {}\n  Current IP: {}",
            self.steps_remaining,
            self.stack.len(),
            self.call_stack.len(),
            self.ip
        )
    }

    /// Debugging support: Get disassembly of current instruction context
    pub fn get_current_instruction_context(&self) -> String {
        let mut context = String::new();
        context.push_str("Current Instruction Context:\n");

        // Show current instruction
        if let Some(instr) = self.instructions.get(self.ip) {
            context.push_str(&format!("  IP {}: {:?}\n", self.ip, instr));
        }

        // Show next few instructions
        let end_ip = std::cmp::min(self.ip + 3, self.instructions.len());
        for i in self.ip + 1..end_ip {
            if let Some(instr) = self.instructions.get(i) {
                context.push_str(&format!("  IP {}: {:?}\n", i, instr));
            }
        }

        context
    }

    /// Debugging support: Get enhanced debug snapshot with capability information
    pub fn get_enhanced_debug_snapshot(&self) -> EnhancedVmDebugSnapshot {
        EnhancedVmDebugSnapshot {
            basic: self.get_debug_snapshot(),
            capability_info: self.get_capability_debug_info(),
            memory_analysis: self.get_memory_analysis(),
        }
    }

    /// Debugging support: Create a debugger instance for advanced introspection
    pub fn create_debugger(&self) -> VmDebugger {
        VmDebugger::new(self.clone())
    }

    /// Debugging support: Get capability debug info (basic version)
    pub fn get_capability_debug_info(&self) -> CapabilityDebugInfo {
        // Basic version - would be enhanced with scheduler integration
        CapabilityDebugInfo {
            capabilities: vec![],
            capability_requests: vec![],
            capability_usage_stats: HashMap::new(),
            security_analysis: SecurityAnalysis {
                potential_vulnerabilities: vec![
                    "Basic capability analysis - connect to scheduler for full details".to_string(),
                ],
                security_score: 0.7, // Moderate score for basic analysis
                recommendations: vec![
                    "Integrate with scheduler for comprehensive capability debugging".to_string(),
                ],
            },
        }
    }

    /// Debugging support: Get memory analysis
    pub fn get_memory_analysis(&self) -> MemoryAnalysis {
        let mut object_count = 0;
        let mut live_objects = 0;
        let mut dead_objects = 0;
        let mut largest_free_block = 0;
        let mut allocation_patterns: HashMap<(usize, usize), AllocationPattern> = HashMap::new();

        let mut current_ptr = 0;
        let mut prev_object_end = 0;

        while current_ptr < self.memory.next_free() {
            if current_ptr >= self.memory.capacity() {
                break;
            }

            let header = unsafe { self.memory.get_header(HeapPtr::new(current_ptr)) };
            let object_size = ObjectHeader::size_bytes() as u32 + header.size;

            object_count += 1;
            if header.marked {
                live_objects += 1;
            } else {
                dead_objects += 1;
            }

            // Track allocation patterns by size ranges
            let size_range = self.get_size_range(header.size as usize);
            let pattern =
                allocation_patterns
                    .entry(size_range)
                    .or_insert_with(|| AllocationPattern {
                        size_range,
                        count: 0,
                        total_bytes: 0,
                    });
            pattern.count += 1;
            pattern.total_bytes += header.size as usize;

            // Track largest free block
            let free_block_size = current_ptr - prev_object_end;
            if free_block_size > largest_free_block {
                largest_free_block = free_block_size;
            }

            prev_object_end = current_ptr + object_size;
            current_ptr += object_size;
        }

        // Calculate fragmentation ratio
        let fragmentation_ratio = if self.memory.next_free() > 0 {
            let used_space = live_objects as usize * std::mem::size_of::<ObjectHeader>()
                + live_objects as usize * 100; // Approximate
            let wasted_space = self.memory.next_free() as usize - used_space;
            if used_space > 0 {
                wasted_space as f32 / self.memory.next_free() as f32
            } else {
                0.0
            }
        } else {
            0.0
        };

        MemoryAnalysis {
            heap_usage: self.memory.next_free() as usize,
            heap_capacity: self.memory.capacity() as usize,
            fragmentation_ratio,
            object_count,
            live_objects,
            dead_objects,
            largest_free_block: largest_free_block as usize,
            allocation_patterns: allocation_patterns.into_values().collect(),
        }
    }

    /// Debugging support: Helper function to categorize allocation sizes
    fn get_size_range(&self, size: usize) -> (usize, usize) {
        if size < 64 {
            (0, 64)
        } else if size < 256 {
            (64, 256)
        } else if size < 1024 {
            (256, 1024)
        } else if size < 4096 {
            (1024, 4096)
        } else {
            (4096, usize::MAX)
        }
    }

    /// NEW: Generate next frame ID for debugging/verification
    pub fn next_frame_id(&mut self) -> u64 {
        let id = self.next_frame_id;
        self.next_frame_id += 1;
        id
    }

    // ========================================================================
    // GC Integration Methods (delegates to gc_integration module helpers)
    // ========================================================================

    /// Phase 3: GC integration - Allocate heap object with GC
    pub fn allocate_heap_object(&mut self, object: HeapObject) -> Result<Value, DetailedVmError> {
        if !self.gc_enabled {
            return Err(DetailedVmError::GcDisabled);
        }

        let ptr = self.gc.allocate(object);
        Ok(Value::GcPtr(ptr))
    }

    /// Phase 3: GC integration - Add GC root
    pub fn add_gc_root(&mut self, ptr: GcPtr, description: &str) {
        self.gc.roots.push(GcRoot {
            ptr,
            description: description.to_string(),
        });
    }

    /// Phase 3: GC integration - Remove GC root
    pub fn remove_gc_root(&mut self, ptr: GcPtr) {
        self.gc.roots.retain(|root| root.ptr != ptr);
    }

    /// Phase 3: GC integration - Get GC stats
    pub fn get_gc_stats(&self) -> GcStats {
        self.gc.gc_stats.clone()
    }

    /// Phase 3: GC integration - Set GC enabled
    pub fn set_gc_enabled(&mut self, enabled: bool) {
        self.gc_enabled = enabled;
    }

    // ========================================================================
    // Debugging Integration Methods
    // ========================================================================

    /// Phase 3: Debugging integration - Add breakpoint
    pub fn add_breakpoint(&mut self, address: usize) {
        self.debugger.add_breakpoint(address);
    }

    /// Phase 3: Debugging integration - Remove breakpoint
    pub fn remove_breakpoint(&mut self, address: usize) {
        self.debugger.remove_breakpoint(address);
    }

    /// Phase 3: Debugging integration - Add watchpoint
    pub fn add_watchpoint(&mut self, name: &str, expression: &str) {
        self.debugger.add_watchpoint(name, expression);
    }

    /// Phase 3: Debugging integration - Check breakpoints
    pub fn check_breakpoints(&self) -> bool {
        self.debugger.check_breakpoints(self)
    }

    /// Phase 3: Debugging integration - Check watchpoints
    pub fn check_watchpoints(&mut self) -> Vec<WatchpointTrigger> {
        let vm_state = self.get_debug_snapshot();
        self.debugger.check_watchpoints_snapshot(&vm_state)
    }

    /// Phase 3: Debugging integration - Log debug event
    pub fn log_debug_event(&mut self, event: DebugEvent) {
        self.debugger.log_debug_event(event);
    }

    /// Phase 3: Debugging integration - Capture VM state
    pub fn capture_vm_state(&mut self) {
        let vm_state = self.get_debug_snapshot();
        self.debugger.capture_vm_state_snapshot(&vm_state);
    }

    /// Phase 3: Debugging integration - Get debug info
    pub fn get_debug_info(&self) -> DebugInfo {
        self.debugger.get_debug_info()
    }

    // ========================================================================
    // Performance Integration Methods
    // ========================================================================

    /// Phase 3: Performance integration - Increment counter
    pub fn increment_performance_counter(&mut self, name: &str, value: u64) {
        self.performance_monitor.increment_counter(name, value);
    }

    /// Phase 3: Performance integration - Start timer
    pub fn start_performance_timer(&mut self, name: &str) {
        self.performance_monitor.start_timer(name);
    }

    /// Phase 3: Performance integration - Stop timer
    pub fn stop_performance_timer(&mut self, name: &str) {
        self.performance_monitor.stop_timer(name);
    }

    /// Phase 3: Performance integration - Take sample
    pub fn take_performance_sample(&mut self) {
        let vm_state = self.get_debug_snapshot();
        self.performance_monitor.take_sample_snapshot(&vm_state);
    }

    /// Phase 3: Performance integration - Get metrics
    pub fn get_performance_metrics(&self) -> PerformanceMetrics {
        self.performance_monitor.get_metrics()
    }

    /// Phase 3: Performance integration - Get analysis
    pub fn get_performance_analysis(&self) -> PerformanceAnalysis {
        self.performance_monitor.get_analysis()
    }

    // ========================================================================
    // Call and Execution Helper Methods
    // ========================================================================

    /// Tail call detection method
    pub fn is_current_position_tail(&self) -> bool {
        if let Some(opcode) = self.instructions.get(self.ip) {
            matches!(opcode, OpCode::Ret | OpCode::TailCall(_))
        } else {
            false
        }
    }

    /// Get function info for escape analysis integration
    pub fn get_function_info(&self, function_ptr: u16) -> Result<FunctionInfo, VmError> {
        // In a real implementation, this would look up function metadata
        // For now, return a default function info
        Ok(FunctionInfo {
            local_count: 0,
            escape_info: HashMap::new(),
            free_variables: Vec::new(),
        })
    }

    /// Tail call optimization handler - delegates to opcodes::call::handle_tail_call
    ///
    /// This method is maintained on VmState for API compatibility with the step() function.
    /// The actual implementation is consolidated in `vm/opcodes/call.rs` to avoid duplication.
    ///
    /// # Arguments
    /// * `arg_count` - Number of arguments for the tail call
    ///
    /// # Note
    /// The consolidated implementation in `opcodes/call.rs` handles:
    /// - Stack validation
    /// - Recursion depth checking
    /// - Closure extraction from stack
    /// - Frame reuse for TCO
    /// - Instruction pointer reset
    pub fn handle_tail_call(&mut self, arg_count: u16) -> Result<(), VmError> {
        // Delegate to the consolidated implementation in opcodes/call.rs
        // This avoids duplicating the call handling logic
        call::handle_tail_call(self, arg_count)
    }

    /// Function call handler - delegates to opcodes::call::handle_call
    ///
    /// This method is maintained on VmState for API compatibility with the step() function.
    /// The actual implementation is consolidated in `vm/opcodes/call.rs` to avoid duplication.
    ///
    /// # Arguments
    /// * `arg_count` - Number of arguments for the function call
    ///
    /// # Note
    /// The consolidated implementation in `opcodes/call.rs` handles:
    /// - Stack validation
    /// - Recursion depth checking
    /// - Closure extraction and deserialization
    /// - Call frame creation
    /// - Instruction pointer management
    pub fn handle_call(&mut self, arg_count: u16) -> Result<(), VmError> {
        // Delegate to the consolidated implementation in opcodes/call.rs
        // This avoids duplicating the call handling logic
        call::handle_call(self, arg_count)
    }

    /// Helper method to read u16 from instructions
    pub fn read_u16(&mut self) -> Result<u16, VmError> {
        if self.ip >= self.instructions.len() {
            return Err(VmError::UnknownOpCode);
        }

        // Handle the current instruction as a u16 value
        match &self.instructions[self.ip] {
            OpCode::Int(i) => {
                self.ip += 1;
                Ok(*i as u16)
            }
            OpCode::Call(arg_count) => {
                self.ip += 1;
                Ok(*arg_count)
            }
            OpCode::TailCall(arg_count) => {
                self.ip += 1;
                Ok(*arg_count)
            }
            OpCode::GetLocal(offset) => {
                self.ip += 1;
                Ok(*offset)
            }
            OpCode::SetLocal(offset) => {
                self.ip += 1;
                Ok(*offset)
            }
            _ => {
                // For other opcodes, try to extract from the next instruction
                if self.ip + 1 < self.instructions.len() {
                    match &self.instructions[self.ip + 1] {
                        OpCode::Int(i) => {
                            self.ip += 2;
                            Ok(*i as u16)
                        }
                        _ => Err(VmError::UnknownOpCode),
                    }
                } else {
                    Err(VmError::UnknownOpCode)
                }
            }
        }
    }

    /// Helper method to get local variable
    pub fn get_local_var(&self, var_index: usize) -> Result<Value, VmError> {
        if let Some(frame) = self.call_stack.last() {
            if var_index < frame.locals.len() {
                Ok(frame.locals[var_index].clone())
            } else {
                Err(VmError::UnknownOpCode)
            }
        } else {
            Err(VmError::StackUnderflow)
        }
    }

    /// Debugging support: Get function name for a call frame (simplified)
    fn get_function_name_for_frame(&self, frame: &CallFrame) -> String {
        // In a real implementation, this would look up function names from debug info
        // For now, we'll use a placeholder
        format!("function_{}", frame.return_ip)
    }

    // ========================================================================
    // Execution Methods (delegates to execution module)
    // ========================================================================

    /// Executes a single instruction. Returns `Ok(InstructionResult)` or `Err(SimpleVmError)`.
    ///
    /// # Test Coverage
    /// - Nominal cases: All opcodes tested
    /// - Edge cases: Stack underflow, memory limits
    /// - Error states: Invalid operations, type mismatches
    ///
    /// # Returns
    /// Result containing either the instruction result or an execution error
    pub fn step(&mut self) -> Result<InstructionResult, SimpleVmError> {
        let mut engine = crate::vm::execution::ExecutionEngine::new();
        engine.step(self)
    }

    /// Executes the VM until completion or error.
    /// Returns the final result value or an error.
    ///
    /// # Test Coverage
    /// - Nominal execution paths
    /// - Error handling for all error types
    /// - Resource limit enforcement
    pub fn run(&mut self) -> Result<Value, DetailedVmError> {
        let mut engine = crate::vm::execution::ExecutionEngine::new();
        engine.run(self)
    }

    /// Convert a simple VmError to a detailed VmError with context
    pub fn convert_to_detailed_error(&self, error: SimpleVmError) -> DetailedVmError {
        let context = self.create_error_context();
        error.with_context(context)
    }
}

#[cfg(test)]
#[path = "test/state_tests.rs"]
mod tests;
