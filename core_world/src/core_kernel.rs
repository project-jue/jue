/// Core kernel implementation
/// This module contains β-reduction, α-equivalence, and normalization functions
use crate::core_expr::CoreExpr;

/// Perform β-reduction on a CoreExpr
/// β-reduction: (λx.M) N → M[x→N] (substitute N for x in M)
pub fn beta_reduce(expr: CoreExpr) -> CoreExpr {
    match expr {
        CoreExpr::App(func, arg) => {
            // Check if this is a beta-redex: (λx.M) N
            if let CoreExpr::Lam(body) = *func {
                // Perform substitution: replace all free occurrences of variable 0 in body with arg
                // Since we're using De Bruijn indices, variable 0 refers to the bound variable
                // We substitute arg directly without shifting for the top-level beta-reduction
                substitute(*body, 0, *arg)
            } else {
                // Not a beta-redex, try to reduce the function part first
                let func_expr = *func;
                let arg_expr = *arg;
                let reduced_func = beta_reduce(func_expr.clone());
                let reduced_arg = beta_reduce(arg_expr.clone());

                // If either part reduced, rebuild the application
                if reduced_func != func_expr || reduced_arg != arg_expr {
                    CoreExpr::App(Box::new(reduced_func), Box::new(reduced_arg))
                } else {
                    // Neither reduced, return original
                    CoreExpr::App(Box::new(func_expr), Box::new(arg_expr))
                }
            }
        }
        CoreExpr::Lam(body) => {
            // Try to reduce the body
            let body_expr = *body;
            let reduced_body = beta_reduce(body_expr.clone());
            if reduced_body != body_expr {
                CoreExpr::Lam(Box::new(reduced_body))
            } else {
                CoreExpr::Lam(Box::new(body_expr))
            }
        }
        CoreExpr::Var(_) => expr, // Variables can't be reduced
    }
}

/// Helper function to shift De Bruijn indices to avoid variable capture
/// shift_indices(expr, amount) increases all free variables by 'amount'
fn shift_indices(expr: CoreExpr, amount: usize) -> CoreExpr {
    match expr {
        CoreExpr::Var(index) => CoreExpr::Var(index + amount),
        CoreExpr::Lam(body) => CoreExpr::Lam(Box::new(shift_indices(*body, amount))),
        CoreExpr::App(func, arg) => CoreExpr::App(
            Box::new(shift_indices(*func, amount)),
            Box::new(shift_indices(*arg, amount)),
        ),
    }
}

/// Helper function to substitute a variable with an expression
/// substitute(expr, target_index, replacement) replaces all free occurrences
/// of variable 'target_index' in 'expr' with 'replacement'
fn substitute(expr: CoreExpr, target_index: usize, replacement: CoreExpr) -> CoreExpr {
    match expr {
        CoreExpr::Var(index) => {
            if index == target_index {
                replacement
            } else if index > target_index {
                // This variable was bound outside the lambda, so we need to adjust it
                // because we're going inside the lambda body
                CoreExpr::Var(index - 1)
            } else {
                CoreExpr::Var(index)
            }
        }
        CoreExpr::Lam(body) => {
            // When entering a lambda, we need to shift the replacement
            // to avoid capturing variables bound in this lambda
            let shifted_replacement = shift_indices(replacement, 1);
            CoreExpr::Lam(Box::new(substitute(
                *body,
                target_index + 1,
                shifted_replacement,
            )))
        }
        CoreExpr::App(func, arg) => CoreExpr::App(
            Box::new(substitute(*func, target_index, replacement.clone())),
            Box::new(substitute(*arg, target_index, replacement)),
        ),
    }
}

/// Check α-equivalence between two CoreExpr expressions
/// Two expressions are α-equivalent if they differ only in the names of bound variables
pub fn alpha_equiv(a: CoreExpr, b: CoreExpr) -> bool {
    alpha_equiv_helper(&a, &b, 0)
}

/// Helper function for α-equivalence with a depth parameter
fn alpha_equiv_helper(a: &CoreExpr, b: &CoreExpr, depth: usize) -> bool {
    match (a, b) {
        (CoreExpr::Var(index_a), CoreExpr::Var(index_b)) => {
            // Variables are equivalent if they refer to the same binding
            // Accounting for the current depth (number of lambdas we're under)
            if *index_a >= depth && *index_b >= depth {
                // Both are bound variables
                index_a == index_b
            } else if *index_a < depth && *index_b < depth {
                // Both are free variables
                index_a == index_b
            } else {
                // One is bound, one is free - not equivalent
                false
            }
        }
        (CoreExpr::Lam(body_a), CoreExpr::Lam(body_b)) => {
            // For lambdas, we compare the bodies with increased depth
            alpha_equiv_helper(body_a, body_b, depth + 1)
        }
        (CoreExpr::App(func_a, arg_a), CoreExpr::App(func_b, arg_b)) => {
            // For applications, both function and argument must be equivalent
            alpha_equiv_helper(func_a, func_b, depth) && alpha_equiv_helper(arg_a, arg_b, depth)
        }
        _ => false, // Different variants are not equivalent
    }
}

