use crate::ast::AstNode;
use crate::error::CompilationError;
use crate::integration::physics::PhysicsWorldCompiler;
use physics_world::types::OpCode;

impl PhysicsWorldCompiler {
    /// Compile if expression to bytecode
    pub fn compile_if(
        &mut self,
        condition: &AstNode,
        then_branch: &AstNode,
        else_branch: &AstNode,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Compile the condition
        let condition_bytecode = self.compile_to_physics(condition)?;
        bytecode.extend(condition_bytecode);

        // Add placeholder for JmpIfFalse (jump to else branch if condition is false)
        let jmp_if_false_pos = bytecode.len();
        bytecode.push(OpCode::JmpIfFalse(0)); // Offset will be updated later

        // Compile the then branch and remember its length
        let then_bytecode = self.compile_to_physics(then_branch)?;
        let then_bytecode_len = then_bytecode.len();
        bytecode.extend(then_bytecode);

        // Add unconditional jump over the else branch
        let jmp_to_end_pos = bytecode.len();
        bytecode.push(OpCode::Jmp(0)); // Offset will be updated later

        // Compile the else branch and remember its length
        let else_bytecode = self.compile_to_physics(else_branch)?;
        let else_bytecode_len = else_bytecode.len();
        bytecode.extend(else_bytecode);

        // Calculate and fix jump offsets
        // JmpIfFalse should jump to the else branch: then_branch + 1 (for the Jmp instruction)
        let jmp_if_false_offset = (then_bytecode_len + 1) as i16;
        // Jmp should jump to the instruction AFTER the else branch
        let jmp_to_end_offset = else_bytecode_len as i16;

        // Fix up the JmpIfFalse offset (jump to else branch if condition is false)
        if let Some(OpCode::JmpIfFalse(_)) = bytecode.get_mut(jmp_if_false_pos) {
            bytecode[jmp_if_false_pos] = OpCode::JmpIfFalse(jmp_if_false_offset);
        }

        // Fix up the Jmp offset (jump to end after then branch)
        if let Some(OpCode::Jmp(_)) = bytecode.get_mut(jmp_to_end_pos) {
            bytecode[jmp_to_end_pos] = OpCode::Jmp(jmp_to_end_offset);
        }

        Ok(bytecode)
    }
}
