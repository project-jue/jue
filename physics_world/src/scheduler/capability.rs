/// Capability management for the Physics World scheduler
///
/// This module provides capability types, audit logging, and delegation logic
/// for the Physics World scheduler.

/// Capability audit log entry for tracking capability operations
#[derive(Debug, Clone)]
pub struct CapAuditEntry {
    pub timestamp: u64,
    pub actor_id: u32,
    pub operation: CapOperation,
    pub capability: crate::types::Capability,
    pub result: CapDecisionResult,
}

/// Type of capability operation
#[derive(Debug, Clone, PartialEq)]
pub enum CapOperation {
    Request,
    Grant,
    Revoke,
    Check,
    Delegate,
    ConsensusVote,
}

/// Result of a capability operation
#[derive(Debug, Clone)]
pub enum CapDecisionResult {
    Granted,
    Denied,
    Pending,
    Error(String),
    ConsensusRequired,
    ConsensusApproved,
    ConsensusDenied,
}

/// Decision result for capability requests
#[derive(Debug, Clone)]
pub enum CapDecision {
    Granted,
    Denied,
    PendingConsensus,
}

/// Consensus vote result
#[derive(Debug, Clone)]
pub struct ConsensusVoteResult {
    pub approve: u32,
    pub deny: u32,
    pub abstain: u32,
    pub total: u32,
}
