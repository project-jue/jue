use std::collections::HashMap;

/// Environment for tracking variable bindings during compilation
#[derive(Debug, Clone, Default)]
pub struct Environment {
    /// Maps variable names to their stack positions
    variables: HashMap<String, usize>,
}

impl Environment {
    /// Create a new empty environment
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Bind a variable to a stack position
    pub fn bind_variable(&mut self, name: String, position: usize) {
        self.variables.insert(name, position);
    }

    /// Resolve a variable to its stack position
    pub fn resolve_variable(&self, name: &str) -> Option<usize> {
        self.variables.get(name).copied()
    }

    /// Check if a variable is bound in the current environment
    pub fn is_bound(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }
}
