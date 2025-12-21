use crate::types::{OpCode, Value};
use crate::vm::debug::{DebugEvent, DebugEventType, Debugger};
use crate::vm::gc::{Array, Closure, GarbageCollector, GcPtr, GcRoot, HeapObject};
use crate::vm::performance::{PerformanceMetrics, PerformanceMonitor};
use crate::vm::state::VmState;
use std::time::Duration;

#[test]
fn test_gc_basic_allocation() {
    let mut gc = GarbageCollector::new(100, 50);

    // Test basic allocation
    let closure = HeapObject::Closure(Closure {
        code_ptr: 0,
        environment: std::collections::HashMap::new(),
    });

    let ptr = gc.allocate(closure);
    assert_eq!(ptr.0, 0);
    assert_eq!(gc.heap.len(), 1);
    assert_eq!(gc.allocations_since_last_gc, 1);
}

#[test]
fn test_gc_mark_and_sweep() {
    let mut gc = GarbageCollector::new(100, 2);

    // Allocate some objects
    let obj1 = HeapObject::Array(Array {
        elements: vec![Value::Int(42)],
    });
    let obj2 = HeapObject::Array(Array {
        elements: vec![Value::Int(43)],
    });

    let ptr1 = gc.allocate(obj1);
    let ptr2 = gc.allocate(obj2);

    // Add ptr1 as root
    gc.roots.push(GcRoot {
        ptr: ptr1,
        description: "test root".to_string(),
    });

    // Trigger GC
    assert_eq!(gc.allocations_since_last_gc, 2);
    gc.collect();

    // Should have collected obj2
    assert_eq!(gc.heap.len(), 1);
    assert_eq!(gc.gc_stats.collections, 1);
    assert_eq!(gc.gc_stats.objects_collected, 1);
}

#[test]
fn test_debugger_breakpoints() {
    let mut debugger = Debugger::new();

    // Add breakpoints
    debugger.add_breakpoint(10);
    debugger.add_breakpoint(20);
    debugger.add_breakpoint(30);

    // Check breakpoints
    assert!(debugger.breakpoints.contains(&10));
    assert!(debugger.breakpoints.contains(&20));
    assert!(debugger.breakpoints.contains(&30));

    // Remove breakpoint
    debugger.remove_breakpoint(20);
    assert!(!debugger.breakpoints.contains(&20));
}

#[test]
fn test_debugger_watchpoints() {
    let mut debugger = Debugger::new();

    // Add watchpoints
    debugger.add_watchpoint("var1", "x + 1");
    debugger.add_watchpoint("var2", "y * 2");

    // Check watchpoints
    assert_eq!(debugger.watchpoints.len(), 2);
    assert!(debugger.watchpoints.contains_key("var1"));
    assert!(debugger.watchpoints.contains_key("var2"));
}

#[test]
fn test_performance_monitor() {
    let mut monitor = PerformanceMonitor::new(Duration::from_secs(1));

    // Test counters
    monitor.increment_counter("instructions", 10);
    monitor.increment_counter("instructions", 5);
    monitor.increment_counter("memory_ops", 3);

    assert_eq!(monitor.counters.get("instructions"), Some(&15));
    assert_eq!(monitor.counters.get("memory_ops"), Some(&3));

    // Test metrics
    let metrics = monitor.get_metrics();
    assert_eq!(metrics.counter_increments, 3);
}

#[test]
fn test_vm_state_integration() {
    let instructions = vec![OpCode::Int(42), OpCode::Int(43), OpCode::Add];
    let constants = vec![];
    let mut vm = VmState::new(instructions, constants, 100, 1024, 1, 100);

    // Test GC integration
    assert!(vm.gc_enabled);
    assert_eq!(vm.gc_threshold, 512);

    // Test debugger integration
    vm.add_breakpoint(1);
    assert!(vm.check_breakpoints());

    // Test performance integration
    vm.increment_performance_counter("test_counter", 1);
    let metrics = vm.get_performance_metrics();
    assert_eq!(metrics.counter_increments, 1);
}

#[test]
fn test_vm_state_gc_methods() {
    let instructions = vec![];
    let constants = vec![];
    let mut vm = VmState::new(instructions, constants, 100, 1024, 1, 100);

    // Test GC methods
    let closure = HeapObject::Closure(Closure {
        code_ptr: 0,
        environment: std::collections::HashMap::new(),
    });

    let result = vm.allocate_heap_object(closure);
    assert!(result.is_ok());

    let ptr = result.unwrap();
    if let Value::GcPtr(gc_ptr) = ptr {
        vm.add_gc_root(gc_ptr, "test root");
        assert_eq!(vm.gc.roots.len(), 1);

        vm.remove_gc_root(gc_ptr);
        assert_eq!(vm.gc.roots.len(), 0);
    }

    // Test GC stats
    let stats = vm.get_gc_stats();
    assert_eq!(stats.collections, 0);
}

#[test]
fn test_vm_state_debug_methods() {
    let instructions = vec![];
    let constants = vec![];
    let mut vm = VmState::new(instructions, constants, 100, 1024, 1, 100);

    // Test debug methods
    vm.add_breakpoint(10);
    vm.add_watchpoint("test_var", "x + 1");

    assert!(vm.check_breakpoints());

    let watchpoints = vm.check_watchpoints();
    assert_eq!(watchpoints.len(), 0); // No changes yet

    let debug_info = vm.get_debug_info();
    assert_eq!(debug_info.breakpoints.len(), 1);
    assert_eq!(debug_info.watchpoints.len(), 1);
}

#[test]
fn test_vm_state_performance_methods() {
    let instructions = vec![];
    let constants = vec![];
    let mut vm = VmState::new(instructions, constants, 100, 1024, 1, 100);

    // Test performance methods
    vm.increment_performance_counter("instructions", 10);
    vm.start_performance_timer("test_timer");
    vm.stop_performance_timer("test_timer");
    vm.take_performance_sample();

    let metrics = vm.get_performance_metrics();
    assert_eq!(metrics.counter_increments, 1);

    let analysis = vm.get_performance_analysis();
    assert!(analysis.instructions_per_second >= 0.0);
}
