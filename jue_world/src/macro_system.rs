use crate::ast::AstNode;
use crate::error::{CompilationError, SourceLocation};
use crate::trust_tier::TrustTier;
use physics_world::types::Capability;
use serde::{Deserialize, Serialize};
/// Macro system with capability requirements for Jue-World V2.0
use std::collections::HashSet;

/// Macro definition with capability requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MacroDefinition {
    /// Macro name
    pub name: String,

    /// Macro parameters
    pub parameters: Vec<String>,

    /// Macro body (AST)
    pub body: AstNode,

    /// Required capabilities for this macro
    pub required_capabilities: Vec<Capability>,

    /// Declared trust tier for this macro
    pub declared_tier: TrustTier,

    /// Whether this macro is hygienic
    pub is_hygienic: bool,

    /// Source location
    pub location: SourceLocation,
}

/// Macro context for expansion
#[derive(Debug, Clone)]
pub struct MacroContext {
    /// Caller's capabilities
    pub caller_capabilities: HashSet<Capability>,

    /// Expansion capabilities (intersection of macro and caller)
    pub expansion_capabilities: HashSet<Capability>,

    /// Current trust tier
    pub tier: TrustTier,

    /// Hygiene scope
    pub hygiene_scope: HygieneScope,

    /// Source location
    pub location: SourceLocation,
}

/// Hygiene scope for macro expansion
#[derive(Debug, Clone, Default)]
pub struct HygieneScope {
    /// Current hygiene level
    pub level: usize,

    /// Captured variables
    pub captured: Vec<String>,
}

impl HygieneScope {
    /// Create a new hygiene scope
    pub fn new() -> Self {
        Self::default()
    }

    /// Enter a new hygiene level
    pub fn enter(&mut self) {
        self.level += 1;
    }

    /// Exit current hygiene level
    pub fn exit(&mut self) {
        if self.level > 0 {
            self.level -= 1;
        }
    }

    /// Capture a variable
    pub fn capture(&mut self, name: String) {
        self.captured.push(name);
    }

    /// Check if a variable is captured
    pub fn is_captured(&self, name: &str) -> bool {
        self.captured.contains(&name.to_string())
    }
}

/// Macro expander
pub struct MacroExpander {
    /// Available macros
    pub macros: Vec<MacroDefinition>,

    /// Current context
    pub context: MacroContext,
}

impl MacroExpander {
    /// Create a new macro expander
    pub fn new(tier: TrustTier) -> Self {
        Self {
            macros: Vec::new(),
            context: MacroContext {
                caller_capabilities: tier.granted_capabilities(),
                expansion_capabilities: tier.granted_capabilities(),
                tier,
                hygiene_scope: HygieneScope::new(),
                location: SourceLocation::default(),
            },
        }
    }

    /// Add a macro definition
    pub fn add_macro(&mut self, macro_def: MacroDefinition) {
        self.macros.push(macro_def);
    }

    /// Find a macro by name
    pub fn find_macro(&self, name: &str) -> Option<&MacroDefinition> {
        self.macros.iter().find(|m| m.name == name)
    }

    /// Expand a macro call
    pub fn expand_macro(
        &mut self,
        name: &str,
        arguments: Vec<AstNode>,
        location: SourceLocation,
    ) -> Result<AstNode, CompilationError> {
        // Find the macro and clone it to avoid borrow conflicts
        let macro_def = self
            .find_macro(name)
            .ok_or_else(|| {
                CompilationError::MacroExpansionError(format!("Macro {} not found", name))
            })?
            .clone();

        // Check tier compatibility
        if !self.context.tier.is_at_least(&macro_def.declared_tier) {
            return Err(CompilationError::MacroExpansionError(format!(
                "Macro {} requires tier {:?}, but current tier is {:?}",
                name, macro_def.declared_tier, self.context.tier
            )));
        }

        // Calculate expansion capabilities (intersection)
        let macro_caps: HashSet<_> = macro_def.required_capabilities.iter().cloned().collect();
        let expansion_caps = self
            .context
            .caller_capabilities
            .intersection(&macro_caps)
            .cloned()
            .collect();

        // Update context
        self.context.expansion_capabilities = expansion_caps;
        self.context.location = location;

        // Check that we have all required capabilities
        for cap in &macro_def.required_capabilities {
            if !self.context.expansion_capabilities.contains(cap) {
                return Err(CompilationError::CapabilityError(
                    crate::error::CapabilityViolation {
                        required: cap.clone(),
                        tier: self.context.tier,
                        location: macro_def.location.clone(),
                        suggestion: format!(
                            "Macro {} requires capability {:?} but it's not available in the current context",
                            name, cap
                        ),
                    }
                ));
            }
        }

        // Create parameter bindings
        if macro_def.parameters.len() != arguments.len() {
            return Err(CompilationError::MacroExpansionError(format!(
                "Macro {} expects {} arguments, got {}",
                name,
                macro_def.parameters.len(),
                arguments.len()
            )));
        }

        let mut bindings = Vec::new();
        for (param, arg) in macro_def.parameters.iter().zip(arguments.iter()) {
            bindings.push((param.clone(), arg.clone()));
        }

        // Expand the macro body with bindings
        self.expand_with_bindings(&macro_def.body, &bindings)
    }

    /// Expand AST with variable bindings
    fn expand_with_bindings(
        &self,
        ast: &AstNode,
        bindings: &[(String, AstNode)],
    ) -> Result<AstNode, CompilationError> {
        match ast {
            AstNode::Variable(name) => {
                // Check if this variable is bound
                if let Some(bound_value) = bindings.iter().find(|(n, _)| n == name) {
                    Ok(bound_value.1.clone())
                } else {
                    Ok(ast.clone())
                }
            }
            AstNode::Call {
                function,
                arguments,
                location,
            } => {
                let expanded_function = self.expand_with_bindings(function, bindings)?;
                let expanded_args = arguments
                    .iter()
                    .map(|arg| self.expand_with_bindings(arg, bindings))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(AstNode::Call {
                    function: Box::new(expanded_function),
                    arguments: expanded_args,
                    location: location.clone(),
                })
            }
            // Handle other AST nodes similarly...
            _ => Ok(ast.clone()),
        }
    }

    /// Expand all macros in an AST
    pub fn expand_all(&mut self, ast: AstNode) -> Result<AstNode, CompilationError> {
        match ast {
            AstNode::MacroExpansion {
                name,
                arguments,
                location,
            } => self.expand_macro(&name, arguments, location),
            AstNode::Call {
                function,
                arguments,
                location,
            } => {
                let expanded_function = self.expand_all(*function.clone())?;
                let expanded_args = arguments
                    .into_iter()
                    .map(|arg| self.expand_all(arg))
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(AstNode::Call {
                    function: Box::new(expanded_function),
                    arguments: expanded_args,
                    location: location.clone(),
                })
            }
            // Handle other AST nodes...
            _ => Ok(ast),
        }
    }
}

/// Macro expansion with capability checking
pub fn expand_macros(ast: AstNode, tier: TrustTier) -> Result<AstNode, CompilationError> {
    let mut expander = MacroExpander::new(tier);
    expander.expand_all(ast)
}

#[cfg(test)]
#[path = "test/macro_system.rs"]
mod tests;
