use crate::error::{CompilationError, SourceMap};
use crate::trust_tier::TrustTier;
use core_world::core_expr::CoreExpr;
use core_world::proof_checker::Proof;
use physics_world::types::{Capability, OpCode};
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

/// Main compilation pipeline for Jue-World V2.0
///
/// This is the primary entry point for compiling Jue source code into executable bytecode.
/// The compilation process adapts based on the trust tier, providing different levels of
/// verification and safety guarantees.
///
/// # Trust Tier Processing
/// - **Formal/Verified**: Compiles through Core-World with mathematical proof verification
/// - **Empirical/Experimental**: Compiles directly to Physics-World with capability checking
///
/// # Parameters
/// * `source`: Jue source code to compile
/// * `tier`: Trust level determining verification approach and safety measures
/// * `default_step_limit`: Maximum execution steps for safety
/// * `default_mem_limit`: Maximum memory usage for safety
///
/// # Returns
/// Complete compilation result including bytecode, proofs, and audit information
///
/// # Errors
/// Returns `CompilationError` if parsing, capability analysis, or compilation fails
pub fn compile(
    source: &str,
    tier: TrustTier,
    default_step_limit: u64,
    default_mem_limit: usize,
) -> Result<CompilationResult, CompilationError> {
    // 1. Parse source to AST
    let ast = crate::parser::parse(source)?;

    // 2. Expand macros (with capability checking)
    let expanded_ast = crate::macro_system::expand_macros(ast, tier)?;

    // 3. Analyze capability requirements
    let required_caps = super::capability_analysis::analyze_capabilities(&expanded_ast)?;

    // 4. Verify tier allows required capabilities
    super::capability_analysis::validate_tier_capabilities(tier, &required_caps)?;

    // 5. Based on tier, choose compilation path
    let result = match tier {
        TrustTier::Formal | TrustTier::Verified => {
            compile_to_core_and_verify(expanded_ast, tier, default_step_limit, default_mem_limit)
        }
        TrustTier::Empirical | TrustTier::Experimental => compile_to_physics_with_checks(
            expanded_ast,
            tier,
            default_step_limit,
            default_mem_limit,
        ),
    }?;

    // 6. Package results with capability audit trail
    Ok(result)
}
/// Compilation result containing all outputs from the compilation process
///
/// This struct contains all information generated during compilation,
/// including bytecode, proofs, capability information, and audit trails.
/// Different fields are populated based on the trust tier of the compilation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationResult {
    /// Generated bytecode for Physics-World execution
    /// Instructions that will be executed by the virtual machine
    pub bytecode: Vec<OpCode>,

    /// Constants used by the bytecode
    /// Literal values referenced by the generated instructions
    pub constants: Vec<physics_world::types::Value>,

    /// Maximum execution steps allowed
    /// Safety limit to prevent infinite loops
    pub step_limit: u64,

    /// Maximum memory usage allowed
    /// Safety limit to prevent memory exhaustion
    pub memory_limit: usize,

    /// Formal path results (for Formal/Verified tiers)
    /// Mathematical proof of correctness, if available
    pub core_proof: Option<Proof>,

    /// Core expression for formal verification
    /// Lambda calculus representation for proof checking
    pub core_expr: Option<CoreExpr>,

    /// Capabilities required by the compiled code
    /// Operations that the code needs permission to perform
    pub required_capabilities: Vec<Capability>,

    /// Capabilities granted by the trust tier
    /// Operations permitted by the current trust level
    pub granted_capabilities: Vec<Capability>,

    /// Empirical validation results
    /// Test results for empirical trust tiers
    pub empirical_check: EmpiricalResult,

    /// Whether execution is sandboxed
    /// Additional isolation for experimental code
    pub sandboxed: bool,

    /// Source mapping for debugging
    /// Maps bytecode positions back to source locations
    pub source_map: SourceMap,

    /// Audit trail of capability checks performed
    /// Record of security validations during compilation
    pub capability_audit: Vec<super::capability_checking::CapabilityCheck>,
}

