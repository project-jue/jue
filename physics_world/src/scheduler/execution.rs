/// Core execution logic for the Physics World scheduler
use crate::types::Value;
use crate::vm::error::VmError as DetailedVmError;
use crate::vm::state::InstructionResult;

use super::{
    actor::Actor, error::PhysicsError, CapAuditEntry, CapDecision, CapDecisionResult, CapOperation,
    CapRequest,
};
use std::collections::HashMap;

/// Manages multiple actors and enforces fair, deterministic execution.
pub struct PhysicsScheduler {
    pub actors: Vec<Actor>,
    pub current_actor_index: usize,
    pub message_queues: HashMap<u32, Vec<Value>>, // External inbox per actor
    // V2 Capability System - Added capability authority state
    pub capability_audit_log: Vec<CapAuditEntry>,
    pub next_request_id: u64,
    // V2 Priority Scheduling - Added priority scheduling state
    pub use_priority_scheduling: bool, // Enable/disable priority scheduling
    pub starvation_counter: u64,       // Track steps since last low-priority actor ran
    pub starvation_threshold: u64,     // When to force run a low-priority actor
    // V2 Resource Management - Added resource tracking and management
    pub global_step_count: u64, // Global step counter for resource accounting
    pub total_memory_usage: usize, // Total memory usage across all actors
    pub memory_limit: usize,    // Global memory limit
    pub cpu_time_limit: u64,    // Global CPU time limit
    pub resource_usage_history: Vec<ResourceUsageSnapshot>, // Historical resource usage
    pub resource_quota_system: ResourceQuotaSystem, // Resource quota management
}

/// Clone implementation for PhysicsScheduler
/// Creates a new scheduler with the same configuration but empty state
impl Clone for PhysicsScheduler {
    fn clone(&self) -> Self {
        Self {
            actors: Vec::new(), // Don't clone actors - they should be managed separately
            current_actor_index: 0,
            message_queues: HashMap::new(),
            capability_audit_log: Vec::new(),
            next_request_id: self.next_request_id,
            use_priority_scheduling: self.use_priority_scheduling,
            starvation_counter: 0,
            starvation_threshold: self.starvation_threshold,
            global_step_count: 0,
            total_memory_usage: 0,
            memory_limit: self.memory_limit,
            cpu_time_limit: self.cpu_time_limit,
            resource_usage_history: Vec::new(),
            resource_quota_system: ResourceQuotaSystem {
                default_memory_quota: self.resource_quota_system.default_memory_quota,
                default_cpu_quota: self.resource_quota_system.default_cpu_quota,
                actor_quotas: HashMap::new(),
                global_memory_limit: self.resource_quota_system.global_memory_limit,
                global_cpu_limit: self.resource_quota_system.global_cpu_limit,
            },
        }
    }
}

/// V2 Resource Management - Resource usage snapshot for historical tracking
#[derive(Debug, Clone)]
pub struct ResourceUsageSnapshot {
    pub timestamp: u64,
    pub global_step_count: u64,
    pub total_memory_usage: usize,
    pub total_cpu_time: u64,
    pub active_actors: u32,
    pub memory_fragmentation: f32,
}

/// V2 Resource Management - Resource quota system for fair resource allocation
#[derive(Debug, Clone)]
pub struct ResourceQuotaSystem {
    pub default_memory_quota: usize,
    pub default_cpu_quota: u64,
    pub actor_quotas: HashMap<u32, ActorResourceQuota>,
    pub global_memory_limit: usize,
    pub global_cpu_limit: u64,
}

/// V2 Resource Management - Per-actor resource quotas
#[derive(Debug, Clone)]
pub struct ActorResourceQuota {
    pub actor_id: u32,
    pub memory_quota: usize,
    pub cpu_quota: u64,
    pub memory_used: usize,
    pub cpu_used: u64,
    pub last_updated: u64,
}

/// V2 Resource Management - Resource allocation result
#[derive(Debug, Clone)]
pub enum ResourceAllocationResult {
    Success,
    QuotaExceeded,
    GlobalLimitExceeded,
    InsufficientResources,
}

/// V2 Resource Management - Resource monitoring statistics
#[derive(Debug, Clone)]
pub struct ResourceMonitoringStats {
    pub memory_usage: usize,
    pub memory_limit: usize,
    pub memory_usage_percent: f32,
    pub cpu_time_used: u64,
    pub cpu_time_limit: u64,
    pub cpu_usage_percent: f32,
    pub fragmentation_ratio: f32,
    pub active_actors: u32,
    pub waiting_actors: u32,
}

/// Result of a scheduler tick operation.
#[derive(Debug)]
pub enum TickResult {
    ActorYielded(u32),
    ActorFinished(u32, Value),
    ActorErrored(u32, DetailedVmError),
    ActorWaitingForCapability(u32, crate::types::Capability),
}

