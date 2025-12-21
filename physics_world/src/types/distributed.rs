/// Distributed types for Physics World
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::capability::Capability;

/// V3 Distributed Scheduling - Node and network communication types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedNode {
    pub node_id: u32,
    pub address: String,
    pub capabilities: std::collections::HashSet<Capability>,
    pub load_factor: f32,
    pub last_heartbeat: u64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorMigrationRequest {
    pub actor_id: u32,
    pub source_node: u32,
    pub target_node: u32,
    pub migration_priority: u8,
    pub state_snapshot: SerializedActorState,
    pub migration_status: MigrationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MigrationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Rollback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializedActorState {
    pub actor_id: u32,
    pub vm_state: Vec<u8>, // Serialized VmState
    pub capabilities: Vec<Capability>,
    pub mailbox: Vec<crate::types::Value>,
    pub priority: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedConsensusRequest {
    pub request_id: u64,
    pub capability: Capability,
    pub requesting_actor: u32,
    pub requesting_node: u32,
    pub justification: String,
    pub votes_required: u32,
    pub votes_received: HashMap<u32, bool>, // node_id -> vote
    pub status: ConsensusStatus,
    pub created_at: u64,
    pub expires_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConsensusStatus {
    Open,
    Approved,
    Denied,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHeartbeat {
    pub node_id: u32,
    pub timestamp: u64,
    pub load_factor: f32,
    pub active_actors: u32,
    pub available_capabilities: Vec<Capability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DistributedMessage {
    Heartbeat(NodeHeartbeat),
    MigrationRequest(ActorMigrationRequest),
    MigrationResponse(MigrationResponse),
    ConsensusVote(ConsensusVote),
    RemoteExecutionRequest(RemoteExecutionRequest),
    RemoteExecutionResponse(RemoteExecutionResponse),
    ErrorResponse(DistributedError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationResponse {
    pub request_id: u64,
    pub success: bool,
    pub error_message: Option<String>,
    pub actor_status: Option<ActorStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorStatus {
    pub actor_id: u32,
    pub node_id: u32,
    pub execution_state: ExecutionState,
    pub last_updated: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionState {
    Running,
    Paused,
    Completed,
    Errored,
    Migrating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusVote {
    pub request_id: u64,
    pub voting_node: u32,
    pub vote: bool,
    pub timestamp: u64,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteExecutionRequest {
    pub request_id: u64,
    pub actor_id: u32,
    pub source_node: u32,
    pub target_node: u32,
    pub bytecode: Vec<crate::types::OpCode>,
    pub constants: Vec<crate::types::Value>,
    pub capabilities: Vec<Capability>,
    pub priority: u8,
    pub timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteExecutionResponse {
    pub request_id: u64,
    pub success: bool,
    pub result: Option<crate::types::Value>,
    pub error: Option<String>,
    pub execution_time: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributedError {
    pub error_type: String,
    pub message: String,
    pub node_id: u32,
    pub timestamp: u64,
    pub context: Option<String>,
}
