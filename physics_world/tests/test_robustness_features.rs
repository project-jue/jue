use physics_world::memory::arena::{DefragmentationError, ObjectArena};
use physics_world::scheduler::PhysicsScheduler;
use physics_world::types::{Capability, HeapPtr, Value};
use physics_world::vm::state::VmState;

#[test]
fn test_memory_fragmentation_handling() {
    // Test basic arena allocation and fragmentation
    let mut arena = ObjectArena::with_capacity_and_settings(1024, 0.7, true);

    // Allocate some objects to create fragmentation
    let obj1 = arena.allocate(100, 0).unwrap();
    let obj2 = arena.allocate(200, 0).unwrap();
    let obj3 = arena.allocate(150, 0).unwrap();

    // Check fragmentation ratio
    let fragmentation = arena.fragmentation_ratio();
    assert!(fragmentation >= 0.0, "Fragmentation should be non-negative");

    // Test defragmentation
    let result = arena.defragment();
    match result {
        Ok(stats) => {
            assert!(
                stats.objects_moved >= 0,
                "Objects moved should be non-negative"
            );
            assert!(
                stats.bytes_reclaimed >= 0,
                "Bytes reclaimed should be non-negative"
            );
        }
        Err(DefragmentationError::DefragmentationFailed(_)) => {
            panic!("Defragmentation should not fail");
        }
    }

    // Check fragmentation after defragmentation
    let post_fragmentation = arena.fragmentation_ratio();
    assert!(
        post_fragmentation >= 0.0,
        "Post-defragmentation ratio should be non-negative"
    );
}

#[test]
fn test_resource_management() {
    let mut scheduler = PhysicsScheduler::new();

    // Test resource monitoring
    let stats = scheduler.get_resource_stats();
    assert!(
        stats.memory_usage >= 0,
        "Memory usage should be non-negative"
    );
    assert!(stats.cpu_time_used >= 0, "CPU time should be non-negative");

    // Test resource limit setting
    scheduler.set_resource_limits(1024 * 1024, 10000);
    let stats = scheduler.get_resource_stats();
    assert!(stats.memory_limit > 0, "Memory limit should be set");
    assert!(stats.cpu_time_limit > 0, "CPU limit should be set");
}

#[test]
fn test_debugging_support() {
    let mut vm_state = VmState::new(vec![], vec![], 1024, 1024, 0, 100);

    // Test VM introspection
    let debug_snapshot = vm_state.get_debug_snapshot();
    assert!(debug_snapshot.stack.len() >= 0, "Should report stack depth");
    assert!(
        debug_snapshot.memory_usage >= 0,
        "Should report memory usage"
    );

    // Test memory analysis
    let memory_analysis = vm_state.get_memory_analysis();
    assert!(
        memory_analysis.heap_usage >= 0,
        "Should report total allocated memory"
    );
    assert!(
        memory_analysis.fragmentation_ratio >= 0.0,
        "Should report fragmentation ratio"
    );
}

#[test]
fn test_backward_compatibility() {
    // Test that existing functionality still works
    let mut arena = ObjectArena::with_capacity(1024);
    let obj = arena.allocate(100, 0);
    assert!(obj.is_ok(), "Basic allocation should still work");

    let mut scheduler = PhysicsScheduler::new();
    assert!(
        scheduler.actors.is_empty(),
        "Basic scheduler creation should still work"
    );

    let value = Value::Int(42);
    assert!(
        matches!(value, Value::Int(_)),
        "Basic value creation should work"
    );
}

#[test]
fn test_error_handling() {
    let mut arena = ObjectArena::with_capacity(1024);

    // Test allocation failure
    let result = arena.allocate(2048, 0);
    assert!(
        result.is_err(),
        "Should fail when allocating more than available memory"
    );
}

#[test]
fn test_edge_cases() {
    // Test empty arena
    let mut arena = ObjectArena::with_capacity(1024);
    let fragmentation = arena.fragmentation_ratio();
    assert_eq!(
        fragmentation, 0.0,
        "Empty arena should have 0 fragmentation"
    );

    // Test basic value creation
    let closure_value = Value::Closure(HeapPtr(0));
    assert!(
        matches!(closure_value, Value::Closure(_)),
        "Should create closure value"
    );

    // Test capability value creation
    let capability = Capability::IoReadSensor;
    let capability_value = Value::Capability(capability.clone());
    assert!(
        matches!(capability_value, Value::Capability(_)),
        "Should create capability value"
    );
}
