use crate::error::{CompilationError, SourceLocation};
use crate::trust_tier::TrustTier;
use physics_world::types::Capability;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Structured error with detailed context and recovery information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredError {
    /// Error type classification
    pub error_type: ErrorType,
    /// Error severity level
    pub severity: ErrorSeverity,
    /// Detailed error message
    pub message: String,
    /// Source location where error occurred
    pub location: SourceLocation,
    /// Context information about what was being processed
    pub context: ErrorContext,
    /// Suggested recovery actions
    pub recovery_suggestions: Vec<String>,
    /// Related capabilities (if applicable)
    pub related_capabilities: Vec<Capability>,
    /// Trust tier information
    pub trust_tier: TrustTier,
    /// Error code for programmatic handling
    pub error_code: String,
    /// Stack trace (if available)
    pub stack_trace: Option<String>,
}

/// Error type classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorType {
    /// Syntax or parsing error
    SyntaxError,
    /// Type system error
    TypeError,
    /// Capability system error
    CapabilityError,
    /// Resource limit error
    ResourceLimitError,
    /// Proof system error
    ProofError,
    /// Compile-time execution error
    ComptimeError,
    /// Foreign function interface error
    FfiError,
    /// Internal compiler error
    InternalError,
    /// Security violation error
    SecurityError,
}

/// Error severity level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    /// Informational message
    Info,
    /// Warning that doesn't prevent compilation
    Warning,
    /// Error that prevents compilation
    Error,
    /// Critical error that indicates compiler bug
    Critical,
}

impl fmt::Display for ErrorSeverity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorSeverity::Info => write!(f, "Info"),
            ErrorSeverity::Warning => write!(f, "Warning"),
            ErrorSeverity::Error => write!(f, "Error"),
            ErrorSeverity::Critical => write!(f, "Critical"),
        }
    }
}

/// Context information about what was being processed when error occurred
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Current compilation phase
    pub phase: CompilationPhase,
    /// Current trust tier
    pub trust_tier: TrustTier,
    /// Current module or file being processed
    pub module: String,
    /// Current function or expression being processed
    pub function: Option<String>,
    /// Related source code snippet
    pub source_snippet: Option<String>,
}

/// Compilation phase where error occurred
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompilationPhase {
    /// Lexing/tokenization phase
    Lexing,
    /// Parsing phase
    Parsing,
    /// Macro expansion phase
    MacroExpansion,
    /// Type checking phase
    TypeChecking,
    /// Capability analysis phase
    CapabilityAnalysis,
    /// Proof generation phase
    ProofGeneration,
    /// Bytecode generation phase
    BytecodeGeneration,
    /// Compile-time execution phase
    ComptimeExecution,
    /// Optimization phase
    Optimization,
    /// Linking phase
    Linking,
}

/// Structured error builder for fluent error construction
pub struct StructuredErrorBuilder {
    /// Error being constructed
    error: StructuredError,
}

impl StructuredErrorBuilder {
    /// Create a new error builder
    pub fn new(error_type: ErrorType, message: String) -> Self {
        Self {
            error: StructuredError {
                error_type,
                severity: ErrorSeverity::Error,
                message,
                location: SourceLocation::default(),
                context: ErrorContext {
                    phase: CompilationPhase::Parsing,
                    trust_tier: TrustTier::Formal,
                    module: "unknown".to_string(),
                    function: None,
                    source_snippet: None,
                },
                recovery_suggestions: Vec::new(),
                related_capabilities: Vec::new(),
                trust_tier: TrustTier::Formal,
                error_code: "GENERIC".to_string(),
                stack_trace: None,
            },
        }
    }

