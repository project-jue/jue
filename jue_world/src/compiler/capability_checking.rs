use crate::error::SourceLocation;
use crate::trust_tier::TrustTier;
use physics_world::types::{Capability, OpCode};
use serde::{Deserialize, Serialize};

/// Capability check information for audit trail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityCheck {
    /// Source location of the check
    pub location: SourceLocation,
    /// Capability being checked
    pub capability: Capability,
    /// Type of check performed
    pub check_type: CheckType,
}

impl CapabilityCheck {
    /// Create a new capability check
    ///
    /// # Arguments
    /// * `location` - Source location of the check
    /// * `capability` - Capability being checked
    /// * `check_type` - Type of check performed
    ///
    /// # Returns
    /// * `Self` - New CapabilityCheck instance
    pub fn new(location: SourceLocation, capability: Capability, check_type: CheckType) -> Self {
        Self {
            location,
            capability,
            check_type,
        }
    }
}

/// Type of capability check
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CheckType {
    /// Verified at compile time
    Static,
    /// Inserted opcode check
    Runtime,
    /// Covered by formal proof
    Proof,
}

/// Insert runtime capability checks into bytecode
pub fn insert_capability_checks(
    bytecode: Vec<OpCode>,
    ast: &crate::ast::AstNode,
    tier: TrustTier,
) -> (Vec<OpCode>, Vec<CapabilityCheck>) {
    let mut bytecode = bytecode;
    let mut capability_audit = Vec::new();

    // Check if this tier requires runtime checks
    if tier == TrustTier::Empirical || tier == TrustTier::Experimental {
        // Analyze the AST to find operations that require capability checks
        let required_checks = analyze_required_checks(ast);

        // Insert capability checks before privileged operations
        for (capability, location) in required_checks {
            // Add HasCap instruction to check for the capability
            let cap_index = 0; // For now, use index 0 - in real implementation this would be the capability's index in constants
            bytecode.insert(0, OpCode::HasCap(cap_index));

            // Add capability check to audit trail
            capability_audit.push(CapabilityCheck::new(
                location,
                capability,
                CheckType::Runtime,
            ));
        }
    }

    (bytecode, capability_audit)
}

/// Analyze the AST to find operations that require capability checks
fn analyze_required_checks(ast: &crate::ast::AstNode) -> Vec<(Capability, SourceLocation)> {
    let mut required_checks = Vec::new();

    match ast {
        crate::ast::AstNode::RequireCapability {
            capability,
            location,
        } => {
            // Parse capability from string
            if let Ok(cap) = parse_capability(capability) {
                required_checks.push((cap, location.clone()));
            }
        }
        crate::ast::AstNode::HasCapability {
            capability,
            location,
        } => {
            // Parse capability from string
            if let Ok(cap) = parse_capability(capability) {
                required_checks.push((cap, location.clone()));
            }
        }
        crate::ast::AstNode::FfiCall {
            function, location, ..
        } => {
            // FFI calls require specific capabilities based on function name
            if let Some(required_cap) =
                crate::compiler::capability_analysis::get_ffi_function_capability(function)
            {
                required_checks.push((required_cap, location.clone()));
            } else {
                // Default to MacroUnsafe if function not found
                required_checks.push((Capability::MacroUnsafe, location.clone()));
            }
        }
        crate::ast::AstNode::MacroDefinition { .. } => {
            // Macro definitions require MacroHygienic capability
            required_checks.push((Capability::MacroHygienic, SourceLocation::default()));
        }
        _ => {
            // Recursively check child nodes
            for child in get_child_nodes(ast) {
                required_checks.extend(analyze_required_checks(child));
            }
        }
    }

    required_checks
}

/// Helper function to parse capability from string
fn parse_capability(cap_str: &str) -> Result<Capability, ()> {
    match cap_str {
        "meta-self-modify" => Ok(Capability::MetaSelfModify),
        "meta-grant" => Ok(Capability::MetaGrant),
        "macro-hygienic" => Ok(Capability::MacroHygienic),
        "macro-unsafe" => Ok(Capability::MacroUnsafe),
        "comptime-eval" => Ok(Capability::ComptimeEval),
        "io-read-sensor" => Ok(Capability::IoReadSensor),
        "io-write-actuator" => Ok(Capability::IoWriteActuator),
        "io-network" => Ok(Capability::IoNetwork),
        "io-persist" => Ok(Capability::IoPersist),
        "sys-create-actor" => Ok(Capability::SysCreateActor),
        "sys-terminate-actor" => Ok(Capability::SysTerminateActor),
        "sys-clock" => Ok(Capability::SysClock),
        _ => Err(()),
    }
}

/// Helper function to get child nodes from AST
fn get_child_nodes(ast: &crate::ast::AstNode) -> Vec<&crate::ast::AstNode> {
    let mut children = Vec::new();

    match ast {
        crate::ast::AstNode::Call {
            function,
            arguments,
            ..
        } => {
            children.push(&**function);
            for arg in arguments {
                children.push(arg);
            }
        }
        crate::ast::AstNode::Lambda { body, .. } => {
            children.push(&**body);
        }
        crate::ast::AstNode::Let { bindings, body, .. } => {
            for (_, value) in bindings {
                children.push(value);
            }
            children.push(&**body);
        }
        crate::ast::AstNode::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            children.push(&**condition);
            children.push(&**then_branch);
            children.push(&**else_branch);
        }
        crate::ast::AstNode::TrustTier { expression, .. } => {
            children.push(&**expression);
        }
        crate::ast::AstNode::MacroDefinition { body, .. } => {
            children.push(&**body);
        }
        crate::ast::AstNode::MacroExpansion { arguments, .. } => {
            for arg in arguments {
                children.push(arg);
            }
        }
        crate::ast::AstNode::FfiCall { arguments, .. } => {
            for arg in arguments {
                children.push(arg);
            }
        }
        crate::ast::AstNode::List { elements, .. } => {
            for elem in elements {
                children.push(elem);
            }
        }
        crate::ast::AstNode::Cons { car, cdr, .. } => {
            children.push(&**car);
            children.push(&**cdr);
        }
        _ => {} // No children for other node types
    }

    children
}
