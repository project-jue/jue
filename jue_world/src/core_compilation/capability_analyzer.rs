use crate::error::{CapabilityViolation, CompilationError, SourceLocation};
use crate::trust_tier::TrustTier;
use physics_world::types::Capability;
use std::collections::HashSet;

/// Analyze capability requirements from AST
pub fn analyze_capabilities(
    ast: &crate::ast::AstNode,
) -> Result<Vec<Capability>, CompilationError> {
    let mut capabilities = Vec::new();

    // Check for specific AST nodes that require capabilities
    match ast {
        crate::ast::AstNode::RequireCapability {
            capability,
            location: _,
        } => {
            // Parse capability from string
            if let Ok(cap) = parse_capability(capability) {
                capabilities.push(cap);
            }
        }
        crate::ast::AstNode::HasCapability {
            capability,
            location: _,
        } => {
            // Parse capability from string
            if let Ok(cap) = parse_capability(capability) {
                capabilities.push(cap);
            }
        }
        crate::ast::AstNode::FfiCall { function, .. } => {
            // FFI calls require specific capabilities based on function name
            if let Some(required_cap) = get_ffi_function_capability(function) {
                capabilities.push(required_cap);
            } else {
                // Default to MacroUnsafe if function not found
                capabilities.push(Capability::MacroUnsafe);
            }
        }
        crate::ast::AstNode::MacroDefinition { .. } => {
            // Macro definitions require macro capability
            capabilities.push(Capability::MacroHygienic);
        }
        _ => {
            // Recursively check child nodes
            for child in get_child_nodes(ast) {
                let mut child_caps = analyze_capabilities(child)?;
                capabilities.append(&mut child_caps);
            }
        }
    }

    // Remove duplicates using a HashSet
    let unique_caps: HashSet<_> = capabilities.into_iter().collect();
    let unique_caps_vec: Vec<_> = unique_caps.into_iter().collect();

    Ok(unique_caps_vec)
}

/// Get the capability required for a specific FFI function
pub fn get_ffi_function_capability(function_name: &str) -> Option<Capability> {
    // Use the standard capability mapping from FFI generator
    let capability_mapping = create_standard_ffi_capability_mapping();
    capability_mapping.get(function_name).cloned()
}

/// Create standard capability mapping for FFI functions
fn create_standard_ffi_capability_mapping() -> std::collections::HashMap<String, Capability> {
    let mut mapping = std::collections::HashMap::new();

    // I/O capabilities
    mapping.insert("read-sensor".to_string(), Capability::IoReadSensor);
    mapping.insert("write-actuator".to_string(), Capability::IoWriteActuator);
    mapping.insert("network-send".to_string(), Capability::IoNetwork);
    mapping.insert("network-receive".to_string(), Capability::IoNetwork);
    mapping.insert("persist-write".to_string(), Capability::IoPersist);
    mapping.insert("persist-read".to_string(), Capability::IoPersist);

    // System capabilities
    mapping.insert("get-wall-clock".to_string(), Capability::SysClock);
    mapping.insert("spawn-actor".to_string(), Capability::SysCreateActor);
    mapping.insert("terminate-actor".to_string(), Capability::SysTerminateActor);

    mapping
}

/// Helper function to parse capability from string
pub fn parse_capability(cap_str: &str) -> Result<Capability, CompilationError> {
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
        _ => Err(CompilationError::ParseError {
            message: format!("Unknown capability: {}", cap_str),
            location: SourceLocation::default(),
        }),
    }
}

/// Helper function to get child nodes from AST
pub fn get_child_nodes(ast: &crate::ast::AstNode) -> Vec<&crate::ast::AstNode> {
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

/// Validate that tier allows required capabilities
pub fn validate_tier_capabilities(
    tier: TrustTier,
    required_caps: &[Capability],
) -> Result<(), CompilationError> {
    let granted_caps = tier.granted_capabilities();

    for cap in required_caps {
        if !granted_caps.contains(cap) {
            return Err(CompilationError::CapabilityError(CapabilityViolation {
                required: cap.clone(),
                tier,
                location: SourceLocation::default(), // TODO: Get actual location
                suggestion: format!(
                    "This capability is not available in {:?} tier. Consider using a higher trust tier.",
                    tier
                ),
            }));
        }
    }

    Ok(())
}

/// Validate FFI call against trust tier capabilities
pub fn validate_ffi_call(tier: TrustTier, function_name: &str) -> Result<(), CompilationError> {
    // Get the required capability for this FFI function
    let required_cap = get_ffi_function_capability(function_name).ok_or_else(|| {
        CompilationError::FfiError(format!(
            "FFI function {} not found in capability mapping",
            function_name
        ))
    })?;

    // Clone the capability for use in error message
    let required_cap_clone = required_cap.clone();

    // Validate against trust tier
    let granted_caps = tier.granted_capabilities();

    if !granted_caps.contains(&required_cap) {
        return Err(CompilationError::CapabilityError(CapabilityViolation {
            required: required_cap_clone,
            tier,
            location: SourceLocation::default(),
            suggestion: format!(
                "FFI call to {} requires capability {:?} not granted for trust tier {:?}. Consider using a higher trust tier.",
                function_name, required_cap, tier
            ),
        }));
    }

    Ok(())
}
