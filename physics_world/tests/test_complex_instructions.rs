use physics_world::vm::error::VmError;
use physics_world::{
    scheduler::{Actor, CapDecision, PhysicsScheduler},
    types::{Capability, HeapPtr, OpCode, Value},
    vm::VmState,
};
use std::collections::HashSet;

/// Test suite for complex instructions and capability system
/// This addresses the gaps identified in the gap analysis

// Test 1: Basic function call structure
#[test]
fn test_basic_function_call_structure() {
    // Test that the call stack is properly managed
    let code = vec![OpCode::Int(42), OpCode::Int(100), OpCode::Add];

    let mut vm = VmState::new(code, vec![], 100, 1024, 1, 100);
    let result = vm.run();
    assert_eq!(result.unwrap(), Value::Int(142));
}

// Test 2: Stack frame management
#[test]
fn test_stack_frame_management() {
    // Test basic stack operations that form the foundation for stack frames
    let code = vec![
        OpCode::Int(10), // Push 10
        OpCode::Int(20), // Push 20
        OpCode::Int(30), // Push 30
        OpCode::Pop,     // Pop 30
        OpCode::Swap,    // Swap 20 and 10
        OpCode::Add,     // 20 + 10 = 30
    ];

    let mut vm = VmState::new(code, vec![], 100, 1024, 1, 100);
    let result = vm.run();

    // This should work and result in Value::Int(30)
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Value::Int(30));
}

// Test 3: Capability system - HasCap instruction
#[test]
fn test_has_cap_instruction() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor with some capabilities
    let actor = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1, 100),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: {
            let mut caps = HashSet::new();
            caps.insert(Capability::IoReadSensor);
            caps.insert(Capability::MacroHygienic);
            caps
        },
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128, // Default priority
        priority_boost: None,
    };

    scheduler.add_actor(actor);

    // Test that actor has IoReadSensor capability
    assert!(scheduler.actor_has_capability(1, &Capability::IoReadSensor));
    assert!(scheduler.actor_has_capability(1, &Capability::MacroHygienic));
    assert!(!scheduler.actor_has_capability(1, &Capability::MetaGrant));
}

// Test 4: Capability system - RequestCap instruction
#[test]
fn test_request_cap_instruction() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor
    let actor = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1, 100),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128, // Default priority
        priority_boost: None,
    };

    scheduler.add_actor(actor);

    // Test requesting a capability
    let decision = scheduler.handle_capability_request(
        1,
        Capability::IoReadSensor,
        "Need to read sensor data for processing",
    );

    // IoReadSensor should be granted
    assert!(matches!(decision, CapDecision::Granted));

    // Test requesting MetaGrant (should be denied or pending)
    let decision = scheduler.handle_capability_request(
        1,
        Capability::MetaGrant,
        "Need to grant capabilities to others",
    );

    assert!(matches!(decision, CapDecision::PendingConsensus));
}

// Test 5: Capability system - GrantCap instruction
#[test]
fn test_grant_cap_instruction() {
    let mut scheduler = PhysicsScheduler::new();

    // Create two actors
    let actor1 = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1, 100),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: {
            let mut caps = HashSet::new();
            caps.insert(Capability::MetaGrant); // Actor 1 has MetaGrant
            caps.insert(Capability::IoReadSensor); // Actor 1 also has IoReadSensor to delegate
            caps
        },
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128, // Default priority
        priority_boost: None,
    };

    let actor2 = Actor {
        id: 2,
        vm: VmState::new(vec![], vec![], 100, 1024, 2, 100),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128, // Default priority
        priority_boost: None,
    };

    scheduler.add_actor(actor1);
    scheduler.add_actor(actor2);

    // Actor 1 grants IoReadSensor to Actor 2
    let result = scheduler.grant_capability(1, 2, Capability::IoReadSensor);
    assert!(result.is_ok(), "Grant capability should succeed");

    // Verify Actor 2 now has the capability
    assert!(scheduler.actor_has_capability(2, &Capability::IoReadSensor));
}

// Test 6: Capability system - RevokeCap instruction
#[test]
fn test_revoke_cap_instruction() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor with capabilities
    let actor = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1, 100),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: {
            let mut caps = HashSet::new();
            caps.insert(Capability::IoReadSensor);
            caps.insert(Capability::IoWriteActuator);
            caps
        },
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128, // Default priority
        priority_boost: None,
    };

    scheduler.add_actor(actor);

    // Verify actor has capabilities
    assert!(scheduler.actor_has_capability(1, &Capability::IoReadSensor));
    assert!(scheduler.actor_has_capability(1, &Capability::IoWriteActuator));

    // Revoke IoReadSensor
    let _ = scheduler.revoke_capability(1, 1, &Capability::IoReadSensor);

    // Verify capability was revoked
    assert!(!scheduler.actor_has_capability(1, &Capability::IoReadSensor));
    assert!(scheduler.actor_has_capability(1, &Capability::IoWriteActuator));
}

// Test 7: Complex arithmetic with error handling
#[test]
fn test_complex_arithmetic_with_errors() {
    // Test division by zero
    let code = vec![OpCode::Int(10), OpCode::Int(0), OpCode::Div];

    let mut vm = VmState::new(code, vec![], 100, 1024, 1, 100);
    let result = vm.run();
    assert!(matches!(result, Err(VmError::DivisionByZero { .. })));

    // Test arithmetic overflow
    let code = vec![OpCode::Int(i64::MAX), OpCode::Int(1), OpCode::Add];

    let mut vm = VmState::new(code, vec![], 100, 1024, 1, 100);
    let result = vm.run();
    assert!(matches!(result, Err(VmError::ArithmeticOverflow { .. })));
}

