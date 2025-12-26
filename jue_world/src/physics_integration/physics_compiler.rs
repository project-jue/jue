use crate::ast::AstNode;
use crate::compiler::environment::CompilationEnvironment;
use crate::error::{CompilationError, SourceLocation};
use crate::ffi_system::ffi_call_generator::FfiCallGenerator;
use crate::ffi_system::standard_functions::create_standard_ffi_registry;
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
    /// Debug flag to disable TCO
    pub disable_tco: bool,
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
            disable_tco: false, // Default: TCO enabled
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

    /// Compile AST to Physics-World bytecode with tail context tracking
    ///
    /// # Arguments
    /// * `ast` - The AST node to compile
    /// * `in_tail_position` - Whether this expression is in tail position (last expr that determines return value)
    pub fn compile_to_physics_with_tail_context(
        &mut self,
        ast: &AstNode,
        in_tail_position: bool,
    ) -> Result<Vec<OpCode>, CompilationError> {
        match ast {
            AstNode::Literal(lit) => self.compile_literal(lit),
            AstNode::Variable(name) => self.compile_variable(name),
            AstNode::Symbol(name) => self.compile_symbol(name),
            AstNode::Call {
                function,
                arguments,
                ..
            } => self.compile_call(function, arguments, in_tail_position),
            AstNode::Lambda {
                parameters, body, ..
            } => self.compile_lambda(parameters, body),
            AstNode::Let { bindings, body, .. } => {
                self.compile_let(bindings, body, in_tail_position)
            }
            AstNode::TrustTier { expression, .. } => {
                self.compile_to_physics_with_tail_context(expression, in_tail_position)
            }
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
            } => self.compile_if(condition, then_branch, else_branch, in_tail_position),
            AstNode::FfiCall {
                function,
                arguments,
                location,
            } => self.compile_ffi_call(function, arguments, location),
            AstNode::Define { name, value, .. } => self.compile_define(name.clone(), value),
            AstNode::Letrec { bindings, body, .. } => {
                self.compile_letrec(bindings, body, in_tail_position)
            }
            // Handle other AST nodes...
            _ => Err(CompilationError::InternalError(format!(
                "Unsupported AST node for Physics-World compilation: {:?}",
                ast
            ))),
        }
    }

    /// Compile AST to Physics-World bytecode (backward-compatible wrapper)
    pub fn compile_to_physics(&mut self, ast: &AstNode) -> Result<Vec<OpCode>, CompilationError> {
        self.compile_to_physics_with_tail_context(ast, false)
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
    ///
    /// Auto-detects FFI function calls when the function is a Symbol
    /// that matches a registered FFI function.
    ///
    /// # Arguments
    /// * `function` - The function to call
    /// * `arguments` - The arguments to pass
    /// * `in_tail_position` - Whether this call is in tail position
    pub fn compile_call(
        &mut self,
        function: &AstNode,
        arguments: &[AstNode],
        in_tail_position: bool,
    ) -> Result<Vec<OpCode>, CompilationError> {
        // Check if this is a symbol-based call that might be an FFI function
        if let AstNode::Symbol(name) = function {
            // Check FFI registry first - FFI functions take priority
            // We need to avoid borrow conflict, so we check existence first
            let is_ffi_function = self.ffi_registry.registry.find_function(name).is_some();
            if is_ffi_function {
                // Clone location to avoid borrow conflict with mutable self
                let location = self.location.clone();
                return self.compile_ffi_call(name, arguments, &location);
            }
        }

        // Regular function call - compile as closure call
        let mut bytecode = Vec::new();

        // Compile arguments in reverse order (NOT in tail position)
        for arg in arguments.iter().rev() {
            bytecode.extend(self.compile_to_physics_with_tail_context(arg, false)?);
        }

        // Compile function (NOT in tail position)
        bytecode.extend(self.compile_to_physics_with_tail_context(function, false)?);

        // Emit Call or TailCall based on position
        if in_tail_position && !self.disable_tco {
            bytecode.push(OpCode::TailCall(arguments.len() as u16));
        } else {
            bytecode.push(OpCode::Call(arguments.len() as u16));
        }

        Ok(bytecode)
    }

    /// Compile a lambda function
    /// Lambda body is ALWAYS compiled in tail position (per Scheme semantics)
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

        // Compile lambda body - ALWAYS in tail position (per expert guidance)
        let body_bytecode = self.compile_to_physics_with_tail_context(body, true)?;

        // Pop environment scope
        self.environment.pop_scope();

        // Create closure
        bytecode.push(OpCode::MakeClosure(parameters.len(), body_bytecode.len()));
        bytecode.extend(body_bytecode);

        Ok(bytecode)
    }

    /// Compile a let binding
    ///
    /// # Arguments
    /// * `bindings` - Variable bindings
    /// * `body` - Body expression
    /// * `in_tail_position` - Whether the body is in tail position
    pub fn compile_let(
        &mut self,
        bindings: &[(String, AstNode)],
        body: &AstNode,
        in_tail_position: bool,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Create new environment scope
        self.environment.push_scope();

        // Compile each binding (values NOT in tail position)
        for (name, value) in bindings {
            let value_bytecode = self.compile_to_physics_with_tail_context(value, false)?;
            bytecode.extend(value_bytecode);

            // Add variable to environment
            let index = self.environment.add_variable(name.clone(), 0);
            bytecode.push(OpCode::SetLocal(index as u16));
        }

        // Compile body - propagate tail context
        let body_bytecode = self.compile_to_physics_with_tail_context(body, in_tail_position)?;

        // Pop environment scope
        self.environment.pop_scope();

        bytecode.extend(body_bytecode);
        Ok(bytecode)
    }

    /// Compile a letrec binding (recursive - names visible in values)
    ///
    /// # Arguments
    /// * `bindings` - Variable bindings
    /// * `body` - Body expression
    /// * `in_tail_position` - Whether the body is in tail position
    pub fn compile_letrec(
        &mut self,
        bindings: &[(String, AstNode)],
        body: &AstNode,
        in_tail_position: bool,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Create new environment scope
        self.environment.push_scope();

        // First, register all binding names (so they're visible in the values)
        // This enables mutual recursion in lambda bodies
        for (name, _value) in bindings {
            self.environment.add_variable(name.clone(), 0);
        }

        // Now compile each binding (they can reference each other via the environment)
        // Binding values are NOT in tail position
        for (name, value) in bindings {
            // Compile the value expression
            let value_bytecode = self.compile_to_physics_with_tail_context(value, false)?;
            bytecode.extend(value_bytecode);

            // Store the compiled value in the variable slot
            if let Some(index) = self.environment.get_variable_index(name) {
                bytecode.push(OpCode::SetLocal(index as u16));
            }
        }

        // Compile body - propagate tail context
        let body_bytecode = self.compile_to_physics_with_tail_context(body, in_tail_position)?;

        // Pop environment scope
        self.environment.pop_scope();

        bytecode.extend(body_bytecode);
        Ok(bytecode)
    }

    /// Compile a top-level define (stores in global environment)
    pub fn compile_define(
        &mut self,
        name: String,
        value: &AstNode,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Compile the value (NOT in tail position for defines)
        let value_bytecode = self.compile_to_physics_with_tail_context(value, false)?;
        bytecode.extend(value_bytecode);

        // Add variable to environment and store
        let index = self.environment.add_variable(name, 0);
        bytecode.push(OpCode::SetLocal(index as u16));

        Ok(bytecode)
    }

    /// Compile a require capability statement (takes String from AST)
    pub fn compile_require_capability_string(
        &mut self,
        capability: &str,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let cap = string_to_capability(capability).ok_or_else(|| {
            CompilationError::InternalError(format!("Unknown capability: {}", capability))
        })?;
        let cap_index = self.get_capability_index(&cap);
        Ok(vec![OpCode::RequestCap(cap_index, 0)])
    }

    /// Compile a has capability check (takes String from AST)
    pub fn compile_has_capability_string(
        &mut self,
        capability: &str,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let cap = string_to_capability(capability).ok_or_else(|| {
            CompilationError::InternalError(format!("Unknown capability: {}", capability))
        })?;
        let cap_index = self.get_capability_index(&cap);
        Ok(vec![OpCode::HasCap(cap_index)])
    }

    /// Compile an if expression
    ///
    /// # Arguments
    /// * `condition` - The condition expression
    /// * `then_branch` - The then branch
    /// * `else_branch` - The else branch
    /// * `in_tail_position` - Whether the if expression is in tail position
    pub fn compile_if(
        &mut self,
        condition: &AstNode,
        then_branch: &AstNode,
        else_branch: &AstNode,
        in_tail_position: bool,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Compile condition (never in tail position)
        bytecode.extend(self.compile_to_physics_with_tail_context(condition, false)?);

        // Reserve space for conditional jump
        bytecode.push(OpCode::JmpIfFalse(0));
        let cond_jump_idx = bytecode.len() - 1;

        // Compile then branch - propagate tail context
        bytecode.extend(self.compile_to_physics_with_tail_context(then_branch, in_tail_position)?);

        // Reserve space for jump over else branch
        bytecode.push(OpCode::Jmp(0));
        let skip_else_jump_idx = bytecode.len() - 1;

        // Compile else branch
        let else_start_idx = bytecode.len();
        bytecode.extend(self.compile_to_physics_with_tail_context(else_branch, in_tail_position)?);

        // Patch conditional jump: jump to else_start_idx if condition is false
        // offset = else_start_idx - cond_jump_idx - 1 (per expert guidance)
        let else_offset = (else_start_idx as i16) - (cond_jump_idx as i16) - 1;
        if let OpCode::JmpIfFalse(offset) = &mut bytecode[cond_jump_idx] {
            *offset = else_offset;
        }

        // Patch skip-else jump: jump past else branch
        let skip_else_offset = (bytecode.len() as i16) - (skip_else_jump_idx as i16) - 1;
        if let OpCode::Jmp(offset) = &mut bytecode[skip_else_jump_idx] {
            *offset = skip_else_offset;
        }

        Ok(bytecode)
    }

    /// Compile an FFI call
    pub fn compile_ffi_call(
        &mut self,
        function: &str,
        arguments: &[AstNode],
        _location: &SourceLocation,
    ) -> Result<Vec<OpCode>, CompilationError> {
        // Check if this is an associative operation that can be folded
        let is_associative = matches!(function, "add" | "fadd" | "mul" | "fmul");

        if is_associative && arguments.len() > 2 {
            // Use n-ary associative folding
            self.compile_nary_associative(function, arguments)
        } else if arguments.len() > 2 {
            // Non-associative with too many arguments - use binary folding as fallback
            self.compile_nary_associative(function, arguments)
        } else {
            // Binary or unary - compile normally
            self.compile_binary_ffi_call(function, arguments)
        }
    }

    /// Compile n-ary associative operations using left-fold
    /// (add a b c d) -> (add (add (add a b) c) d)
    fn compile_nary_associative(
        &mut self,
        function: &str,
        arguments: &[AstNode],
    ) -> Result<Vec<OpCode>, CompilationError> {
        if arguments.is_empty() {
            // Identity element for associative operations
            match function {
                "add" | "fadd" => return Ok(vec![OpCode::Int(0)]),
                "mul" | "fmul" => return Ok(vec![OpCode::Int(1)]),
                "sub" | "fsub" | "div" | "fdiv" => {
                    // For non-associative, empty is undefined - return nil
                    return Ok(vec![OpCode::Nil]);
                }
                _ => return Ok(vec![OpCode::Nil]),
            }
        }

        // Left-fold: compile first argument, then for each subsequent argument,
        // compile it and emit a binary HostCall
        let mut bytecode = Vec::new();

        // Look up FFI function FIRST (before any mutable borrows)
        let func = self
            .ffi_registry
            .registry
            .find_function(function)
            .ok_or_else(|| CompilationError::FfiFunctionNotFound(function.to_string()))?;

        // Get capability index (None means no capability required)
        let cap_idx = match &func.required_capability {
            Some(capability) => self
                .ffi_registry
                .registry
                .get_capability_index(capability)
                .unwrap_or(0),
            None => 0,
        };

        // Store host_function for later use (avoid borrow in loop)
        let host_function = func.host_function as u16;

        // Compile first argument
        bytecode.extend(self.compile_to_physics_with_tail_context(&arguments[0], false)?);

        // For each subsequent argument: compile arg, then binary HostCall
        for arg in &arguments[1..] {
            // Compile the next argument
            bytecode.extend(self.compile_to_physics_with_tail_context(arg, false)?);

            // Emit binary HostCall (2 arguments)
            bytecode.push(OpCode::HostCall {
                cap_idx,
                func_id: host_function,
                args: 2, // Always binary for folding
            });
        }

        Ok(bytecode)
    }

    /// Compile a binary FFI call (original behavior for 1-2 arguments)
    fn compile_binary_ffi_call(
        &mut self,
        function: &str,
        arguments: &[AstNode],
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Compile arguments in reverse order (NOT in tail position)
        for arg in arguments.iter().rev() {
            bytecode.extend(self.compile_to_physics_with_tail_context(arg, false)?);
        }

        // Look up FFI function
        let func = self
            .ffi_registry
            .registry
            .find_function(function)
            .ok_or_else(|| CompilationError::FfiFunctionNotFound(function.to_string()))?;

        // Get capability index for this function (None means no capability required)
        let cap_idx = match &func.required_capability {
            Some(capability) => self
                .ffi_registry
                .registry
                .get_capability_index(capability)
                .unwrap_or(0),
            None => {
                // No capability required - use a placeholder that will be ignored
                0
            }
        };

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