    /// Set error severity
    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.error.severity = severity;
        self
    }

    /// Set source location
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.error.location = location;
        self
    }

    /// Set compilation phase
    pub fn with_phase(mut self, phase: CompilationPhase) -> Self {
        self.error.context.phase = phase;
        self
    }

    /// Set module name
    pub fn with_module(mut self, module: &str) -> Self {
        self.error.context.module = module.to_string();
        self
    }

    /// Set function name
    pub fn with_function(mut self, function: &str) -> Self {
        self.error.context.function = Some(function.to_string());
        self
    }

    /// Set source snippet
    pub fn with_source_snippet(mut self, snippet: &str) -> Self {
        self.error.context.source_snippet = Some(snippet.to_string());
        self
    }

    /// Add recovery suggestion
    pub fn with_recovery_suggestion(mut self, suggestion: &str) -> Self {
        self.error.recovery_suggestions.push(suggestion.to_string());
        self
    }

    /// Add related capability
    pub fn with_related_capability(mut self, capability: Capability) -> Self {
        self.error.related_capabilities.push(capability);
        self
    }

    /// Set trust tier
    pub fn with_trust_tier(mut self, tier: TrustTier) -> Self {
        self.error.trust_tier = tier;
        self.error.context.trust_tier = tier;
        self
    }

    /// Set error code
    pub fn with_error_code(mut self, code: &str) -> Self {
        self.error.error_code = code.to_string();
        self
    }

    /// Set stack trace
    pub fn with_stack_trace(mut self, trace: &str) -> Self {
        self.error.stack_trace = Some(trace.to_string());
        self
    }

    /// Build the structured error
    pub fn build(self) -> StructuredError {
        self.error
    }
}

/// Structured error handler for centralized error management
#[derive(Debug, Clone)]
pub struct StructuredErrorHandler {
    /// Collection of errors
    errors: Vec<StructuredError>,
    /// Current trust tier
    trust_tier: TrustTier,
    /// Current module
    current_module: String,
}

impl StructuredErrorHandler {
    /// Create a new error handler
    pub fn new(trust_tier: TrustTier) -> Self {
        Self {
            errors: Vec::new(),
            trust_tier,
            current_module: "main".to_string(),
        }
    }

    /// Set current module
    pub fn set_current_module(&mut self, module: &str) {
        self.current_module = module.to_string();
    }

    /// Add a structured error
    pub fn add_error(&mut self, error: StructuredError) {
        self.errors.push(error);
    }

    /// Create and add a capability violation error
    pub fn add_capability_violation(
        &mut self,
        required: Capability,
        location: SourceLocation,
        suggestion: &str,
    ) {
        let error = StructuredErrorBuilder::new(
            ErrorType::CapabilityError,
            format!("Capability {:?} required but not available", required),
        )
        .with_severity(ErrorSeverity::Error)
        .with_location(location)
        .with_phase(CompilationPhase::CapabilityAnalysis)
        .with_module(&self.current_module)
        .with_related_capability(required)
        .with_trust_tier(self.trust_tier)
        .with_recovery_suggestion(suggestion)
        .with_error_code("CAP_VIOLATION")
        .build();

        self.errors.push(error);
    }

    /// Create and add a type mismatch error
    pub fn add_type_mismatch(&mut self, expected: &str, found: &str, location: SourceLocation) {
        let error = StructuredErrorBuilder::new(
            ErrorType::TypeError,
            format!("Type mismatch: expected {}, found {}", expected, found),
        )
        .with_severity(ErrorSeverity::Error)
        .with_location(location)
        .with_phase(CompilationPhase::TypeChecking)
        .with_module(&self.current_module)
        .with_trust_tier(self.trust_tier)
        .with_recovery_suggestion("Check your type annotations and function signatures")
        .with_error_code("TYPE_MISMATCH")
        .build();

        self.errors.push(error);
    }

    /// Create and add a resource limit error
    pub fn add_resource_limit_error(
        &mut self,
        resource_type: &str,
        limit: u64,
        location: SourceLocation,
    ) {
        let error = StructuredErrorBuilder::new(
            ErrorType::ResourceLimitError,
            format!("{} limit exceeded: {}", resource_type, limit),
        )
        .with_severity(ErrorSeverity::Error)
        .with_location(location)
        .with_phase(CompilationPhase::ComptimeExecution)
        .with_module(&self.current_module)
        .with_trust_tier(self.trust_tier)
        .with_recovery_suggestion("Increase resource limits or optimize your code")
        .with_error_code("RESOURCE_LIMIT")
        .build();

        self.errors.push(error);
    }