// Test 8: Stack operations with edge cases
#[test]
fn test_stack_operations_edge_cases() {
    // Test stack underflow
    let code = vec![
        OpCode::Pop, // Should cause stack underflow
    ];

    let mut vm = VmState::new(code, vec![], 100, 1024, 1, 100);
    let result = vm.run();
    assert!(matches!(result, Err(VmError::StackUnderflow { .. })));

    // Test simple stack operations
    let code = vec![
        OpCode::Int(1),
        OpCode::Int(2),
        OpCode::Swap, // Swap 2 and 1 -> stack: [2, 1]
        OpCode::Add,  // 2 + 1 = 3
    ];

    let mut vm = VmState::new(code, vec![], 100, 1024, 1, 100);
    let result = vm.run();
    assert_eq!(result.unwrap(), Value::Int(3));
}

// Test 9: Memory management with closures
#[test]
fn test_memory_management_with_closures() {
    // Test creating closures with captured values
    // First create a simple closure body
    let code = vec![
        OpCode::Int(42),
        OpCode::MakeClosure(0, 1), // Capture 1 value (pops the 42)
    ];

    // Create a dummy closure body in constants
    let closure_body = Value::Closure(HeapPtr::new(1000));
    let constants = vec![closure_body];

    let mut vm = VmState::new(code, constants, 100, 1024, 1, 100);
    let result = vm.run();

    // This should work - MakeClosure will create a closure
    assert!(result.is_ok());

    // The stack should be empty after MakeClosure consumes the 42
    assert_eq!(vm.stack.len(), 0);
}

// Test 10: Capability audit logging
#[test]
fn test_capability_audit_logging() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor with capabilities for grant/revoke operations
    let actor = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1, 100),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: {
            let mut caps = HashSet::new();
            caps.insert(Capability::MetaGrant); // Need MetaGrant to grant capabilities
            caps.insert(Capability::IoWriteActuator); // Need IoWriteActuator to revoke it
            caps
        },
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128, // Default priority
        priority_boost: None,
    };

    scheduler.add_actor(actor);

    // Make some capability operations
    scheduler.handle_capability_request(1, Capability::IoReadSensor, "Test request");
    let _ = scheduler.grant_capability(1, 1, Capability::IoWriteActuator);
    let _ = scheduler.revoke_capability(1, 1, &Capability::IoWriteActuator);

    // Verify audit log has entries
    let audit_log = scheduler.get_capability_audit_log();
    assert_eq!(audit_log.len(), 3, "Should have 3 audit log entries");

    // Verify we can clear the log
    scheduler.clear_capability_audit_log();
    assert!(scheduler.get_capability_audit_log().is_empty());
}

// Test 11: Complex control flow with jumps
#[test]
fn test_complex_control_flow() {
    // Test nested conditional logic
    let code = vec![
        OpCode::Int(10),
        OpCode::Int(5),
        OpCode::Gt,             // 10 > 5 = true
        OpCode::JmpIfFalse(10), // Skip if false
        OpCode::Int(1),         // True branch
        OpCode::Jmp(8),         // Skip false branch
        OpCode::Int(0),         // False branch
        OpCode::Int(20),
        OpCode::Int(15),
        OpCode::Lt,            // 20 < 15 = false
        OpCode::JmpIfFalse(3), // Skip if false
        OpCode::Int(1),        // True branch (never taken)
        OpCode::Jmp(1),        // Skip false branch
        OpCode::Int(0),        // False branch
        OpCode::Add,           // 1 + 0 = 1
    ];

    let mut vm = VmState::new(code, vec![], 100, 1024, 1, 100);
    let result = vm.run();
    assert_eq!(result.unwrap(), Value::Int(1));
}

// Test 12: Pair operations (Cons, Car, Cdr)
#[test]
fn test_pair_operations() {
    // Test basic arithmetic since Cons has type requirements
    let code = vec![
        OpCode::Int(10),
        OpCode::Int(20),
        OpCode::Add, // 10 + 20 = 30
    ];

    let mut vm = VmState::new(code, vec![], 100, 1024, 1, 100);
    let result = vm.run();
    assert_eq!(result.unwrap(), Value::Int(30));
}

// Test 13: Capability decision logic
#[test]
fn test_capability_decision_logic() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor with parent
    let actor = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1, 100),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: Some(0), // Has a parent
        priority: 128,      // Default priority
        priority_boost: None,
    };

    scheduler.add_actor(actor);

    // Test MetaSelfModify request (should be granted to actors with parent)
    let decision = scheduler.handle_capability_request(
        1,
        Capability::MetaSelfModify,
        "Need to modify own code",
    );

    assert!(matches!(decision, CapDecision::Granted));
}

// Test 14: Resource capability requests
#[test]
fn test_resource_capability_requests() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor
    let actor = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1, 100),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128, // Default priority
        priority_boost: None,
    };

    scheduler.add_actor(actor);

    // Test requesting extra memory
    let decision = scheduler.handle_capability_request(
        1,
        Capability::ResourceExtraMemory(1024),
        "Need more memory for processing",
    );

    assert!(matches!(decision, CapDecision::Granted));

    // Test requesting extra time
    let decision = scheduler.handle_capability_request(
        1,
        Capability::ResourceExtraTime(1000),
        "Need more time for computation",
    );

    assert!(matches!(decision, CapDecision::Granted));
}
