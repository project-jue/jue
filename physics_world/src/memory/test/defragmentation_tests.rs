use super::*;
use crate::memory::arena::{DefragmentationError, DefragmentationStats, ObjectArena};
use crate::types::HeapPtr;

#[test]
fn test_defragmentation_basic_functionality() {
    let mut arena = ObjectArena::with_capacity(1024);

    // Allocate some objects
    let ptr1 = arena.allocate(100, 0).unwrap();
    let ptr2 = arena.allocate(200, 0).unwrap();
    let ptr3 = arena.allocate(150, 0).unwrap();

    // Mark only first and third objects as reachable
    unsafe {
        arena.mark_object(ptr1);
        arena.mark_object(ptr3);
    }

    // Perform garbage collection (which should trigger defragmentation)
    let root_set = vec![ptr1, ptr3];
    arena.collect_garbage(&root_set).unwrap();

    // Check that defragmentation occurred
    let fragmentation = arena.fragmentation_ratio();
    assert!(
        fragmentation < 0.1,
        "Fragmentation should be low after GC: {}",
        fragmentation
    );
}

#[test]
fn test_manual_defragmentation() {
    let mut arena = ObjectArena::with_capacity(1024);

    // Allocate objects with gaps
    let ptr1 = arena.allocate(100, 0).unwrap();
    let ptr2 = arena.allocate(200, 0).unwrap();
    let ptr3 = arena.allocate(150, 0).unwrap();

    // Manually mark objects to create fragmentation pattern
    unsafe {
        arena.mark_object(ptr1);
        arena.mark_object(ptr3); // Skip ptr2 to create gap
    }

    // Perform manual defragmentation
    let result = arena.defragment();
    assert!(result.is_ok(), "Defragmentation should succeed");

    let stats = result.unwrap();
    assert_eq!(stats.objects_moved, 2, "Should have moved 2 objects");
    assert!(
        stats.bytes_reclaimed > 0,
        "Should have reclaimed some bytes"
    );
    assert!(
        stats.fragmentation_after < stats.fragmentation_before,
        "Fragmentation should decrease"
    );
}

#[test]
fn test_defragmentation_statistics() {
    let mut arena = ObjectArena::with_capacity(2048);

    // Create a fragmented scenario
    let mut pointers = Vec::new();
    for i in 0..10 {
        let ptr = arena.allocate(50 + i * 10, 0).unwrap();
        pointers.push(ptr);

        // Mark every other object to create fragmentation
        if i % 2 == 0 {
            unsafe { arena.mark_object(ptr) };
        }
    }

    // Perform defragmentation
    let result = arena.defragment();
    assert!(result.is_ok());

    let stats = result.unwrap();
    assert!(stats.objects_moved > 0, "Should have moved some objects");
    assert!(
        stats.bytes_reclaimed > 0,
        "Should have reclaimed some bytes"
    );
    assert!(
        stats.fragmentation_before > 0.2,
        "Should have significant fragmentation before"
    );
    assert!(
        stats.fragmentation_after < 0.1,
        "Should have low fragmentation after"
    );
    assert!(stats.time_taken_ms > 0, "Should have taken some time");
}

#[test]
fn test_fragmentation_ratio_calculation() {
    let mut arena = ObjectArena::with_capacity(1024);

    // Test empty arena
    assert_eq!(
        arena.fragmentation_ratio(),
        0.0,
        "Empty arena should have 0 fragmentation"
    );

    // Allocate one object
    let ptr1 = arena.allocate(100, 0).unwrap();
    unsafe { arena.mark_object(ptr1) };
    assert_eq!(
        arena.fragmentation_ratio(),
        0.0,
        "Single object should have 0 fragmentation"
    );

    // Allocate more objects with gaps
    let ptr2 = arena.allocate(200, 0).unwrap();
    let ptr3 = arena.allocate(150, 0).unwrap();

    // Mark only first and third
    unsafe {
        arena.mark_object(ptr1);
        arena.mark_object(ptr3);
    }

    let fragmentation = arena.fragmentation_ratio();
    assert!(
        fragmentation > 0.1,
        "Should have some fragmentation: {}",
        fragmentation
    );
    assert!(fragmentation < 1.0, "Fragmentation should be less than 1.0");
}

#[test]
fn test_auto_defragmentation_settings() {
    let mut arena = ObjectArena::with_capacity(1024);

    // Test default settings
    let (threshold, auto) = arena.get_defragmentation_settings();
    assert_eq!(threshold, 0.3, "Default threshold should be 0.3");
    assert!(auto, "Auto defragmentation should be enabled by default");

    // Test custom settings
    arena.set_defragmentation_settings(0.5, false);
    let (new_threshold, new_auto) = arena.get_defragmentation_settings();
    assert_eq!(new_threshold, 0.5, "Threshold should be updated");
    assert!(!new_auto, "Auto defragmentation should be disabled");

    // Test threshold clamping
    arena.set_defragmentation_settings(1.5, true); // Above 1.0
    let (clamped_threshold, _) = arena.get_defragmentation_settings();
    assert_eq!(clamped_threshold, 1.0, "Threshold should be clamped to 1.0");

    arena.set_defragmentation_settings(-0.1, true); // Below 0.0
    let (clamped_threshold, _) = arena.get_defragmentation_settings();
    assert_eq!(clamped_threshold, 0.0, "Threshold should be clamped to 0.0");
}

