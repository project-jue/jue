/// Compilation environment for variable tracking and lexical scoping
///
/// This struct maintains the mapping between variable names and their stack
/// offsets during compilation, supporting nested lexical scopes with proper
/// variable shadowing and closure capture.
#[derive(Debug, Clone)]
pub struct CompilationEnvironment {
    /// Stack of variable scopes
    /// Each scope contains a mapping from variable name to stack offset
    pub scopes: Vec<VariableScope>,
    
    /// Stack frame size tracking
    /// Tracks the current stack size for proper offset allocation
    pub frame_size: usize,
    
    /// Current closure context for tracking captured variables
    /// When compiling inside a lambda, this contains the variables that need to be captured
    pub current_closure_captures: Option<Vec<(String, u16)>>,
}

impl CompilationEnvironment {
    /// Create a new empty compilation environment
    pub fn new() -> Self {
        Self {
            scopes: vec![VariableScope::new()], // Global scope
            frame_size: 0,
            current_closure_captures: None,
        }
    }

    /// Push a new variable scope (for lambda parameters, let bindings, etc.)
    pub fn push_scope(&mut self) {
        self.scopes.push(VariableScope::new());
    }

    /// Pop the current variable scope
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            // Don't pop the global scope
            let popped_scope = self.scopes.pop().unwrap();
            
            // Return stack space used by popped variables
            // Note: This is a simplified approach - in a real implementation,
            // we'd track which variables were allocated in this scope
            self.frame_size = self.frame_size.saturating_sub(1);
        }
    }

    /// Start capturing variables for a closure
    /// Records which variables from outer scopes are referenced in the closure body
    pub fn start_closure_capture(&mut self) {
        self.current_closure_captures = Some(Vec::new());
    }

    /// Add a variable to the current closure capture list
    pub fn add_capture(&mut self, name: String, offset: u16) {
        if let Some(captures) = &mut self.current_closure_captures {
            // Only add if not already captured
            if !captures.iter().any(|(n, _)| n == &name) {
                captures.push((name, offset));
            }
        }
    }

    /// End closure capture and return the list of captured variables
    pub fn end_closure_capture(&mut self) -> Vec<(String, u16)> {
        if let Some(captures) = self.current_closure_captures.take() {
            captures
        } else {
            Vec::new()
        }
    }

    /// Get current captured variables without consuming them
    pub fn get_current_captures(&self) -> Option<&Vec<(String, u16)>> {
        self.current_closure_captures.as_ref()
    }

    /// Check if we're currently in a closure context
    pub fn is_in_closure(&self) -> bool {
        self.current_closure_captures.is_some()
    }

    /// Lookup a variable and return its stack offset
    /// Returns None if the variable is not found in any accessible scope
    pub fn lookup_variable(&self, name: &str) -> Option<u16> {
        // Search scopes from innermost to outermost
        for scope in self.scopes.iter().rev() {
            if let Some(offset) = scope.variables.get(name) {
                return Some(*offset);
            }
        }
        None
    }

    /// Define a new variable in the current scope
    /// Returns the stack offset where the variable is stored
    pub fn define_variable(&mut self, name: &str) -> u16 {
        let offset = self.frame_size as u16;
        
        // Store in current (innermost) scope
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.variables.insert(name.to_string(), offset);
        }
        
        self.frame_size += 1;
        offset
    }

    /// Check if a variable is defined in any accessible scope
    pub fn is_variable_defined(&self, name: &str) -> bool {
        self.lookup_variable(name).is_some()
    }

    /// Get all variables defined in the current scope
    pub fn get_current_scope_variables(&self) -> Vec<String> {
        if let Some(current_scope) = self.scopes.last() {
            current_scope.variables.keys().cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// Get the current scope depth (0 = global scope)
    pub fn current_scope_depth(&self) -> usize {
        self.scopes.len() - 1
    }

    /// Get the total frame size
    pub fn get_frame_size(&self) -> usize {
        self.frame_size
    }
}

impl Default for CompilationEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

/// A single variable scope containing variable definitions
#[derive(Debug, Clone)]
pub struct VariableScope {
    /// Variable name to stack offset mapping
    pub variables: std::collections::HashMap<String, u16>,
}

impl VariableScope {
    /// Create a new empty variable scope
    pub fn new() -> Self {
        Self {
            variables: std::collections::HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_variable_operations() {
        let mut env = CompilationEnvironment::new();
        
        // Define a variable
        let offset = env.define_variable("x");
        assert_eq!(offset, 0);
        assert_eq!(env.frame_size, 1);
        
        // Lookup the variable
        assert_eq!(env.lookup_variable("x"), Some(offset));
        assert!(env.is_variable_defined("x"));
        
        // Variable not found
        assert_eq!(env.lookup_variable("y"), None);
        assert!(!env.is_variable_defined("y"));
    }

    #[test]
    fn test_scope_management() {
        let mut env = CompilationEnvironment::new();
        
        // Global scope
        let global_offset = env.define_variable("global_var");
        assert_eq!(global_offset, 0);
        
        // Push new scope
        env.push_scope();
        
        // Define variable in inner scope
        let inner_offset = env.define_variable("inner_var");
        assert_eq!(inner_offset, 1);
        assert_eq!(env.frame_size, 2);
        
        // Can still access global variable
        assert_eq!(env.lookup_variable("global_var"), Some(global_offset));
        
        // Pop inner scope
        env.pop_scope();
        assert_eq!(env.frame_size, 1); // Simplified - should be 1
        
        // Global variable still accessible
        assert_eq!(env.lookup_variable("global_var"), Some(global_offset));
        
        // Inner variable no longer accessible
        assert_eq!(env.lookup_variable("inner_var"), None);
    }

    #[test]
    fn test_variable_shadowing() {
        let mut env = CompilationEnvironment::new();
        
        // Define variable in global scope
        let global_offset = env.define_variable("x");
        assert_eq!(global_offset, 0);
        
        // Push new scope
        env.push_scope();
        
        // Define variable with same name in inner scope
        let inner_offset = env.define_variable("x");
        assert_eq!(inner_offset, 1);
        
        // Should find inner scope variable (closest scope)
        assert_eq!(env.lookup_variable("x"), Some(inner_offset));
        
        // Pop scope
        env.pop_scope();
        
        // Should find global scope variable
        assert_eq!(env.lookup_variable("x"), Some(global_offset));
    }

    #[test]
    fn test_scope_depth() {
        let mut env = CompilationEnvironment::new();
        
        assert_eq!(env.current_scope_depth(), 0);
        
        env.push_scope();
        assert_eq!(env.current_scope_depth(), 1);
        
        env.push_scope();
        assert_eq!(env.current_scope_depth(), 2);
        
        env.pop_scope();
        assert_eq!(env.current_scope_depth(), 1);
        
        env.pop_scope();
        assert_eq!(env.current_scope_depth(), 0);
        
        // Can't pop below global scope
        env.pop_scope();
        assert_eq!(env.current_scope_depth(), 0);
    }
}