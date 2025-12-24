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

                // RECURSIVE REFERENCE HANDLING:
                // Before pushing the lambda scope, scan for variables in outer scopes
                // that might be recursive references (variables defined in the current let scope)
                let outer_scope_vars: Vec<(String, u16)> = if env.scopes.len() > 1 {
                    // Get variables from the immediate outer scope (the let scope)
                    env.scopes
                        .last()
                        .map(|scope| {
                            scope
                                .variables
                                .iter()
                                .map(|(name, offset)| (name.clone(), *offset))
                                .collect()
                        })
                        .unwrap_or_default()
                } else {
                    Vec::new()
                };

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
                let mut captures = env.end_closure_capture();

                // RECURSIVE CAPTURE: Add any outer scope variables that are referenced
                // in the lambda body but weren't captured through normal lookup
                // This handles the case where a recursive function references itself
                for (name, offset) in &outer_scope_vars {
                    // Check if this variable is used in the body but not yet captured
                    if !captures.iter().any(|(n, _)| n == name) {
                        // Check if the body references this variable
                        if references_variable(body, name) {
                            captures.push((name.clone(), *offset));
                        }
                    }
                }

                // Store the closure body in the constants
                // For now, we'll use a placeholder - the VM will create an identity function
                let code_idx = constants.len();
                constants.push(Value::Int(0)); // Placeholder - will be replaced by VM

                // Capture variables onto the stack
                for (_name, offset) in &captures {
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

                // TWO-PASS APPROACH FOR RECURSIVE BINDINGS:
                // Pass 1: Pre-define all variables in the scope with placeholder offsets
                // This allows recursive references within lambda bodies
                let mut binding_offsets: Vec<(String, u16)> = Vec::new();
                for (name, _value) in bindings {
                    let offset = env.define_variable(name);
                    binding_offsets.push((name.clone(), offset));
                    // Push a placeholder value onto the stack for this variable
                    bytecode.push(OpCode::Int(0)); // Placeholder
                }

                // Pass 2: Compile the actual values and update the stack slots
                for (idx, (name, value)) in bindings.iter().enumerate() {
                    let offset = binding_offsets[idx].1;

                    // Compile the value expression (now with all let-bound variables visible)
                    generate_node(bytecode, constants, closure_bodies, value, env);

                    // Update the stack slot with the actual value
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

/// Helper function to check if an AST node references a specific variable
/// Used for detecting recursive references in lambda bodies
fn references_variable(node: &ast::AstNode, var_name: &str) -> bool {
    match node {
        ast::AstNode::Variable(name) => name == var_name,
        ast::AstNode::Symbol(name) => name == var_name,
        ast::AstNode::Lambda {
            body, parameters, ..
        } => {
            // Don't count if the variable is shadowed by a parameter
            if parameters.iter().any(|p| p == var_name) {
                false
            } else {
                references_variable(body, var_name)
            }
        }
        ast::AstNode::Call {
            function,
            arguments,
            ..
        } => {
            references_variable(function, var_name)
                || arguments
                    .iter()
                    .any(|arg| references_variable(arg, var_name))
        }
        ast::AstNode::Let { bindings, body, .. } => {
            // Check bindings (but not if the variable is being defined)
            let in_bindings = bindings
                .iter()
                .any(|(name, value)| name != var_name && references_variable(value, var_name));
            // Check body (but not if the variable is shadowed)
            let shadowed = bindings.iter().any(|(name, _)| name == var_name);
            in_bindings || (!shadowed && references_variable(body, var_name))
        }
        ast::AstNode::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            references_variable(condition, var_name)
                || references_variable(then_branch, var_name)
                || references_variable(else_branch, var_name)
        }
        ast::AstNode::Literal(_) => false,
        ast::AstNode::TrustTier { expression, .. } => references_variable(expression, var_name),
        ast::AstNode::RequireCapability { .. } => false,
        ast::AstNode::HasCapability { .. } => false,
        ast::AstNode::TypeSignature { .. } => false,
        ast::AstNode::MacroDefinition {
            body, parameters, ..
        } => {
            // Don't count if the variable is shadowed by a parameter
            if parameters.iter().any(|p| p == var_name) {
                false
            } else {
                references_variable(body, var_name)
            }
        }
        ast::AstNode::MacroExpansion { arguments, .. } => arguments
            .iter()
            .any(|arg| references_variable(arg, var_name)),
        ast::AstNode::FfiCall { arguments, .. } => arguments
            .iter()
            .any(|arg| references_variable(arg, var_name)),
        ast::AstNode::List { elements, .. } => elements
            .iter()
            .any(|elem| references_variable(elem, var_name)),
        ast::AstNode::Cons { car, cdr, .. } => {
            references_variable(car, var_name) || references_variable(cdr, var_name)
        }
    }
}
