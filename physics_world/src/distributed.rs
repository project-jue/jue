/// Distributed scheduling and multi-node execution for Physics World V3
use crate::scheduler::PhysicsScheduler;
use crate::types::{
    ActorMigrationRequest, Capability, ConsensusStatus, DistributedConsensusRequest, DistributedError,
    DistributedNode, RemoteExecutionRequest, RemoteExecutionResponse,
};
use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// Main distributed scheduler that coordinates execution across multiple nodes
#[derive(Clone)]
pub struct DistributedScheduler {
    pub local_scheduler: PhysicsScheduler,
    pub node_id: u32,
    pub local_address: String,
    pub remote_nodes: HashMap<u32, DistributedNode>,
    pub migration_queue: Vec<ActorMigrationRequest>,
    pub consensus_votes: HashMap<u64, DistributedConsensusRequest>,
    pub pending_remote_executions: HashMap<u64, RemoteExecutionRequest>,
    pub message_sequence: u64,
    pub network_stats: NetworkStatistics,
    pub load_balancer: LoadBalancer,
    pub fault_detector: FaultDetector,
    pub is_running: bool,
}

impl DistributedScheduler {
    /// Create a new distributed scheduler with the given node ID and address
    pub fn new(node_id: u32, local_address: String) -> Self {
        Self {
            local_scheduler: PhysicsScheduler::new(),
            node_id,
            local_address,
            remote_nodes: HashMap::new(),
            migration_queue: Vec::new(),
            consensus_votes: HashMap::new(),
            pending_remote_executions: HashMap::new(),
            message_sequence: 0,
            network_stats: NetworkStatistics::new(),
            load_balancer: LoadBalancer::new(),
            fault_detector: FaultDetector::new(),
            is_running: false,
        }
    }

    /// Start the distributed scheduler and begin network operations
    pub fn start(&mut self) -> Result<(), DistributedError> {
        if self.is_running {
            return Err(DistributedError {
                error_type: "AlreadyRunning".to_string(),
                message: "Distributed scheduler is already running".to_string(),
                node_id: self.node_id,
                timestamp: current_timestamp(),
                context: None,
            });
        }

        self.is_running = true;
        // In a real implementation, this would start network threads
        // For now, we'll just mark it as running

        Ok(())
    }

    /// Stop the distributed scheduler and clean up resources
    pub fn stop(&mut self) -> Result<(), DistributedError> {
        self.is_running = false;
        Ok(())
    }

    /// Add a remote node to the distributed network
    pub fn add_remote_node(
        &mut self,
        node_id: u32,
        address: String,
    ) -> Result<(), DistributedError> {
        if node_id == self.node_id {
            return Err(DistributedError {
                error_type: "InvalidNodeId".to_string(),
                message: "Cannot add self as remote node".to_string(),
                node_id: self.node_id,
                timestamp: current_timestamp(),
                context: None,
            });
        }

        if self.remote_nodes.contains_key(&node_id) {
            return Err(DistributedError {
                error_type: "NodeAlreadyExists".to_string(),
                message: format!("Node {} already exists", node_id),
                node_id: self.node_id,
                timestamp: current_timestamp(),
                context: None,
            });
        }

        let node = DistributedNode {
            node_id,
            address,
            capabilities: HashSet::new(),
            load_factor: 0.0,
            last_heartbeat: current_timestamp(),
            is_active: true,
        };

        self.remote_nodes.insert(node_id, node);
        self.network_stats.nodes_joined += 1;

        Ok(())
    }

    /// Remove a remote node from the distributed network
    pub fn remove_remote_node(&mut self, node_id: u32) -> Result<(), DistributedError> {
        if node_id == self.node_id {
            return Err(DistributedError {
                error_type: "InvalidNodeId".to_string(),
                message: "Cannot remove self".to_string(),
                node_id: self.node_id,
                timestamp: current_timestamp(),
                context: None,
            });
        }

        if !self.remote_nodes.contains_key(&node_id) {
            return Err(DistributedError {
                error_type: "NodeNotFound".to_string(),
                message: format!("Node {} not found", node_id),
                node_id: self.node_id,
                timestamp: current_timestamp(),
                context: None,
            });
        }

        self.remote_nodes.remove(&node_id);
        self.network_stats.nodes_left += 1;

        Ok(())
    }

    /// Initiate a distributed consensus vote for a capability request
    pub fn initiate_distributed_consensus(
        &mut self,
        capability: Capability,
        requesting_actor: u32,
        justification: String,
    ) -> Result<u64, DistributedError> {
        let request_id = self.generate_request_id();

        let consensus_request = DistributedConsensusRequest {
            request_id,
            capability: capability.clone(),
            requesting_actor,
            requesting_node: self.node_id,
            justification,
            votes_required: self.calculate_votes_required(),
            votes_received: HashMap::new(),
            status: ConsensusStatus::Open,
            created_at: current_timestamp(),
            expires_at: current_timestamp() + 60, // 60 second timeout
        };

        self.consensus_votes
            .insert(request_id, consensus_request.clone());

        Ok(request_id)
    }

    /// Calculate the number of votes required for consensus
    fn calculate_votes_required(&self) -> u32 {
        // Simple majority of active nodes
        let active_nodes = self.remote_nodes.values().filter(|n| n.is_active).count() as u32 + 1; // +1 for self
        (active_nodes * 2 / 3) + 1 // 2/3 majority
    }

