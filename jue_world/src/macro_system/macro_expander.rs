/// Macro expander for Jue-World V2.0
///
/// This module handles hygienic macro expansion with explicit capture escapes.
use crate::error::{CapabilityViolation, CompilationError};
use crate::shared::ast::AstNode;
use crate::shared::trust_tier::TrustTier;
use physics_world::types::Capability;
use std::collections::HashMap;
/// Macro definition
#[derive(Debug, Clone)]
pub struct MacroDefinition {
    /// Macro name
    pub name: String,
    /// Macro parameters
    pub parameters: Vec<String>,
    /// Macro body
    pub body: AstNode,
    /// Trust tier requirement
    pub trust_tier: TrustTier,
}

/// Macro expansion context
pub struct MacroExpansionContext {
    /// Defined macros
    pub macros: HashMap<String, MacroDefinition>,
    /// Current trust tier
    pub trust_tier: TrustTier,
}

/// Create a new macro expansion context
pub fn create_macro_expansion_context(trust_tier: TrustTier) -> MacroExpansionContext {
    MacroExpansionContext {
        macros: HashMap::new(),
        trust_tier,
    }
}

/// Define a new macro
pub fn define_macro(
    context: &mut MacroExpansionContext,
    name: String,
    parameters: Vec<String>,
    body: AstNode,
    trust_tier: TrustTier,
) -> Result<(), CompilationError> {
    // Check if the macro trust tier is compatible with current context
    if !context.trust_tier.is_at_least(&trust_tier) {
        return Err(CompilationError::CapabilityError(CapabilityViolation {
            required: Capability::MacroHygienic,
            tier: context.trust_tier,
            location: Default::default(),
            suggestion: format!(
                "Macro requires trust tier {:?} but current tier is {:?}",
                trust_tier, context.trust_tier
            ),
        }));
    }

    context.macros.insert(
        name.clone(),
        MacroDefinition {
            name,
            parameters,
            body,
            trust_tier,
        },
    );

    Ok(())
}

/// Expand a macro call
pub fn expand_macro(
    context: &MacroExpansionContext,
    macro_name: &str,
    arguments: Vec<AstNode>,
) -> Result<AstNode, CompilationError> {
    // Find the macro definition
    let macro_def = context
        .macros
        .get(macro_name)
        .ok_or_else(|| CompilationError::ParseError {
            message: format!("Macro {} not found", macro_name),
            location: Default::default(),
        })?;

    // Check parameter count
    if macro_def.parameters.len() != arguments.len() {
        return Err(CompilationError::ParseError {
            message: format!(
                "Macro {} expects {} arguments but got {}",
                macro_name,
                macro_def.parameters.len(),
                arguments.len()
            ),
            location: Default::default(),
        });
    }

    // Create parameter substitution mapping
    let mut substitutions = HashMap::new();
    for (param, arg) in macro_def.parameters.iter().zip(arguments.iter()) {
        substitutions.insert(param.clone(), arg.clone());
    }

    // Perform substitution in the macro body
    substitute_variables(&macro_def.body, &substitutions)
}

/// Substitute variables in an AST node
fn substitute_variables(
    node: &AstNode,
    substitutions: &HashMap<String, AstNode>,
) -> Result<AstNode, CompilationError> {
    match node {
        AstNode::Variable(name) => {
            if let Some(replacement) = substitutions.get(name) {
                Ok(replacement.clone())
            } else {
                Ok(node.clone())
            }
        }
        AstNode::Lambda {
            parameters,
            body,
            location,
        } => {
            let new_body = substitute_variables(body, substitutions)?;
            Ok(AstNode::Lambda {
                parameters: parameters.clone(),
                body: Box::new(new_body),
                location: location.clone(),
            })
        }
        AstNode::Call {
            function,
            arguments,
            location,
        } => {
            let new_function = substitute_variables(function, substitutions)?;
            let new_arguments = arguments
                .iter()
                .map(|arg| substitute_variables(arg, substitutions))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(AstNode::Call {
                function: Box::new(new_function),
                arguments: new_arguments,
                location: location.clone(),
            })
        }
        // Handle other AST node types
        _ => Ok(node.clone()),
    }
}

/// Expand all macros in an AST node
pub fn expand_macros(
    node: &AstNode,
    context: &MacroExpansionContext,
) -> Result<AstNode, CompilationError> {
    match node {
        AstNode::MacroExpansion {
            name: macro_name,
            arguments,
            location: _,
        } => {
            let expanded = expand_macro(context, &macro_name, arguments.clone())?;
            expand_macros(&expanded, context)
        }
        AstNode::Lambda {
            parameters,
            body,
            location,
        } => {
            let new_body = expand_macros(body, context)?;
            Ok(AstNode::Lambda {
                parameters: parameters.clone(),
                body: Box::new(new_body),
                location: location.clone(),
            })
        }
        AstNode::Call {
            function,
            arguments,
            location,
        } => {
            let new_function = expand_macros(function, context)?;
            let new_arguments = arguments
                .iter()
                .map(|arg| expand_macros(arg, context))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(AstNode::Call {
                function: Box::new(new_function),
                arguments: new_arguments,
                location: location.clone(),
            })
        }
        // Handle other AST node types
        _ => Ok(node.clone()),
    }
}
