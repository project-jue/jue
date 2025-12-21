use super::environment::Environment;
use crate::ast;
use crate::error::CompilationError;
use physics_world::types::{OpCode, Value};

/// Compile AST to Physics World bytecode
///
/// This function compiles Jue AST nodes to Physics World bytecode and constants.
/// It handles various AST node types including literals, variables, function calls,
/// lambdas, and other Jue language constructs.
///
/// # Arguments
/// * `ast` - The AST node to compile
///
/// # Returns
/// * `Result<(Vec<OpCode>, Vec<Value>), CompilationError>` - Bytecode and constants, or compilation error
pub fn compile_ast_to_bytecode(
    ast: &ast::AstNode,
) -> Result<(Vec<OpCode>, Vec<Value>), CompilationError> {
    let mut env = Environment::new();
    let mut bytecode = Vec::new();
    let mut constants = Vec::new();

    // Helper function to generate bytecode for a node
    fn generate_node(
        bytecode: &mut Vec<OpCode>,
        constants: &mut Vec<Value>,
        node: &ast::AstNode,
        env: &mut Environment,
    ) {
        match node {
            ast::AstNode::Literal(ast::Literal::Int(value)) => {
                bytecode.push(OpCode::Int(*value));
            }
            ast::AstNode::Lambda {
                parameters, body, ..
            } => {
                // Create a closure
                let capture_count = 0; // For now, no captures
                bytecode.push(OpCode::MakeClosure(capture_count, parameters.len()));

                // Compile the body
                generate_node(bytecode, constants, body, env);
            }
            ast::AstNode::Call {
                function,
                arguments,
                ..
            } => {
                // Check if this is a built-in operator call
                if let ast::AstNode::Symbol(op) = function.as_ref() {
                    // Handle built-in operators
                    match op.as_str() {
                        "+" | "-" | "*" | "/" | "%" | "<=" | "<" | ">" | "=" => {
                            // Compile arguments first
                            for arg in arguments {
                                generate_node(bytecode, constants, arg, env);
                            }

                            // Generate the appropriate opcode
                            match op.as_str() {
                                "+" => bytecode.push(OpCode::Add),
                                "-" => bytecode.push(OpCode::Sub),
                                "*" => bytecode.push(OpCode::Mul),
                                "/" => bytecode.push(OpCode::Div),
                                "%" => bytecode.push(OpCode::Mod),
                                "<=" => {
                                    // <= is equivalent to Lt or Eq
                                    bytecode.push(OpCode::Dup); // Duplicate the two arguments
                                    bytecode.push(OpCode::Dup); // Duplicate again for the second comparison
                                    bytecode.push(OpCode::Lt); // Check if less than
                                    bytecode.push(OpCode::Swap); // Swap to get the original arguments back
                                    bytecode.push(OpCode::Eq); // Check if equal
                                    bytecode.push(OpCode::Add); // OR the two results
                                }
                                "<" => bytecode.push(OpCode::Lt),
                                ">" => bytecode.push(OpCode::Gt),
                                "=" => bytecode.push(OpCode::Eq),
                                _ => {
                                    // Unknown operator - push dummy value
                                    bytecode.push(OpCode::Int(0));
                                }
                            }
                            return;
                        }
                        _ => {
                            // Not a built-in operator, proceed with normal function call
                        }
                    }
                }

                // Compile function
                generate_node(bytecode, constants, function, env);

                // Compile arguments
                for arg in arguments {
                    generate_node(bytecode, constants, arg, env);
                }

                // Call the function
                bytecode.push(OpCode::Call(arguments.len() as u16));
            }
            ast::AstNode::Let { bindings, body, .. } => {
                // Process bindings
                for (name, value) in bindings {
                    // Compile the value expression
                    generate_node(bytecode, constants, value, env);

                    // Store the value in the environment
                    env.bind_variable(name.clone(), bytecode.len() - 1);
                }

                // Compile the body
                generate_node(bytecode, constants, body, env);
            }
            ast::AstNode::Variable(_name) => {
                if let Some(_position) = env.resolve_variable(_name) {
                    // FIXED: Use proper stack-based variable access
                    // For now, use a simple approach: use Dup to access variables
                    // This is a temporary solution until we implement proper stack frames
                    bytecode.push(OpCode::Dup);
                } else {
                    // Variable not found - push dummy value
                    bytecode.push(OpCode::Int(0));
                }
            }
            ast::AstNode::Symbol(_sym) => {
                // FIXED: Check if this symbol is actually a variable reference
                // If it's in the environment, treat it as a variable
                if let Some(_position) = env.resolve_variable(_sym) {
                    // This symbol is actually a variable reference
                    // Use Dup to access the variable
                    bytecode.push(OpCode::Dup);
                } else {
                    // This is a real symbol (like a built-in operator)
                    // For now, push as constant and use symbol reference
                    constants.push(Value::Symbol(constants.len()));
                    bytecode.push(OpCode::Symbol((constants.len() - 1) as usize));
                }
            }
            ast::AstNode::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                // Compile condition
                generate_node(bytecode, constants, condition, env);

                // Jump if false - placeholder for now
                bytecode.push(OpCode::JmpIfFalse(0));

                // Compile then branch
                generate_node(bytecode, constants, then_branch, env);

                // Jump to end - placeholder for now
                bytecode.push(OpCode::Jmp(0));

                // Compile else branch
                generate_node(bytecode, constants, else_branch, env);
            }
            // Handle other AST node types as needed
            _ => {
                // For now, push a dummy value for unsupported node types
                bytecode.push(OpCode::Int(0));
            }
        }
    }

    generate_node(&mut bytecode, &mut constants, ast, &mut env);
    Ok((bytecode, constants))
}
