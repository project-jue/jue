use super::capability::{CapDecision, CapDecisionResult, CapOperation, ConsensusVoteResult};
use super::error::PhysicsError;
/// Consensus mechanisms for the Physics World scheduler
use super::core::PhysicsScheduler;

impl PhysicsScheduler {
    /// V2 Capability System - Handle consensus voting for dangerous capabilities
    pub fn handle_consensus_vote(
        &mut self,
        voter_id: u32,
        requester_id: u32,
        capability: &crate::types::Capability,
        vote: bool, // true = approve, false = deny
    ) -> Result<CapDecision, PhysicsError> {
        // Find the voter actor
        let voter = self.actors.iter().find(|a| a.id == voter_id);
        if voter.is_none() {
            return Err(PhysicsError::ActorNotFound(voter_id));
        }
        let _voter = voter.unwrap();

        // Find the requester actor
        let requester = self.actors.iter().find(|a| a.id == requester_id);
        if requester.is_none() {
            return Err(PhysicsError::ActorNotFound(requester_id));
        }

        // Log the consensus vote
        self.capability_audit_log
            .push(super::capability::CapAuditEntry {
                timestamp: self.next_request_id,
                actor_id: voter_id,
                operation: CapOperation::ConsensusVote,
                capability: capability.clone(),
                result: if vote {
                    CapDecisionResult::Granted
                } else {
                    CapDecisionResult::Denied
                },
            });
        self.next_request_id += 1;

        // Get the current consensus state for this request
        let consensus_result = self.get_consensus_state(requester_id, capability)?;

        // Check if consensus threshold is met (75% supermajority for dangerous capabilities)
        let required_approval = (consensus_result.total as f32 * 0.75).ceil() as u32;

        if consensus_result.approve >= required_approval {
            // Consensus approved - grant the capability
            if let Some(target) = self.actors.iter_mut().find(|a| a.id == requester_id) {
                target.capabilities.insert(capability.clone());

                // Update the audit log with final decision
                self.capability_audit_log.last_mut().unwrap().result =
                    CapDecisionResult::ConsensusApproved;
            }
            Ok(CapDecision::Granted)
        } else if consensus_result.deny > consensus_result.total / 2 {
            // Majority denial - deny the capability
            self.capability_audit_log.last_mut().unwrap().result =
                CapDecisionResult::ConsensusDenied;
            Ok(CapDecision::Denied)
        } else {
            // Still pending
            Ok(CapDecision::PendingConsensus)
        }
    }

    /// V2 Capability System - Get current consensus state for a capability request
    fn get_consensus_state(
        &self,
        requester_id: u32,
        capability: &crate::types::Capability,
    ) -> Result<ConsensusVoteResult, PhysicsError> {
        // Count votes from the audit log
        let mut approve = 0;
        let mut deny = 0;
        let mut abstain = 0;

        for entry in &self.capability_audit_log {
            if entry.operation == CapOperation::ConsensusVote
                && entry.actor_id == requester_id
                && entry.capability == *capability
            {
                match entry.result {
                    CapDecisionResult::Granted => approve += 1,
                    CapDecisionResult::Denied => deny += 1,
                    _ => abstain += 1,
                }
            }
        }

        // Count total eligible voters (actors with MetaGrant capability)
        let total_eligible = self
            .actors
            .iter()
            .filter(|a| {
                a.capabilities
                    .contains(&crate::types::Capability::MetaGrant)
            })
            .count() as u32;

        Ok(ConsensusVoteResult {
            approve,
            deny,
            abstain,
            total: total_eligible,
        })
    }
}
