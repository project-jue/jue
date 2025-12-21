use crate::error::{CompilationError, SourceLocation};
use crate::trust_tier::TrustTier;
use physics_world::types::{Capability, HostFunction, OpCode, Value};
use std::collections::HashMap;

/// Capability-mediated FFI call with runtime capability checking
#[derive(Debug, Clone)]
pub struct CapabilityMediatedFfiCall {
    /// FFI function name
    pub function_name: String,
    /// Required capability for this call
    pub required_capability: Capability,
    /// Arguments for the FFI call
    pub arguments: Vec<Value>,
    /// Source location of the call
    pub location: SourceLocation,
}

/// Capability-mediated FFI call generator
pub struct CapabilityMediatedFfiGenerator {
    /// Trust tier for capability validation
    pub trust_tier: TrustTier,
    /// Capability mapping for FFI functions
    pub capability_mapping: HashMap<String, Capability>,
    /// Host function mapping
    pub host_function_mapping: HashMap<String, HostFunction>,
}

impl CapabilityMediatedFfiGenerator {
    /// Create a new capability-mediated FFI generator
    pub fn new(trust_tier: TrustTier) -> Self {
        Self {
            trust_tier,
            capability_mapping: Self::create_standard_capability_mapping(),
            host_function_mapping: Self::create_standard_host_function_mapping(),
        }
    }

    /// Create standard capability mapping for FFI functions
    fn create_standard_capability_mapping() -> HashMap<String, Capability> {
        let mut mapping = HashMap::new();

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

    /// Create standard host function mapping
    fn create_standard_host_function_mapping() -> HashMap<String, HostFunction> {
        let mut mapping = HashMap::new();

        mapping.insert("read-sensor".to_string(), HostFunction::ReadSensor);
        mapping.insert("write-actuator".to_string(), HostFunction::WriteActuator);
        mapping.insert("get-wall-clock".to_string(), HostFunction::GetWallClockNs);
        mapping.insert("spawn-actor".to_string(), HostFunction::SpawnActor);
        mapping.insert("terminate-actor".to_string(), HostFunction::TerminateActor);
        mapping.insert("network-send".to_string(), HostFunction::NetworkSend);
        mapping.insert("network-receive".to_string(), HostFunction::NetworkReceive);
        mapping.insert("persist-write".to_string(), HostFunction::PersistWrite);
        mapping.insert("persist-read".to_string(), HostFunction::PersistRead);

        mapping
    }

    /// Generate capability-mediated FFI call bytecode
    pub fn generate_capability_mediated_ffi_call(
        &self,
        function_name: &str,
        arguments: &[Value],
        _location: SourceLocation,
    ) -> Result<Vec<OpCode>, CompilationError> {
        // Validate the FFI function exists
        if !self.capability_mapping.contains_key(function_name) {
            return Err(CompilationError::FfiError(format!(
                "FFI function {} not found",
                function_name
            )));
        }

        // Get required capability
        let required_capability = self.capability_mapping[function_name].clone();
        let host_function = self.host_function_mapping[function_name].clone();

        // Validate capability against trust tier
        self.validate_capability_for_tier(&required_capability)?;

        // Generate capability check bytecode
        let capability_check_bytecode = self.generate_capability_check(&required_capability);

        // Generate argument setup bytecode
        let argument_setup_bytecode = self.generate_argument_setup(arguments);

        // Generate the HostCall opcode
        let host_call_opcode = OpCode::HostCall {
            cap_idx: 0, // Capability index in constant pool
            func_id: host_function as u16,
            args: arguments.len() as u8,
        };

        // Combine all bytecode
        let mut bytecode = Vec::new();
        bytecode.extend(capability_check_bytecode);
        bytecode.extend(argument_setup_bytecode);
        bytecode.push(host_call_opcode);

        Ok(bytecode)
    }

    /// Validate capability against trust tier
    pub fn validate_capability_for_tier(
        &self,
        capability: &Capability,
    ) -> Result<(), CompilationError> {
        let granted_capabilities = self.trust_tier.granted_capabilities();

        if !granted_capabilities.contains(capability) {
            return Err(CompilationError::FfiError(format!(
                "Capability {:?} not granted for trust tier {:?}",
                capability, self.trust_tier
            )));
        }

        Ok(())
    }

    /// Generate capability check bytecode
    pub fn generate_capability_check(&self, _capability: &Capability) -> Vec<OpCode> {
        // Generate HasCap opcode to check if capability is available
        // In a real implementation, we would need to add the capability to the constant pool
        // and use the correct index. For now, we'll use a placeholder.

        vec![
            OpCode::HasCap(0),     // Check if capability is available
            OpCode::JmpIfFalse(2), // Jump over the call if capability not available
        ]
    }

    /// Generate argument setup bytecode
    pub fn generate_argument_setup(&self, arguments: &[Value]) -> Vec<OpCode> {
        let mut bytecode = Vec::new();

        // Push arguments to stack in reverse order
        for arg in arguments.iter().rev() {
            match arg {
                Value::Nil => bytecode.push(OpCode::Nil),
                Value::Bool(b) => bytecode.push(OpCode::Bool(*b)),
                Value::Int(i) => bytecode.push(OpCode::Int(*i)),
                Value::Float(f) => bytecode.push(OpCode::Float(*f)), // Handle float values
                Value::String(_s) => {
                    // Handle string values - push nil as placeholder for now
                    bytecode.push(OpCode::Nil);
                }
                Value::Symbol(s) => bytecode.push(OpCode::Symbol(*s)),
                Value::Pair(_ptr) => {
                    // Handle pair values - push nil as placeholder
                    bytecode.push(OpCode::Nil);
                }
                Value::Closure(_ptr) => {
                    // Handle closure values - push nil as placeholder
                    bytecode.push(OpCode::Nil);
                }
                Value::ActorId(_id) => {
                    // Handle actor ID values - push nil as placeholder
                    bytecode.push(OpCode::Nil);
                }
                Value::Capability(_cap) => {
                    // Handle capability values - push nil as placeholder
                    bytecode.push(OpCode::Nil);
                }
                Value::GcPtr(_ptr) => {
                    // Handle GC pointer values - push nil as placeholder
                    bytecode.push(OpCode::Nil);
                }
            }
        }

        bytecode
    }

    /// Generate capability error handling bytecode
    pub fn generate_capability_error_handling(&self) -> Vec<OpCode> {
        // Generate error handling for when capability is not available
        vec![
            OpCode::Symbol(0), // Error symbol (would be in constant pool)
            OpCode::Jmp(1),    // Jump to error handler
        ]
    }

    /// Get required capability for an FFI function
    pub fn get_required_capability(&self, function_name: &str) -> Option<Capability> {
        self.capability_mapping.get(function_name).cloned()
    }

    /// Get host function for an FFI function
    pub fn get_host_function(&self, function_name: &str) -> Option<HostFunction> {
        self.host_function_mapping.get(function_name).cloned()
    }

    /// Check if an FFI function is available for the current trust tier
    pub fn is_ffi_function_available(&self, function_name: &str) -> bool {
        if let Some(required_capability) = self.get_required_capability(function_name) {
            self.trust_tier
                .granted_capabilities()
                .contains(&required_capability)
        } else {
            false
        }
    }
}

/// Capability-mediated FFI call validator
pub struct CapabilityMediatedFfiValidator {
    /// Trust tier for validation
    pub trust_tier: TrustTier,
    /// Capability mapping
    pub capability_mapping: HashMap<String, Capability>,
}

impl CapabilityMediatedFfiValidator {
    /// Create a new capability-mediated FFI validator
    pub fn new(trust_tier: TrustTier) -> Self {
        Self {
            trust_tier,
            capability_mapping: CapabilityMediatedFfiGenerator::create_standard_capability_mapping(
            ),
        }
    }

