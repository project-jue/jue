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
