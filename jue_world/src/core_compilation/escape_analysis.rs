use crate::error::{CompilationError, SourceMap};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

/// Escape analysis information for functions
///
/// This struct tracks variable escape analysis across function boundaries.
/// Variables that escape the scope where they were defined need heap allocation
/// rather than stack allocation, which affects memory management and performance.
///
/// # Key Concepts
/// - **Escaping variables**: Variables referenced outside their defining scope
/// - **Variable environments**: Scopes containing variable definitions
/// - **Function boundaries**: Points where variables may escape to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscapeAnalysis {
    /// Set of variable indices that escape their defining scope
    /// These variables must be allocated on the heap rather than stack
    pub escaping_vars: HashSet<usize>,

    /// Currently analyzed function identifier, if any
    /// Used to track which function context we're analyzing
    pub current_function: Option<FunctionId>,

    /// Stack of variable environments representing lexical scoping
    /// Each element represents variables available in a specific scope
    pub variable_environments: Vec<HashSet<usize>>,

    /// Pre-computed escape information for all analyzed functions
    /// Maps function IDs to their detailed escape analysis results
    pub function_info: HashMap<FunctionId, FunctionInfo>,
}

/// Function identifier for escape analysis
///
/// A simple wrapper around a usize that uniquely identifies functions
/// during escape analysis. The actual value is implementation-specific
/// and typically derived from source position or compilation order.
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionId(pub usize);

/// Function information for escape analysis
///
/// Contains detailed escape analysis results for a specific function,
/// including which variables escape and which don't. This information
/// is used for memory allocation decisions during code generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// Number of local variables in this function
    /// Used for stack frame size calculations
    pub local_count: usize,

    /// Escape status for each variable index
    /// Maps variable indices to their escape behavior
    pub escape_info: HashMap<usize, EscapeStatus>,

    /// List of free variables that escape this function
    /// These variables must be captured in closures
    pub free_variables: Vec<usize>,
}

/// Escape status for variables
///
/// Represents whether a variable escapes its defining scope or not.
/// Variables that escape must be allocated on the heap and captured
/// in closures, while non-escaping variables can use stack allocation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EscapeStatus {
    /// Variable escapes its defining scope
    /// Must be heap-allocated and captured in closures
    Escaping,
    /// Variable does not escape its defining scope
    /// Can be stack-allocated for better performance
    NonEscaping,
}

/// Analysis context for escape analysis
///
/// Maintains state during escape analysis of expressions.
/// Tracks the current function context, variable tracking, and error reporting.
#[derive(Debug, Clone)]
pub struct AnalysisContext {
    /// Currently analyzed function identifier, if any
    /// Used to determine which function's scope we're analyzing
    pub current_function: Option<FunctionId>,

    /// Maximum variable index seen so far
    /// Used for tracking variable allocation ranges
    pub max_variable: usize,

    /// Whether the current expression is captured in a closure
    /// Variables in captured contexts may need heap allocation
    pub is_captured: bool,

    /// Compilation errors encountered during analysis
    /// Collected for reporting after analysis completes
    pub errors: Vec<CompilationError>,
}

impl AnalysisContext {
    /// Creates a new analysis context for escape analysis
    ///
    /// Initializes an empty context with no current function,
    /// zero maximum variable index, no captured state, and empty error list.
    ///
    /// # Returns
    /// A new `AnalysisContext` instance ready for escape analysis
    pub fn new() -> Self {
        Self {
            current_function: None,
            max_variable: 0,
            is_captured: false,
            errors: Vec::new(),
        }
    }

    /// Reports a compilation error during analysis
    ///
    /// Adds the given error to the context's error collection for later reporting.
    /// This allows collecting multiple errors during analysis rather than failing fast.
    ///
    /// # Parameters
    /// * `error`: The compilation error to report
    pub fn report_error(&mut self, error: CompilationError) {
        self.errors.push(error);
    }

    /// Checks whether the current expression is captured in a closure
    ///
    /// Returns whether the expression being analyzed is within a captured context.
    /// Variables in captured contexts may need heap allocation rather than stack allocation.
    ///
    /// # Returns
    /// `true` if the expression is captured, `false` otherwise
    pub fn is_captured(&self) -> bool {
        self.is_captured
    }
}

impl EscapeAnalysis {
    /// Creates a new escape analysis context
    ///
    /// Initializes an empty escape analysis with no escaping variables,
    /// no current function, empty variable environments, and no function info.
    ///
    /// # Returns
    /// A new `EscapeAnalysis` instance ready for analysis
    pub fn new() -> Self {
        Self {
            escaping_vars: HashSet::new(),
            current_function: None,
            variable_environments: Vec::new(),
            function_info: HashMap::new(),
        }
    }

