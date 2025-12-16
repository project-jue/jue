#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::SourceLocation;
    use crate::trust_tier::TrustTier;
    use physics_world::Capability;

    // Explicit imports for type system types
    use crate::type_system::{
        ErrorHandlingStrategy, Type, TypeChecker, TypeEnvironment, TypeSignature,
    };

    #[test]
    fn test_type_signature_creation() {
        let location = SourceLocation::default();

        let sig = TypeSignature {
            name: "safe-add".to_string(),
            parameters: vec![
                Type::Basic("Int".to_string()),
                Type::Basic("Int".to_string()),
            ],
            return_type: Box::new(Type::Basic("Int".to_string())),
            required_capabilities: vec![],
            proof_obligation: Some("commutativity".to_string()),
            error_handling: ErrorHandlingStrategy::Proof,
            location,
        };

        assert_eq!(sig.name, "safe-add");
        assert_eq!(sig.parameters.len(), 2);
        assert_eq!(sig.required_capabilities.len(), 0);
        assert_eq!(sig.proof_obligation, Some("commutativity".to_string()));
    }

    #[test]
    fn test_type_environment() {
        let mut env = TypeEnvironment::new(TrustTier::Empirical);

        // Test that empirical tier has expected capabilities
        assert!(env.has_capability(&Capability::MacroHygienic));
        assert!(env.has_capability(&Capability::IoReadSensor));
        assert!(!env.has_capability(&Capability::MacroUnsafe));

        // Add a type binding
        env.add_binding("x".to_string(), Type::Basic("Int".to_string()));

        // Lookup the binding
        assert_eq!(env.lookup("x"), Some(&Type::Basic("Int".to_string())));
        assert_eq!(env.lookup("y"), None);
    }

    #[test]
    fn test_type_checker() {
        let checker = TypeChecker::new(TrustTier::Empirical);

        // Test basic type checking
        let basic_type = Type::Basic("Int".to_string());
        assert!(checker.check_type(&basic_type).is_ok());

        // Test function type
        let func_type = Type::Function {
            parameters: vec![Type::Basic("Int".to_string())],
            return_type: Box::new(Type::Basic("Bool".to_string())),
        };
        assert!(checker.check_type(&func_type).is_ok());

        // Test capability type with available capability
        let cap_type = Type::CapabilityType {
            base_type: Box::new(Type::Basic("Int".to_string())),
            capabilities: vec![Capability::IoReadSensor],
        };
        assert!(checker.check_type(&cap_type).is_ok());

        // Test capability type with unavailable capability
        let invalid_cap_type = Type::CapabilityType {
            base_type: Box::new(Type::Basic("Int".to_string())),
            capabilities: vec![Capability::MacroUnsafe],
        };
        assert!(checker.check_type(&invalid_cap_type).is_err());
    }

    #[test]
    fn test_error_handling_strategies() {
        let formal_checker = TypeChecker::new(TrustTier::Formal);
        let verified_checker = TypeChecker::new(TrustTier::Verified);
        let empirical_checker = TypeChecker::new(TrustTier::Empirical);
        let experimental_checker = TypeChecker::new(TrustTier::Experimental);

        assert_eq!(
            formal_checker.get_error_handling_strategy(),
            ErrorHandlingStrategy::Proof
        );
        assert_eq!(
            verified_checker.get_error_handling_strategy(),
            ErrorHandlingStrategy::Static
        );
        assert_eq!(
            empirical_checker.get_error_handling_strategy(),
            ErrorHandlingStrategy::Runtime
        );
        assert_eq!(
            experimental_checker.get_error_handling_strategy(),
            ErrorHandlingStrategy::Documentation
        );
    }

    #[test]
    fn test_type_signature_validation() {
        let mut env = TypeEnvironment::new(TrustTier::Empirical);
        let mut checker = TypeChecker {
            env,
            location: SourceLocation::default(),
        };

        // Valid signature with available capabilities
        let valid_sig = TypeSignature {
            name: "read-sensor".to_string(),
            parameters: vec![],
            return_type: Box::new(Type::Basic("Int".to_string())),
            required_capabilities: vec![Capability::IoReadSensor],
            proof_obligation: None,
            error_handling: ErrorHandlingStrategy::Runtime,
            location: SourceLocation::default(),
        };

        assert!(checker.check_signature(&valid_sig).is_ok());

        // Invalid signature with unavailable capabilities
        let invalid_sig = TypeSignature {
            name: "unsafe-macro".to_string(),
            parameters: vec![],
            return_type: Box::new(Type::Basic("Int".to_string())),
            required_capabilities: vec![Capability::MacroUnsafe],
            proof_obligation: None,
            error_handling: ErrorHandlingStrategy::Runtime,
            location: SourceLocation::default(),
        };

        assert!(checker.check_signature(&invalid_sig).is_err());
    }

    #[test]
    fn test_type_equality() {
        let type1 = Type::Basic("Int".to_string());
        let type2 = Type::Basic("Int".to_string());
        let type3 = Type::Basic("Bool".to_string());

        assert_eq!(type1, type2);
        assert_ne!(type1, type3);

        let func1 = Type::Function {
            parameters: vec![Type::Basic("Int".to_string())],
            return_type: Box::new(Type::Basic("Bool".to_string())),
        };

        let func2 = Type::Function {
            parameters: vec![Type::Basic("Int".to_string())],
            return_type: Box::new(Type::Basic("Bool".to_string())),
        };

        assert_eq!(func1, func2);
    }
}
