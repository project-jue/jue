/// Physics-World integration for Jue-World V2.0
use crate::ast::AstNode;
use crate::compiler::environment::CompilationEnvironment;
use crate::error::{CompilationError, SourceLocation};
use crate::ffi::{create_standard_ffi_registry, FfiCallGenerator};
use crate::trust_tier::TrustTier;
use physics_world::types::{Capability, OpCode, Value};

/// Physics-World compiler for Empirical/Experimental tiers
pub struct PhysicsWorldCompiler {
    /// Current trust tier
    pub tier: TrustTier,

    /// Current source location
    pub location: SourceLocation,

    /// Capability index mapping
    pub capability_indices: Vec<Capability>,

    /// String constant pool for deduplication
    pub string_pool: Vec<String>,

    /// FFI registry for function lookup
    pub ffi_registry: FfiCallGenerator,

    /// Compilation environment for variable tracking
    pub environment: CompilationEnvironment,
}

impl PhysicsWorldCompiler {
    /// Create a new Physics-World compiler
    pub fn new(tier: TrustTier) -> Self {
        Self {
            tier,
            location: SourceLocation::default(),
            capability_indices: Vec::new(),
            string_pool: Vec::new(),
            ffi_registry: FfiCallGenerator {
                registry: create_standard_ffi_registry(),
                location: SourceLocation::default(),
            },
            environment: CompilationEnvironment::new(),
        }
    }

    /// Get or create capability index
    pub fn get_capability_index(&mut self, capability: &Capability) -> usize {
        if let Some(index) = self.capability_indices.iter().position(|c| c == capability) {
            index
        } else {
            self.capability_indices.push(capability.clone());
            self.capability_indices.len() - 1
        }
    }

    /// Get or create string index in constant pool
    pub fn get_string_index(&mut self, string: &str) -> usize {
        if let Some(index) = self.string_pool.iter().position(|s| s == string) {
            index
        } else {
            self.string_pool.push(string.to_string());
            self.string_pool.len() - 1
        }
    }

    /// Compile AST to Physics-World bytecode
    pub fn compile_to_physics(&mut self, ast: &AstNode) -> Result<Vec<OpCode>, CompilationError> {
        match ast {
            AstNode::Literal(lit) => self.compile_literal(lit),
            AstNode::Variable(name) => self.compile_variable(name),
            AstNode::Symbol(name) => self.compile_symbol(name),
            AstNode::Call {
                function,
                arguments,
                ..
            } => self.compile_call(function, arguments),
            AstNode::Lambda {
                parameters, body, ..
            } => self.compile_lambda(parameters, body),
            AstNode::Let { bindings, body, .. } => self.compile_let(bindings, body),
            AstNode::TrustTier { expression, .. } => self.compile_to_physics(expression),
            AstNode::RequireCapability { capability, .. } => {
                self.compile_require_capability(capability)
            }
            AstNode::HasCapability { capability, .. } => self.compile_has_capability(capability),
            AstNode::FfiCall {
                function,
                arguments,
                location,
            } => self.compile_ffi_call(function, arguments, location),
            // Handle other AST nodes...
            _ => Err(CompilationError::InternalError(format!(
                "Unsupported AST node for Physics-World compilation: {:?}",
                ast
            ))),
        }
    }

    /// Compile literal to bytecode
    fn compile_literal(
        &mut self,
        lit: &crate::ast::Literal,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let opcode = match lit {
            crate::ast::Literal::Nil => OpCode::Nil,
            crate::ast::Literal::Bool(b) => OpCode::Bool(*b),
            crate::ast::Literal::Int(i) => OpCode::Int(*i),
            crate::ast::Literal::Float(f) => OpCode::Float(*f), // FIXED: Proper float handling
            crate::ast::Literal::String(s) => {
                // Get string index from constant pool
                let string_index = self.get_string_index(s);
                OpCode::LoadString(string_index)
            }
        };

        Ok(vec![opcode])
    }

