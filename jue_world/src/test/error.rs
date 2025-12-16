#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{
        CapabilityViolation, CompilationError, SourceLocation, SourceMap, TypeMismatch,
    };
    use crate::trust_tier::TrustTier;
    use physics_world::types::Capability;

    #[test]
    fn test_source_location() {
        let loc = SourceLocation {
            line: 42,
            column: 8,
            offset: 337,
        };

        assert_eq!(loc.line, 42);
        assert_eq!(loc.column, 8);
        assert_eq!(loc.offset, 337);
    }

    #[test]
    fn test_capability_violation_display() {
        let violation = CapabilityViolation {
            required: Capability::IoReadSensor,
            tier: TrustTier::Formal,
            location: SourceLocation {
                line: 10,
                column: 5,
                offset: 100,
            },
            suggestion: "Consider using :empirical tier".to_string(),
        };

        let display = violation.to_string();
        assert!(display.contains("Capability violation"));
        assert!(display.contains("Formal"));
        assert!(display.contains("IoReadSensor"));
        assert!(display.contains("10:5"));
        assert!(display.contains("Consider using :empirical tier"));
    }

    #[test]
    fn test_type_mismatch_display() {
        let mismatch = TypeMismatch {
            expected: "Int".to_string(),
            found: "Bool".to_string(),
            location: SourceLocation {
                line: 20,
                column: 15,
                offset: 200,
            },
        };

        let display = mismatch.to_string();
        assert!(display.contains("Type mismatch"));
        assert!(display.contains("20:15"));
        assert!(display.contains("expected Int"));
        assert!(display.contains("found Bool"));
    }

    #[test]
    fn test_source_map() {
        let mut source_map = SourceMap::new();

        let loc1 = SourceLocation {
            line: 1,
            column: 1,
            offset: 0,
        };

        let loc2 = SourceLocation {
            line: 2,
            column: 5,
            offset: 10,
        };

        source_map.add_mapping(0, loc1.clone());
        source_map.add_mapping(10, loc2.clone());

        assert_eq!(source_map.find_source_location(0), Some(&loc1));
        assert_eq!(source_map.find_source_location(10), Some(&loc2));
        assert_eq!(source_map.find_source_location(99), None);

        assert_eq!(source_map.find_bytecode_offset(&loc1), Some(&0));
        assert_eq!(source_map.find_bytecode_offset(&loc2), Some(&10));
        assert_eq!(
            source_map.find_bytecode_offset(&SourceLocation {
                line: 99,
                column: 99,
                offset: 99
            }),
            None
        );
    }

    #[test]
    fn test_compilation_error_variants() {
        let parse_error = CompilationError::ParseError {
            message: "Unexpected token".to_string(),
            location: SourceLocation::default(),
        };

        let capability_error = CompilationError::CapabilityError(CapabilityViolation {
            required: Capability::MacroUnsafe,
            tier: TrustTier::Formal,
            location: SourceLocation::default(),
            suggestion: "Use higher trust tier".to_string(),
        });

        let type_error = CompilationError::TypeError(TypeMismatch {
            expected: "Function".to_string(),
            found: "Int".to_string(),
            location: SourceLocation::default(),
        });

        // Test that all variants can be created and matched
        match parse_error {
            CompilationError::ParseError { .. } => assert!(true),
            _ => panic!("Expected ParseError"),
        }

        match capability_error {
            CompilationError::CapabilityError(_) => assert!(true),
            _ => panic!("Expected CapabilityError"),
        }

        match type_error {
            CompilationError::TypeError(_) => assert!(true),
            _ => panic!("Expected TypeError"),
        }
    }
}
