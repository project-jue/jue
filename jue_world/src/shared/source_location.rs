/// Source location tracking for compilation errors and warnings

/// Represents a location in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceLocation {
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// File path
    pub file: &'static str,
}

impl SourceLocation {
    /// Create a new source location
    pub fn new(line: usize, column: usize, file: &'static str) -> Self {
        SourceLocation { line, column, file }
    }

    /// Create a dummy location for generated code
    pub fn generated() -> Self {
        SourceLocation {
            line: 0,
            column: 0,
            file: "generated",
        }
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        SourceLocation::generated()
    }
}

/// Represents a span in source code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan {
    /// Start location
    pub start: SourceLocation,
    /// End location
    pub end: SourceLocation,
}

impl SourceSpan {
    /// Create a new source span
    pub fn new(start: SourceLocation, end: SourceLocation) -> Self {
        SourceSpan { start, end }
    }

    /// Create a single-point span
    pub fn point(location: SourceLocation) -> Self {
        SourceSpan {
            start: location,
            end: location,
        }
    }
}
