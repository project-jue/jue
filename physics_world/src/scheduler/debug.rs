use super::actor::Actor;
use super::actor::CapRequest;
use super::capability::{CapDecision, CapDecisionResult, CapOperation};
use super::error::PhysicsError;
/// Capability management and debugging utilities for the Physics World scheduler
use super::execution::PhysicsScheduler;

impl PhysicsScheduler {
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

        self.capability_audit_log
            .push(super::capability::CapAuditEntry {
                timestamp,
                actor_id: requester_id,
                operation: CapOperation::Request,
                capability: capability.clone(),
                result: CapDecisionResult::Pending,
            });

        // Find the requesting actor
        let actor = self.actors.iter_mut().find(|a| a.id == requester_id);
        if actor.is_none() {
            if let Some(last_entry) = self.capability_audit_log.last_mut() {
                last_entry.result = CapDecisionResult::Error("Actor not found".to_string());
            }
            return CapDecision::Denied;
        }
        let actor = actor.unwrap();

        // Check if actor already has the capability
        if actor.capabilities.contains(&capability) {
            if let Some(last_entry) = self.capability_audit_log.last_mut() {
                last_entry.result = CapDecisionResult::Granted;
            }
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
        if let Some(last_entry) = self.capability_audit_log.last_mut() {
            last_entry.result = match decision {
                CapDecision::Granted => CapDecisionResult::Granted,
                CapDecision::Denied => CapDecisionResult::Denied,
                CapDecision::PendingConsensus => CapDecisionResult::ConsensusRequired,
            };
        }

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
        // Find the actor indices by ID
        let granter_index = self
            .actors
            .iter()
            .position(|a| a.id == granter_id)
            .ok_or(PhysicsError::ActorNotFound(granter_id))?;

        let target_index = self
            .actors
            .iter()
            .position(|a| a.id == target_id)
            .ok_or(PhysicsError::ActorNotFound(target_id))?;

        // Ensure we have actors to work with
        if self.actors.is_empty() {
            return Err(PhysicsError::ActorNotFound(granter_id));
        }

        // Get references to both actors for validation
        let granter = &self.actors[granter_index];
        let target = &self.actors[target_index];

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
        self.capability_audit_log
            .push(super::capability::CapAuditEntry {
                timestamp: self.next_request_id,
                actor_id: target_id,
                operation: CapOperation::Delegate,
                capability: capability.clone(),
                result: CapDecisionResult::Granted,
            });
        self.next_request_id += 1;

        // Add the capability to the target
        self.actors[target_index].capabilities.insert(capability);

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
        // Find the actor indices by ID
        let revoker_index = self
            .actors
            .iter()
            .position(|a| a.id == revoker_id)
            .ok_or(PhysicsError::ActorNotFound(revoker_id))?;

        let target_index = self
            .actors
            .iter()
            .position(|a| a.id == target_id)
            .ok_or(PhysicsError::ActorNotFound(target_id))?;

        // Ensure we have actors to work with
        if self.actors.is_empty() {
            return Err(PhysicsError::ActorNotFound(revoker_id));
        }

        // Get references to both actors for validation
        let revoker = &self.actors[revoker_index];
        let target = &self.actors[target_index];

        // Check if revoker has permission to revoke this capability
        if !PhysicsScheduler::can_revoke_capability(revoker, target, capability) {
            return Err(PhysicsError::CapabilityError(
                "Revoker does not have permission to revoke this capability".to_string(),
            ));
        }

        // Log the revoke operation
        self.capability_audit_log
            .push(super::capability::CapAuditEntry {
                timestamp: self.next_request_id,
                actor_id: target_id,
                operation: CapOperation::Revoke,
                capability: capability.clone(),
                result: CapDecisionResult::Granted,
            });
        self.next_request_id += 1;

        // Remove the capability from the target
        self.actors[target_index].capabilities.remove(capability);

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
    pub fn get_capability_audit_log(&self) -> &[super::capability::CapAuditEntry] {
        &self.capability_audit_log
    }

    /// V2 Capability System - Clear capability audit log
    pub fn clear_capability_audit_log(&mut self) {
        self.capability_audit_log.clear();
    }
}