    /// Compile variable to bytecode
    fn compile_variable(&mut self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
        // Look up the variable in the environment
        if let Some(offset) = self.environment.lookup_variable(name) {
            // Variable found - generate GetLocal instruction
            Ok(vec![OpCode::GetLocal(offset)])
        } else {
            // Variable not found - this is an error
            Err(CompilationError::ParseError {
                message: format!("Undefined variable: {}", name),
                location: self.location.clone(),
            })
        }
    }

    /// Compile symbol to bytecode - FIXED: Type-aware symbol resolution
    fn compile_symbol(&mut self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
        let opcode = match name {
            // Integer arithmetic operations
            "add" => OpCode::Add,
            "sub" => OpCode::Sub,
            "mul" => OpCode::Mul,
            "div" => OpCode::Div,
            "mod" => OpCode::Mod,
            // Float arithmetic operations - TESTS EXPECT THESE FOR FLOAT OPS
            "fadd" => OpCode::FAdd,
            "fsub" => OpCode::FSub,
            "fmul" => OpCode::FMul,
            "fdiv" => OpCode::FDiv,
            // Comparison operations
            "eq" => OpCode::Eq,
            "lt" => OpCode::Lt,
            "gt" => OpCode::Gt,
            // String operations
            "str-concat" => OpCode::StrConcat,
            "str-len" => OpCode::StrLen,
            "str-index" => OpCode::StrIndex,
            // Variable and stack operations
            "swap" => OpCode::Swap,
            "dup" => OpCode::Dup,
            "pop" => OpCode::Pop,
            // Control flow
            "call" => OpCode::Call(0), // Will be overridden with arg count in actual calls
            "tail-call" => OpCode::TailCall(0), // Will be overridden with arg count
            "ret" => OpCode::Ret,
            "jmp" => OpCode::Jmp(0), // Will be overridden with jump target
            "jmp-if-false" => OpCode::JmpIfFalse(0), // Will be overridden with jump target
            // Actor operations
            "yield" => OpCode::Yield,
            "send" => OpCode::Send,
            // Closure operations
            "make-closure" => OpCode::MakeClosure(0, 0), // Will be overridden
            // List operations
            "cons" => OpCode::Cons,
            "car" => OpCode::Car,
            "cdr" => OpCode::Cdr,
            // Resource management
            "check-step-limit" => OpCode::CheckStepLimit,
            // Sandbox operations
            "init-sandbox" => OpCode::InitSandbox,
            "isolate-capabilities" => OpCode::IsolateCapabilities,
            "set-error-handler" => OpCode::SetErrorHandler(0), // Will be overridden
            "log-sandbox-violation" => OpCode::LogSandboxViolation,
            "cleanup-sandbox" => OpCode::CleanupSandbox,
            _ => {
                return Err(CompilationError::InternalError(format!(
                    "Unknown symbol '{}' for Physics-World compilation",
                    name
                )))
            }
        };

        Ok(vec![opcode])
    }

    /// Compile function call to bytecode with type inference
    fn compile_call(
        &mut self,
        function: &AstNode,
        arguments: &[AstNode],
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Special handling for type-aware arithmetic operations
        if let AstNode::Symbol(symbol_name) = function {
            if self.is_type_aware_arithmetic(symbol_name) {
                return self.compile_type_aware_arithmetic_call(symbol_name, arguments);
            }
        }

        // Compile arguments (in reverse order for stack)
        for arg in arguments.iter().rev() {
            let arg_bytecode = self.compile_to_physics(arg)?;
            bytecode.extend(arg_bytecode);
        }

        // Compile function
        let func_bytecode = self.compile_to_physics(function)?;
        bytecode.extend(func_bytecode);

        // Add call instruction
        bytecode.push(OpCode::Call(arguments.len() as u16));

        Ok(bytecode)
    }

    /// Check if symbol needs type-aware compilation
    fn is_type_aware_arithmetic(&self, symbol_name: &str) -> bool {
        matches!(symbol_name, "add" | "sub" | "mul" | "div" | "str-concat")
    }

