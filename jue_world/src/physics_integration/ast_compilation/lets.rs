use crate::ast::AstNode;
use crate::error::CompilationError;
use crate::integration::physics::PhysicsWorldCompiler;
use physics_world::types::OpCode;

impl PhysicsWorldCompiler {
    /// Compile let binding to bytecode with support for recursive bindings
    pub fn compile_let(
        &mut self,
        bindings: &[(String, AstNode)],
        body: &AstNode,
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Push new scope for let bindings
        self.environment.push_scope();

        // COMPREHENSIVE APPROACH FOR RECURSIVE BINDINGS:
        // For recursive lambdas, we need to handle them specially to capture self-references

        // First, identify which bindings are recursive lambdas
        let mut recursive_lambda_indices = Vec::new();
        for (idx, (_name, value_expr)) in bindings.iter().enumerate() {
            if matches!(value_expr, AstNode::Lambda { .. }) {
                recursive_lambda_indices.push(idx);
            }
        }

        // If we have recursive lambdas, we need to compile them differently
        if !recursive_lambda_indices.is_empty() {
            // Pass 1: Pre-define variables and push placeholders
            let mut binding_offsets: Vec<(String, u16)> = Vec::new();
            for (name, _value) in bindings {
                let offset = self.environment.define_variable(name);
                binding_offsets.push((name.clone(), offset));
            }

            // For recursive lambdas, we need to capture the closure value BEFORE creating the closure
            // so that recursive calls can find it via GetLocal
            // Push placeholder values for all bindings first
            for (idx, (_name, value_expr)) in bindings.iter().enumerate() {
                if matches!(value_expr, AstNode::Lambda { .. }) {
                    // Push a placeholder that will be replaced by the actual closure
                    bytecode.push(OpCode::Int(0));
                }
            }

            // Pass 2: Compile values, handling recursive lambdas specially
            let mut placeholder_idx = 0;
            for (idx, (name, value_expr)) in bindings.iter().enumerate() {
                let offset = binding_offsets[idx].1;

                let value_bytecode = match value_expr {
                    AstNode::Lambda {
                        parameters, body, ..
                    } => {
                        // For recursive lambdas, we need to:
                        // 1. First, push the placeholder value (already done above)
                        // 2. Compile the lambda body
                        // 3. The recursive calls will look up 'name' at offset and find this placeholder

                        self.compile_lambda(parameters, body)?
                    }
                    _ => self.compile_to_physics(value_expr)?,
                };

                // The first bytecode instruction should be the closure value
                // which was pushed as a placeholder above
                bytecode.extend(value_bytecode);

                // Store the compiled value in the local variable slot
                bytecode.push(OpCode::SetLocal(offset));

                placeholder_idx += 1;
            }
        } else {
            // Non-recursive case - simple compilation
            let mut binding_offsets: Vec<(String, u16)> = Vec::new();
            for (name, _value) in bindings {
                let offset = self.environment.define_variable(name);
                binding_offsets.push((name.clone(), offset));
            }

            for (idx, (_name, value_expr)) in bindings.iter().enumerate() {
                let offset = binding_offsets[idx].1;
                let value_bytecode = self.compile_to_physics(value_expr)?;
                bytecode.extend(value_bytecode);
                bytecode.push(OpCode::SetLocal(offset));
            }
        }

        // Compile the let body
        let body_bytecode = self.compile_to_physics(body)?;
        bytecode.extend(body_bytecode);

        // Pop scope after compilation
        self.environment.pop_scope();

        Ok(bytecode)
    }
}
