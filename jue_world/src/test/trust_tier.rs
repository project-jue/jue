#[cfg(test)]
mod tests {
    use physics_world::types::Capability;

    // Explicit import for TrustTier
    use crate::TrustTier;

    #[test]
    fn test_formal_tier_capabilities() {
        let tier = TrustTier::Formal;
        let caps = tier.granted_capabilities();

        assert!(caps.contains(&Capability::MacroHygienic));
        assert_eq!(caps.len(), 1); // Only MacroHygienic
    }

    #[test]
    fn test_verified_tier_capabilities() {
        let tier = TrustTier::Verified;
        let caps = tier.granted_capabilities();

        assert!(caps.contains(&Capability::MacroHygienic));
        assert!(caps.contains(&Capability::ComptimeEval));
        assert_eq!(caps.len(), 2);
    }

    #[test]
    fn test_empirical_tier_capabilities() {
        let tier = TrustTier::Empirical;
        let caps = tier.granted_capabilities();

        assert!(caps.contains(&Capability::MacroHygienic));
        assert!(caps.contains(&Capability::ComptimeEval));
        assert!(caps.contains(&Capability::IoReadSensor));
        assert!(caps.contains(&Capability::IoWriteActuator));
        assert_eq!(caps.len(), 4);
    }

    #[test]
    fn test_experimental_tier_capabilities() {
        let tier = TrustTier::Experimental;
        let caps = tier.granted_capabilities();

        // Should have most capabilities
        assert!(caps.contains(&Capability::MacroHygienic));
        assert!(caps.contains(&Capability::MacroUnsafe));
        assert!(caps.contains(&Capability::ComptimeEval));
        assert!(caps.contains(&Capability::IoReadSensor));
        assert!(caps.contains(&Capability::IoWriteActuator));
        assert!(caps.contains(&Capability::IoNetwork));
        assert!(caps.contains(&Capability::IoPersist));
        assert!(caps.contains(&Capability::SysCreateActor));
        assert!(caps.contains(&Capability::SysClock));

        // Should NOT have the most dangerous capabilities
        assert!(!caps.contains(&Capability::MetaGrant));
        assert!(!caps.contains(&Capability::SysTerminateActor));
        assert!(!caps.contains(&Capability::MetaSelfModify));
    }

    #[test]
    fn test_allows_capability() {
        let formal = TrustTier::Formal;
        assert!(formal.allows_capability(&Capability::MacroHygienic));
        assert!(!formal.allows_capability(&Capability::ComptimeEval));

        let empirical = TrustTier::Empirical;
        assert!(empirical.allows_capability(&Capability::IoReadSensor));
        assert!(!empirical.allows_capability(&Capability::MacroUnsafe));
    }

    #[test]
    fn test_tier_hierarchy() {
        let formal = TrustTier::Formal;
        let verified = TrustTier::Verified;
        let empirical = TrustTier::Empirical;
        let experimental = TrustTier::Experimental;

        // Test is_at_least relationships
        assert!(experimental.is_at_least(&formal));
        assert!(empirical.is_at_least(&verified));
        assert!(verified.is_at_least(&formal));
        assert!(!formal.is_at_least(&verified));
        assert!(!empirical.is_at_least(&experimental));
    }
}