    /// Recursively check if an AST node will evaluate to a float
    fn ast_evaluates_to_float(&self, ast: &AstNode) -> bool {
        match ast {
            AstNode::Literal(crate::ast::Literal::Float(_)) => true,
            AstNode::Call {
                function,
                arguments,
                ..
            } => {
                // Check if this is an arithmetic call that produces floats
                if let AstNode::Symbol(symbol_name) = function.as_ref() {
                    match symbol_name.as_str() {
                        "mul" | "div" | "add" | "sub" => {
                            // If any argument is a float, this will produce a float
                            arguments.iter().any(|arg| self.ast_evaluates_to_float(arg))
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Compile type-aware arithmetic call (e.g., add with float args -> FAdd)
    fn compile_type_aware_arithmetic_call(
        &mut self,
        symbol_name: &str,
        arguments: &[AstNode],
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Analyze argument types to determine operation
        let has_floats = arguments.iter().any(|arg| self.ast_evaluates_to_float(arg));
        let has_strings = arguments
            .iter()
            .any(|arg| matches!(arg, AstNode::Literal(crate::ast::Literal::String(_))));

        // Choose appropriate opcode based on types
        let opcodes = match symbol_name {
            "add" => {
                if has_floats {
                    vec![OpCode::FAdd; arguments.len().saturating_sub(1)]
                } else {
                    vec![OpCode::Add; arguments.len().saturating_sub(1)]
                }
            }
            "sub" => {
                if has_floats {
                    vec![OpCode::FSub; arguments.len().saturating_sub(1)]
                } else {
                    vec![OpCode::Sub; arguments.len().saturating_sub(1)]
                }
            }
            "mul" => {
                if has_floats {
                    vec![OpCode::FMul; arguments.len().saturating_sub(1)]
                } else {
                    vec![OpCode::Mul; arguments.len().saturating_sub(1)]
                }
            }
            "div" => {
                if has_floats {
                    vec![OpCode::FDiv; arguments.len().saturating_sub(1)]
                } else {
                    vec![OpCode::Div; arguments.len().saturating_sub(1)]
                }
            }
            "str-concat" if has_strings => {
                vec![OpCode::StrConcat; arguments.len().saturating_sub(1)]
            }
            "str-concat" => vec![OpCode::StrConcat; arguments.len().saturating_sub(1)],
            _ => {
                return Err(CompilationError::InternalError(format!(
                    "Type inference failed for symbol '{}' with arguments",
                    symbol_name
                )))
            }
        };

        // Compile arguments (in reverse order for stack)
        for arg in arguments.iter().rev() {
            let arg_bytecode = self.compile_to_physics(arg)?;
            bytecode.extend(arg_bytecode);
        }

        // Add the type-aware operations directly (no Call instruction for primitive ops)
        bytecode.extend(opcodes);

        Ok(bytecode)
    }

    /// Compile lambda to bytecode
    fn compile_lambda(
        &mut self,
        parameters: &[String],
        body: &AstNode,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Push new scope for lambda parameters
        self.environment.push_scope();

        // Define parameters in the new scope
        for param in parameters {
            self.environment.define_variable(param);
        }

        // Compile body
        let body_bytecode = self.compile_to_physics(body)?;
        bytecode.extend(body_bytecode);

        // Pop scope after compilation
        self.environment.pop_scope();

        // Create closure
        // TODO: Implement proper closure creation with environment capture
        bytecode.push(OpCode::MakeClosure(0, parameters.len()));

        Ok(bytecode)
    }

    /// Compile let binding to bytecode
    fn compile_let(
        &mut self,
        bindings: &[(String, AstNode)],
        body: &AstNode,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Push new scope for let bindings
        self.environment.push_scope();

        // Compile each binding and define variables
        for (name, value_expr) in bindings {
            // Compile the binding value
            let value_bytecode = self.compile_to_physics(value_expr)?;
            bytecode.extend(value_bytecode);

            // Define the variable in the current scope
            let offset = self.environment.define_variable(name);

            // Generate SetLocal instruction to store the value
            bytecode.push(OpCode::SetLocal(offset));
        }

        // Compile the let body
        let body_bytecode = self.compile_to_physics(body)?;
        bytecode.extend(body_bytecode);

        // Pop scope after compilation
        self.environment.pop_scope();

        Ok(bytecode)
    }

    /// Compile require-capability declaration
    fn compile_require_capability(
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
    fn compile_has_capability(
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

    /// Compile FFI call to bytecode
    fn compile_ffi_call(
        &mut self,
        function_name: &str,
        arguments: &[AstNode],
        location: &SourceLocation,
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
    fn get_ffi_capability(&self, function_name: &str) -> Result<Capability, CompilationError> {
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
    fn get_ffi_host_function(
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

    /// Analyze bytecode to identify required capabilities
    fn analyze_capabilities_from_bytecode(&self, bytecode: &[OpCode]) -> Vec<Capability> {
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

    /// Insert runtime capability checks for empirical tier
    pub fn insert_runtime_capability_checks(
        &mut self,
        bytecode: Vec<OpCode>,
    ) -> Result<Vec<OpCode>, CompilationError> {
        // Analyze the bytecode to find required capabilities
        let required_caps = self.analyze_capabilities_from_bytecode(&bytecode);

        if required_caps.is_empty() {
            // No capabilities required, return original bytecode
            return Ok(bytecode);
        }

        // Create capability check preamble
        let mut check_bytecode = Vec::new();

        // Insert capability checks at the beginning
        for cap in &required_caps {
            let cap_index = self.get_capability_index(cap);
            check_bytecode.push(OpCode::HasCap(cap_index));
            // Jump over subsequent check if capability is missing
            // This creates a chain of capability checks
            check_bytecode.push(OpCode::JmpIfFalse(2));
        }

        // Prepend capability checks to the main bytecode
        check_bytecode.extend(bytecode);
        Ok(check_bytecode)
    }

    /// Add sandbox wrapper for experimental tier
    pub fn add_sandbox_wrapper(
        &mut self,
        mut bytecode: Vec<OpCode>,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut wrapper = Vec::new();

        // 1. Add resource monitoring initialization
        wrapper.push(OpCode::InitSandbox);

        // 2. Add capability isolation setup
        wrapper.push(OpCode::IsolateCapabilities);

        // 3. Add error boundary setup
        let error_handler_offset = bytecode.len() + 2; // After wrapper setup
        wrapper.push(OpCode::SetErrorHandler(error_handler_offset as i16));

        // 4. Add main bytecode
        wrapper.extend(bytecode);

        // 5. Add cleanup
        wrapper.push(OpCode::CleanupSandbox);
        // Note: Don't add Ret here - let the VM naturally finish execution

        // 6. Add error handler
        wrapper.push(OpCode::LogSandboxViolation);
        // Don't add Ret here either - let error handler complete naturally

        Ok(wrapper)
    }
}

/// Physics-World integration functions
pub fn compile_to_physics_world(
    ast: &AstNode,
    tier: TrustTier,
) -> Result<(Vec<OpCode>, Vec<Value>), CompilationError> {
    let mut compiler = PhysicsWorldCompiler::new(tier);
    let mut bytecode = compiler.compile_to_physics(ast)?;

    // Add tier-specific processing
    match tier {
        TrustTier::Empirical => {
            bytecode = compiler.insert_runtime_capability_checks(bytecode)?;
        }
        TrustTier::Experimental => {
            bytecode = compiler.add_sandbox_wrapper(bytecode)?;
        }
        _ => {} // Formal/Verified tiers handled by Core-World
    }

    // Extract string constants from compiler's string pool
    let string_constants: Vec<Value> = compiler
        .string_pool
        .into_iter()
        .map(|s| Value::String(s))
        .collect();

    Ok((bytecode, string_constants))
}

#[cfg(test)]
#[path = "../test/integration_physics.rs"]
mod tests;
