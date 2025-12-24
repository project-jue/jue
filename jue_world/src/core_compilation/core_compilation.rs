use crate::error::{CompilationError, SourceLocation};
use core_world::core_expr::{app, lam, nat, var, CoreExpr};
use core_world::proof_checker::{prove_beta, prove_eta, prove_normalization, verify, Proof};

/// Compile AST to CoreExpr with proof obligations
pub fn compile_ast_to_core_expr_with_proofs(
    ast: &crate::ast::AstNode,
) -> Result<(CoreExpr, Option<Proof>), CompilationError> {
    match ast {
        crate::ast::AstNode::Literal(crate::ast::Literal::Int(n)) => {
            let expr = nat(*n as u64);
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::Literal(crate::ast::Literal::Bool(b)) => {
            // Convert boolean to natural number (true = 1, false = 0)
            let expr = nat(if *b { 1 } else { 0 });
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::Literal(crate::ast::Literal::Nil) => {
            // Nil can be represented as 0 in CoreExpr
            let expr = nat(0);
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::Literal(crate::ast::Literal::Float(f)) => {
            // Convert float to integer (truncate)
            let expr = nat(*f as u64);
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::Literal(crate::ast::Literal::String(s)) => {
            // Convert string to its length as a number
            let expr = nat(s.len() as u64);
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::Variable(_name) => {
            // For now, we'll use a simple variable mapping
            // In a real implementation, we'd need proper scope analysis
            // This is a placeholder that will need refinement
            let expr = var(0); // Default to index 0
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::Symbol(_sym) => {
            // Symbols need to be resolved to their definitions
            // For now, treat as variable
            let expr = var(0);
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::Call {
            function,
            arguments,
            ..
        } => {
            // Check if this is a built-in comparison operator
            if let crate::ast::AstNode::Symbol(sym) = &**function {
                match sym.as_str() {
                    ">" => {
                        // Compile greater-than comparison
                        if arguments.len() != 2 {
                            return Err(CompilationError::ParseError {
                                message: format!(
                                    "> expects exactly 2 arguments, got {}",
                                    arguments.len()
                                ),
                                location: SourceLocation::default(),
                            });
                        }
                        let _left_expr = compile_ast_to_core_expr(&arguments[0])?;
                        let _right_expr = compile_ast_to_core_expr(&arguments[1])?;
                        // For CoreExpr, we'll use a placeholder - real implementation would need proper comparison
                        let expr = nat(0); // Placeholder
                        return Ok((expr, None)); // No proof for comparison operators yet
                    }
                    "<" => {
                        // Compile less-than comparison
                        if arguments.len() != 2 {
                            return Err(CompilationError::ParseError {
                                message: format!(
                                    "< expects exactly 2 arguments, got {}",
                                    arguments.len()
                                ),
                                location: SourceLocation::default(),
                            });
                        }
                        let _left_expr = compile_ast_to_core_expr(&arguments[0])?;
                        let _right_expr = compile_ast_to_core_expr(&arguments[1])?;
                        // For CoreExpr, we'll use a placeholder - real implementation would need proper comparison
                        let expr = nat(0); // Placeholder
                        return Ok((expr, None)); // No proof for comparison operators yet
                    }
                    "=" => {
                        // Compile equality comparison
                        if arguments.len() != 2 {
                            return Err(CompilationError::ParseError {
                                message: format!(
                                    "= expects exactly 2 arguments, got {}",
                                    arguments.len()
                                ),
                                location: SourceLocation::default(),
                            });
                        }
                        let _left_expr = compile_ast_to_core_expr(&arguments[0])?;
                        let _right_expr = compile_ast_to_core_expr(&arguments[1])?;
                        // For CoreExpr, we'll use a placeholder - real implementation would need proper comparison
                        let expr = nat(0); // Placeholder
                        return Ok((expr, None)); // No proof for comparison operators yet
                    }
                    _ => {
                        // Regular function call
                    }
                }
            }

            // Compile function and arguments
            let (func_expr, func_proof) = compile_ast_to_core_expr_with_proofs(function)?;
            let arg_results: Result<Vec<(CoreExpr, Option<Proof>)>, CompilationError> = arguments
                .iter()
                .map(compile_ast_to_core_expr_with_proofs)
                .collect();

            let arg_results = arg_results?;
            let mut arg_exprs = Vec::new();
            let mut arg_proofs = Vec::new();

            for (arg_expr, arg_proof) in arg_results {
                arg_exprs.push(arg_expr);
                arg_proofs.push(arg_proof);
            }

            // Apply arguments in sequence: ((func arg1) arg2) arg3...
            let mut result = func_expr;
            let mut combined_proof = func_proof;

            for (i, arg) in arg_exprs.into_iter().enumerate() {
                let app_expr = app(result.clone(), arg.clone());

                // Generate proof for this application
                let step_proof = if let Some(func_p) = &combined_proof {
                    // If we have a proof for the function, we can generate a congruence proof
                    generate_application_proof(&result, &arg, func_p, arg_proofs[i].as_ref())
                } else {
                    // Otherwise, try to generate a simple proof
                    generate_simple_proof(&app_expr)
                };

                // Combine proofs using transitivity
                if let Some(step_p) = step_proof {
                    combined_proof = Some(combine_proofs(combined_proof, Some(step_p)));
                }

                result = app_expr;
            }

            Ok((result, combined_proof))
        }
        crate::ast::AstNode::Lambda {
            parameters, body, ..
        } => {
            // Compile the body
            let (body_expr, body_proof) = compile_ast_to_core_expr_with_proofs(body)?;

            // Create lambda abstraction
            // For multiple parameters, we need nested lambdas: λx.λy.body
            let mut result = body_expr;
            let mut combined_proof = body_proof;

            for _ in 0..parameters.len() {
                let lam_expr = lam(result.clone());

                // Generate congruence proof for lambda
                if let Some(body_p) = &combined_proof {
                    let lam_proof = Proof::CongLam {
                        proof_b: Box::new(body_p.clone()),
                    };
                    combined_proof = Some(lam_proof);
                }

                result = lam_expr;
            }

            Ok((result, combined_proof))
        }
        crate::ast::AstNode::Let { bindings, body, .. } => {
            // Let bindings: (let ((x val1) (y val2)) body)
            // Compile to: ((λx.λy.body) val1) val2
            let (body_expr, body_proof) = compile_ast_to_core_expr_with_proofs(body)?;

            // Create nested lambdas for each binding
            let mut result = body_expr;
            let mut combined_proof = body_proof;

            for (_, _) in bindings {
                let lam_expr = lam(result.clone());

                // Generate congruence proof for lambda
                if let Some(body_p) = &combined_proof {
                    let lam_proof = Proof::CongLam {
                        proof_b: Box::new(body_p.clone()),
                    };
                    combined_proof = Some(lam_proof);
                }

                result = lam_expr;
            }

            // Apply the values
            for (_, value) in bindings.iter().rev() {
                let (value_expr, value_proof) = compile_ast_to_core_expr_with_proofs(value)?;
                let app_expr = app(result.clone(), value_expr.clone());

                // Generate proof for this application
                let step_proof = if let Some(body_p) = &combined_proof {
                    generate_application_proof(&result, &value_expr, body_p, value_proof.as_ref())
                } else {
                    generate_simple_proof(&app_expr)
                };

                // Combine proofs using transitivity
                if let Some(step_p) = step_proof {
                    combined_proof = Some(combine_proofs(combined_proof, Some(step_p)));
                }

                result = app_expr;
            }

            Ok((result, combined_proof))
        }
        crate::ast::AstNode::If {
            condition,
            then_branch,
            else_branch,
            ..
        } => {
            // If expression: (if cond then else)
            // We'll compile this to a conditional using boolean logic
            // For now, we'll use a simple approach: (cond then else) where cond is 1 or 0
            let (_cond_expr, _) = compile_ast_to_core_expr_with_proofs(condition)?;
            let (then_expr, then_proof) = compile_ast_to_core_expr_with_proofs(then_branch)?;
            let (_else_expr, _) = compile_ast_to_core_expr_with_proofs(else_branch)?;

            // This is a simplified approach - real implementation would need proper conditional logic
            // For now, we'll just return the then branch (this is a placeholder)
            Ok((then_expr, then_proof))
        }
        crate::ast::AstNode::TrustTier { expression, .. } => {
            // Compile the inner expression
            compile_ast_to_core_expr_with_proofs(expression)
        }
        crate::ast::AstNode::RequireCapability { .. } => {
            // Capability requirements don't affect CoreExpr compilation
            // They're handled at the capability analysis phase
            let expr = nat(0); // Return dummy value
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::HasCapability { .. } => {
            // Capability checks don't affect CoreExpr compilation
            // They're handled at the bytecode generation phase
            let expr = nat(0); // Return dummy value
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::TypeSignature { .. } => {
            // Type signatures don't affect CoreExpr compilation
            let expr = nat(0); // Return dummy value
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::MacroDefinition { .. } => {
            // Macro definitions are expanded before compilation
            let expr = nat(0); // Return dummy value
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::MacroExpansion { .. } => {
            // Macro expansions are expanded before compilation
            let expr = nat(0); // Return dummy value
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::FfiCall { .. } => {
            // FFI calls need special handling
            // For now, return a dummy value
            let expr = nat(0);
            Ok((expr, None)) // No proof for FFI calls
        }
        crate::ast::AstNode::List { .. } => {
            // Lists can be represented as nested pairs
            // For now, return a dummy value
            let expr = nat(0);
            let proof = Proof::Refl(expr.clone());
            Ok((expr, Some(proof)))
        }
        crate::ast::AstNode::Cons { car, cdr, .. } => {
            // Cons creates a pair
            let (car_expr, car_proof) = compile_ast_to_core_expr_with_proofs(car)?;
            let (cdr_expr, cdr_proof) = compile_ast_to_core_expr_with_proofs(cdr)?;

            let pair_expr = core_world::core_expr::pair(car_expr, cdr_expr);

            // Generate congruence proof for pair
            let pair_proof = if let (Some(car_p), Some(cdr_p)) = (car_proof, cdr_proof) {
                Some(Proof::CongApp {
                    proof_f: Box::new(car_p),
                    proof_a: Box::new(cdr_p),
                })
            } else {
                generate_simple_proof(&pair_expr)
            };

            Ok((pair_expr, pair_proof))
        }
    }
}

/// Generate a proof for function application using congruence
fn generate_application_proof(
    func: &CoreExpr,
    arg: &CoreExpr,
    func_proof: &Proof,
    arg_proof: Option<&Proof>,
) -> Option<Proof> {
    if let Some(arg_p) = arg_proof {
        Some(Proof::CongApp {
            proof_f: Box::new(func_proof.clone()),
            proof_a: Box::new(arg_p.clone()),
        })
    } else {
        // If we don't have a proof for the argument, try to generate a simple proof
        generate_simple_proof(&app(func.clone(), arg.clone()))
    }
}

/// Combine two proofs using transitivity
fn combine_proofs(proof1: Option<Proof>, proof2: Option<Proof>) -> Proof {
    match (proof1, proof2) {
        (Some(p1), Some(p2)) => Proof::Trans {
            proof_a: Box::new(p1),
            proof_b: Box::new(p2),
        },
        (Some(p), None) | (None, Some(p)) => p,
        (None, None) => Proof::Refl(CoreExpr::Var(0)), // Fallback
    }
}

/// Compile AST to CoreExpr (original function for backward compatibility)
pub fn compile_ast_to_core_expr(ast: &crate::ast::AstNode) -> Result<CoreExpr, CompilationError> {
    let (expr, _) = compile_ast_to_core_expr_with_proofs(ast)?;
    Ok(expr)
}

/// Generate a proof for the CoreExpr
pub fn generate_simple_proof(expr: &CoreExpr) -> Option<Proof> {
    // Try to generate a normalization proof
    match prove_normalization(expr.clone(), 100) {
        Ok(proof) => Some(proof),
        Err(_) => {
            // If normalization proof fails, try to generate a reflexivity proof
            // for expressions that are already in normal form
            if core_world::core_kernel::is_normal_form(expr) {
                Some(Proof::Refl(expr.clone()))
            } else {
                // If we can't generate a proof, return None
                None
            }
        }
    }
}

/// Generate a proof for a specific reduction step
pub fn generate_reduction_proof(expr: &CoreExpr) -> Option<Proof> {
    // Try beta reduction first
    let beta_reduced = core_world::core_kernel::beta_reduce_step(expr.clone());
    if !core_world::core_kernel::alpha_equiv(beta_reduced.clone(), expr.clone()) {
        return Some(prove_beta(expr.clone()));
    }

    // Try eta reduction if beta didn't make progress
    let eta_reduced = core_world::core_kernel::eta_reduce(expr.clone());
    if !core_world::core_kernel::alpha_equiv(eta_reduced.clone(), expr.clone()) {
        return match prove_eta(expr.clone()) {
            Ok(proof) => Some(proof),
            Err(_) => None,
        };
    }

    // If no reduction possible, return reflexivity proof
    Some(Proof::Refl(expr.clone()))
}

/// Generate a comprehensive proof for lambda calculus operations
pub fn generate_comprehensive_proof(expr: &CoreExpr) -> Option<Proof> {
    // First, try to generate a normalization proof
    if let Some(normalization_proof) = generate_simple_proof(expr) {
        return Some(normalization_proof);
    }

    // If normalization fails, try to generate a step-by-step reduction proof
    let mut current_expr = expr.clone();
    let mut step_proofs = Vec::new();

    // Generate reduction steps until we reach normal form or hit step limit
    for _ in 0..50 {
        if core_world::core_kernel::is_normal_form(&current_expr) {
            break;
        }

        if let Some(step_proof) = generate_reduction_proof(&current_expr) {
            step_proofs.push(step_proof.clone());
            // Update current expression by applying the reduction
            match step_proof {
                Proof::BetaStep { contractum, .. } => {
                    current_expr = contractum;
                }
                Proof::EtaStep { contractum, .. } => {
                    current_expr = contractum;
                }
                _ => break,
            }
        } else {
            break;
        }
    }

    // Chain the step proofs together with transitivity
    if step_proofs.is_empty() {
        return Some(Proof::Refl(expr.clone()));
    }

    let mut combined_proof = step_proofs[0].clone();
    for step_proof in step_proofs.iter().skip(1) {
        combined_proof = Proof::Trans {
            proof_a: Box::new(combined_proof),
            proof_b: Box::new(step_proof.clone()),
        };
    }

    Some(combined_proof)
}

/// Verify a proof against the Core-World kernel
pub fn verify_proof_against_kernel(
    proof: &Proof,
) -> Result<(CoreExpr, CoreExpr), CompilationError> {
    // Use the Core-World proof checker to verify the proof
    match verify(proof) {
        Ok((left, right)) => {
            // The Core-World kernel's verify function already checks equivalence
            // We just need to return the result
            Ok((left, right))
        }
        Err(proof_error) => Err(CompilationError::ProofGenerationFailed(format!(
            "Proof verification failed: {}",
            proof_error
        ))),
    }
}

/// Verify that a CoreExpr and its proof are consistent
pub fn verify_core_expr_with_proof(expr: &CoreExpr, proof: &Proof) -> Result<(), CompilationError> {
    // Verify the proof
    let (left, right) = verify_proof_against_kernel(proof)?;

    // Check that the original expression matches the left side of the proof
    if !core_world::core_kernel::alpha_equiv(expr.clone(), left) {
        return Err(CompilationError::ProofGenerationFailed(
            "Proof does not match original expression".to_string(),
        ));
    }

    // Check that the right side is the normal form
    let normal_form = core_world::core_kernel::normalize(expr.clone());
    if !core_world::core_kernel::alpha_equiv(right, normal_form) {
        return Err(CompilationError::ProofGenerationFailed(
            "Proof does not reduce to normal form".to_string(),
        ));
    }

    Ok(())
}

/// Compile CoreExpr to Physics World bytecode
pub fn compile_core_expr_to_bytecode(
    expr: &CoreExpr,
) -> (
    Vec<physics_world::types::OpCode>,
    Vec<physics_world::types::Value>,
) {
    let mut bytecode = Vec::new();
    let mut constants = Vec::new();

    // Simple compilation strategy for now
    match expr {
        CoreExpr::Var(_index) => {
            // Variables would need to be resolved to stack positions
            // For now, push a dummy value
            bytecode.push(physics_world::types::OpCode::Int(0));
        }
        CoreExpr::Lam(_body) => {
            // Lambda compilation is complex - for now push dummy
            bytecode.push(physics_world::types::OpCode::Int(0));
        }
        CoreExpr::App(func, arg) => {
            // Compile function and argument
            let (func_bytecode, func_constants) = compile_core_expr_to_bytecode(func);
            let (arg_bytecode, arg_constants) = compile_core_expr_to_bytecode(arg);

            // Combine bytecode and constants
            bytecode.extend(func_bytecode);
            bytecode.extend(arg_bytecode);
            bytecode.push(physics_world::types::OpCode::Add); // Placeholder - real implementation would handle application

            constants.extend(func_constants);
            constants.extend(arg_constants);
        }
        CoreExpr::Nat(n) => {
            // Convert natural number to integer
            bytecode.push(physics_world::types::OpCode::Int(*n as i64));
        }
        CoreExpr::Pair(first, second) => {
            // Compile pair elements
            let (first_bytecode, first_constants) = compile_core_expr_to_bytecode(first);
            let (second_bytecode, second_constants) = compile_core_expr_to_bytecode(second);

            // Combine bytecode
            bytecode.extend(first_bytecode);
            bytecode.extend(second_bytecode);
            bytecode.push(physics_world::types::OpCode::Cons); // Create pair

            constants.extend(first_constants);
            constants.extend(second_constants);
        }
    }

    (bytecode, constants)
}
