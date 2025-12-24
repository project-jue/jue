use crate::ast::Literal;
use crate::error::CompilationError;
use crate::integration::physics::PhysicsWorldCompiler;
use physics_world::types::OpCode;

impl PhysicsWorldCompiler {
    /// Compile literal to bytecode
    pub fn compile_literal(&mut self, lit: &Literal) -> Result<Vec<OpCode>, CompilationError> {
        let opcode = match lit {
            Literal::Nil => OpCode::Nil,
            Literal::Bool(b) => OpCode::Bool(*b),
            Literal::Int(i) => OpCode::Int(*i),
            Literal::Float(f) => OpCode::Float(*f), // FIXED: Proper float handling
            Literal::String(s) => {
                // Get string index from constant pool
                let string_index = self.get_string_index(s);
                OpCode::LoadString(string_index)
            }
        };

        Ok(vec![opcode])
    }
}
