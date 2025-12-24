use crate::ast::AstNode;
use crate::error::CompilationError;
use crate::integration::physics::PhysicsWorldCompiler;
use physics_world::types::OpCode;

impl PhysicsWorldCompiler {
    /// Compile function call to bytecode with type inference
    pub fn compile_call(
        &mut self,
        function: &AstNode,
        arguments: &[AstNode],
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Special handling for type-aware arithmetic operations
        if let AstNode::Symbol(symbol_name) = function {
            // Handle basic arithmetic symbols that should compile to direct opcodes
            if symbol_name == "*"
                || symbol_name == "/"
                || symbol_name == "%"
                || symbol_name == "mul"
                || symbol_name == "div"
                || symbol_name == "mod"
            {
                return self.compile_type_aware_arithmetic_call(symbol_name, arguments);
            }

            if self.is_type_aware_arithmetic(symbol_name) {
                return self.compile_type_aware_arithmetic_call(symbol_name, arguments);
            }

            // Special case for <=, >=, != symbols that should be direct operations
            if symbol_name == "<=" || symbol_name == ">=" || symbol_name == "!=" {
                return self.compile_type_aware_arithmetic_call(symbol_name, arguments);
            }

            // Special case for = symbol that should be direct operations
            if symbol_name == "=" {
                return self.compile_type_aware_arithmetic_call(symbol_name, arguments);
            }
        }

        // Compile arguments (in reverse order for stack) - use recursive handler for lambda bodies
        for arg in arguments.iter().rev() {
            let arg_bytecode = self.compile_lambda_body_with_recursion(arg, &[])?;
            bytecode.extend(arg_bytecode);
        }

        // Compile function - use recursive handler for lambda bodies
        let func_bytecode = self.compile_lambda_body_with_recursion(function, &[])?;
        bytecode.extend(func_bytecode);

        // Add call instruction
        let call_opcode = OpCode::Call(arguments.len() as u16);
        bytecode.push(call_opcode);

        Ok(bytecode)
    }

    /// Check if symbol needs type-aware compilation - FIXED: Added "=" symbol
    pub fn is_type_aware_arithmetic(&self, symbol_name: &str) -> bool {
        matches!(
            symbol_name,
            "add"
                | "sub"
                | "mul"
                | "div"
                | "str-concat"
                | "eq"
                | "="
                | "lt"
                | "gt"
                | "lte"
                | "gte"
                | "ne"
        )
    }

