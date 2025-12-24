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

        // Create new scope with current scope as parent
        let new_scope = VariableScope {
            bindings: HashMap::new(),
            parent: Some(Box::new(self.current_scope.clone())),
        };
        self.current_scope = new_scope;
        self.frame_size = 0;
    }

    /// Pop the current scope
    pub fn pop_scope(&mut self) {
        if let Some(saved_frame_size) = self.saved_frame_sizes.pop() {
            // Restore parent scope
            if let Some(parent) = self.current_scope.parent.take() {
                self.current_scope = *parent;
            }
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
    pub fn add_variable(&mut self, name: String, _offset: usize) -> usize {
        // Use current frame_size as the index
        let index = self.frame_size;
        self.current_scope.bindings.insert(name.clone(), index);
        self.frame_size += 1;
        eprintln!("DEBUG: add_variable '{}' at index {}", name, index);
        index
    }

    /// Debug: print all bindings in current scope chain
    pub fn debug_print_scopes(&self) {
        eprintln!("DEBUG: Scope chain:");
        let mut current = Some(&self.current_scope);
        let mut depth = 0;
        while let Some(scope) = current {
            eprintln!(
                "  Depth {}: {:?}",
                depth,
                scope.bindings.keys().collect::<Vec<_>>()
            );
            current = scope.parent.as_ref().map(|b| b.as_ref());
            depth += 1;
        }
    }
}
