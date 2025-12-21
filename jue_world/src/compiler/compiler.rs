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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscapeAnalysis {
    pub escaping_vars: HashSet<usize>,
    pub current_function: Option<FunctionId>,
    pub variable_environments: Vec<HashSet<usize>>,
    pub function_info: HashMap<FunctionId, FunctionInfo>,
}

/// Function identifier for escape analysis
#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionId(pub usize);

/// Function information for escape analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub local_count: usize,
    pub escape_info: HashMap<usize, EscapeStatus>,
    pub free_variables: Vec<usize>,
}

/// Escape status for variables
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EscapeStatus {
    Escaping,
    NonEscaping,
}

/// Analysis context for escape analysis
#[derive(Debug, Clone)]
pub struct AnalysisContext {
    pub current_function: Option<FunctionId>,
    pub max_variable: usize,
    pub is_captured: bool,
    pub errors: Vec<CompilationError>,
}

impl AnalysisContext {
    pub fn new() -> Self {
        Self {
            current_function: None,
            max_variable: 0,
            is_captured: false,
            errors: Vec::new(),
        }
    }

    pub fn report_error(&mut self, error: CompilationError) {
        self.errors.push(error);
    }

    pub fn is_captured(&self) -> bool {
        self.is_captured
    }
}

impl EscapeAnalysis {
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

    fn analyze_default(&mut self, _expr: &crate::ast::AstNode, _context: &mut AnalysisContext) {
        // Default analysis for expressions we don't specifically handle
        // In a real implementation, this would analyze the expression structure
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

                // Analyze body with parameters in scope
                let mut new_environments = self.variable_environments.clone();
                new_environments.push(param_vars);
                let mut temp_analysis = EscapeAnalysis {
                    variable_environments: new_environments,
                    ..Default::default()
                };
                temp_analysis.find_free_variables_recursive(body, free_vars);
            }
            crate::ast::AstNode::Let { bindings, body, .. } => {
                // Add let bindings to scope
                let binding_vars: HashSet<usize> = bindings
                    .iter()
                    .map(|(name, _)| self.get_variable_index(name))
                    .collect();

                // Analyze bindings and body with bindings in scope
                let mut new_environments = self.variable_environments.clone();
                new_environments.push(binding_vars);
                let mut temp_analysis = EscapeAnalysis {
                    variable_environments: new_environments,
                    ..Default::default()
                };

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

/// Compilation result containing all outputs from the compilation process
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilationResult {
    /// Generated bytecode for Physics-World execution
    pub bytecode: Vec<OpCode>,
    /// Constants used by the bytecode
    pub constants: Vec<physics_world::types::Value>,
    /// Maximum execution steps allowed
    pub step_limit: u64,
    /// Maximum memory usage allowed
    pub memory_limit: usize,

    /// Formal path results (for Formal/Verified tiers)
    pub core_proof: Option<Proof>,
    /// Core expression for formal verification
    pub core_expr: Option<CoreExpr>,

    /// Capability information
    pub required_capabilities: Vec<Capability>,
    /// Capabilities granted by the trust tier
    pub granted_capabilities: Vec<Capability>,

    /// Empirical path results
    pub empirical_check: EmpiricalResult,
    /// Whether execution is sandboxed
    pub sandboxed: bool,

    /// Debug information
    pub source_map: SourceMap,
    /// Capability audit trail
    pub capability_audit: Vec<super::capability_checking::CapabilityCheck>,
}

/// Empirical validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmpiricalResult {
    /// Tests passed with coverage information
    Passed {
        /// Number of tests run
        tests_run: usize,
        /// Code coverage percentage
        coverage: f64,
    },
    /// Tests failed with reason
    Failed {
        /// Failure reason
        reason: String,
        /// Failing test case
        failing_case: String,
    },
    /// No empirical validation performed
    NotApplicable,
}

/// Main compilation function - the public API for Jue-World
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

/// Compile to Core-World for Formal/Verified tiers
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