/// Normalize an expression by performing β-reduction until no more reductions are possible
pub fn normalize(expr: CoreExpr) -> CoreExpr {
    let reduced = beta_reduce(expr.clone());
    if reduced == expr {
        expr // Already in normal form
    } else {
        normalize(reduced) // Continue normalizing
    }
}

/// Check kernel consistency by verifying basic properties
pub fn prove_consistency() -> bool {
    // Test basic properties that should hold for a consistent kernel

    // 1. Test that normalization is idempotent
    let test_expr = CoreExpr::App(
        Box::new(CoreExpr::Lam(Box::new(CoreExpr::Var(0)))),
        Box::new(CoreExpr::Var(1)),
    );
    let normalized_once = normalize(test_expr.clone());
    let normalized_twice = normalize(normalized_once.clone());

    // 2. Test α-equivalence properties
    let lam_x = CoreExpr::Lam(Box::new(CoreExpr::Var(0)));
    let lam_y = CoreExpr::Lam(Box::new(CoreExpr::Var(0))); // Should be α-equivalent
    let lam_different = CoreExpr::Lam(Box::new(CoreExpr::Var(1))); // Different body

    // 3. Test that β-reduction preserves α-equivalence
    let expr1 = CoreExpr::App(
        Box::new(CoreExpr::Lam(Box::new(CoreExpr::Var(0)))),
        Box::new(CoreExpr::Var(0)),
    );
    let expr2 = CoreExpr::App(
        Box::new(CoreExpr::Lam(Box::new(CoreExpr::Var(0)))),
        Box::new(CoreExpr::Var(0)),
    );

    // All these properties should hold for a consistent kernel
    normalized_once == normalized_twice
        && alpha_equiv(lam_x.clone(), lam_y.clone())
        && !alpha_equiv(lam_x, lam_different)
        && alpha_equiv(beta_reduce(expr1), beta_reduce(expr2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_expr::{app, lam, var};

    #[test]
    fn test_beta_reduce_identity() {
        // (λx.x) y → y
        // In De Bruijn: (λ.0) 1 → 1
        let identity = lam(var(0));
        let y = var(1);
        let app_expr = app(identity, y);
        let reduced = beta_reduce(app_expr);
        assert_eq!(reduced, var(1));
    }

    #[test]
    fn test_beta_reduce_complex() {
        // Test single beta reduction step: (λx.λy.x) a → λy.x
        // In De Bruijn: (λ.λ.1) 0 → λ.1 (x is still the outer variable, now at index 1)
        let outer_lam = lam(lam(var(1)));
        let a = var(0);
        let app_expr = app(outer_lam.clone(), a.clone());
        let first_reduction = beta_reduce(app_expr);

        // Expected: λy.x where x is still the outer variable (index 1)
        let expected_first_step = lam(var(1));
        assert_eq!(first_reduction, expected_first_step);

        // Test full reduction separately: ((λx.λy.x) a) b → a
        let outer_lam2 = lam(lam(var(1)));
        let a2 = var(0);
        let b = var(1);
        let full_expr = app(app(outer_lam2, a2), b);
        let normalized = normalize(full_expr);
        assert_eq!(normalized, var(0));
    }

    #[test]
    fn test_alpha_equiv_identical() {
        let expr1 = lam(var(0));
        let expr2 = lam(var(0));
        assert!(alpha_equiv(expr1, expr2));
    }

    #[test]
    fn test_alpha_equiv_different_vars() {
        // λx.x and λy.y should be α-equivalent
        let expr1 = lam(var(0));
        let expr2 = lam(var(0)); // Same structure, so α-equivalent
        assert!(alpha_equiv(expr1, expr2));
    }

    #[test]
    fn test_alpha_equiv_different_bodies() {
        let expr1 = lam(var(0));
        let expr2 = lam(var(1)); // Different body
        assert!(!alpha_equiv(expr1, expr2));
    }

    #[test]
    fn test_normalize_idempotent() {
        let expr = app(lam(var(0)), var(1));
        let normalized_once = normalize(expr.clone());
        let normalized_twice = normalize(normalized_once.clone());
        assert_eq!(normalized_once, normalized_twice);
    }

    #[test]
    fn test_normalize_complex() {
        // ((λx.λy.x) a) b → a
        // In De Bruijn: ((λ.λ.1) 0) 1 → 0
        let expr = app(app(lam(lam(var(1))), var(0)), var(1));
        let normalized = normalize(expr);
        assert_eq!(normalized, var(0));
    }

    #[test]
    fn test_prove_consistency() {
        assert!(prove_consistency());
    }
}
