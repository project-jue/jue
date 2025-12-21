use super::*;
use crate::error::SourceLocation;

#[test]
fn test_structured_error_creation() {
    let location = SourceLocation {
        line: 10,
        column: 5,
        offset: 50,
    };

    let error = StructuredErrorBuilder::new(
        ErrorType::CapabilityError,
        "Test capability error".to_string(),
    )
    .with_severity(ErrorSeverity::Error)
    .with_location(location)
    .with_phase(CompilationPhase::CapabilityAnalysis)
    .with_module("test_module")
    .with_function("test_function")
    .with_source_snippet("test code")
    .with_recovery_suggestion("Fix the capability issue")
    .with_related_capability(Capability::IoReadSensor)
    .with_trust_tier(TrustTier::Empirical)
    .with_error_code("TEST_ERROR")
    .with_stack_trace("test trace")
    .build();

    assert!(error.error_type == ErrorType::CapabilityError);
    assert!(error.severity == ErrorSeverity::Error);
    assert!(error.location.line == 10);
    assert!(error.context.module == "test_module");
    assert!(error.recovery_suggestions.len() == 1);
    assert!(error.related_capabilities.len() == 1);
    assert!(error.trust_tier == TrustTier::Empirical);
    assert!(error.error_code == "TEST_ERROR");
}

#[test]
fn test_error_handler() {
    let mut handler = StructuredErrorHandler::new(TrustTier::Empirical);

    // Add a capability violation
    handler.add_capability_violation(
        Capability::IoReadSensor,
        SourceLocation::default(),
        "Upgrade trust tier or request capability",
    );

    // Add a type mismatch
    handler.add_type_mismatch("Int", "Bool", SourceLocation::default());

    assert!(handler.has_errors());
    assert!(!handler.has_critical_errors());
    assert!(handler.get_errors().len() == 2);
}

#[test]
fn test_error_reporting() {
    let mut handler = StructuredErrorHandler::new(TrustTier::Empirical);

    handler.add_capability_violation(
        Capability::IoReadSensor,
        SourceLocation {
            line: 42,
            column: 10,
            offset: 420,
        },
        "Upgrade trust tier",
    );

    let reporter = ErrorReporter::new(handler, ErrorFormat::Text);
    let report = reporter.report_errors();

    assert!(report.contains("Error [CAP_VIOLATION]"));
    assert!(report.contains("42:10"));
    assert!(report.contains("Capability IoReadSensor required but not available"));
}

#[test]
fn test_error_formats() {
    let mut handler = StructuredErrorHandler::new(TrustTier::Empirical);

    handler.add_capability_violation(
        Capability::IoReadSensor,
        SourceLocation::default(),
        "Test suggestion",
    );

    // Test text format
    let text_reporter = ErrorReporter::new(handler.clone(), ErrorFormat::Text);
    let text_report = text_reporter.report_errors();
    assert!(text_report.contains("Error [CAP_VIOLATION]"));

    // Test JSON format
    let json_reporter = ErrorReporter::new(handler.clone(), ErrorFormat::Json);
    let json_report = json_reporter.report_errors();
    assert!(json_report.contains("CAP_VIOLATION"));

    // Test compact format
    let compact_reporter = ErrorReporter::new(handler, ErrorFormat::Compact);
    let compact_report = compact_reporter.report_errors();
    assert!(compact_report.contains("CAP_VIOLATION"));
}

#[test]
fn test_error_display() {
    let error = StructuredErrorBuilder::new(ErrorType::TypeError, "Type mismatch".to_string())
        .with_severity(ErrorSeverity::Error)
        .with_location(SourceLocation {
            line: 1,
            column: 1,
            offset: 0,
        })
        .with_recovery_suggestion("Check your types")
        .build();

    let display = format!("{}", error);
    assert!(display.contains("Error [GENERIC]"));
    assert!(display.contains("Type mismatch"));
    assert!(display.contains("1:1"));
    assert!(display.contains("Check your types"));
}

#[test]
fn test_error_severity_display() {
    assert_eq!(format!("{}", ErrorSeverity::Info), "Info");
    assert_eq!(format!("{}", ErrorSeverity::Warning), "Warning");
    assert_eq!(format!("{}", ErrorSeverity::Error), "Error");
    assert_eq!(format!("{}", ErrorSeverity::Critical), "Critical");
}
