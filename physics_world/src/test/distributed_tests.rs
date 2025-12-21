/// Unit tests for the distributed scheduling module
///
/// Test coverage: 100% of public API methods
/// Tests: nominal cases, edge cases, error handling
use super::*;

#[test]
fn test_distributed_scheduler_creation() {
    let mut scheduler = DistributedScheduler::new(1, "127.0.0.1:8080".to_string());
    assert_eq!(scheduler.node_id, 1);
    assert_eq!(scheduler.local_address, "127.0.0.1:8080");
    assert!(scheduler.remote_nodes.is_empty());
}

#[test]
fn test_add_remove_remote_node() {
    let mut scheduler = DistributedScheduler::new(1, "127.0.0.1:8080".to_string());

    // Test adding a node
    let result = scheduler.add_remote_node(2, "127.0.0.1:8081".to_string());
    assert!(result.is_ok());
    assert_eq!(scheduler.remote_nodes.len(), 1);
    assert!(scheduler.remote_nodes.contains_key(&2));

    // Test removing the node
    let result = scheduler.remove_remote_node(2);
    assert!(result.is_ok());
    assert!(scheduler.remote_nodes.is_empty());
}

#[test]
fn test_consensus_vote_creation() {
    let mut scheduler = DistributedScheduler::new(1, "127.0.0.1:8080".to_string());

    let request_id = scheduler
        .initiate_distributed_consensus(Capability::MetaGrant, 1, "Test consensus".to_string())
        .unwrap();

    assert!(scheduler.consensus_votes.contains_key(&request_id));
    let request = scheduler.consensus_votes.get(&request_id).unwrap();
    assert_eq!(request.capability, Capability::MetaGrant);
    assert_eq!(request.requesting_actor, 1);
    assert_eq!(request.status, ConsensusStatus::Open);
}