    /// Validate an FFI call against trust tier capabilities
    pub fn validate_ffi_call(&self, function_name: &str) -> Result<(), CompilationError> {
        // Check if the FFI function exists
        if !self.capability_mapping.contains_key(function_name) {
            return Err(CompilationError::FfiError(format!(
                "FFI function {} not found",
                function_name
            )));
        }

        // Get required capability
        let required_capability = self.capability_mapping[function_name].clone();

        // Check if capability is granted for this trust tier
        let granted_capabilities = self.trust_tier.granted_capabilities();

        if !granted_capabilities.contains(&required_capability) {
            return Err(CompilationError::FfiError(format!(
                "FFI call to {} requires capability {:?} not granted for trust tier {:?}",
                function_name, required_capability, self.trust_tier
            )));
        }

        Ok(())
    }

    /// Validate multiple FFI calls
    pub fn validate_ffi_calls(&self, function_names: &[String]) -> Result<(), CompilationError> {
        for function_name in function_names {
            self.validate_ffi_call(function_name)?;
        }
        Ok(())
    }

    /// Get available FFI functions for the current trust tier
    pub fn get_available_ffi_functions(&self) -> Vec<String> {
        let granted_capabilities = self.trust_tier.granted_capabilities();

        self.capability_mapping
            .iter()
            .filter(|(_, capability)| granted_capabilities.contains(capability))
            .map(|(name, _)| name.clone())
            .collect()
    }
}

/// Capability-mediated FFI call builder
pub struct CapabilityMediatedFfiBuilder {
    /// Trust tier
    pub trust_tier: TrustTier,
    /// Function name
    pub function_name: String,
    /// Arguments
    pub arguments: Vec<Value>,
    /// Source location
    pub location: SourceLocation,
}

impl CapabilityMediatedFfiBuilder {
    /// Create a new FFI call builder
    pub fn new(function_name: &str, trust_tier: TrustTier) -> Self {
        Self {
            trust_tier,
            function_name: function_name.to_string(),
            arguments: Vec::new(),
            location: SourceLocation::default(),
        }
    }