/// Empirical validation result
///
/// Represents the outcome of empirical testing for code compiled
/// at Empirical or Experimental trust tiers. Empirical validation
/// involves running tests to verify code behavior rather than
/// formal mathematical proofs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmpiricalResult {
    /// Empirical tests passed successfully
    /// Indicates the code behaves correctly according to test cases
    Passed {
        /// Number of tests that were executed
        tests_run: usize,

        /// Percentage of code paths covered by tests
        /// Ranges from 0.0 to 100.0, indicating test comprehensiveness
        coverage: f64,
    },

    /// Empirical tests failed
    /// Indicates potential issues with the compiled code
    Failed {
        /// Human-readable explanation of why tests failed
        reason: String,

        /// Specific test case that revealed the failure
        /// Helps developers understand and reproduce the issue
        failing_case: String,
    },

    /// No empirical validation was performed
    /// Used for Formal/Verified tiers that rely on proofs instead
    NotApplicable,
}

/// Compile to Core-World for Formal/Verified tiers
///
/// This function compiles Jue AST through the formal Core-World layer,
/// providing mathematical guarantees about code correctness. Used for
/// Formal and Verified trust tiers that require proof-based validation.
///
/// # Parameters
/// * `ast`: Abstract Syntax Tree to compile
/// * `tier`: Trust tier (Formal or Verified)
/// * `step_limit`: Maximum execution steps for safety
/// * `mem_limit`: Maximum memory usage for safety
///
/// # Returns
/// Complete compilation result with formal proofs and verified bytecode
///
/// # Process
/// 1. Convert AST to CoreExpr with proof obligations
/// 2. Generate comprehensive mathematical proofs
/// 3. Compile verified CoreExpr to executable bytecode
fn compile_to_core_and_verify(
    ast: crate::ast::AstNode,
    tier: TrustTier,
    step_limit: u64,
    mem_limit: usize,
) -> Result<CompilationResult, CompilationError> {
    // 1. Compile AST to CoreExpr with proof obligations
    let (core_expr, core_proof) =
        super::core_compilation::compile_ast_to_core_expr_with_proofs(&ast)?;

    // 2. Generate comprehensive proof if we don't already have one
    let final_proof = if core_proof.is_some() {
        core_proof
    } else {
        super::core_compilation::generate_comprehensive_proof(&core_expr)
    };

    // 3. Generate verified bytecode from CoreExpr
    let (bytecode, constants) = super::core_compilation::compile_core_expr_to_bytecode(&core_expr);

    Ok(CompilationResult {
        bytecode,
        constants,
        step_limit,
        memory_limit: mem_limit,
        core_proof: final_proof,
        core_expr: Some(core_expr),
        required_capabilities: Vec::new(),
        granted_capabilities: tier.granted_capabilities().into_iter().collect(),
        empirical_check: EmpiricalResult::NotApplicable,
        sandboxed: false,
        source_map: SourceMap::new(),
        capability_audit: Vec::new(),
    })
}

/// Compile to Physics-World for Empirical/Experimental tiers
///
/// This function compiles Jue AST directly to Physics-World bytecode,
/// bypassing formal verification for faster execution. Used for Empirical
/// and Experimental trust tiers that prioritize performance over proofs.
///
/// # Parameters
/// * `ast`: Abstract Syntax Tree to compile
/// * `tier`: Trust tier (Empirical or Experimental)
/// * `step_limit`: Maximum execution steps for safety
/// * `mem_limit`: Maximum memory usage for safety
///
/// # Returns
/// Complete compilation result with capability checking and audit trails
///
/// # Process
/// 1. Compile AST directly to bytecode for performance
/// 2. Insert runtime capability checks for security
/// 3. Analyze and track required capabilities
/// 4. Apply sandboxing for Experimental tier code
fn compile_to_physics_with_checks(
    ast: crate::ast::AstNode,
    tier: TrustTier,
    step_limit: u64,
    mem_limit: usize,
) -> Result<CompilationResult, CompilationError> {
    // 1. Compile AST directly to bytecode
    let (bytecode, constants) = super::bytecode_generation::compile_ast_to_bytecode(&ast)?;

    // 2. Insert runtime capability checks if needed
    let (bytecode, capability_audit) =
        super::capability_checking::insert_capability_checks(bytecode, &ast, tier);

    // 3. Analyze required capabilities for audit trail
    let required_capabilities = super::capability_analysis::analyze_capabilities(&ast)?;

    Ok(CompilationResult {
        bytecode,
        constants,
        step_limit,
        memory_limit: mem_limit,
        core_proof: None,
        core_expr: None,
        required_capabilities,
        granted_capabilities: tier.granted_capabilities().into_iter().collect(),
        empirical_check: EmpiricalResult::NotApplicable,
        sandboxed: tier == TrustTier::Experimental,
        source_map: SourceMap::new(),
        capability_audit,
    })
}
