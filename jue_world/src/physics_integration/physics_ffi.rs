use crate::ast::AstNode;
use crate::error::CompilationError;
use crate::integration::physics::PhysicsWorldCompiler;
use physics_world::types::{Capability, OpCode};

impl PhysicsWorldCompiler {
    /// Compile FFI call to bytecode
    pub fn compile_ffi_call(
        &mut self,
        function_name: &str,
        arguments: &[AstNode],
        location: &crate::error::SourceLocation,
    ) -> Result<Vec<OpCode>, CompilationError> {
        // First, validate the FFI call against the trust tier
        let required_capability = self.get_ffi_capability(function_name)?;

        // Check if this tier allows the capability
        if !self.tier.allows_capability(&required_capability) {
            return Err(CompilationError::CapabilityError(
                crate::error::CapabilityViolation {
                    required: required_capability.clone(),
                    tier: self.tier,
                    location: location.clone(),
                    suggestion: format!(
                        "FFI call to {} requires capability {:?} not granted for trust tier {:?}",
                        function_name, required_capability, self.tier
                    ),
                },
            ));
        }

        // Compile arguments first
        let mut bytecode = Vec::new();
        for arg in arguments.iter().rev() {
            let arg_bytecode = self.compile_to_physics(arg)?;
            bytecode.extend(arg_bytecode);
        }

        // Get capability index for the required capability
        let cap_index = self.get_capability_index(&required_capability);

        // Generate capability check
        bytecode.push(OpCode::HasCap(cap_index));
        bytecode.push(OpCode::JmpIfFalse(2)); // Jump over the call if capability not available

        // Generate the HostCall opcode
        let host_function = self.get_ffi_host_function(function_name)?;
        let opcode = OpCode::HostCall {
            cap_idx: cap_index, // This is already usize
            func_id: host_function as u16,
            args: arguments.len() as u8,
        };
        bytecode.push(opcode);

        // Add error handling (placeholder for now)
        bytecode.push(OpCode::Symbol(0)); // Error symbol
        bytecode.push(OpCode::Jmp(1)); // Jump to error handler

        Ok(bytecode)
    }

    /// Get capability required for an FFI function
    pub fn get_ffi_capability(&self, function_name: &str) -> Result<Capability, CompilationError> {
        // Use the capability FFI generator to get the required capability
        use crate::capability_ffi::CapabilityMediatedFfiGenerator;

        let generator = CapabilityMediatedFfiGenerator::new(self.tier);
        generator
            .get_required_capability(function_name)
            .ok_or_else(|| {
                CompilationError::FfiError(format!("FFI function {} not found", function_name))
            })
    }

    /// Get host function for an FFI function
    pub fn get_ffi_host_function(
        &self,
        function_name: &str,
    ) -> Result<physics_world::types::HostFunction, CompilationError> {
        // Use the capability FFI generator to get the host function
        use crate::capability_ffi::CapabilityMediatedFfiGenerator;

        let generator = CapabilityMediatedFfiGenerator::new(self.tier);
        generator.get_host_function(function_name).ok_or_else(|| {
            CompilationError::FfiError(format!("FFI function {} not found", function_name))
        })
    }
}
