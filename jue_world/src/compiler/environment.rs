/// Compilation environment for Jue-World V2.0
///
/// This module provides the compilation environment used during
/// AST to bytecode compilation.
use std::collections::HashMap;

/// Variable scope
#[derive(Debug, Clone)]
pub struct VariableScope {
    /// Variable bindings in this scope
    pub bindings: HashMap<String, usize>,
    /// Parent scope
    pub parent: Option<Box<VariableScope>>,
}

/// Compilation environment
#[derive(Debug, Clone)]
pub struct CompilationEnvironment {
    /// Current variable scope
    pub current_scope: VariableScope,
    /// Stack frame size
    pub frame_size: usize,
    /// Saved frame sizes for nested scopes
    pub saved_frame_sizes: Vec<usize>,
}

impl CompilationEnvironment {
    /// Create a new compilation environment
    pub fn new() -> Self {
        Self {
            current_scope: VariableScope {
                bindings: HashMap::new(),
                parent: None,
            },
            frame_size: 0,
            saved_frame_sizes: Vec::new(),
        }
    }

    /// Push a new scope
    pub fn push_scope(&mut self) {
        let saved_frame_size = self.frame_size;
        self.saved_frame_sizes.push(saved_frame_size);
    }

    /// Pop the current scope
    pub fn pop_scope(&mut self) {
        if let Some(saved_frame_size) = self.saved_frame_sizes.pop() {
            self.frame_size = saved_frame_size;
        }
    }

    /// Define a variable in the current scope
    pub fn define_variable(&mut self, name: &str) -> usize {
        let offset = self.frame_size;
        self.current_scope.bindings.insert(name.to_string(), offset);
        self.frame_size += 1;
        offset
    }

    /// Lookup a variable in the current scope or parent scopes
    pub fn lookup_variable(&self, name: &str) -> Option<usize> {
        let mut current = &self.current_scope;
        loop {
            if let Some(offset) = current.bindings.get(name) {
                return Some(*offset);
            }
            match &current.parent {
                Some(parent) => current = parent,
                None => return None,
            }
        }
    }

    /// Get variable index (alias for lookup_variable)
    pub fn get_variable_index(&self, name: &str) -> Option<usize> {
        self.lookup_variable(name)
    }

    /// Add a variable to the current scope
    pub fn add_variable(&mut self, name: String, offset: usize) -> usize {
        self.current_scope.bindings.insert(name, offset);
        offset
    }
}
