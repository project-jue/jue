use super::*;
use crate::types::OpCode;

#[test]
fn test_new_scheduler() {
    let scheduler = PhysicsScheduler::new();
    assert_eq!(scheduler.actors.len(), 0);
    assert_eq!(scheduler.current_actor_index, 0);
    assert_eq!(scheduler.message_queues.len(), 0);
    assert!(!scheduler.use_priority_scheduling); // Default should be false
    assert_eq!(scheduler.starvation_threshold, 1000);
}

#[test]
fn test_round_robin_scheduling() {
    let mut scheduler = PhysicsScheduler::new();

    // Add two actors
    let actor1 = Actor {
        id: 1,
        vm: VmState::new(vec![OpCode::Yield], vec![], 100, 1024, 1),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    let actor2 = Actor {
        id: 2,
        vm: VmState::new(vec![OpCode::Yield], vec![], 100, 1024, 1),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    scheduler.add_actor(actor1);
    scheduler.add_actor(actor2);

    // First tick should execute actor 1
    let result1 = scheduler.tick();
    assert!(matches!(result1, Ok(TickResult::ActorYielded(1))));

    // Second tick should execute actor 2
    let result2 = scheduler.tick();
    assert!(matches!(result2, Ok(TickResult::ActorYielded(2))));

    // Third tick should execute actor 1 again, but it has no more instructions so it finishes
    let result3 = scheduler.tick();
    assert!(matches!(result3, Ok(TickResult::ActorFinished(1, _))));
}

// Capability System Tests
#[test]
fn test_capability_management() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor with MetaGrant capability
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

    // Grant MetaGrant to the actor first
    scheduler
        .grant_capability(1, 1, crate::types::Capability::MetaGrant)
        .unwrap();

    // Test initial state - no capabilities
    assert!(!scheduler.actor_has_capability(1, &crate::types::Capability::IoReadSensor));

    // Grant a capability using the new signature
    scheduler
        .grant_capability(1, 1, crate::types::Capability::IoReadSensor)
        .unwrap();
    assert!(scheduler.actor_has_capability(1, &crate::types::Capability::IoReadSensor));

    // Revoke the capability using the new signature
    scheduler
        .revoke_capability(1, 1, &crate::types::Capability::IoReadSensor)
        .unwrap();
    assert!(!scheduler.actor_has_capability(1, &crate::types::Capability::IoReadSensor));
}

#[test]
fn test_capability_request_processing() {
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
        crate::types::Capability::IoNetwork,
        "Need network access for testing",
    );

    // Should be granted based on the scheduler's logic
    assert!(matches!(decision, crate::scheduler::CapDecision::Granted));

    // Verify the capability was granted
    assert!(scheduler.actor_has_capability(1, &crate::types::Capability::IoNetwork));
}

#[test]
fn test_capability_audit_logging() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor with MetaGrant capability
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

    // Grant MetaGrant to the actor first
    scheduler
        .grant_capability(1, 1, crate::types::Capability::MetaGrant)
        .unwrap();

    // Clear any existing logs
    scheduler.clear_capability_audit_log();

    // Perform capability operations
    scheduler.handle_capability_request(1, crate::types::Capability::IoReadSensor, "Testing");
    scheduler
        .grant_capability(1, 1, crate::types::Capability::SysClock)
        .unwrap();
    scheduler
        .revoke_capability(1, 1, &crate::types::Capability::SysClock)
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
        crate::scheduler::CapOperation::Delegate
    ));
    assert!(matches!(
        audit_log[2].operation,
        crate::scheduler::CapOperation::Revoke
    ));
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
        crate::types::Capability::MetaGrant,
        "Need to grant capabilities to others",
    );

    // Should be denied or pending consensus based on scheduler logic
    assert!(matches!(
        decision,
        crate::scheduler::CapDecision::Denied | crate::scheduler::CapDecision::PendingConsensus
    ));
}