    fn get_variable_index(&self, var_name: &str) -> usize {
        // Simple hash-based indexing for demonstration
        // In a real implementation, this would use a proper symbol table
        let mut hasher = DefaultHasher::new();
        var_name.hash(&mut hasher);
        hasher.finish() as usize
    }

    /// Analyzes an AST expression for escape analysis
    ///
    /// This method performs escape analysis on the given AST node, tracking
    /// which variables escape their defining scope and need heap allocation.
    /// The analysis follows the structure of the expression recursively.
    ///
    /// # Parameters
    /// * `expr`: The AST expression to analyze
    /// * `context`: The analysis context maintaining current scope and state
    ///
    /// # Behavior
    /// - **Variable**: Converts variable name to index and checks if it escapes
    /// - **Lambda**: Analyzes function parameters and body, tracking free variables
    /// - **Let**: Analyzes binding expressions and body with proper scoping
    /// - **Other**: Falls back to default analysis for unhandled expression types
    pub fn analyze_expression(
        &mut self,
        expr: &crate::ast::AstNode,
        context: &mut AnalysisContext,
    ) {
        match expr {
            crate::ast::AstNode::Variable(var) => {
                // Convert variable name to index (simplified for now)
                let var_index = self.get_variable_index(var);
                self.analyze_variable(var_index, context);
            }
            crate::ast::AstNode::Lambda {
                parameters, body, ..
            } => {
                // Convert parameter names to indices
                let param_indices: Vec<usize> = parameters
                    .iter()
                    .map(|param| self.get_variable_index(param))
                    .collect();
                self.analyze_lambda(&param_indices, body, context);
            }
            crate::ast::AstNode::Let { bindings, body, .. } => {
                // Convert binding names to indices
                let binding_indices: Vec<(usize, crate::ast::AstNode)> = bindings
                    .iter()
                    .map(|(name, expr)| (self.get_variable_index(name), expr.clone()))
                    .collect();
                self.analyze_let(&binding_indices, body, context);
            }
            crate::ast::AstNode::Call {
                function,
                arguments,
                ..
            } => {
                // Analyze function and arguments for escape
                self.analyze_expression(function, context);
                for arg in arguments {
                    self.analyze_expression(arg, context);
                }
            }
            crate::ast::AstNode::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                // Analyze all branches of the conditional
                self.analyze_expression(condition, context);
                self.analyze_expression(then_branch, context);
                self.analyze_expression(else_branch, context);
            }
            // Other expression types...
            _ => {
                // Default analysis for other expressions
                self.analyze_default(expr, context);
            }
        }
    }

    fn analyze_variable(&mut self, var: usize, context: &mut AnalysisContext) {
        // Check if this variable is defined in current or outer scopes
        for vars in self.variable_environments.iter().rev() {
            if vars.contains(&var) {
                // Variable is in scope - check if it's used in a way that requires escaping
                if context.is_captured() {
                    self.escaping_vars.insert(var);
                }
                return;
            }
        }

        // Variable not found - error condition
        context.report_error(CompilationError::ParseError {
            message: format!("Variable not found: {}", var),
            location: Default::default(),
        });
    }

    fn analyze_lambda(
        &mut self,
        params: &[usize],
        body: &crate::ast::AstNode,
        context: &mut AnalysisContext,
    ) {
        // Push new scope for lambda parameters
        let param_vars: HashSet<usize> = params.iter().cloned().collect();
        self.variable_environments.push(param_vars);

        // Analyze lambda body
        self.analyze_expression(body, context);

        // Pop parameter scope
        self.variable_environments.pop();

        // All free variables in this lambda escape
        let free_vars = self.find_free_variables(body);
        for var in &free_vars {
            self.escaping_vars.insert(*var);
        }

        // Store escape info for this function
        if let Some(ref func_id) = context.current_function {
            let mut escape_info = HashMap::new();
            for var in 0..context.max_variable {
                let status = if self.escaping_vars.contains(&var) {
                    EscapeStatus::Escaping
                } else {
                    EscapeStatus::NonEscaping
                };
                escape_info.insert(var, status);
            }
            self.function_info.insert(
                func_id.clone(),
                FunctionInfo {
                    local_count: params.len(),
                    escape_info,
                    free_variables: free_vars,
                },
            );
        }
    }

    fn analyze_let(
        &mut self,
        bindings: &[(usize, crate::ast::AstNode)],
        body: &crate::ast::AstNode,
        context: &mut AnalysisContext,
    ) {
        // Push new scope for let bindings
        let binding_vars: HashSet<usize> = bindings.iter().map(|(var, _)| *var).collect();
        self.variable_environments.push(binding_vars);

        // Analyze bindings and body
        for (_, expr) in bindings {
            self.analyze_expression(expr, context);
        }
        self.analyze_expression(body, context);

        // Pop binding scope
        self.variable_environments.pop();
    }

    fn analyze_default(&mut self, expr: &crate::ast::AstNode, context: &mut AnalysisContext) {
        // Default analysis for expressions we don't specifically handle
        // This handles primitive literals, symbols, and other basic expressions
        match expr {
            crate::ast::AstNode::Literal(_) => {
                // Literals don't introduce or reference variables
                // No analysis needed for escape behavior
            }
            crate::ast::AstNode::Symbol(_) => {
                // Symbols are typically resolved at compile time and don't escape
                // No escape analysis needed for symbols
            }
            crate::ast::AstNode::TrustTier { expression, .. } => {
                // Analyze the underlying expression with the trust tier
                self.analyze_expression(expression, context);
            }
            crate::ast::AstNode::RequireCapability { .. }
            | crate::ast::AstNode::HasCapability { .. } => {
                // Capability checks don't introduce variables that escape
                // No escape analysis needed
            }
            crate::ast::AstNode::TypeSignature { .. } => {
                // Type signatures are compile-time information, no runtime variables
                // No escape analysis needed
            }
            crate::ast::AstNode::MacroDefinition { body, .. } => {
                // Analyze macro body for escape behavior
                self.analyze_expression(body, context);
            }
            crate::ast::AstNode::MacroExpansion { arguments, .. } => {
                // Analyze macro arguments for escape behavior
                for arg in arguments {
                    self.analyze_expression(arg, context);
                }
            }
            crate::ast::AstNode::FfiCall { arguments, .. } => {
                // Analyze FFI call arguments for escape behavior
                for arg in arguments {
                    self.analyze_expression(arg, context);
                }
            }
            crate::ast::AstNode::List { elements, .. } => {
                // Analyze list elements for escape behavior
                for elem in elements {
                    self.analyze_expression(elem, context);
                }
            }
            crate::ast::AstNode::Cons { car, cdr, .. } => {
                // Analyze both parts of the cons cell for escape behavior
                self.analyze_expression(car, context);
                self.analyze_expression(cdr, context);
            }
            // For any other unhandled expression types, report an error
            _ => {
                context.report_error(CompilationError::ParseError {
                    message: format!("Unsupported expression type in escape analysis: {:?}", expr),
                    location: Default::default(),
                });
            }
        }
    }

    fn find_free_variables(&self, expr: &crate::ast::AstNode) -> Vec<usize> {
        let mut free_vars = Vec::new();
        self.find_free_variables_recursive(expr, &mut free_vars);
        free_vars
    }

    fn find_free_variables_recursive(
        &self,
        expr: &crate::ast::AstNode,
        free_vars: &mut Vec<usize>,
    ) {
        match expr {
            crate::ast::AstNode::Variable(var) => {
                // Convert variable name to index
                let var_index = self.get_variable_index(var);
                // Check if variable is bound in current scope
                let mut is_bound = false;
                for scope in &self.variable_environments {
                    if scope.contains(&var_index) {
                        is_bound = true;
                        break;
                    }
                }
                if !is_bound {
                    free_vars.push(var_index);
                }
            }
            crate::ast::AstNode::Lambda {
                parameters, body, ..
            } => {
                // Add parameters to scope
                let param_vars: HashSet<usize> = parameters
                    .iter()
                    .map(|param| self.get_variable_index(param))
                    .collect();

                // Create new environment with parameters added
                let mut new_environments = self.variable_environments.clone();
                new_environments.push(param_vars);

                // Create a temporary analysis with the new environment
                let temp_analysis = EscapeAnalysis {
                    variable_environments: new_environments,
                    ..Default::default()
                };

                // Analyze body with parameters in scope
                temp_analysis.find_free_variables_recursive(body, free_vars);
            }
            crate::ast::AstNode::Let { bindings, body, .. } => {
                // Add let bindings to scope
                let binding_vars: HashSet<usize> = bindings
                    .iter()
                    .map(|(name, _)| self.get_variable_index(name))
                    .collect();

                // Create new environment with bindings added
                let mut new_environments = self.variable_environments.clone();
                new_environments.push(binding_vars);

                // Create a temporary analysis with the new environment
                let temp_analysis = EscapeAnalysis {
                    variable_environments: new_environments,
                    ..Default::default()
                };

                // Analyze bindings and body with bindings in scope
                for (_, expr) in bindings {
                    temp_analysis.find_free_variables_recursive(expr, free_vars);
                }
                temp_analysis.find_free_variables_recursive(body, free_vars);
            }
            crate::ast::AstNode::Call {
                function,
                arguments,
                ..
            } => {
                self.find_free_variables_recursive(function, free_vars);
                for arg in arguments {
                    self.find_free_variables_recursive(arg, free_vars);
                }
            }
            // Other expression types would be handled here
            _ => {}
        }
    }
}

impl Default for EscapeAnalysis {
    fn default() -> Self {
        Self {
            escaping_vars: HashSet::new(),
            current_function: None,
            variable_environments: Vec::new(),
            function_info: HashMap::new(),
        }
    }
}
