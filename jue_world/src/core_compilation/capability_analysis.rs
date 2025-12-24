use crate::error::{CapabilityViolation, CompilationError};
/// Capability analysis for Jue-World V2.0
///
/// This module analyzes AST expressions to determine required capabilities
/// and validates that the trust tier provides sufficient capabilities.
use crate::shared::ast::AstNode;
use crate::shared::trust_tier::TrustTier;
use physics_world::types::Capability;
use std::collections::HashSet;

/// Convert string capability name to Capability enum
fn string_to_capability(cap_str: &str) -> Option<Capability> {
    match cap_str {
        "MetaSelfModify" => Some(Capability::MetaSelfModify),
        "MetaGrant" => Some(Capability::MetaGrant),
        "MacroHygienic" => Some(Capability::MacroHygienic),
        "MacroUnsafe" => Some(Capability::MacroUnsafe),
        "ComptimeEval" => Some(Capability::ComptimeEval),
        "IoReadSensor" => Some(Capability::IoReadSensor),
        "IoWriteActuator" => Some(Capability::IoWriteActuator),
        "IoNetwork" => Some(Capability::IoNetwork),
        "IoPersist" => Some(Capability::IoPersist),
        "SysCreateActor" => Some(Capability::SysCreateActor),
        "SysTerminateActor" => Some(Capability::SysTerminateActor),
        "SysClock" => Some(Capability::SysClock),
        _ => None,
    }
}

/// Analyze capabilities required by an AST expression
pub fn analyze_capabilities(ast: &AstNode) -> Result<HashSet<Capability>, CompilationError> {
    let mut required_caps = HashSet::new();
    analyze_expression(ast, &mut required_caps);
    Ok(required_caps)
}

/// Validate that the trust tier provides required capabilities
pub fn validate_tier_capabilities(
    tier: TrustTier,
    required_caps: &HashSet<Capability>,
) -> Result<(), CompilationError> {
    let granted_caps = tier.granted_capabilities();

    for cap in required_caps {
        if !granted_caps.contains(cap) {
            return Err(CompilationError::CapabilityError(CapabilityViolation {
                required: cap.clone(),
                tier,
                location: Default::default(),
                suggestion: format!(
                    "Trust tier {:?} does not grant required capability {:?}",
                    tier, cap
                ),
            }));
        }
    }

    Ok(())
}

/// Recursively analyze expressions for capability requirements
fn analyze_expression(ast: &AstNode, required_caps: &mut HashSet<Capability>) {
    match ast {
        AstNode::FfiCall { arguments, .. } => {
            // FFI calls may require capabilities based on the function being called
            // For now, we'll analyze the arguments
            for arg in arguments {
                analyze_expression(arg, required_caps);
            }
        }
        AstNode::RequireCapability { capability, .. } => {
            if let Some(cap) = string_to_capability(capability) {
                required_caps.insert(cap);
            }
        }
        AstNode::HasCapability { capability, .. } => {
            if let Some(cap) = string_to_capability(capability) {
                required_caps.insert(cap);
            }
        }
        AstNode::Call {
            function,
            arguments,
            ..
        } => {
            analyze_expression(function, required_caps);
            for arg in arguments {
                analyze_expression(arg, required_caps);
            }
        }
        AstNode::Lambda { body, .. } => {
            analyze_expression(body, required_caps);
        }
        AstNode::Let { bindings, body, .. } => {
            for (_, expr) in bindings {
                analyze_expression(expr, required_caps);
            }
            analyze_expression(body, required_caps);
        }
        AstNode::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            analyze_expression(condition, required_caps);
            analyze_expression(then_branch, required_caps);
            analyze_expression(else_branch, required_caps);
        }
        AstNode::List { elements, .. } => {
            for elem in elements {
                analyze_expression(elem, required_caps);
            }
        }
        AstNode::Cons { car, cdr, .. } => {
            analyze_expression(car, required_caps);
            analyze_expression(cdr, required_caps);
        }
        AstNode::MacroExpansion { arguments, .. } => {
            for arg in arguments {
                analyze_expression(arg, required_caps);
            }
        }
        _ => {
            // Other expression types don't require special capabilities
        }
    }
}
