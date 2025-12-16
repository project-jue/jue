#[cfg(test)]
mod tests {
    use crate::ast::{AstNode, Literal, Type};
    use crate::error::SourceLocation;

    #[test]
    fn test_literal_display() {
        assert_eq!(format!("{}", Literal::Nil), "nil");
        assert_eq!(format!("{}", Literal::Bool(true)), "true");
        assert_eq!(format!("{}", Literal::Int(42)), "42");
        assert_eq!(format!("{}", Literal::Float(3.14)), "3.14");
        assert_eq!(
            format!("{}", Literal::String("hello".to_string())),
            "\"hello\""
        );
    }

    #[test]
    fn test_type_display() {
        let basic = Type::Basic("Int".to_string());
        assert_eq!(format!("{}", basic), "Int");

        let function = Type::Function {
            parameters: vec![Type::Basic("Int".to_string())],
            return_type: Box::new(Type::Basic("Bool".to_string())),
        };
        assert_eq!(format!("{}", function), "(Int -> Bool)");

        let cap_type = Type::CapabilityType {
            base_type: Box::new(Type::Basic("Int".to_string())),
            capabilities: vec!["IoReadSensor".to_string()],
        };
        assert_eq!(format!("{}", cap_type), "Int [IoReadSensor]");
    }

    #[test]
    fn test_ast_node_creation() {
        let location = SourceLocation::default();

        let literal = AstNode::Literal(Literal::Int(42));
        let symbol = AstNode::Symbol("test".to_string());
        let variable = AstNode::Variable("x".to_string());

        let call = AstNode::Call {
            function: Box::new(variable.clone()),
            arguments: vec![literal.clone()],
            location: location.clone(),
        };

        let lambda = AstNode::Lambda {
            parameters: vec!["x".to_string()],
            body: Box::new(call.clone()),
            location: location.clone(),
        };

        // Test that all nodes can be created and displayed
        assert_eq!(format!("{}", literal), "42");
        assert_eq!(format!("{}", symbol), "'test");
        assert_eq!(format!("{}", variable), "x");
        assert!(format!("{}", call).contains("x"));
        assert!(format!("{}", lambda).contains("lambda"));
    }

    #[test]
    fn test_trust_tier_node() {
        let location = SourceLocation::default();
        let inner = AstNode::Literal(Literal::Int(42));

        let tier_node = AstNode::TrustTier {
            tier: ":empirical".to_string(),
            expression: Box::new(inner),
            location,
        };

        let display = format!("{}", tier_node);
        assert!(display.contains(":empirical"));
        assert!(display.contains("42"));
    }

    #[test]
    fn test_capability_nodes() {
        let location = SourceLocation::default();

        let require_cap = AstNode::RequireCapability {
            capability: "IoReadSensor".to_string(),
            location: location.clone(),
        };

        let has_cap = AstNode::HasCapability {
            capability: "MacroHygienic".to_string(),
            location,
        };

        assert!(format!("{}", require_cap).contains("require-capability"));
        assert!(format!("{}", has_cap).contains("has-capability?"));
    }

    #[test]
    fn test_ast_equality() {
        let loc1 = SourceLocation::default();
        let loc2 = SourceLocation::default();

        let ast1 = AstNode::Literal(Literal::Int(42));
        let ast2 = AstNode::Literal(Literal::Int(42));
        let ast3 = AstNode::Literal(Literal::Int(43));

        assert_eq!(ast1, ast2);
        assert_ne!(ast1, ast3);

        let _call1 = AstNode::Call {
            function: Box::new(AstNode::Variable("f".to_string())),
            arguments: vec![AstNode::Literal(Literal::Int(1))],
            location: loc1,
        };

        let _call2 = AstNode::Call {
            function: Box::new(AstNode::Variable("f".to_string())),
            arguments: vec![AstNode::Literal(Literal::Int(1))],
            location: loc2,
        };
    }
}