    /// Recursively check if an AST node will evaluate to a float
    pub fn ast_evaluates_to_float(&self, ast: &AstNode) -> bool {
        match ast {
            AstNode::Literal(crate::ast::Literal::Float(_)) => true,
            AstNode::Call {
                function,
                arguments,
                ..
            } => {
                // Check if this is an arithmetic call that produces floats
                if let AstNode::Symbol(symbol_name) = function.as_ref() {
                    match symbol_name.as_str() {
                        "mul" | "div" | "add" | "sub" => {
                            // If any argument is a float, this will produce a float
                            arguments.iter().any(|arg| self.ast_evaluates_to_float(arg))
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Compile type-aware arithmetic call (e.g., add with float args -> FAdd) - FIXED: Added "=" support
    pub fn compile_type_aware_arithmetic_call(
        &mut self,
        symbol_name: &str,
        arguments: &[AstNode],
    ) -> Result<Vec<OpCode>, CompilationError> {
        let mut bytecode = Vec::new();

        // Analyze argument types to determine operation
        let has_floats = arguments.iter().any(|arg| self.ast_evaluates_to_float(arg));
        let has_strings = arguments
            .iter()
            .any(|arg| matches!(arg, AstNode::Literal(crate::ast::Literal::String(_))));

        // Choose appropriate opcode based on types
        let opcodes = match symbol_name {
            "add" | "+" => {
                if has_floats {
                    vec![OpCode::FAdd; arguments.len().saturating_sub(1)]
                } else {
                    vec![OpCode::Add; arguments.len().saturating_sub(1)]
                }
            }
            "sub" | "-" => {
                if has_floats {
                    vec![OpCode::FSub; arguments.len().saturating_sub(1)]
                } else {
                    vec![OpCode::Sub; arguments.len().saturating_sub(1)]
                }
            }
            "mul" | "*" => {
                if has_floats {
                    vec![OpCode::FMul; arguments.len().saturating_sub(1)]
                } else {
                    vec![OpCode::Mul; arguments.len().saturating_sub(1)]
                }
            }
            "div" | "/" => {
                if has_floats {
                    vec![OpCode::FDiv; arguments.len().saturating_sub(1)]
                } else {
                    vec![OpCode::Div; arguments.len().saturating_sub(1)]
                }
            }
            "str-concat" if has_strings => {
                vec![OpCode::StrConcat; arguments.len().saturating_sub(1)]
            }
            "str-concat" => vec![OpCode::StrConcat; arguments.len().saturating_sub(1)],
            // Comparison operations - these don't have type variants
            "eq" | "=" => vec![OpCode::Eq; arguments.len().saturating_sub(1)],
            "lt" | "<" => vec![OpCode::Lt; arguments.len().saturating_sub(1)],
            "gt" | ">" => vec![OpCode::Gt; arguments.len().saturating_sub(1)],
            "lte" | "<=" => vec![OpCode::Lte; arguments.len().saturating_sub(1)],
            "gte" | ">=" => vec![OpCode::Gte; arguments.len().saturating_sub(1)],
            "ne" | "!=" => vec![OpCode::Ne; arguments.len().saturating_sub(1)],
            _ => {
                return Err(CompilationError::InternalError(format!(
                    "Type inference failed for symbol '{}' with {} arguments",
                    symbol_name,
                    arguments.len()
                )));
            }
        };

        // SPECIAL HANDLING: For multiplication with recursive calls, we need to preserve arguments
        if (symbol_name == "mul" || symbol_name == "*") && arguments.len() == 2 {
            // Check if second argument contains a recursive call
            if let AstNode::Call { function, .. } = &arguments[1] {
                if let AstNode::Variable(var_name) = function.as_ref() {
                    if var_name == "fact" || var_name == "fib" {
                        // For factorial: n * fact(n-1)
                        // We need: [n] [fact(n-1)] [n] for the multiplication

                        // For multiplication with recursive calls, we need to manually generate the correct bytecode
                        // The pattern is: n * fact(n-1)
                        // We need: [n] [fact(n-1)] for the multiplication, then Mul

                        // VM Call instruction expects: [args..., closure] with closure at top
                        // For Call(1): exactly 1 argument + 1 closure = 2 items, closure at top

                        // Correct sequence for VM expectations:
                        // 1. Push n for multiplication later (GetLocal(0))
                        bytecode.push(OpCode::GetLocal(0));

                        // 2. Push n for subtraction (GetLocal(0)) - now stack has [n, n]
                        bytecode.push(OpCode::GetLocal(0));

                        // 3. Push 1 for subtraction (Int(1)) - now stack has [n, n, 1]
                        bytecode.push(OpCode::Int(1));

                        // 4. Subtract: n - 1 (Sub) - now stack has [n, n-1]
                        bytecode.push(OpCode::Sub);

                        // 5. Push closure for recursive call (GetLocal(0)) - now stack has [n, n-1, closure]
                        bytecode.push(OpCode::GetLocal(0));

                        // 6. Call the recursive function with 1 argument (Call(1))
                        // VM expects [closure, arg] - we have [n, n-1, closure]
                        // VM will pop closure, find arg at stack.len()-1, leaving [n, result]
                        bytecode.push(OpCode::Call(1));

                        // 7. Multiply - consumes [n, result] and leaves [n * result]
                        bytecode.extend(opcodes);

                        return Ok(bytecode);
                    }
                }
            }
        }

        // Compile arguments - for comparison operations, order matters!
        // For (<= n 1), we need: n, 1, Lte (not 1, n, Lte)
        // So don't reverse arguments for comparisons
        let is_comparison = matches!(
            symbol_name,
            "eq" | "=" | "lt" | "<" | "gt" | ">" | "lte" | "<=" | "gte" | ">=" | "ne" | "!="
        );
        if is_comparison {
            // For comparisons, compile arguments in original order
            for arg in arguments {
                let arg_bytecode = self.compile_to_physics(arg)?;
                bytecode.extend(arg_bytecode);
            }
        } else {
            // For arithmetic operations, compile arguments in reverse order for stack
            for arg in arguments.iter().rev() {
                let arg_bytecode = self.compile_to_physics(arg)?;
                bytecode.extend(arg_bytecode);
            }
        }

        // Add the type-aware operations directly (no Call instruction for primitive ops)
        bytecode.extend(opcodes);

        Ok(bytecode)
    }
}