#[test]
fn test_capability_inheritance() {
    let mut scheduler = PhysicsScheduler::new();

    // Create a parent actor
    let parent = Actor {
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
        crate::types::Capability::MetaSelfModify,
        "Child actor needs self-modification",
    );

    // Should be granted because it has a parent
    assert!(matches!(decision, crate::scheduler::CapDecision::Granted));
}

// Advanced Capability Tests
#[test]
fn test_capability_delegation_with_metagrant() {
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

    // Grant MetaGrant to parent first
    scheduler
        .grant_capability(1, 1, crate::types::Capability::MetaGrant)
        .unwrap();

    // Parent can now delegate capabilities to child
    let result = scheduler.grant_capability(
        1, // granter
        2, // target
        crate::types::Capability::IoReadSensor,
    );

    assert!(result.is_ok());
    assert!(scheduler.actor_has_capability(2, &crate::types::Capability::IoReadSensor));
}

#[test]
fn test_capability_delegation_without_metagrant_fails() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor without MetaGrant
    let actor1 = Actor {
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

    // Create another actor
    let actor2 = Actor {
        id: 2,
        vm: VmState::new(vec![], vec![], 100, 1024, 2),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    scheduler.add_actor(actor1);
    scheduler.add_actor(actor2);

    // Try to delegate without MetaGrant - should fail
    let result = scheduler.grant_capability(
        1, // granter without MetaGrant
        2, // target
        crate::types::Capability::IoReadSensor,
    );

    assert!(matches!(
        result,
        Err(crate::scheduler::PhysicsError::CapabilityError(_))
    ));
}

#[test]
fn test_capability_revocation_validation() {
    let mut scheduler = PhysicsScheduler::new();

    // Create an actor with MetaGrant
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

    // Grant MetaGrant to the actor
    scheduler
        .grant_capability(1, 1, crate::types::Capability::MetaGrant)
        .unwrap();

    // Grant another capability
    scheduler
        .grant_capability(1, 1, crate::types::Capability::IoReadSensor)
        .unwrap();

    // Actor can revoke their own capabilities
    let result = scheduler.revoke_capability(1, 1, &crate::types::Capability::IoReadSensor);
    assert!(result.is_ok());
    assert!(!scheduler.actor_has_capability(1, &crate::types::Capability::IoReadSensor));

    // Try to revoke MetaGrant from self - should work
    let result = scheduler.revoke_capability(1, 1, &crate::types::Capability::MetaGrant);
    assert!(result.is_ok());
}

#[test]
fn test_consensus_mechanism_for_dangerous_capabilities() {
    let mut scheduler = PhysicsScheduler::new();

    // Create a requester actor
    let requester = Actor {
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

    // Create voter actors with MetaGrant
    let voter1 = Actor {
        id: 2,
        vm: VmState::new(vec![], vec![], 100, 1024, 2),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::from([crate::types::Capability::MetaGrant]),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    let voter2 = Actor {
        id: 3,
        vm: VmState::new(vec![], vec![], 100, 1024, 3),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::from([crate::types::Capability::MetaGrant]),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    let voter3 = Actor {
        id: 4,
        vm: VmState::new(vec![], vec![], 100, 1024, 4),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::from([crate::types::Capability::MetaGrant]),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    scheduler.add_actor(requester);
    scheduler.add_actor(voter1);
    scheduler.add_actor(voter2);
    scheduler.add_actor(voter3);

    // Request MetaGrant capability (requires consensus)
    let decision = scheduler.handle_capability_request(
        1,
        crate::types::Capability::MetaGrant,
        "Need MetaGrant for testing",
    );

    assert!(matches!(
        decision,
        crate::scheduler::CapDecision::PendingConsensus
    ));

    // Cast votes - need 75% supermajority (3 out of 4 voters)
    scheduler
        .handle_consensus_vote(2, 1, &crate::types::Capability::MetaGrant, true)
        .unwrap(); // approve
    scheduler
        .handle_consensus_vote(3, 1, &crate::types::Capability::MetaGrant, true)
        .unwrap(); // approve
    scheduler
        .handle_consensus_vote(4, 1, &crate::types::Capability::MetaGrant, false)
        .unwrap(); // deny

    // Check consensus state - should still be pending (2/3 approve, need 3/4)
    let consensus_state = scheduler
        .get_consensus_state(1, &crate::types::Capability::MetaGrant)
        .unwrap();
    assert_eq!(consensus_state.approve, 2);
    assert_eq!(consensus_state.deny, 1);
    assert_eq!(consensus_state.total, 3); // 3 voters with MetaGrant

    // Add one more approval to reach consensus
    scheduler
        .handle_consensus_vote(3, 1, &crate::types::Capability::MetaGrant, true)
        .unwrap(); // change to approve

    // Now consensus should be approved
    let final_decision = scheduler
        .handle_consensus_vote(2, 1, &crate::types::Capability::MetaGrant, true)
        .unwrap();
    assert!(matches!(
        final_decision,
        crate::scheduler::CapDecision::Granted
    ));

    // Verify the capability was granted
    assert!(scheduler.actor_has_capability(1, &crate::types::Capability::MetaGrant));
}

#[test]
fn test_consensus_denial() {
    let mut scheduler = PhysicsScheduler::new();

    // Create a requester actor
    let requester = Actor {
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

    // Create voter actors with MetaGrant
    let voter1 = Actor {
        id: 2,
        vm: VmState::new(vec![], vec![], 100, 1024, 2),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::from([crate::types::Capability::MetaGrant]),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    let voter2 = Actor {
        id: 3,
        vm: VmState::new(vec![], vec![], 100, 1024, 3),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::from([crate::types::Capability::MetaGrant]),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128,
        priority_boost: None,
    };

    scheduler.add_actor(requester);
    scheduler.add_actor(voter1);
    scheduler.add_actor(voter2);

    // Request MetaGrant capability (requires consensus)
    let decision = scheduler.handle_capability_request(
        1,
        crate::types::Capability::MetaGrant,
        "Need MetaGrant for testing",
    );

    assert!(matches!(
        decision,
        crate::scheduler::CapDecision::PendingConsensus
    ));

    // Cast votes - majority denial
    scheduler
        .handle_consensus_vote(2, 1, &crate::types::Capability::MetaGrant, false)
        .unwrap(); // deny
    scheduler
        .handle_consensus_vote(3, 1, &crate::types::Capability::MetaGrant, false)
        .unwrap(); // deny

    // Check consensus state - should be denied
    let consensus_state = scheduler
        .get_consensus_state(1, &crate::types::Capability::MetaGrant)
        .unwrap();
    assert_eq!(consensus_state.deny, 2);
    assert_eq!(consensus_state.approve, 0);

    // Final decision should be denied
    let final_decision = scheduler
        .handle_consensus_vote(2, 1, &crate::types::Capability::MetaGrant, false)
        .unwrap();
    assert!(matches!(
        final_decision,
        crate::scheduler::CapDecision::Denied
    ));
}

// Priority Scheduling Tests
#[test]
fn test_priority_scheduling_enabled() {
    let mut scheduler = PhysicsScheduler::new();
    scheduler.enable_priority_scheduling();
    assert!(scheduler.use_priority_scheduling);
}

#[test]
fn test_priority_scheduling_disabled() {
    let mut scheduler = PhysicsScheduler::new();
    scheduler.disable_priority_scheduling();
    assert!(!scheduler.use_priority_scheduling);
}

#[test]
fn test_set_actor_priority() {
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

    // Set priority
    scheduler.set_priority(1, 200).unwrap();
    assert_eq!(scheduler.get_priority(1).unwrap(), 200);
}

#[test]
fn test_priority_boost() {
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

    // Set priority boost
    scheduler.set_priority_boost(1, 50).unwrap();

    // The effective priority should be calculated in the scheduler
    // We can't directly test it here without accessing private methods
    assert!(scheduler.actors[0].priority_boost.is_some());
}