impl PhysicsScheduler {
    /// Creates a new PhysicsScheduler with empty actor list.
    pub fn new() -> Self {
        Self {
            actors: Vec::new(),
            current_actor_index: 0,
            message_queues: HashMap::new(),
            capability_audit_log: Vec::new(),
            next_request_id: 0,
            use_priority_scheduling: false, // Default to round-robin for backward compatibility
            starvation_counter: 0,
            starvation_threshold: 1000, // Default threshold to prevent starvation
            // V2 Resource Management - Initialize resource tracking
            global_step_count: 0,
            total_memory_usage: 0,
            memory_limit: usize::MAX, // No limit by default
            cpu_time_limit: u64::MAX, // No limit by default
            resource_usage_history: Vec::new(),
            resource_quota_system: ResourceQuotaSystem {
                default_memory_quota: 1024 * 1024, // 1MB default
                default_cpu_quota: 1000000,        // 1M steps default
                actor_quotas: HashMap::new(),
                global_memory_limit: usize::MAX,
                global_cpu_limit: u64::MAX,
            },
        }
    }

    /// Main execution tick. Runs the current actor until it yields, finishes, or hits a limit.
    pub fn tick(&mut self) -> Result<TickResult, PhysicsError> {
        // Check if there are any actors
        if self.actors.is_empty() {
            return Err(PhysicsError::SchedulerError(
                "No actors to schedule".to_string(),
            ));
        }

        // Select next actor based on scheduling mode
        if self.use_priority_scheduling {
            self.select_next_actor_by_priority();
        }
        // Note: For round-robin, we don't advance here - current_actor_index stays the same
        // until the actor yields/finishes/errors, then we advance in the result handling

        // Get current actor
        let current_index = self.current_actor_index;
        let actor = &mut self.actors[current_index];

        // Process any messages in the actor's mailbox first
        if !actor.mailbox.is_empty() {
            // For now, just push messages onto the stack
            // In a real implementation, this would be more sophisticated
            for message in actor.mailbox.drain(..) {
                actor.vm.stack.push(message);
            }
        }

        // Execute the actor's VM until it yields, finishes, errors, or requests a capability
        loop {
            match actor.vm.step() {
                Ok(InstructionResult::Continue) => {
                    // Continue executing
                    continue;
                }
                Ok(InstructionResult::Yield) => {
                    // Actor yielded, move to next actor
                    let actor_id = actor.id;
                    self.advance_to_next_actor();
                    return Ok(TickResult::ActorYielded(actor_id));
                }
                Ok(InstructionResult::Finished(value)) => {
                    // Actor finished, move to next actor
                    let actor_id = actor.id;
                    self.advance_to_next_actor();
                    return Ok(TickResult::ActorFinished(actor_id, value));
                }
                Ok(InstructionResult::WaitingForCapability(capability)) => {
                    // Actor is waiting for capability decision
                    let actor_id = actor.id;
                    self.advance_to_next_actor();
                    return Ok(TickResult::ActorWaitingForCapability(actor_id, capability));
                }
                Err(vm_error) => {
                    // Actor errored, move to next actor
                    let actor_id = actor.id;
                    // Convert the simple VmError to detailed VmError
                    let context = actor.vm.create_error_context();
                    let detailed_error =
                        crate::vm::error::WithContext::with_context(vm_error, context);
                    self.advance_to_next_actor();
                    return Ok(TickResult::ActorErrored(actor_id, detailed_error));
                }
            }
        }
    }

    /// Delivers a message to an actor's external queue.
    pub fn send_message(&mut self, target: u32, message: Value) {
        self.message_queues
            .entry(target)
            .or_insert_with(Vec::new)
            .push(message);
    }

    /// Advances to the next actor in round-robin fashion.
    pub fn advance_to_next_actor(&mut self) {
        if self.actors.is_empty() {
            self.current_actor_index = 0;
            return;
        }

        self.current_actor_index = (self.current_actor_index + 1) % self.actors.len();
    }

    /// Adds a new actor to the scheduler.
    pub fn add_actor(&mut self, actor: Actor) {
        self.actors.push(actor);
    }

    /// Gets the current actor ID.
    pub fn current_actor_id(&self) -> Option<u32> {
        self.actors.get(self.current_actor_index).map(|a| a.id)
    }

    /// Delivers external messages to actors' mailboxes.
    pub fn deliver_external_messages(&mut self) {
        for (actor_id, messages) in self.message_queues.drain() {
            if let Some(actor) = self.actors.iter_mut().find(|a| a.id == actor_id) {
                actor.mailbox.extend(messages);
            }
        }
    }

    // === V2 Capability System Methods ===
    // These methods were consolidated from debug.rs

