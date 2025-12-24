use crate::error::CompilationError;
use crate::integration::physics::PhysicsWorldCompiler;
use physics_world::types::OpCode;

impl PhysicsWorldCompiler {
    /// Compile variable to bytecode
    pub fn compile_variable(&mut self, name: &str) -> Result<Vec<OpCode>, CompilationError> {
        // Look up the variable in the environment - lookup_variable already searches all scopes
        if let Some(offset) = self.environment.lookup_variable(name) {
            // Variable found - generate GetLocal instruction
            // Parameters in the environment are defined with offset starting from 0
            // When inside a lambda body, the closure is stored at position 0 in call_frame.locals
            // So parameters are at their original offsets + 1: parameter 0 is at locals[1]
            // Check if we're currently compiling a recursive lambda
            let adjusted_offset = if self.is_compiling_recursive_lambda {
                offset + 1 // Add +1 offset for recursive lambdas to account for closure at position 0
            } else {
                offset // Normal offset for non-recursive contexts
            };
            Ok(vec![OpCode::GetLocal(adjusted_offset)])
        } else {
            // Variable not found anywhere - this is an error
            Err(CompilationError::ParseError {
                message: format!("Undefined variable: {}", name),
                location: self.location.clone(),
            })
        }
    }
}
