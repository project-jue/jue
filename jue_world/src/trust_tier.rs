use physics_world::types::Capability;
use std::collections::HashSet;

/// Trust Tier definitions and capability grants for Jue-World V2.0

/// Trust tiers determine compilation path, capability grants, and verification requirements
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TrustTier {
    /// Must have mathematical proof of correctness
    /// Granted Capabilities: MacroHygienic only
    Formal,

    /// Proven via automated theorem proving or exhaustive checking
    /// Granted Capabilities: MacroHygienic, ComptimeEval
    Verified,

    /// No proof, but must pass extensive test suites
    /// Granted Capabilities: MacroHygienic, ComptimeEval, IoReadSensor, IoWriteActuator
    Empirical,

    /// Unsandboxed, user-accepted risk
    /// Granted Capabilities: All except MetaGrant, SysTerminateActor
    Experimental,
}

impl Default for TrustTier {
    fn default() -> Self {
        TrustTier::Formal
    }
}

impl TrustTier {
    /// Get the capabilities automatically granted for this trust tier
    pub fn granted_capabilities(&self) -> HashSet<Capability> {
        match self {
            TrustTier::Formal => {
                let mut caps = HashSet::new();
                caps.insert(Capability::MacroHygienic);
                caps
            }
            TrustTier::Verified => {
                let mut caps = HashSet::new();
                caps.insert(Capability::MacroHygienic);
                caps.insert(Capability::ComptimeEval);
                caps
            }
            TrustTier::Empirical => {
                let mut caps = HashSet::new();
                caps.insert(Capability::MacroHygienic);
                caps.insert(Capability::ComptimeEval);
                caps.insert(Capability::IoReadSensor);
                caps.insert(Capability::IoWriteActuator);
                caps
            }
            TrustTier::Experimental => {
                let mut caps = HashSet::new();
                // All capabilities except the most dangerous ones
                caps.insert(Capability::MacroHygienic);
                caps.insert(Capability::MacroUnsafe);
                caps.insert(Capability::ComptimeEval);
                caps.insert(Capability::IoReadSensor);
                caps.insert(Capability::IoWriteActuator);
                caps.insert(Capability::IoNetwork);
                caps.insert(Capability::IoPersist);
                caps.insert(Capability::SysCreateActor);
                caps.insert(Capability::SysClock);
                // Note: MetaGrant and SysTerminateActor are NOT granted
                caps
            }
        }
    }

    /// Check if this tier allows the given capability
    pub fn allows_capability(&self, capability: &Capability) -> bool {
        let granted = self.granted_capabilities();
        granted.contains(capability)
    }

    /// Check if this tier is at least as privileged as another tier
    pub fn is_at_least(&self, other: &TrustTier) -> bool {
        // Order from least to most privileged
        let order = [
            TrustTier::Formal,
            TrustTier::Verified,
            TrustTier::Empirical,
            TrustTier::Experimental,
        ];

        let self_index = order.iter().position(|&t| t == *self).unwrap();
        let other_index = order.iter().position(|&t| t == *other).unwrap();

        self_index >= other_index
    }
}

#[cfg(test)]
#[path = "test/trust_tier.rs"]
mod tests;
