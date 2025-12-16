use crate::trust_tier::TrustTier;
use physics_world::types::Capability;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Source location for error reporting
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SourceLocation {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Character offset in source
    pub offset: usize,
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            line: 0,
            column: 0,
            offset: 0,
        }
    }
}

/// Capability violation error
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilityViolation {
    /// The capability that was required
    pub required: Capability,
    /// The trust tier that was insufficient
    pub tier: TrustTier,
    /// Source location of the violation
    pub location: SourceLocation,
    /// Suggestion for fixing the issue
    pub suggestion: String,
}

impl Error for CapabilityViolation {}

impl fmt::Display for CapabilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Capability violation: {:?} tier requires capability {:?} at {}:{}",
            self.tier, self.required, self.location.line, self.location.column
        )?;
        if !self.suggestion.is_empty() {
            write!(f, "\nSuggestion: {}", self.suggestion)?;
        }
        Ok(())
    }
}

/// Type mismatch error
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TypeMismatch {
    /// Expected type
    pub expected: String,
    /// Found type
    pub found: String,
    /// Source location
    pub location: SourceLocation,
}

impl Error for TypeMismatch {}

impl fmt::Display for TypeMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type mismatch at {}:{} - expected {}, found {}",
            self.location.line, self.location.column, self.expected, self.found
        )
    }
}

/// Parser resource limit error
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParserError {
    /// Error message
    pub message: String,
    /// Source location
    pub location: SourceLocation,
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parser resource limit exceeded at {}:{} - {}",
            self.location.line, self.location.column, self.message
        )
    }
}

/// Compilation error enum
#[derive(Debug, thiserror::Error)]
pub enum CompilationError {
    /// Parse error with message and location
    #[error("Parse error at {location:?}: {message}")]
    ParseError {
        /// Error message
        message: String,
        /// Source location
        location: SourceLocation,
    },

    /// Parser resource limit exceeded
    #[error("Parser resource limit: {0}")]
    ParserResourceLimit(#[from] ParserError),

    /// Capability violation error
    #[error("Capability violation: {0}")]
    CapabilityError(#[from] CapabilityViolation),

    /// Type mismatch error
    #[error("Type mismatch: {0}")]
    TypeError(#[from] TypeMismatch),

    /// Proof generation failed
    #[error("Proof generation failed: {0}")]
    ProofGenerationFailed(String),

    /// Empirical validation failed
    #[error("Empirical validation failed: {0}")]
    EmpiricalValidationFailed(String),

    /// Macro expansion error
    #[error("Macro expansion error: {0}")]
    MacroExpansionError(String),

    /// Comptime execution error
    #[error("Comptime execution error: {0}")]
    ComptimeError(String),

    /// FFI error
    #[error("FFI error: {0}")]
    FfiError(String),

    /// Internal compiler error
    #[error("Internal compiler error: {0}")]
    InternalError(String),
}

/// Source map for debugging information
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SourceMap {
    /// Mapping from bytecode offsets to source locations
    pub bytecode_to_source: Vec<(usize, SourceLocation)>,
    /// Mapping from source locations to bytecode offsets
    pub source_to_bytecode: Vec<(SourceLocation, usize)>,
}

impl SourceMap {
    /// Create a new empty source map
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a mapping between bytecode offset and source location
    pub fn add_mapping(&mut self, bytecode_offset: usize, source_location: SourceLocation) {
        self.bytecode_to_source
            .push((bytecode_offset, source_location.clone()));
        self.source_to_bytecode
            .push((source_location, bytecode_offset));
    }

    /// Find source location for a bytecode offset
    pub fn find_source_location(&self, bytecode_offset: usize) -> Option<&SourceLocation> {
        self.bytecode_to_source
            .iter()
            .find(|(offset, _)| *offset == bytecode_offset)
            .map(|(_, location)| location)
    }

    /// Find bytecode offset for a source location
    pub fn find_bytecode_offset(&self, source_location: &SourceLocation) -> Option<&usize> {
        self.source_to_bytecode
            .iter()
            .find(|(location, _)| *location == *source_location)
            .map(|(_, offset)| offset)
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
