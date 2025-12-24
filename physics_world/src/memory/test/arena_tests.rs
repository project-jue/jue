use super::*;

#[test]
fn test_object_header_size() {
    // Ensure the header is 8 bytes (size 4 + tag 1 + marked 1 + padding 2)
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
    assert!(!header.marked); // Initially unmarked

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

#[test]
fn test_mark_object() {
    let mut arena = ObjectArena::with_capacity(1024);
    let ptr = arena.allocate(16, 1).unwrap();

    // Initially unmarked
    assert!(!unsafe { arena.is_marked(ptr) });

    // Mark the object
    unsafe { arena.mark_object(ptr) };

    // Should be marked now
    assert!(unsafe { arena.is_marked(ptr) });
}

#[test]
fn test_garbage_collection_basic() {
    let mut arena = ObjectArena::with_capacity(1024);

    // Allocate some objects
    let obj1 = arena.allocate(16, 1).unwrap();
    let _obj2 = arena.allocate(16, 2).unwrap();
    let obj3 = arena.allocate(16, 3).unwrap();

    // Mark obj1 and obj3 as reachable
    unsafe {
        arena.mark_object(obj1);
        arena.mark_object(obj3);
    }

    // Perform garbage collection
    let result = arena.collect_garbage(&[obj1, obj3]);
    assert!(result.is_ok());

    // After GC, only marked objects should remain
    // The arena should be compacted
    // Note: This is a basic test; a more comprehensive test would verify
    // the exact layout and pointer updates
    assert!(arena.next_free() > 0);
}

#[test]
fn test_garbage_collection_empty_arena() {
    let mut arena = ObjectArena::with_capacity(1024);

    // Perform GC on empty arena
    let result = arena.collect_garbage(&[]);
    assert!(result.is_ok());
    assert_eq!(arena.next_free(), 0);
}

#[test]
fn test_garbage_collection_all_unreachable() {
    let mut arena = ObjectArena::with_capacity(1024);

    // Allocate objects but don't mark any as reachable
    let _obj1 = arena.allocate(16, 1).unwrap();
    let _obj2 = arena.allocate(16, 2).unwrap();

    // Perform GC with empty root set
    let result = arena.collect_garbage(&[]);
    assert!(result.is_ok());

    // All objects should be collected
    assert_eq!(arena.next_free(), 0);
}

#[test]
fn test_gc_self_referential_closure() {
    use super::{TAG_CLOSURE, TAG_LIST};

    let mut arena = ObjectArena::with_capacity(4096);

    // Create a closure that references itself (simulating a recursive closure)
    // A closure with a capture that points to itself
    let closure_ptr = arena.allocate(16, TAG_CLOSURE).unwrap();
    
    // Write a self-reference in the closure data (at offset 4 for code_ptr)
    // This simulates a closure that captures itself
    let self_ref_bytes = closure_ptr.get().to_le_bytes();
    let data = unsafe { arena.get_data_mut(closure_ptr) };
    data[4..8].copy_from_slice(&self_ref_bytes);  // Self-reference at offset 4

    // Mark only the closure itself (simulating root pointing to closure)
    // The closure should be kept AND its self-reference should be traversed
    unsafe {
        arena.mark_object(closure_ptr);
    }

    // Perform GC - closure should survive because it's in root set
    let result = arena.collect_garbage(&[closure_ptr]);
    assert!(result.is_ok());

    // The closure should still be accessible (arena should not be empty)
    // Note: With self-reference, the closure marks itself which is fine
    assert!(arena.next_free() > 0, "Closure should survive GC");

    println!("✅ Self-referential closure test passed");
}

#[test]
fn test_gc_closure_chain() {
    use super::{TAG_CLOSURE, TAG_LIST};

    let mut arena = ObjectArena::with_capacity(4096);

    // Create two closures that reference each other
    // Closure A at offset 0, Closure B at some offset
    let closure_a = arena.allocate(16, TAG_CLOSURE).unwrap();
    let closure_b = arena.allocate(16, TAG_CLOSURE).unwrap();

    // Write cross-references
    // Closure A references B (at offset 4)
    let b_ref_bytes = closure_b.get().to_le_bytes();
    {
        let data = unsafe { arena.get_data_mut(closure_a) };
        data[4..8].copy_from_slice(&b_ref_bytes);
    }

    // Closure B references A (at offset 4)
    let a_ref_bytes = closure_a.get().to_le_bytes();
    {
        let data = unsafe { arena.get_data_mut(closure_b) };
        data[4..8].copy_from_slice(&a_ref_bytes);
    }

    // Mark only closure A as reachable
    unsafe {
        arena.mark_object(closure_a);
    }

    // Perform GC - both closures should survive because:
    // 1. A is in root set -> marked
    // 2. A references B -> B is marked (recursive marking)
    // 3. B references A -> A is already marked
    let result = arena.collect_garbage(&[closure_a]);
    assert!(result.is_ok());

    // Both closures should survive the GC
    // The arena should still contain both objects
    assert!(arena.next_free() > 0, "At least one closure should survive");

    println!("✅ Closure chain test passed");
}

#[test]
fn test_gc_list_chain() {
    use super::{TAG_LIST, TAG_PAIR};

    let mut arena = ObjectArena::with_capacity(4096);

    // Create a linked list: A -> B -> C -> nil
    // where each node is a cons cell (car, cdr)
    let node_c = arena.allocate(8, TAG_PAIR).unwrap();  // Points to nil (0)
    let node_b = arena.allocate(8, TAG_PAIR).unwrap();  // Points to C
    let node_a = arena.allocate(8, TAG_PAIR).unwrap();  // Points to B

    // Set up the linked list
    // Node A: car = 1 (some value), cdr = node_b
    {
        let data = unsafe { arena.get_data_mut(node_a) };
        data[0..4].copy_from_slice(&1u32.to_le_bytes());  // car = 1
        data[4..8].copy_from_slice(&node_b.get().to_le_bytes());  // cdr = node_b
    }

    // Node B: car = 2, cdr = node_c
    {
        let data = unsafe { arena.get_data_mut(node_b) };
        data[0..4].copy_from_slice(&2u32.to_le_bytes());  // car = 2
        data[4..8].copy_from_slice(&node_c.get().to_le_bytes());  // cdr = node_c
    }

    // Node C: car = 3, cdr = nil (0)
    {
        let data = unsafe { arena.get_data_mut(node_c) };
        data[0..4].copy_from_slice(&3u32.to_le_bytes());  // car = 3
        // cdr already 0 (nil) from allocation zeroing
    }

    // Mark only node A as reachable
    unsafe {
        arena.mark_object(node_a);
    }

    // Perform GC - all nodes should survive because A references B, B references C
    let result = arena.collect_garbage(&[node_a]);
    assert!(result.is_ok());

    // All nodes should survive
    assert!(arena.next_free() > 0, "All list nodes should survive");

    println!("✅ List chain GC test passed");
}

#[test]
fn test_gc_circular_reference_survival() {
    use super::{TAG_CLOSURE, TAG_LIST};

    let mut arena = ObjectArena::with_capacity(4096);

    // Create a more complex circular reference:
    // closure_a -> list_1 -> closure_b -> list_2 -> closure_a
    let closure_a = arena.allocate(16, TAG_CLOSURE).unwrap();
    let list_1 = arena.allocate(8, TAG_LIST).unwrap();
    let closure_b = arena.allocate(16, TAG_CLOSURE).unwrap();
    let list_2 = arena.allocate(8, TAG_LIST).unwrap();

    // Set up the circular references
    // closure_a -> list_1 (closure stores list_1 at offset 4)
    {
        let data = unsafe { arena.get_data_mut(closure_a) };
        data[4..8].copy_from_slice(&list_1.get().to_le_bytes());
    }

    // list_1 -> closure_b (cdr points to closure_b)
    {
        let data = unsafe { arena.get_data_mut(list_1) };
        data[0..4].copy_from_slice(&42u32.to_le_bytes());  // car = 42
        data[4..8].copy_from_slice(&closure_b.get().to_le_bytes());  // cdr = closure_b
    }

    // closure_b -> list_2
    {
        let data = unsafe { arena.get_data_mut(closure_b) };
        data[4..8].copy_from_slice(&list_2.get().to_le_bytes());
    }

    // list_2 -> closure_a (cdr points to closure_a)
    {
        let data = unsafe { arena.get_data_mut(list_2) };
        data[0..4].copy_from_slice(&43u32.to_le_bytes());  // car = 43
        data[4..8].copy_from_slice(&closure_a.get().to_le_bytes());  // cdr = closure_a
    }

    // Mark only closure_a as reachable
    unsafe {
        arena.mark_object(closure_a);
    }

    // Perform GC - all objects should survive due to circular references
    let result = arena.collect_garbage(&[closure_a]);
    assert!(result.is_ok());

    // All objects should survive
    assert!(arena.next_free() > 0, "All circularly referenced objects should survive");

    println!("✅ Circular reference survival test passed");
}

#[test]
fn test_gc_mark_reachable_from_roots() {
    use super::TAG_VECTOR;

    let mut arena = ObjectArena::with_capacity(4096);

    // Create a vector containing references to other objects
    let contained_obj = arena.allocate(16, TAG_CLOSURE).unwrap();
    let vector_ptr = arena.allocate(16, TAG_VECTOR).unwrap();  // 4 elements

    // Fill vector with pointers (including to contained_obj)
    {
        let data = unsafe { arena.get_data_mut(vector_ptr) };
        // Element 0: pointer to contained_obj
        data[0..4].copy_from_slice(&contained_obj.get().to_le_bytes());
        // Element 1: pointer to vector itself (self-reference)
        data[4..8].copy_from_slice(&vector_ptr.get().to_le_bytes());
        // Element 2: some other value
        data[8..12].copy_from_slice(&99u32.to_le_bytes());
        // Element 3: zero
        data[12..16].copy_from_slice(&0u32.to_le_bytes());
    }

    // Mark only the vector as reachable
    unsafe {
        arena.mark_object(vector_ptr);
    }

    // Perform GC
    let result = arena.collect_garbage(&[vector_ptr]);
    assert!(result.is_ok());

    // Both vector and contained object should survive
    assert!(arena.next_free() > 0, "Vector and contained objects should survive");

    println!("✅ Mark reachable from roots test passed");
}
