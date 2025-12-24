use crate::error::CompilationError;
use crate::integration::physics::PhysicsWorldCompiler;
use physics_world::types::{Capability, OpCode};

impl PhysicsWorldCompiler {
    /// Compile require-capability declaration
    pub fn compile_require_capability(
        &mut self,
        capability: &str,
    ) -> Result<Vec<OpCode>, CompilationError> {
        // Parse capability name
        let cap = match capability {
            "MacroHygienic" => Capability::MacroHygienic,
            "MacroUnsafe" => Capability::MacroUnsafe,
            "ComptimeEval" => Capability::ComptimeEval,
            "IoReadSensor" => Capability::IoReadSensor,
            "IoWriteActuator" => Capability::IoWriteActuator,
            "IoNetwork" => Capability::IoNetwork,
            "IoPersist" => Capability::IoPersist,
            "SysCreateActor" => Capability::SysCreateActor,
            "SysTerminateActor" => Capability::SysTerminateActor,
            "SysClock" => Capability::SysClock,
            _ => {
                return Err(CompilationError::InternalError(format!(
                    "Unknown capability: {}",
                    capability
                )))
            }
        };

        // Check if this tier allows the capability
        if !self.tier.allows_capability(&cap) {
            return Err(CompilationError::CapabilityError(
                crate::error::CapabilityViolation {
                    required: cap,
                    tier: self.tier,
                    location: self.location.clone(),
                    suggestion: "This capability is not available in the current trust tier"
                        .to_string(),
                },
            ));
        }

        // For now, just return empty bytecode
        // In a real implementation, this would generate capability checks
        Ok(Vec::new())
    }

    /// Compile has-capability? check
    pub fn compile_has_capability(
        &mut self,
        capability: &str,
    ) -> Result<Vec<OpCode>, CompilationError> {
        // Parse capability name
        let cap = match capability {
            "MacroHygienic" => Capability::MacroHygienic,
            "MacroUnsafe" => Capability::MacroUnsafe,
            "ComptimeEval" => Capability::ComptimeEval,
            "IoReadSensor" => Capability::IoReadSensor,
            "IoWriteActuator" => Capability::IoWriteActuator,
            "IoNetwork" => Capability::IoNetwork,
            "IoPersist" => Capability::IoPersist,
            "SysCreateActor" => Capability::SysCreateActor,
            "SysTerminateActor" => Capability::SysTerminateActor,
            "SysClock" => Capability::SysClock,
            _ => {
                return Err(CompilationError::InternalError(format!(
                    "Unknown capability: {}",
                    capability
                )))
            }
        };

        // Get capability index
        let cap_index = self.get_capability_index(&cap);

        // Generate HasCap opcode
        Ok(vec![OpCode::HasCap(cap_index)])
    }

    /// Analyze bytecode to identify required capabilities
    pub fn analyze_capabilities_from_bytecode(&self, bytecode: &[OpCode]) -> Vec<Capability> {
        let mut required_capabilities = Vec::new();

        for opcode in bytecode {
            match opcode {
                OpCode::HasCap(cap_idx) => {
                    if let Some(cap) = self.capability_indices.get(*cap_idx) {
                        if !required_capabilities.contains(cap) {
                            required_capabilities.push(cap.clone());
                        }
                    }
                }
                OpCode::HostCall { cap_idx, .. } => {
                    if let Some(cap) = self.capability_indices.get(*cap_idx) {
                        if !required_capabilities.contains(cap) {
                            required_capabilities.push(cap.clone());
                        }
                    }
                }
                _ => {
                    // Check for other capability-requiring operations
                    // This would be expanded based on specific opcode analysis
                }
            }
        }

        required_capabilities
    }
}
