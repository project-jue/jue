#[cfg(test)]
mod tests {
    use crate::ast::{AstNode, Literal};
    use crate::error::{CompilationError, SourceLocation};
    use crate::trust_tier::TrustTier;
    use physics_world::types::Capability;

    // Explicit imports for macro system types
    use crate::macro_system::{HygieneScope, MacroContext, MacroDefinition, MacroExpander};

    #[test]
    fn test_macro_definition() {
        let location = SourceLocation::default();
        let body = AstNode::Literal(Literal::Int(42));

        let macro_def = MacroDefinition {
            name: "test-macro".to_string(),
            parameters: vec!["x".to_string()],
            body,
            required_capabilities: vec![Capability::MacroHygienic],
            declared_tier: TrustTier::Formal,
            is_hygienic: true,
            location,
        };

        assert_eq!(macro_def.name, "test-macro");
        assert_eq!(macro_def.parameters.len(), 1);
        assert_eq!(macro_def.required_capabilities.len(), 1);
        assert!(macro_def.is_hygienic);
    }

    #[test]
    fn test_hygiene_scope() {
        let mut scope = HygieneScope::new();

        assert_eq!(scope.level, 0);
        assert_eq!(scope.captured.len(), 0);

        scope.enter();
        assert_eq!(scope.level, 1);

        scope.capture("x".to_string());
        assert!(scope.is_captured("x"));
        assert!(!scope.is_captured("y"));

        scope.exit();
        assert_eq!(scope.level, 0);
    }

    #[test]
    fn test_macro_context() {
        let context = MacroContext {
            caller_capabilities: TrustTier::Empirical.granted_capabilities(),
            expansion_capabilities: TrustTier::Empirical.granted_capabilities(),
            tier: TrustTier::Empirical,
            hygiene_scope: HygieneScope::new(),
            location: SourceLocation::default(),
        };

        assert!(context
            .caller_capabilities
            .contains(&Capability::MacroHygienic));
        assert!(context
            .expansion_capabilities
            .contains(&Capability::IoReadSensor));
        assert_eq!(context.tier, TrustTier::Empirical);
    }

    #[test]
    fn test_macro_expander_creation() {
        let expander = MacroExpander::new(TrustTier::Empirical);

        assert_eq!(expander.macros.len(), 0);
        assert_eq!(expander.context.tier, TrustTier::Empirical);
        assert!(expander
            .context
            .caller_capabilities
            .contains(&Capability::MacroHygienic));
    }

    #[test]
    fn test_macro_expansion_simple() {
        let mut expander = MacroExpander::new(TrustTier::Empirical);

        // Define a simple macro
        let macro_def = MacroDefinition {
            name: "add-one".to_string(),
            parameters: vec!["x".to_string()],
            body: AstNode::Call {
                function: Box::new(AstNode::Variable("+".to_string())),
                arguments: vec![
                    AstNode::Variable("x".to_string()),
                    AstNode::Literal(Literal::Int(1)),
                ],
                location: SourceLocation::default(),
            },
            required_capabilities: vec![Capability::MacroHygienic],
            declared_tier: TrustTier::Formal,
            is_hygienic: true,
            location: SourceLocation::default(),
        };

        expander.add_macro(macro_def);

        // Test expansion
        let result = expander.expand_macro(
            "add-one",
            vec![AstNode::Literal(Literal::Int(5))],
            SourceLocation::default(),
        );

        assert!(result.is_ok());
        let expanded = result.unwrap();

        // Should be a call to + with 5 and 1
        match expanded {
            AstNode::Call {
                function,
                arguments,
                ..
            } => {
                assert!(matches!(*function, AstNode::Variable(ref s) if s == "+"));
                assert_eq!(arguments.len(), 2);
                assert!(matches!(arguments[0], AstNode::Literal(Literal::Int(5))));
                assert!(matches!(arguments[1], AstNode::Literal(Literal::Int(1))));
            }
            _ => panic!("Expected Call node"),
        }
    }

    #[test]
    fn test_macro_expansion_capability_check() {
        let mut expander = MacroExpander::new(TrustTier::Experimental);

        // Define a macro that requires a capability not available in Experimental tier
        let macro_def = MacroDefinition {
            name: "unsafe-macro".to_string(),
            parameters: vec![],
            body: AstNode::Literal(Literal::Int(42)),
            required_capabilities: vec![Capability::MetaGrant],
            declared_tier: TrustTier::Experimental,
            is_hygienic: false,
            location: SourceLocation::default(),
        };

        expander.add_macro(macro_def);

        // This should fail because Experimental tier doesn't have MetaGrant
        let result = expander.expand_macro("unsafe-macro", vec![], SourceLocation::default());

        assert!(result.is_err());
        match result {
            Err(CompilationError::CapabilityError(_)) => assert!(true),
            Err(e) => panic!("Expected CapabilityError, got: {:?}", e),
            _ => panic!("Expected CapabilityError"),
        }
    }

    #[test]
    fn test_macro_expansion_tier_check() {
        let mut expander = MacroExpander::new(TrustTier::Formal);

        // Define a macro that requires a higher tier
        let macro_def = MacroDefinition {
            name: "verified-macro".to_string(),
            parameters: vec![],
            body: AstNode::Literal(Literal::Int(42)),
            required_capabilities: vec![Capability::MacroHygienic],
            declared_tier: TrustTier::Verified,
            is_hygienic: true,
            location: SourceLocation::default(),
        };

        expander.add_macro(macro_def);

        // This should fail because Formal tier is lower than Verified
        let result = expander.expand_macro("verified-macro", vec![], SourceLocation::default());

        assert!(result.is_err());
        match result {
            Err(CompilationError::MacroExpansionError(_)) => assert!(true),
            _ => panic!("Expected MacroExpansionError"),
        }
    }

    #[test]
    fn test_macro_expansion_wrong_arg_count() {
        let mut expander = MacroExpander::new(TrustTier::Empirical);

        let macro_def = MacroDefinition {
            name: "two-arg-macro".to_string(),
            parameters: vec!["x".to_string(), "y".to_string()],
            body: AstNode::Literal(Literal::Int(42)),
            required_capabilities: vec![Capability::MacroHygienic],
            declared_tier: TrustTier::Formal,
            is_hygienic: true,
            location: SourceLocation::default(),
        };

        expander.add_macro(macro_def);

        // Call with wrong number of arguments
        let result = expander.expand_macro(
            "two-arg-macro",
            vec![AstNode::Literal(Literal::Int(1))], // Only 1 argument
            SourceLocation::default(),
        );

        assert!(result.is_err());
        match result {
            Err(CompilationError::MacroExpansionError(_)) => assert!(true),
            _ => panic!("Expected MacroExpansionError"),
        }
    }
}
