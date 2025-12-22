use super::environment::CompilationEnvironment;
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
    let mut env = CompilationEnvironment::new();
    let mut bytecode = Vec::new();
    let mut constants = Vec::new();
    let mut closure_bodies = Vec::new();

    // Helper function to generate bytecode for a node
    fn generate_node(
        bytecode: &mut Vec<OpCode>,
        constants: &mut Vec<Value>,
        closure_bodies: &mut Vec<Vec<OpCode>>,
        node: &ast::AstNode,
        env: &mut CompilationEnvironment,
    ) {
        match node {
            ast::AstNode::Literal(ast::Literal::Int(value)) => {
                bytecode.push(OpCode::Int(*value));
            }
            ast::AstNode::Lambda {
                parameters, body, ..
            } => {
                // Start closure capture context
                env.start_closure_capture();

                // Push new scope for lambda parameters
                env.push_scope();

                // Define parameters in the new scope
                for param in parameters {
                    env.define_variable(param);
                }

                // Compile the body into a separate bytecode vector for the closure
                let mut closure_bytecode = Vec::new();
                generate_node(&mut closure_bytecode, constants, closure_bodies, body, env);

                // Pop scope after compilation
                env.pop_scope();

                // Get captured variables and end capture context
                let captures = env.end_closure_capture();

                // Store the closure body in the constants
                // For now, we'll use a placeholder - the VM will create an identity function
                let code_idx = constants.len();
                constants.push(Value::Int(0)); // Placeholder - will be replaced by VM

                // Capture variables onto the stack
                for (name, offset) in &captures {
                    // Generate instruction to load the captured variable
                    bytecode.push(OpCode::GetLocal(*offset));
                }

                // Create closure with proper code index and capture count
                let capture_count = captures.len() as usize;
                bytecode.push(OpCode::MakeClosure(code_idx, capture_count));
            }
            ast::AstNode::Call {
                function,
                arguments,
                ..
            } => {
                // Check if this is a built-in operator call
                if let ast::AstNode::Symbol(op) = function.as_ref() {
                    // Handle built-in operators (both symbol and operator forms)
                    match op.as_str() {
                        "+" | "-" | "*" | "/" | "%" | "<=" | "<" | ">" | "=" | "add" | "sub"
                        | "mul" | "div" | "mod" | "lt" | "gt" | "eq" | "le" => {
                            // Compile arguments first
                            for arg in arguments {
                                generate_node(bytecode, constants, closure_bodies, arg, env);
                            }

                            // Map function names to operators
                            let operator = match op.as_str() {
                                "+" | "add" => "add",
                                "-" | "sub" => "sub",
                                "*" | "mul" => "mul",
                                "/" | "div" => "div",
                                "%" | "mod" => "mod",
                                "<" | "lt" => "lt",
                                ">" | "gt" => "gt",
                                "=" | "eq" => "eq",
                                "<=" | "le" => "le",
                                _ => "unknown",
                            };

                            // Handle multi-argument operators by generating multiple binary operations
                            match operator {
                                "add" => {
                                    // For addition, we need to add all arguments together
                                    // If there are multiple arguments, generate multiple Add ops
                                    for _ in 1..arguments.len() {
                                        bytecode.push(OpCode::Add);
                                    }
                                }
                                "sub" => {
                                    // For subtraction, it's left-associative: a - b - c = (a - b) - c
                                    for _ in 1..arguments.len() {
                                        bytecode.push(OpCode::Sub);
                                    }
                                }
                                "mul" => {
                                    // For multiplication, we need to multiply all arguments together
                                    for _ in 1..arguments.len() {
                                        bytecode.push(OpCode::Mul);
                                    }
                                }
                                "div" => {
                                    // For division, it's left-associative: a / b / c = (a / b) / c
                                    for _ in 1..arguments.len() {
                                        bytecode.push(OpCode::Div);
                                    }
                                }
                                "mod" => {
                                    // For modulo, it's left-associative: a % b % c = (a % b) % c
                                    for _ in 1..arguments.len() {
                                        bytecode.push(OpCode::Mod);
                                    }
                                }
                                "le" => {
                                    // <= is equivalent to Lt or Eq
                                    if arguments.len() == 2 {
                                        bytecode.push(OpCode::Dup); // Duplicate the two arguments
                                        bytecode.push(OpCode::Dup); // Duplicate again for the second comparison
                                        bytecode.push(OpCode::Lt); // Check if less than
                                        bytecode.push(OpCode::Swap); // Swap to get the original arguments back
                                        bytecode.push(OpCode::Eq); // Check if equal
                                        bytecode.push(OpCode::Add); // OR the two results
                                    } else {
                                        // For multiple arguments, this is more complex - use fallback
                                        bytecode.push(OpCode::Int(0));
                                    }
                                }
                                "lt" => {
                                    if arguments.len() == 1 {
                                        bytecode.push(OpCode::Int(0)); // Unary < not supported
                                    } else if arguments.len() == 2 {
                                        bytecode.push(OpCode::Lt);
                                    } else {
                                        // Multiple arguments - use fallback
                                        bytecode.push(OpCode::Int(0));
                                    }
                                }
                                "gt" => {
                                    if arguments.len() == 1 {
                                        bytecode.push(OpCode::Int(0)); // Unary > not supported
                                    } else if arguments.len() == 2 {
                                        bytecode.push(OpCode::Gt);
                                    } else {
                                        // Multiple arguments - use fallback
                                        bytecode.push(OpCode::Int(0));
                                    }
                                }
                                "eq" => {
                                    if arguments.len() == 1 {
                                        bytecode.push(OpCode::Int(1)); // Unary = is always true
                                    } else if arguments.len() == 2 {
                                        bytecode.push(OpCode::Eq);
                                    } else {
                                        // Multiple arguments - use fallback
                                        bytecode.push(OpCode::Int(0));
                                    }
                                }
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
                generate_node(bytecode, constants, closure_bodies, function, env);

                // Compile arguments
                for arg in arguments {
                    generate_node(bytecode, constants, closure_bodies, arg, env);
                }

                // Call the function
                bytecode.push(OpCode::Call(arguments.len() as u16));
            }
            ast::AstNode::Let { bindings, body, .. } => {
                // Push new scope for let bindings
                env.push_scope();

                // Process bindings
                for (name, value) in bindings {
                    // Compile the value expression
                    generate_node(bytecode, constants, closure_bodies, value, env);

                    // Define the variable in the current scope
                    let offset = env.define_variable(name);

                    // Generate SetLocal instruction to store the value
                    bytecode.push(OpCode::SetLocal(offset));
                }

                // Compile the body
                generate_node(bytecode, constants, closure_bodies, body, env);

                // Pop scope after compilation
                env.pop_scope();
            }
            ast::AstNode::Variable(name) => {
                // Look up the variable in the environment
                if let Some(offset) = env.lookup_variable(name) {
                    // Check if we're in a closure and this is an outer variable
                    if env.is_in_closure() {
                        // Check if this variable is in an outer scope (not current scope)
                        let current_scope_depth = env.current_scope_depth();
                        if current_scope_depth > 0 {
                            // This is likely a free variable that needs capture
                            env.add_capture(name.clone(), offset as u16);
                        }
                    }

                    // Variable found - generate GetLocal instruction
                    bytecode.push(OpCode::GetLocal(offset));
                } else {
                    // Variable not found - push dummy value for now
                    bytecode.push(OpCode::Int(0));
                }
            }
            ast::AstNode::Symbol(_sym) => {
                // For now, push as constant and use symbol reference
                constants.push(Value::Symbol(constants.len()));
                bytecode.push(OpCode::Symbol((constants.len() - 1) as usize));
            }
            ast::AstNode::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                // Compile condition
                generate_node(bytecode, constants, closure_bodies, condition, env);

                // Jump if false - placeholder for now
                bytecode.push(OpCode::JmpIfFalse(0));

                // Compile then branch
                generate_node(bytecode, constants, closure_bodies, then_branch, env);

                // Jump to end - placeholder for now
                bytecode.push(OpCode::Jmp(0));

                // Compile else branch
                generate_node(bytecode, constants, closure_bodies, else_branch, env);
            }
            // Handle other AST node types as needed
            _ => {
                // For now, push a dummy value for unsupported node types
                bytecode.push(OpCode::Int(0));
            }
        }
    }

    generate_node(
        &mut bytecode,
        &mut constants,
        &mut closure_bodies,
        ast,
        &mut env,
    );
    Ok((bytecode, constants))
}
