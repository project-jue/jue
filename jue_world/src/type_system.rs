use crate::error::{CompilationError, SourceLocation};
use physics_world::types::Capability;
use serde::{Deserialize, Serialize};
/// Capability-aware type system for Jue-World V2.0
use std::collections::HashSet;

/// Type signature with capability requirements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeSignature {
    /// Function or value name
    pub name: String,

    /// Parameter types
    pub parameters: Vec<Type>,

    /// Return type
    pub return_type: Box<Type>,

    /// Required capabilities for this signature
    pub required_capabilities: Vec<Capability>,

    /// Optional proof obligation hint
    pub proof_obligation: Option<String>,

    /// Error handling strategy based on trust tier
    pub error_handling: ErrorHandlingStrategy,

    /// Source location
    pub location: SourceLocation,
}

/// Type representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    /// Basic type (Int, Bool, etc.)
    Basic(String),

    /// Function type
    Function {
        /// Parameter types
        parameters: Vec<Type>,
        /// Return type
        return_type: Box<Type>,
    },

    /// Capability-annotated type
    CapabilityType {
        /// Base type being annotated
        base_type: Box<Type>,
        /// Required capabilities
        capabilities: Vec<Capability>,
    },

    /// Type variable
    Variable(String),

    /// Result type for error handling
    Result {
        /// Success type
        ok_type: Box<Type>,
        /// Error type
        err_type: Box<Type>,
    },
}

/// Error handling strategy based on trust tier
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorHandlingStrategy {
    /// Formal tier - proven correct, no runtime checks
    Proof,

    /// Verified tier - static contracts with minimal runtime checks
    Static,

    /// Empirical tier - runtime contracts with full checks
    Runtime,

    /// Experimental tier - documentation only, no checks
    Documentation,
}

/// Type environment for type checking
#[derive(Debug, Clone, Default)]
pub struct TypeEnvironment {
    /// Type bindings
    pub bindings: Vec<(String, Type)>,

    /// Capability context
    pub capabilities: HashSet<Capability>,

    /// Current trust tier
    pub tier: crate::trust_tier::TrustTier,
}

impl TypeEnvironment {
    /// Create a new type environment
    pub fn new(tier: crate::trust_tier::TrustTier) -> Self {
        Self {
            bindings: Vec::new(),
            capabilities: tier.granted_capabilities(),
            tier,
        }
    }

    /// Add a type binding
    pub fn add_binding(&mut self, name: String, ty: Type) {
        self.bindings.push((name, ty));
    }

    /// Lookup a type binding
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.bindings
            .iter()
            .rev()
            .find(|(n, _)| n == name)
            .map(|(_, ty)| ty)
    }

    /// Check if capability is available
    pub fn has_capability(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }

    /// Add a capability to the environment
    pub fn add_capability(&mut self, capability: Capability) {
        self.capabilities.insert(capability);
    }
}

/// Type checker for Jue expressions
pub struct TypeChecker {
    /// Type environment
    pub env: TypeEnvironment,

    /// Current source location
    pub location: SourceLocation,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new(tier: crate::trust_tier::TrustTier) -> Self {
        Self {
            env: TypeEnvironment::new(tier),
            location: SourceLocation::default(),
        }
    }

    /// Check if a type is valid in the current environment
    pub fn check_type(&self, ty: &Type) -> Result<(), CompilationError> {
        match ty {
            Type::Basic(_) => Ok(()),
            Type::Function {
                parameters,
                return_type,
            } => {
                for param in parameters {
                    self.check_type(param)?;
                }
                self.check_type(return_type)
            }
            Type::CapabilityType {
                base_type,
                capabilities,
            } => {
                self.check_type(base_type)?;

                // Check that all required capabilities are available
                for cap in capabilities {
                    if !self.env.has_capability(cap) {
                        return Err(CompilationError::CapabilityError(
                            crate::error::CapabilityViolation {
                                required: cap.clone(),
                                tier: self.env.tier,
                                location: self.location.clone(),
                                suggestion:
                                    "This capability is not available in the current environment"
                                        .to_string(),
                            },
                        ));
                    }
                }

                Ok(())
            }
            Type::Variable(_) => Ok(()),
            Type::Result { ok_type, err_type } => {
                self.check_type(ok_type)?;
                self.check_type(err_type)
            }
        }
    }

    /// Check if a type signature is valid
    pub fn check_signature(&self, sig: &TypeSignature) -> Result<(), CompilationError> {
        // Check all parameter types
        for param in &sig.parameters {
            self.check_type(param)?;
        }

        // Check return type
        self.check_type(&sig.return_type)?;

        // Check capability requirements against current environment
        for cap in &sig.required_capabilities {
            if !self.env.has_capability(cap) {
                return Err(CompilationError::CapabilityError(
                    crate::error::CapabilityViolation {
                        required: cap.clone(),
                        tier: self.env.tier,
                        location: sig.location.clone(),
                        suggestion: "This capability is not available in the current trust tier"
                            .to_string(),
                    },
                ));
            }
        }

        Ok(())
    }

    /// Get the appropriate error handling strategy for the current tier
    pub fn get_error_handling_strategy(&self) -> ErrorHandlingStrategy {
        match self.env.tier {
            crate::trust_tier::TrustTier::Formal => ErrorHandlingStrategy::Proof,
            crate::trust_tier::TrustTier::Verified => ErrorHandlingStrategy::Static,
            crate::trust_tier::TrustTier::Empirical => ErrorHandlingStrategy::Runtime,
            crate::trust_tier::TrustTier::Experimental => ErrorHandlingStrategy::Documentation,
        }
    }
}

#[cfg(test)]
#[path = "test/type_system.rs"]
mod tests;
