use crate::ast::AstNode;
use crate::integration::physics::PhysicsWorldCompiler;

impl PhysicsWorldCompiler {
    /// Collect all variable names referenced in an AST node (for closure capture analysis)
    pub fn collect_variable_references<'a>(&'a self, node: &'a AstNode, refs: &mut Vec<&'a str>) {
        match node {
            AstNode::Variable(name) => {
                refs.push(name);
            }
            AstNode::Call {
                function,
                arguments,
                ..
            } => {
                self.collect_variable_references(function, refs);
                for arg in arguments {
                    self.collect_variable_references(arg, refs);
                }
            }
            AstNode::Lambda { body, .. } => {
                // Don't collect from nested lambda body - they have their own scope
            }
            AstNode::Let { bindings, body, .. } => {
                for (_name, value) in bindings {
                    self.collect_variable_references(value, refs);
                }
                self.collect_variable_references(body, refs);
            }
            AstNode::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                self.collect_variable_references(condition, refs);
                self.collect_variable_references(then_branch, refs);
                self.collect_variable_references(else_branch, refs);
            }
            _ => {}
        }
    }
}
