use crate::ast::AstNode;
use crate::compiler::environment::CompilationEnvironment;
use crate::error::{CompilationError, SourceLocation};
use crate::ffi::{create_standard_ffi_registry, FfiCallGenerator};
use crate::trust_tier::TrustTier;
use physics_world::types::{Capability, OpCode, Value};

/// Convert a string capability name to a Capability enum
/// Maps string names to their corresponding Capability variants
pub fn string_to_capability(name: &str) -> Option<Capability> {
    match name {
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

/// Physics World Compiler
pub struct PhysicsWorldCompiler {
    /// Current trust tier
    pub tier: TrustTier,
    /// Current source location
    pub location: SourceLocation,
    /// Capability indices
    pub capability_indices: Vec<Capability>,
    /// String constant pool
    pub string_pool: Vec<String>,
    /// FFI registry
    pub ffi_registry: FfiCallGenerator,
    /// Compilation environment
    pub environment: CompilationEnvironment,
    /// Recursive lambda compilation flag
    pub is_compiling_recursive_lambda: bool,
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
            is_compiling_recursive_lambda: false,
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
                self.compile_require_capability_string(capability)
            }
            AstNode::HasCapability { capability, .. } => {
                self.compile_has_capability_string(capability)
            }
            AstNode::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => self.compile_if(condition, then_branch, else_branch),
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

    /// Compile a literal value
    pub fn compile_literal(
        &mut self,
        lit: &crate::ast::Literal,
    ) -> Result<Vec<OpCode>, CompilationError> {
        match lit {
            crate::ast::Literal::Int(value) => Ok(vec![OpCode::Int(*value)]),
            crate::ast::Literal::Float(value) => Ok(vec![OpCode::Float(*value)]),
            crate::ast::Literal::String(value) => {
                let string_index = self.get_string_index(value);
                Ok(vec![OpCode::LoadString(string_index)])
            }
            crate::ast::Literal::Bool(value) => Ok(vec![OpCode::Bool(*value)]),
            crate::ast::Literal::Nil => Ok(vec![OpCode::Nil]),
        }
    }

    /// Compile a variable reference
    pub fn compile_variable(&mut self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
        if let Some(index) = self.environment.get_variable_index(name) {
            Ok(vec![OpCode::GetLocal(index as u16)])
        } else {
            Err(CompilationError::VariableNotFound(name.to_string()))
        }
    }

    /// Compile a symbol
    pub fn compile_symbol(&mut self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
        let symbol_index = self.get_string_index(name);
        Ok(vec![OpCode::Symbol(symbol_index)])
    }

    /// Compile a function call
    pub fn compile_call(
        &mut self,
        function: &AstNode,
        arguments: &[AstNode],
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Compile arguments in reverse order (stack grows upwards)
        for arg in arguments.iter().rev() {
            bytecode.extend(self.compile_to_physics(arg)?);
        }

        // Compile function
        bytecode.extend(self.compile_to_physics(function)?);

        // Add call instruction
        bytecode.push(OpCode::Call(arguments.len() as u16));

        Ok(bytecode)
    }

    /// Compile a lambda function
    pub fn compile_lambda(
        &mut self,
        parameters: &[String],
        body: &AstNode,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Create new environment for lambda
        self.environment.push_scope();

        // Add parameters to environment
        for (i, param) in parameters.iter().enumerate() {
            self.environment.add_variable(param.clone(), i);
        }

        // Compile lambda body
        let body_bytecode = self.compile_to_physics(body)?;

        // Pop environment scope
        self.environment.pop_scope();

        // Create closure
        bytecode.push(OpCode::MakeClosure(parameters.len(), body_bytecode.len()));
        bytecode.extend(body_bytecode);

        Ok(bytecode)
    }

    /// Compile a let binding
    pub fn compile_let(
        &mut self,
        bindings: &[(String, AstNode)],
        body: &AstNode,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Create new environment scope
        self.environment.push_scope();

        // Compile each binding
        for (name, value) in bindings {
            let value_bytecode = self.compile_to_physics(value)?;
            bytecode.extend(value_bytecode);

            // Add variable to environment
            let index = self.environment.add_variable(name.clone(), 0);
            bytecode.push(OpCode::SetLocal(index as u16));
        }

        // Compile body
        let body_bytecode = self.compile_to_physics(body)?;

        // Pop environment scope
        self.environment.pop_scope();

        bytecode.extend(body_bytecode);
        Ok(bytecode)
    }

    /// Compile a require capability statement (takes String from AST)
    pub fn compile_require_capability_string(
        &mut self,
        capability: &str,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let cap = string_to_capability(capability)
            .ok_or_else(|| CompilationError::InternalError(format!("Unknown capability: {}", capability)))?;
        let cap_index = self.get_capability_index(&cap);
        Ok(vec![OpCode::RequestCap(cap_index, 0)])
    }

    /// Compile a has capability check (takes String from AST)
    pub fn compile_has_capability_string(
        &mut self,
        capability: &str,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let cap = string_to_capability(capability)
            .ok_or_else(|| CompilationError::InternalError(format!("Unknown capability: {}", capability)))?;
        let cap_index = self.get_capability_index(&cap);
        Ok(vec![OpCode::HasCap(cap_index)])
    }

    /// Compile an if expression
    pub fn compile_if(
        &mut self,
        condition: &AstNode,
        then_branch: &AstNode,
        else_branch: &AstNode,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Compile condition
        bytecode.extend(self.compile_to_physics(condition)?);

        // Add conditional jump (placeholder for now)
        bytecode.push(OpCode::JmpIfFalse(0)); // Will be patched later

        let jump_offset = bytecode.len() - 1;

        // Compile then branch
        bytecode.extend(self.compile_to_physics(then_branch)?);

        // Add unconditional jump over else branch (placeholder)
        bytecode.push(OpCode::Jmp(0)); // Will be patched later

        let else_jump_offset = bytecode.len() - 1;

        // Compile else branch
        bytecode.extend(self.compile_to_physics(else_branch)?);

        // Patch jump offsets
        let else_branch_start = bytecode.len() - self.compile_to_physics(else_branch)?.len();
        let after_else = bytecode.len();

        if let OpCode::JmpIfFalse(ref mut offset) = bytecode[jump_offset] {
            *offset = (else_branch_start - jump_offset) as i16;
        }

        if let OpCode::Jmp(ref mut offset) = bytecode[else_jump_offset] {
            *offset = (after_else - else_jump_offset) as i16;
        }

        Ok(bytecode)
    }

    /// Compile an FFI call
    pub fn compile_ffi_call(
        &mut self,
        function: &str,
        arguments: &[AstNode],
        location: &SourceLocation,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Compile arguments in reverse order
        for arg in arguments.iter().rev() {
            bytecode.extend(self.compile_to_physics(arg)?);
        }

        // Look up FFI function
        let func = self
            .ffi_registry
            .registry
            .find_function(function)
            .ok_or_else(|| CompilationError::FfiFunctionNotFound(function.to_string()))?;

        // Get capability index for this function
        let cap_idx = self
            .ffi_registry
            .registry
            .get_capability_index(&func.required_capability)
            .unwrap_or(0);

        // Add FFI call instruction (HostCall with capability info)
        bytecode.push(OpCode::HostCall {
            cap_idx,
            func_id: func.host_function as u16,
            args: arguments.len() as u8,
        });

        Ok(bytecode)
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

    /// Analyze capabilities from bytecode
    pub fn analyze_capabilities_from_bytecode(&mut self, bytecode: &[OpCode]) -> Vec<Capability> {
        let mut required_caps = Vec::new();

        for opcode in bytecode {
            if let OpCode::RequestCap(cap_index, _) = opcode {
                if *cap_index < self.capability_indices.len() {
                    required_caps.push(self.capability_indices[*cap_index].clone());
                }
            }
        }

        required_caps
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