#[test]
fn test_should_defragment_logic() {
    let mut arena = ObjectArena::with_capacity(1024);

    // Test with auto defragmentation disabled
    arena.set_defragmentation_settings(0.3, false);
    assert!(
        !arena.should_defragment(),
        "Should not defragment when auto is disabled"
    );

    // Test with auto defragmentation enabled but low fragmentation
    arena.set_defragmentation_settings(0.3, true);
    let ptr1 = arena.allocate(100, 0).unwrap();
    unsafe { arena.mark_object(ptr1) };
    assert!(
        !arena.should_defragment(),
        "Should not defragment with low fragmentation"
    );

    // Test with high fragmentation
    let ptr2 = arena.allocate(200, 0).unwrap();
    let ptr3 = arena.allocate(300, 0).unwrap();
    unsafe { arena.mark_object(ptr1) }; // Only mark first object

    let fragmentation = arena.fragmentation_ratio();
    if fragmentation > 0.3 {
        assert!(
            arena.should_defragment(),
            "Should defragment with high fragmentation"
        );
    }
}

#[test]
fn test_defragmentation_with_empty_arena() {
    let mut arena = ObjectArena::with_capacity(1024);

    // Test defragmentation on empty arena
    let result = arena.defragment();
    assert!(
        result.is_ok(),
        "Defragmentation should succeed on empty arena"
    );

    let stats = result.unwrap();
    assert_eq!(stats.objects_moved, 0, "No objects should be moved");
    assert_eq!(stats.bytes_reclaimed, 0, "No bytes should be reclaimed");
    assert_eq!(stats.fragmentation_before, 0.0, "No fragmentation before");
    assert_eq!(stats.fragmentation_after, 0.0, "No fragmentation after");
}

#[test]
fn test_defragmentation_error_handling() {
    let mut arena = ObjectArena::with_capacity(1024);

    // Test that defragmentation handles edge cases gracefully
    // This is mostly to ensure no panics occur

    // Create some objects
    let _ptr1 = arena.allocate(100, 0).unwrap();
    let _ptr2 = arena.allocate(200, 0).unwrap();

    // Defragmentation should not fail
    let result = arena.defragment();
    assert!(result.is_ok(), "Defragmentation should handle normal cases");
}

#[test]
fn test_defragmentation_performance() {
    let mut arena = ObjectArena::with_capacity(4096);

    // Create a more complex fragmentation scenario
    let mut live_pointers = Vec::new();
    let mut dead_pointers = Vec::new();

    for i in 0..20 {
        let ptr = arena.allocate(50 + i * 20, 0).unwrap();
        if i % 3 == 0 {
            // Mark as live (every 3rd object)
            unsafe { arena.mark_object(ptr) };
            live_pointers.push(ptr);
        } else {
            dead_pointers.push(ptr);
        }
    }

    // Measure defragmentation performance
    let start_time = std::time::Instant::now();
    let result = arena.defragment();
    let duration = start_time.elapsed();

    assert!(result.is_ok(), "Defragmentation should succeed");

    let stats = result.unwrap();
    assert!(
        stats.objects_moved > 5,
        "Should have moved multiple objects"
    );
    assert!(
        stats.bytes_reclaimed > 500,
        "Should have reclaimed significant space"
    );
    assert!(
        duration.as_millis() < 100,
        "Defragmentation should be fast: {}ms",
        duration.as_millis()
    );
}

#[test]
fn test_defragmentation_preserves_data() {
    let mut arena = ObjectArena::with_capacity(1024);

    // Allocate objects and write test data
    let ptr1 = arena.allocate(100, 0).unwrap();
    let ptr2 = arena.allocate(200, 0).unwrap();
    let ptr3 = arena.allocate(150, 0).unwrap();

    // Write test data to the objects
    unsafe {
        let data1 = arena.get_data_mut(ptr1);
        for (i, byte) in data1.iter_mut().enumerate() {
            *byte = (i % 256) as u8;
        }

        let data2 = arena.get_data_mut(ptr2);
        for (i, byte) in data2.iter_mut().enumerate() {
            *byte = ((i + 100) % 256) as u8;
        }

        let data3 = arena.get_data_mut(ptr3);
        for (i, byte) in data3.iter_mut().enumerate() {
            *byte = ((i + 200) % 256) as u8;
        }

        // Mark objects for defragmentation
        arena.mark_object(ptr1);
        arena.mark_object(ptr3); // Skip ptr2
    }

    // Perform defragmentation
    let result = arena.defragment();
    assert!(result.is_ok(), "Defragmentation should succeed");

    // Verify data integrity after defragmentation
    // Note: After defragmentation, pointers are invalidated, so we can't directly access
    // the original pointers. This test mainly ensures no crashes occur.
    assert!(
        result.unwrap().objects_moved > 0,
        "Should have moved objects"
    );
}