    /// Create and add a proof generation error
    pub fn add_proof_generation_error(&mut self, message: &str, location: SourceLocation) {
        let error = StructuredErrorBuilder::new(
            ErrorType::ProofError,
            format!("Proof generation failed: {}", message),
        )
        .with_severity(ErrorSeverity::Error)
        .with_location(location)
        .with_phase(CompilationPhase::ProofGeneration)
        .with_module(&self.current_module)
        .with_trust_tier(self.trust_tier)
        .with_recovery_suggestion("Check your proof obligations and logical consistency")
        .with_error_code("PROOF_GEN")
        .build();

        self.errors.push(error);
    }

    /// Check if any critical errors exist
    pub fn has_critical_errors(&self) -> bool {
        self.errors
            .iter()
            .any(|e| e.severity == ErrorSeverity::Critical)
    }

    /// Check if any errors exist
    pub fn has_errors(&self) -> bool {
        self.errors
            .iter()
            .any(|e| e.severity >= ErrorSeverity::Error)
    }

    /// Get all errors
    pub fn get_errors(&self) -> &[StructuredError] {
        &self.errors
    }

    /// Clear all errors
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    /// Convert to compilation error (for compatibility)
    pub fn to_compilation_error(&self) -> Option<CompilationError> {
        if self.has_errors() {
            let error = self.errors.first().unwrap();
            Some(CompilationError::InternalError(error.message.clone()))
        } else {
            None
        }
    }
}

impl Error for StructuredError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl fmt::Display for StructuredError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} [{}]", self.severity, self.error_code)?;
        writeln!(f, "{}", self.message)?;
        writeln!(
            f,
            "Location: {}:{}",
            self.location.line, self.location.column
        )?;

        if !self.context.module.is_empty() {
            writeln!(f, "Module: {}", self.context.module)?;
        }

        if let Some(func) = &self.context.function {
            writeln!(f, "Function: {}", func)?;
        }

        writeln!(f, "Phase: {:?}", self.context.phase)?;
        writeln!(f, "Trust Tier: {:?}", self.trust_tier)?;

        if !self.recovery_suggestions.is_empty() {
            writeln!(f, "\nRecovery suggestions:")?;
            for suggestion in &self.recovery_suggestions {
                writeln!(f, "  - {}", suggestion)?;
            }
        }

        if !self.related_capabilities.is_empty() {
            writeln!(f, "\nRelated capabilities:")?;
            for cap in &self.related_capabilities {
                writeln!(f, "  - {:?}", cap)?;
            }
        }

        if let Some(trace) = &self.stack_trace {
            writeln!(f, "\nStack trace:\n{}", trace)?;
        }

        Ok(())
    }
}

/// Error reporting formatter for different output formats
pub struct ErrorReporter {
    /// Error handler
    handler: StructuredErrorHandler,
    /// Output format
    format: ErrorFormat,
}

impl ErrorReporter {
    /// Create a new error reporter
    pub fn new(handler: StructuredErrorHandler, format: ErrorFormat) -> Self {
        Self { handler, format }
    }

    /// Report all errors
    pub fn report_errors(&self) -> String {
        match self.format {
            ErrorFormat::Text => self.report_text(),
            ErrorFormat::Json => self.report_json(),
            ErrorFormat::Compact => self.report_compact(),
        }
    }

    /// Report errors in text format
    fn report_text(&self) -> String {
        let mut output = String::new();

        for error in self.handler.get_errors() {
            output.push_str(&format!("{}\n\n", error));
        }

        if output.is_empty() {
            "No errors found".to_string()
        } else {
            output
        }
    }

    /// Report errors in JSON format
    fn report_json(&self) -> String {
        serde_json::to_string_pretty(&self.handler.get_errors())
            .unwrap_or_else(|_| "{\"error\": \"Failed to serialize errors\"}".to_string())
    }

    /// Report errors in compact format
    fn report_compact(&self) -> String {
        let mut output = String::new();

        for error in self.handler.get_errors() {
            output.push_str(&format!(
                "{}:{} {} [{}] {}\n",
                error.location.line,
                error.location.column,
                error.severity,
                error.error_code,
                error.message
            ));
        }

        if output.is_empty() {
            "No errors found".to_string()
        } else {
            output
        }
    }
}

/// Error output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorFormat {
    /// Human-readable text format
    Text,
    /// JSON format for programmatic processing
    Json,
    /// Compact format for logging
    Compact,
}
