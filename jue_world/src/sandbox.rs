use crate::error::CompilationError;
use physics_world::types::{Capability, OpCode, Value};
use physics_world::{ExecutionResult, PhysicsWorld};

/// Sandbox configuration for experimental tier execution
#[derive(Debug, Clone)]
pub struct SandboxConfig {
    /// Maximum execution steps allowed
    pub step_limit: u64,
    /// Maximum memory usage allowed
    pub memory_limit: usize,
    /// Maximum recursion depth allowed
    pub recursion_limit: usize,
    /// Allowed capabilities for sandboxed execution
    pub allowed_capabilities: Vec<Capability>,
}

/// Sandbox wrapper for experimental tier execution
pub struct Sandbox {
    config: SandboxConfig,
    physics_world: PhysicsWorld,
}

impl Sandbox {
    /// Create a new sandbox with the given configuration
    pub fn new(config: SandboxConfig) -> Self {
        Self {
            config,
            physics_world: PhysicsWorld::new(),
        }
    }

    /// Execute bytecode in a sandboxed environment
    pub fn execute_sandboxed(
        &mut self,
        bytecode: &[OpCode],
        constants: &[Value],
    ) -> Result<ExecutionResult, CompilationError> {
        // Validate bytecode before execution
        self.validate_bytecode(bytecode, constants)?;

        // Apply sandbox transformations
        let (sandboxed_bytecode, sandboxed_constants) =
            self.apply_sandbox_transformations(bytecode.to_vec(), constants.to_vec());

        // Execute using Physics-World with resource limits
        let result = self.physics_world.execute_actor(
            1, // Actor ID for sandboxed execution
            sandboxed_bytecode,
            sandboxed_constants,
            self.config.step_limit,
            self.config.memory_limit,
        );

        Ok(result)
    }

    /// Validate bytecode before execution
    pub fn validate_bytecode(
        &self,
        bytecode: &[OpCode],
        _constants: &[Value],
    ) -> Result<(), CompilationError> {
        // Check for potentially dangerous operations
        for opcode in bytecode {
            match opcode {
                OpCode::HostCall { .. } => {
                    return Err(CompilationError::ProofGenerationFailed(
                        "Host calls are not allowed in sandboxed execution".to_string(),
                    ));
                }
                OpCode::RequestCap(_, _) => {
                    return Err(CompilationError::ProofGenerationFailed(
                        "Capability requests are not allowed in sandboxed execution".to_string(),
                    ));
                }
                OpCode::GrantCap(_, _) => {
                    return Err(CompilationError::ProofGenerationFailed(
                        "Capability grants are not allowed in sandboxed execution".to_string(),
                    ));
                }
                OpCode::RevokeCap(_, _) => {
                    return Err(CompilationError::ProofGenerationFailed(
                        "Capability revocations are not allowed in sandboxed execution".to_string(),
                    ));
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Apply sandbox transformations to bytecode
    pub fn apply_sandbox_transformations(
        &self,
        bytecode: Vec<OpCode>,
        constants: Vec<Value>,
    ) -> (Vec<OpCode>, Vec<Value>) {
        // For now, we'll return the bytecode as-is
        // In a real implementation, we would:
        // 1. Insert resource limit checks
        // 2. Add capability validation
        // 3. Implement sandbox escape prevention
        (bytecode, constants)
    }
}

/// Sandbox builder for configuring experimental tier execution
pub struct SandboxBuilder {
    config: SandboxConfig,
}

impl SandboxBuilder {
    /// Create a new sandbox builder with default settings
    pub fn new() -> Self {
        Self {
            config: SandboxConfig {
                step_limit: 1000,
                memory_limit: 1024 * 1024, // 1MB
                recursion_limit: 100,
                allowed_capabilities: Vec::new(),
            },
        }
    }

    /// Set the maximum execution steps
    pub fn with_step_limit(mut self, limit: u64) -> Self {
        self.config.step_limit = limit;
        self
    }

    /// Set the maximum memory usage
    pub fn with_memory_limit(mut self, limit: usize) -> Self {
        self.config.memory_limit = limit;
        self
    }

    /// Set the maximum recursion depth
    pub fn with_recursion_limit(mut self, limit: usize) -> Self {
        self.config.recursion_limit = limit;
        self
    }

    /// Add an allowed capability
    pub fn with_capability(mut self, capability: Capability) -> Self {
        self.config.allowed_capabilities.push(capability);
        self
    }

    /// Build the sandbox
    pub fn build(self) -> Sandbox {
        Sandbox::new(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physics_world::types::{Capability, OpCode};

    #[test]
    fn test_sandbox_creation() {
        let config = SandboxConfig {
            step_limit: 100,
            memory_limit: 1024,
            recursion_limit: 50,
            allowed_capabilities: vec![Capability::IoReadSensor],
        };

        let sandbox = Sandbox::new(config);
        assert!(sandbox.config.step_limit == 100);
        assert!(sandbox.config.memory_limit == 1024);
        assert!(sandbox.config.recursion_limit == 50);
        assert!(sandbox
            .config
            .allowed_capabilities
            .contains(&Capability::IoReadSensor));
    }

    #[test]
    fn test_sandbox_builder() {
        let sandbox = SandboxBuilder::new()
            .with_step_limit(200)
            .with_memory_limit(2048)
            .with_recursion_limit(100)
            .with_capability(Capability::IoWriteActuator)
            .build();

        assert!(sandbox.config.step_limit == 200);
        assert!(sandbox.config.memory_limit == 2048);
        assert!(sandbox.config.recursion_limit == 100);
        assert!(sandbox
            .config
            .allowed_capabilities
            .contains(&Capability::IoWriteActuator));
    }

    #[test]
    fn test_bytecode_validation() {
        let sandbox = SandboxBuilder::new().build();

        // Test that safe bytecode passes validation
        let safe_bytecode = vec![OpCode::Int(42), OpCode::Int(1), OpCode::Add];
        let result = sandbox.validate_bytecode(&safe_bytecode, &[]);
        assert!(result.is_ok());

        // Test that host calls are rejected
        let unsafe_bytecode = vec![OpCode::HostCall {
            cap_idx: 0,
            func_id: 0,
            args: 0,
        }];
        let result = sandbox.validate_bytecode(&unsafe_bytecode, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_sandbox_transformations() {
        let sandbox = SandboxBuilder::new().build();

        let bytecode = vec![OpCode::Int(42), OpCode::Int(1), OpCode::Add];
        let constants = vec![];

        let (transformed_bytecode, transformed_constants) =
            sandbox.apply_sandbox_transformations(bytecode.clone(), constants.clone());

        // For now, transformations should be identity
        assert_eq!(transformed_bytecode, bytecode);
        assert_eq!(transformed_constants, constants);
    }
}