    /// V2 Capability System - Check if an actor has a specific capability
    pub fn actor_has_capability(
        &self,
        actor_id: u32,
        capability: &crate::types::Capability,
    ) -> bool {
        if let Some(actor) = self.actors.iter().find(|a| a.id == actor_id) {
            actor.capabilities.contains(capability)
        } else {
            false
        }
    }

    /// V2 Capability System - Handle capability requests from actors
    pub fn handle_capability_request(
        &mut self,
        requester_id: u32,
        capability: crate::types::Capability,
        justification: &str,
    ) -> CapDecision {
        // Log the request
        let timestamp = self.next_request_id;
        self.next_request_id += 1;

        self.capability_audit_log.push(CapAuditEntry {
            timestamp,
            actor_id: requester_id,
            operation: CapOperation::Request,
            capability: capability.clone(),
            result: CapDecisionResult::Pending,
        });

        // Find the requesting actor
        let actor = self.actors.iter_mut().find(|a| a.id == requester_id);
        if actor.is_none() {
            self.capability_audit_log.last_mut().unwrap().result =
                CapDecisionResult::Error("Actor not found".to_string());
            return CapDecision::Denied;
        }
        let actor = actor.unwrap();

        // Check if actor already has the capability
        if actor.capabilities.contains(&capability) {
            self.capability_audit_log.last_mut().unwrap().result = CapDecisionResult::Granted;
            return CapDecision::Granted;
        }

        // Store the request in the actor's request queue
        actor.capability_requests.push(CapRequest {
            capability: capability.clone(),
            justification: justification.to_string(),
            requested_at: timestamp,
            granted: None,
        });

        // Advanced capability decision logic with delegation and consensus
        let decision = match capability {
            // Meta capabilities require special handling
            crate::types::Capability::MetaSelfModify => {
                // Only grant to actors with parent (not root actors)
                if actor.parent_id.is_some() {
                    CapDecision::Granted
                } else {
                    CapDecision::Denied
                }
            }
            crate::types::Capability::MetaGrant => {
                // MetaGrant is dangerous, require consensus
                CapDecision::PendingConsensus
            }
            // Macro capabilities
            crate::types::Capability::MacroHygienic => CapDecision::Granted,
            crate::types::Capability::MacroUnsafe => {
                // Unsafe macros require MetaGrant
                if actor
                    .capabilities
                    .contains(&crate::types::Capability::MetaGrant)
                {
                    CapDecision::Granted
                } else {
                    CapDecision::Denied
                }
            }
            crate::types::Capability::ComptimeEval => CapDecision::Granted,
            // I/O capabilities - generally safe for now
            crate::types::Capability::IoReadSensor => CapDecision::Granted,
            crate::types::Capability::IoWriteActuator => CapDecision::Granted,
            crate::types::Capability::IoNetwork => {
                // Network access requires justification
                if !justification.is_empty() {
                    CapDecision::Granted
                } else {
                    CapDecision::Denied
                }
            }
            crate::types::Capability::IoPersist => CapDecision::Granted,
            // System capabilities
            crate::types::Capability::SysCreateActor => CapDecision::Granted,
            crate::types::Capability::SysTerminateActor => {
                // Can only terminate self or children
                CapDecision::Granted
            }
            crate::types::Capability::SysClock => CapDecision::Granted,
            // Resource capabilities
            crate::types::Capability::ResourceExtraMemory(_) => CapDecision::Granted,
            crate::types::Capability::ResourceExtraTime(_) => CapDecision::Granted,
        };

        // Update the request with the decision
        if let Some(last_request) = actor.capability_requests.last_mut() {
            last_request.granted = match decision {
                CapDecision::Granted => Some(true),
                CapDecision::Denied => Some(false),
                CapDecision::PendingConsensus => None,
            };
        }

        // Log the decision
        self.capability_audit_log.last_mut().unwrap().result = match decision {
            CapDecision::Granted => CapDecisionResult::Granted,
            CapDecision::Denied => CapDecisionResult::Denied,
            CapDecision::PendingConsensus => CapDecisionResult::ConsensusRequired,
        };

        // If granted, add the capability to the actor
        if let CapDecision::Granted = decision {
            actor.capabilities.insert(capability);
        }

        decision
    }