    /// Cast a vote on a consensus request
    pub fn cast_consensus_vote(
        &mut self,
        request_id: u64,
        vote: bool,
    ) -> Result<(), DistributedError> {
        if let Some(request) = self.consensus_votes.get_mut(&request_id) {
            if request.status != ConsensusStatus::Open {
                return Err(DistributedError {
                    error_type: "InvalidRequestState".to_string(),
                    message: format!("Consensus request {} is not open", request_id),
                    node_id: self.node_id,
                    timestamp: current_timestamp(),
                    context: None,
                });
            }

            // Record the vote
            request.votes_received.insert(self.node_id, vote);

            // Check if consensus is reached
            let approve_votes = request.votes_received.values().filter(|&&v| v).count() as u32;
            let deny_votes = request.votes_received.values().filter(|&&v| !v).count() as u32;

            if approve_votes >= request.votes_required {
                request.status = ConsensusStatus::Approved;
                println!(
                    "Consensus request {} approved with {} votes",
                    request.request_id, approve_votes
                );
            } else if deny_votes >= request.votes_required {
                request.status = ConsensusStatus::Denied;
                println!(
                    "Consensus request {} denied with {} votes",
                    request.request_id, deny_votes
                );
            } else if current_timestamp() >= request.expires_at {
                request.status = ConsensusStatus::Expired;
                println!("Consensus request {} expired", request.request_id);
            }

            Ok(())
        } else {
            Err(DistributedError {
                error_type: "RequestNotFound".to_string(),
                message: format!("Consensus request {} not found", request_id),
                node_id: self.node_id,
                timestamp: current_timestamp(),
                context: None,
            })
        }
    }

    /// Request remote execution of an actor on another node
    pub fn request_remote_execution(
        &mut self,
        actor_id: u32,
        target_node: u32,
        bytecode: Vec<crate::types::OpCode>,
        constants: Vec<crate::types::Value>,
        capabilities: Vec<Capability>,
        priority: u8,
        timeout: u64,
    ) -> Result<u64, DistributedError> {
        let request_id = self.generate_request_id();

        let execution_request = RemoteExecutionRequest {
            request_id,
            actor_id,
            source_node: self.node_id,
            target_node,
            bytecode,
            constants,
            capabilities,
            priority,
            timeout,
        };

        self.pending_remote_executions
            .insert(request_id, execution_request.clone());

        Ok(request_id)
    }

    /// Handle a remote execution response
    pub fn handle_remote_execution_response(
        &mut self,
        response: RemoteExecutionResponse,
    ) -> Result<(), DistributedError> {
        if let Some(request) = self.pending_remote_executions.get(&response.request_id) {
            if response.success {
                println!(
                    "Remote execution {} completed successfully on node {}",
                    response.request_id, request.target_node
                );
            } else {
                println!(
                    "Remote execution {} failed on node {}: {}",
                    response.request_id,
                    request.target_node,
                    response.error.unwrap_or_default()
                );
            }

            // Remove the pending request
            self.pending_remote_executions.remove(&response.request_id);
            Ok(())
        } else {
            Err(DistributedError {
                error_type: "RequestNotFound".to_string(),
                message: format!("Remote execution request {} not found", response.request_id),
                node_id: self.node_id,
                timestamp: current_timestamp(),
                context: None,
            })
        }
    }

    /// Generate a unique request ID
    fn generate_request_id(&mut self) -> u64 {
        self.message_sequence += 1;
        self.message_sequence
    }
}

/// Load balancer for distributed scheduling
#[derive(Debug, Clone)]
pub struct LoadBalancer {
    pub target_load_factor: f32,
    pub max_load_imbalance: f32,
    pub migration_threshold: f32,
    pub recent_load_history: Vec<f32>,
}

impl LoadBalancer {
    pub fn new() -> Self {
        Self {
            target_load_factor: 0.7,
            max_load_imbalance: 0.2,
            migration_threshold: 1.5,
            recent_load_history: Vec::new(),
        }
    }
}

/// Fault detector for node health monitoring
#[derive(Debug, Clone)]
pub struct FaultDetector {
    pub heartbeat_timeout: u64,
    pub max_missed_heartbeats: u32,
    pub node_failure_threshold: u64,
}

impl FaultDetector {
    pub fn new() -> Self {
        Self {
            heartbeat_timeout: 15, // 15 seconds
            max_missed_heartbeats: 3,
            node_failure_threshold: 30, // 30 seconds
        }
    }
}

/// Network statistics tracker
#[derive(Debug, Clone)]
pub struct NetworkStatistics {
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub nodes_joined: u32,
    pub nodes_left: u32,
    pub successful_migrations: u32,
    pub failed_migrations: u32,
    pub consensus_requests: u32,
    pub remote_executions: u32,
    pub network_errors: u32,
}

impl NetworkStatistics {
    pub fn new() -> Self {
        Self {
            messages_sent: 0,
            messages_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
            nodes_joined: 0,
            nodes_left: 0,
            successful_migrations: 0,
            failed_migrations: 0,
            consensus_requests: 0,
            remote_executions: 0,
            network_errors: 0,
        }
    }
}

/// Get current timestamp helper function
fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scheduler::PhysicsScheduler;

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
}
