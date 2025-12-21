/// Comprehensive tests for the capability enforcement system
use super::*;
use crate::scheduler::{Actor, CapDecision, PhysicsScheduler};
use crate::types::{Capability, OpCode, Value};
use std::collections::HashSet;

#[test]
fn test_capability_enforcement_basic() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor with some capabilities
    let mut actor = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    // Grant some capabilities
    scheduler
        .grant_capability(1, Capability::IoReadSensor)
        .unwrap();
    scheduler.grant_capability(1, Capability::SysClock).unwrap();

    // Test capability checking
    assert!(scheduler.actor_has_capability(1, &Capability::IoReadSensor));
    assert!(scheduler.actor_has_capability(1, &Capability::SysClock));
    assert!(!scheduler.actor_has_capability(1, &Capability::IoNetwork));
}

#[test]
fn test_capability_request_handling() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor
    let actor = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    scheduler.add_actor(actor);

    // Test capability request
    let decision = scheduler.handle_capability_request(
        1,
        Capability::IoNetwork,
        "Need network access for testing",
    );

    // Should be granted based on the scheduler's logic
    assert!(matches!(decision, CapDecision::Granted));

    // Verify the capability was granted
    assert!(scheduler.actor_has_capability(1, &Capability::IoNetwork));
}

#[test]
fn test_capability_revocation() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor with capabilities
    let mut actor = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    scheduler.add_actor(actor);

    // Grant a capability
    scheduler
        .grant_capability(1, Capability::IoReadSensor)
        .unwrap();
    assert!(scheduler.actor_has_capability(1, &Capability::IoReadSensor));

    // Revoke the capability
    scheduler
        .revoke_capability(1, &Capability::IoReadSensor)
        .unwrap();
    assert!(!scheduler.actor_has_capability(1, &Capability::IoReadSensor));
}

#[test]
fn test_host_call_capability_requirements() {
    // Test that host calls require appropriate capabilities
    // This would be tested through the VM integration
    // For now, we'll test the capability mapping

    use crate::vm::capability_enforcement::CapabilityEnforcer;

    // Test capability requirements for host functions
    assert_eq!(
        CapabilityEnforcer::get_required_capability_for_host_function(0),
        Some(Capability::IoReadSensor)
    );
    assert_eq!(
        CapabilityEnforcer::get_required_capability_for_host_function(1),
        Some(Capability::IoWriteActuator)
    );
    assert_eq!(
        CapabilityEnforcer::get_required_capability_for_host_function(2),
        Some(Capability::SysClock)
    );
    assert_eq!(
        CapabilityEnforcer::get_required_capability_for_host_function(99),
        None
    );
}

#[test]
fn test_meta_capability_requirements() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor without MetaGrant
    let actor = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    scheduler.add_actor(actor);

    // Test that MetaGrant is required for dangerous operations
    let decision = scheduler.handle_capability_request(
        1,
        Capability::MetaGrant,
        "Need to grant capabilities to others",
    );

    // Should be denied or pending consensus based on scheduler logic
    assert!(matches!(
        decision,
        CapDecision::Denied | CapDecision::PendingConsensus
    ));
}

#[test]
fn test_capability_audit_logging() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor
    let actor = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    scheduler.add_actor(actor);

    // Perform capability operations
    scheduler.handle_capability_request(1, Capability::IoReadSensor, "Testing");
    scheduler.grant_capability(1, Capability::SysClock).unwrap();
    scheduler
        .revoke_capability(1, &Capability::SysClock)
        .unwrap();

    // Check that operations were logged
    let audit_log = scheduler.get_capability_audit_log();
    assert_eq!(audit_log.len(), 3); // Request, Grant, Revoke

    // Verify log contents
    assert!(matches!(
        audit_log[0].operation,
        crate::scheduler::CapOperation::Request
    ));
    assert!(matches!(
        audit_log[1].operation,
        crate::scheduler::CapOperation::Grant
    ));
    assert!(matches!(
        audit_log[2].operation,
        crate::scheduler::CapOperation::Revoke
    ));
}

#[test]
fn test_capability_inheritance() {
    let mut scheduler = PhysicsScheduler::new();

    // Create a parent actor with MetaGrant
    let mut parent = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    // Create a child actor
    let child = Actor {
        id: 2,
        vm: VmState::new(vec![], vec![], 100, 1024, 2),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: Some(1),
        priority: 128,
        priority_boost: None,
    };

    scheduler.add_actor(parent);
    scheduler.add_actor(child);

    // Test that child actors can get MetaSelfModify
    let decision = scheduler.handle_capability_request(
        2,
        Capability::MetaSelfModify,
        "Child actor needs self-modification",
    );

    // Should be granted because it has a parent
    assert!(matches!(decision, CapDecision::Granted));
}