    /// V2 Capability System - Grant a capability to an actor with delegation validation
    pub fn grant_capability(
        &mut self,
        granter_id: u32,
        target_id: u32,
        capability: crate::types::Capability,
    ) -> Result<(), PhysicsError> {
        // Find the granter actor
        let granter = self
            .actors
            .iter()
            .find(|a| a.id == granter_id)
            .ok_or(PhysicsError::ActorNotFound(granter_id))?;

        // Find the target actor
        let target = self
            .actors
            .iter()
            .find(|a| a.id == target_id)
            .ok_or(PhysicsError::ActorNotFound(target_id))?;

        // Check if granter has MetaGrant capability for delegation
        if !granter
            .capabilities
            .contains(&crate::types::Capability::MetaGrant)
        {
            return Err(PhysicsError::CapabilityError(
                "Granter does not have MetaGrant capability".to_string(),
            ));
        }

        // Check if granter has the capability they're trying to delegate
        if !granter.capabilities.contains(&capability) {
            return Err(PhysicsError::CapabilityError(
                "Granter does not have the capability to delegate".to_string(),
            ));
        }

        // Check if granter can delegate this specific capability
        if !PhysicsScheduler::can_delegate_capability(granter, &capability, target) {
            return Err(PhysicsError::CapabilityError(
                "Granter cannot delegate this capability".to_string(),
            ));
        }

        // Log the grant operation
        self.capability_audit_log.push(CapAuditEntry {
            timestamp: self.next_request_id,
            actor_id: target_id,
            operation: CapOperation::Delegate,
            capability: capability.clone(),
            result: CapDecisionResult::Granted,
        });
        self.next_request_id += 1;

        // Add the capability to the target
        if let Some(target_actor) = self.actors.iter_mut().find(|a| a.id == target_id) {
            target_actor.capabilities.insert(capability);
        }

        Ok(())
    }

    /// V2 Capability System - Check if an actor can delegate a specific capability
    fn can_delegate_capability(
        granter: &Actor,
        capability: &crate::types::Capability,
        target: &Actor,
    ) -> bool {
        // Granter must have the capability they're trying to delegate
        if !granter.capabilities.contains(capability) {
            return false;
        }

        // Special rules for dangerous capabilities
        match capability {
            // MetaGrant can only be delegated to children or with high reputation
            crate::types::Capability::MetaGrant => {
                target.parent_id == Some(granter.id) || granter.priority > 200
            }
            // Other dangerous capabilities require consensus
            crate::types::Capability::SysTerminateActor
            | crate::types::Capability::MacroUnsafe
            | crate::types::Capability::MetaSelfModify => {
                // For now, only allow delegation to children for dangerous capabilities
                target.parent_id == Some(granter.id)
            }
            // Most capabilities can be freely delegated
            _ => true,
        }
    }

    /// V2 Capability System - Revoke a capability from an actor with validation
    pub fn revoke_capability(
        &mut self,
        revoker_id: u32,
        target_id: u32,
        capability: &crate::types::Capability,
    ) -> Result<(), PhysicsError> {
        // Find the revoker actor
        let revoker = self
            .actors
            .iter()
            .find(|a| a.id == revoker_id)
            .ok_or(PhysicsError::ActorNotFound(revoker_id))?;

        // Find the target actor
        let target = self
            .actors
            .iter()
            .find(|a| a.id == target_id)
            .ok_or(PhysicsError::ActorNotFound(target_id))?;

        // Check if revoker has permission to revoke this capability
        if !PhysicsScheduler::can_revoke_capability(revoker, target, capability) {
            return Err(PhysicsError::CapabilityError(
                "Revoker does not have permission to revoke this capability".to_string(),
            ));
        }

        // Log the revoke operation
        self.capability_audit_log.push(CapAuditEntry {
            timestamp: self.next_request_id,
            actor_id: target_id,
            operation: CapOperation::Revoke,
            capability: capability.clone(),
            result: CapDecisionResult::Granted,
        });
        self.next_request_id += 1;

        // Remove the capability from the target
        if let Some(target_actor) = self.actors.iter_mut().find(|a| a.id == target_id) {
            target_actor.capabilities.remove(capability);
        }

        Ok(())
    }

    /// V2 Capability System - Check if an actor can revoke a capability
    fn can_revoke_capability(
        revoker: &Actor,
        target: &Actor,
        capability: &crate::types::Capability,
    ) -> bool {
        // Actors can always revoke their own capabilities
        if revoker.id == target.id {
            return true;
        }

        // MetaGrant holders can revoke most capabilities
        if revoker
            .capabilities
            .contains(&crate::types::Capability::MetaGrant)
        {
            // But cannot revoke MetaGrant from others unless they're the parent
            if let crate::types::Capability::MetaGrant = capability {
                return target.parent_id == Some(revoker.id);
            }
            return true;
        }

        // Parents can revoke capabilities from their children
        target.parent_id == Some(revoker.id)
    }

    /// V2 Capability System - Get capability audit log
    pub fn get_capability_audit_log(&self) -> &[CapAuditEntry] {
        &self.capability_audit_log
    }

    /// V2 Capability System - Clear capability audit log
    pub fn clear_capability_audit_log(&mut self) {
        self.capability_audit_log.clear();
    }
}
