/// Capability mediator for FFI system
///
/// This module handles capability checking and mediation
/// for foreign function interface calls.
use crate::error::{CapabilityViolation, CompilationError};
use crate::shared::trust_tier::TrustTier;
use physics_world::types::Capability;
/// Capability mediator configuration
pub struct CapabilityMediatorConfig {
    /// Trust tier for the mediator
    pub trust_tier: TrustTier,
    /// Allowed capabilities
    pub allowed_capabilities: Vec<Capability>,
}

/// Capability check result
pub enum CapabilityCheckResult {
    /// Check passed
    Allowed,
    /// Check failed
    Denied(String),
}

/// Create a new capability mediator configuration
pub fn create_capability_mediator_config(trust_tier: TrustTier) -> CapabilityMediatorConfig {
    CapabilityMediatorConfig {
        trust_tier,
        allowed_capabilities: trust_tier.granted_capabilities().into_iter().collect(),
    }
}

/// Check if a capability is allowed
pub fn check_capability(
    capability: &Capability,
    config: &CapabilityMediatorConfig,
) -> CapabilityCheckResult {
    if config.allowed_capabilities.contains(capability) {
        CapabilityCheckResult::Allowed
    } else {
        CapabilityCheckResult::Denied(format!(
            "Capability {:?} not allowed for trust tier {:?}",
            capability, config.trust_tier
        ))
    }
}

/// Mediate a capability request
pub fn mediate_capability_request(
    capability: Capability,
    config: &CapabilityMediatorConfig,
) -> Result<(), CompilationError> {
    match check_capability(&capability, config) {
        CapabilityCheckResult::Allowed => Ok(()),
        CapabilityCheckResult::Denied(message) => {
            Err(CompilationError::CapabilityError(CapabilityViolation {
                required: capability,
                tier: config.trust_tier,
                location: Default::default(),
                suggestion: message,
            }))
        }
    }
}