    /// Add an argument to the FFI call
    pub fn with_argument(mut self, argument: Value) -> Self {
        self.arguments.push(argument);
        self
    }

    /// Set the source location
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = location;
        self
    }

    /// Build the capability-mediated FFI call
    pub fn build(self) -> Result<CapabilityMediatedFfiCall, CompilationError> {
        let generator = CapabilityMediatedFfiGenerator::new(self.trust_tier);

        // Validate the call
        let required_capability = generator
            .get_required_capability(&self.function_name)
            .ok_or_else(|| {
                CompilationError::FfiError(format!("FFI function {} not found", self.function_name))
            })?;

        generator.validate_capability_for_tier(&required_capability)?;

        Ok(CapabilityMediatedFfiCall {
            function_name: self.function_name,
            required_capability,
            arguments: self.arguments,
            location: self.location,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trust_tier::TrustTier;

    #[test]
    fn test_capability_mediated_ffi_generator_creation() {
        let generator = CapabilityMediatedFfiGenerator::new(TrustTier::Formal);
        assert!(generator.trust_tier == TrustTier::Formal);
        assert!(generator.capability_mapping.contains_key("read-sensor"));
        assert!(generator.host_function_mapping.contains_key("read-sensor"));
    }

    #[test]
    fn test_capability_validation() {
        let generator = CapabilityMediatedFfiGenerator::new(TrustTier::Formal);

        // Formal tier should have limited capabilities
        let result = generator.validate_capability_for_tier(&Capability::IoReadSensor);
        assert!(result.is_err()); // Formal tier doesn't grant IoReadSensor

        // Empirical tier grants IoReadSensor
        let generator_empirical = CapabilityMediatedFfiGenerator::new(TrustTier::Empirical);
        let result = generator_empirical.validate_capability_for_tier(&Capability::IoReadSensor);
        assert!(result.is_ok()); // Empirical tier grants IoReadSensor
    }

    #[test]
    fn test_ffi_function_availability() {
        let generator = CapabilityMediatedFfiGenerator::new(TrustTier::Empirical);

        // Check if read-sensor is available (Empirical tier grants IoReadSensor)
        let available = generator.is_ffi_function_available("read-sensor");
        assert!(available); // Empirical tier grants IoReadSensor

        // Check if get-wall-clock is available (Empirical tier doesn't grant SysClock)
        let available = generator.is_ffi_function_available("get-wall-clock");
        assert!(!available); // Empirical tier doesn't grant SysClock
    }

    #[test]
    fn test_capability_mediated_ffi_call_generation() {
        let generator = CapabilityMediatedFfiGenerator::new(TrustTier::Empirical);

        // Generate FFI call for a function that requires capability granted by Empirical tier
        let arguments = vec![];
        let result = generator.generate_capability_mediated_ffi_call(
            "read-sensor",
            &arguments,
            SourceLocation::default(),
        );

        // This should succeed because Empirical tier grants IoReadSensor
        assert!(result.is_ok());
    }

    #[test]
    fn test_capability_mediated_ffi_validator() {
        let validator = CapabilityMediatedFfiValidator::new(TrustTier::Empirical);

        // Validate an FFI call that should be available
        let result = validator.validate_ffi_call("read-sensor");
        assert!(result.is_ok()); // Empirical tier grants IoReadSensor

        // Validate an FFI call that should not be available
        let result = validator.validate_ffi_call("get-wall-clock");
        assert!(result.is_err()); // Empirical tier doesn't grant SysClock

        // Get available FFI functions for Empirical tier
        let available_functions = validator.get_available_ffi_functions();
        assert!(available_functions.contains(&"read-sensor".to_string()));
        assert!(available_functions.contains(&"write-actuator".to_string()));
        assert!(!available_functions.contains(&"get-wall-clock".to_string()));
    }

    #[test]
    fn test_capability_mediated_ffi_builder() {
        let builder = CapabilityMediatedFfiBuilder::new("read-sensor", TrustTier::Empirical)
            .with_argument(Value::Int(42))
            .with_location(SourceLocation::default());

        let result = builder.build();
        assert!(result.is_ok()); // Should succeed because Empirical tier grants IoReadSensor
    }

    #[test]
    fn test_standard_mappings() {
        let generator = CapabilityMediatedFfiGenerator::new(TrustTier::Formal);

        // Test standard capability mapping
        assert_eq!(
            generator.get_required_capability("read-sensor"),
            Some(Capability::IoReadSensor)
        );
        assert_eq!(
            generator.get_required_capability("get-wall-clock"),
            Some(Capability::SysClock)
        );

        // Test standard host function mapping
        assert_eq!(
            generator.get_host_function("read-sensor"),
            Some(HostFunction::ReadSensor)
        );
        assert_eq!(
            generator.get_host_function("get-wall-clock"),
            Some(HostFunction::GetWallClockNs)
        );
    }
}
