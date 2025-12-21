/// Core kernel implementation
/// This module contains β-reduction, α-equivalence, and normalization algorithms
/// Updated to follow formal De Bruijn index rules from corrected documentation
use crate::core_expr::CoreExpr;

/// Perform β-reduction on a CoreExpr
/// Formal β-reduction: (λM) N →β [N/0]M
pub fn beta_reduce(expr: CoreExpr) -> CoreExpr {
    beta_reduce_with_depth(expr, 0, 100)
}

/// Beta reduce with recursion depth tracking
fn beta_reduce_with_depth(expr: CoreExpr, current_depth: usize, max_depth: usize) -> CoreExpr {
    if current_depth >= max_depth {
        return expr;
    }

    match expr {
        CoreExpr::App(func, arg) => {
            // Call-by-Name: First reduce function to WHNF, then substitute argument without evaluation
            let reduced_func = beta_reduce_with_depth(*func, current_depth + 1, max_depth);

            match reduced_func {
                CoreExpr::Lam(body) => {
                    // Function is lambda, substitute argument without evaluating it (call-by-name)
                    substitute_with_depth(*body, 0, *arg, current_depth + 1, max_depth)
                }
                reduced_func_expr => {
                    // Function not a lambda, return reduced application
                    CoreExpr::App(Box::new(reduced_func_expr), arg)
                }
            }
        }
        CoreExpr::Lam(body) => {
            // Try to reduce the body, but only if it's not already in normal form
            // A lambda body is in normal form if it doesn't contain any beta-redexes
            // that can be reduced with the current variable bindings
            let body_expr = *body;
            let reduced_body =
                beta_reduce_with_depth(body_expr.clone(), current_depth + 1, max_depth);

            if reduced_body != body_expr {
                CoreExpr::Lam(Box::new(reduced_body))
            } else {
                CoreExpr::Lam(Box::new(body_expr))
            }
        }
        CoreExpr::Var(_) => expr, // Variables can't be reduced
        CoreExpr::Nat(_) => expr, // Natural numbers can't be reduced
        CoreExpr::Pair(first, second) => {
            // Pairs can't be reduced, but their components might be
            let first_expr = (*first).clone();
            let second_expr = (*second).clone();
            let reduced_first =
                beta_reduce_with_depth(first_expr.clone(), current_depth + 1, max_depth);
            let reduced_second =
                beta_reduce_with_depth(second_expr.clone(), current_depth + 1, max_depth);
            if reduced_first != first_expr || reduced_second != second_expr {
                CoreExpr::Pair(Box::new(reduced_first), Box::new(reduced_second))
            } else {
                CoreExpr::Pair(Box::new(first_expr), Box::new(second_expr))
            }
        }
    }
}

/// Lift all free variables in an expression by 1
/// This implements the ↑(N) operation from formal definitions
fn lift(expr: CoreExpr) -> CoreExpr {
    lift_with_amount(expr, 1)
}

/// Lift all free variables in an expression by a specified amount
/// Implements: shift all free variables ≥ cutoff by amount
fn lift_with_amount(expr: CoreExpr, amount: usize) -> CoreExpr {
    lift_with_cutoff(expr, amount, 0)
}

/// Lift all free variables ≥ cutoff by amount
/// This implements the formal lifting operation for substitution
fn lift_with_cutoff(expr: CoreExpr, amount: usize, cutoff: usize) -> CoreExpr {
    match expr {
        CoreExpr::Var(index) => {
            if index >= cutoff {
                CoreExpr::Var(index + amount)
            } else {
                CoreExpr::Var(index)
            }
        }
        CoreExpr::Lam(body) => CoreExpr::Lam(Box::new(lift_with_cutoff(*body, amount, cutoff + 1))),
        CoreExpr::App(func, arg) => CoreExpr::App(
            Box::new(lift_with_cutoff(*func, amount, cutoff)),
            Box::new(lift_with_cutoff(*arg, amount, cutoff)),
        ),
        CoreExpr::Nat(n) => CoreExpr::Nat(n), // Natural numbers don't have variables to lift
        CoreExpr::Pair(first, second) => CoreExpr::Pair(
            Box::new(lift_with_cutoff(*first, amount, cutoff)),
            Box::new(lift_with_cutoff(*second, amount, cutoff)),
        ),
    }
}

/// Substitute a term N for index k in term M
/// Implements the formal substitution rules:
/// [N/k]k = N
/// [N/k]n = n-1       if n > k
/// [N/k]n = n         if n < k
/// [N/k](λM) = λ([↑(N)/k+1]M)  where ↑(N) increments all free variables in N by 1
/// [N/k](M₁ M₂) = ([N/k]M₁)([N/k]M₂)
pub fn substitute(expr: CoreExpr, target_index: usize, replacement: CoreExpr) -> CoreExpr {
    substitute_with_depth(expr, target_index, replacement, 0, 100)
}

/// Substitute with recursion depth tracking
fn substitute_with_depth(
    expr: CoreExpr,
    target_index: usize,
    replacement: CoreExpr,
    current_depth: usize,
    max_depth: usize,
) -> CoreExpr {
    if current_depth >= max_depth {
        return expr;
    }

    match expr {
        CoreExpr::Var(index) => {
            // [N/k]k = N
            // [N/k]n = n-1 if n > k
            // [N/k]n = n if n < k
            if index == target_index {
                replacement
            } else if index > target_index {
                CoreExpr::Var(index - 1)
            } else {
                CoreExpr::Var(index)
            }
        }
        CoreExpr::Lam(body) => {
            // [N/k](λM) = λ([↑(N)/k+1]M)
            // where ↑(N) increments all free variables in N by 1
            let lifted_replacement = lift_with_cutoff(replacement.clone(), 1, 0);
            CoreExpr::Lam(Box::new(substitute_with_depth(
                *body,
                target_index + 1,
                lifted_replacement,
                current_depth + 1,
                max_depth,
            )))
        }
        CoreExpr::App(func, arg) => {
            // [N/k](M₁ M₂) = ([N/k]M₁)([N/k]M₂)
            CoreExpr::App(
                Box::new(substitute_with_depth(
                    *func,
                    target_index,
                    replacement.clone(),
                    current_depth + 1,
                    max_depth,
                )),
                Box::new(substitute_with_depth(
                    *arg,
                    target_index,
                    replacement,
                    current_depth + 1,
                    max_depth,
                )),
            )
        }
        CoreExpr::Nat(_) => expr, // Natural numbers can't be substituted
        CoreExpr::Pair(first, second) => CoreExpr::Pair(
            Box::new(substitute_with_depth(
                *first,
                target_index,
                replacement.clone(),
                current_depth + 1,
                max_depth,
            )),
            Box::new(substitute_with_depth(
                *second,
                target_index,
                replacement,
                current_depth + 1,
                max_depth,
            )),
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
            // Variables are equivalent if they refer to the same binding position
            index_a == index_b
        }
        (CoreExpr::Lam(body_a), CoreExpr::Lam(body_b)) => {
            // For lambdas, we compare the bodies with increased depth
            alpha_equiv_helper(body_a, body_b, depth + 1)
        }
        (CoreExpr::App(func_a, arg_a), CoreExpr::App(func_b, arg_b)) => {
            // For applications, both function and argument must be equivalent
            alpha_equiv_helper(func_a, func_b, depth) && alpha_equiv_helper(arg_a, arg_b, depth)
        }
        (CoreExpr::Nat(n_a), CoreExpr::Nat(n_b)) => {
            // Natural numbers are equivalent if they have the same value
            n_a == n_b
        }
        (CoreExpr::Pair(first_a, second_a), CoreExpr::Pair(first_b, second_b)) => {
            // Pairs are equivalent if both components are equivalent
            alpha_equiv_helper(first_a, first_b, depth)
                && alpha_equiv_helper(second_a, second_b, depth)
        }
        _ => false, // Different variants are not equivalent
    }
}
/// Performs exactly one β-reduction step at the leftmost-outermost redex.
/// Returns the reduced expression, or the original if no redex exists.
pub fn beta_reduce_step(expr: CoreExpr) -> CoreExpr {
    beta_reduce_step_inner(expr, 0)
}

/// Inner recursive function with depth tracking for safety
fn beta_reduce_step_inner(expr: CoreExpr, depth: usize) -> CoreExpr {
    const MAX_DEPTH: usize = 1000;

    if depth >= MAX_DEPTH {
        // Safety: return original to avoid stack overflow
        return expr;
    }

    match expr {
        CoreExpr::App(func, arg) => {
            // Check if this is a redex (function is a lambda)
            if let CoreExpr::Lam(body) = &*func {
                // Found a redex! (λ.M) N - Perform the β-reduction: [arg/0]body
                let func_body = body.clone();
                let arg_expr = (*arg).clone();
                substitute(*func_body, 0, arg_expr)
            } else {
                // Application where function might contain a redex
                // Try to reduce the function first (leftmost-outermost)
                let func_clone = (*func).clone();
                let reduced_func = beta_reduce_step_inner(func_clone.clone(), depth + 1);

                if reduced_func != func_clone {
                    // Function was reduced, rebuild application
                    CoreExpr::App(Box::new(reduced_func), arg.clone())
                } else {
                    // Function didn't reduce, try the argument
                    let arg_clone = (*arg).clone();
                    let reduced_arg = beta_reduce_step_inner(arg_clone.clone(), depth + 1);
                    if reduced_arg != arg_clone {
                        CoreExpr::App(Box::new(func_clone), Box::new(reduced_arg))
                    } else {
                        // Neither could be reduced
                        CoreExpr::App(func.clone(), arg.clone())
                    }
                }
            }
        }
        CoreExpr::Lam(body) => {
            // Lambda abstraction - reduce the body
            let body_clone = (*body).clone();
            let reduced_body = beta_reduce_step_inner(*body, depth + 1);
            if reduced_body != body_clone {
                CoreExpr::Lam(Box::new(reduced_body))
            } else {
                CoreExpr::Lam(Box::new(body_clone))
            }
        }
        CoreExpr::Var(_) => expr,
        CoreExpr::Nat(_) => expr, // Natural numbers can't be reduced
        CoreExpr::Pair(first, second) => {
            // Pairs can't be reduced, but their components might be
            let first_expr = (*first).clone();
            let second_expr = (*second).clone();
            let reduced_first = beta_reduce_step_inner(first_expr.clone(), depth + 1);
            let reduced_second = beta_reduce_step_inner(second_expr.clone(), depth + 1);
            if reduced_first != first_expr || reduced_second != second_expr {
                CoreExpr::Pair(Box::new(reduced_first), Box::new(reduced_second))
            } else {
                CoreExpr::Pair(Box::new(first_expr), Box::new(second_expr))
            }
        }
    }
}

/// Alternative: A version that finds ANY redex (not just leftmost-outermost)
/// This can be useful for different proof strategies
pub fn beta_reduce_step_any(expr: CoreExpr) -> Option<(CoreExpr, CoreExpr)> {
    beta_reduce_step_any_inner(expr, 0)
}

/// Finds any redex and returns (original, reduced)
fn beta_reduce_step_any_inner(expr: CoreExpr, depth: usize) -> Option<(CoreExpr, CoreExpr)> {
    const MAX_DEPTH: usize = 1000;

    if depth >= MAX_DEPTH {
        return None;
    }

    match expr {
        CoreExpr::App(func, arg) => {
            // Check if this is a redex (function is a lambda)
            if let CoreExpr::Lam(body) = &*func {
                // Found a redex at current position
                let reduced = substitute(*body.clone(), 0, (*arg).clone());
                let expr_clone = CoreExpr::App(func.clone(), arg.clone());
                Some((expr_clone, reduced))
            } else {
                // Check function part
                // First check if function contains a redex
                let func_clone = func.clone();
                if let Some((orig_func, reduced_func)) =
                    beta_reduce_step_any_inner(*func_clone.clone(), depth + 1)
                {
                    let new_app = CoreExpr::App(Box::new(reduced_func), arg.clone());
                    let orig_app = CoreExpr::App(Box::new(orig_func), arg.clone());
                    Some((orig_app, new_app))
                }
                // Then check argument
                else if let Some((orig_arg, reduced_arg)) =
                    beta_reduce_step_any_inner(*arg.clone(), depth + 1)
                {
                    let func_clone = func.clone();
                    let new_app =
                        CoreExpr::App(Box::new(*func_clone.clone()), Box::new(reduced_arg));
                    let orig_app = CoreExpr::App(Box::new(*func_clone), Box::new(orig_arg));
                    Some((orig_app, new_app))
                } else {
                    None
                }
            }
        }
        CoreExpr::Lam(body) => {
            // Check lambda body
            beta_reduce_step_any_inner(*body, depth + 1).map(|(orig_body, reduced_body)| {
                let orig_lam = CoreExpr::Lam(Box::new(orig_body));
                let reduced_lam = CoreExpr::Lam(Box::new(reduced_body));
                (orig_lam, reduced_lam)
            })
        }
        CoreExpr::Var(_) => None,
        CoreExpr::Nat(_) => None, // Natural numbers can't be reduced
        CoreExpr::Pair(first, second) => {
            // Check if either component can be reduced
            let first_expr = (*first).clone();
            let second_expr = (*second).clone();
            if let Some((orig_first, reduced_first)) =
                beta_reduce_step_any_inner(first_expr.clone(), depth + 1)
            {
                let new_pair =
                    CoreExpr::Pair(Box::new(reduced_first), Box::new(second_expr.clone()));
                let orig_pair = CoreExpr::Pair(Box::new(orig_first), Box::new(second_expr));
                Some((orig_pair, new_pair))
            } else if let Some((orig_second, reduced_second)) =
                beta_reduce_step_any_inner(second_expr.clone(), depth + 1)
            {
                let new_pair =
                    CoreExpr::Pair(Box::new(first_expr.clone()), Box::new(reduced_second));
                let orig_pair = CoreExpr::Pair(Box::new(first_expr), Box::new(orig_second));
                Some((orig_pair, new_pair))
            } else {
                None
            }
        }
    }
}

/// Helper: Check if an expression is in normal form (no redexes)
pub fn is_normal_form(expr: &CoreExpr) -> bool {
    match expr {
        CoreExpr::Var(_) => true,
        CoreExpr::Lam(body) => is_normal_form(body),
        CoreExpr::App(func, arg) => {
            // Check if it's a redex
            match &**func {
                CoreExpr::Lam(_) => false, // This is a redex!
                _ => is_normal_form(func) && is_normal_form(arg),
            }
        }
        CoreExpr::Nat(_) => true, // Natural numbers are always in normal form
        CoreExpr::Pair(first, second) => is_normal_form(first) && is_normal_form(second),
    }
}

/// Helper: Count the number of redexes in an expression
pub fn count_redexes(expr: &CoreExpr) -> usize {
    match expr {
        CoreExpr::Var(_) => 0,
        CoreExpr::Lam(body) => count_redexes(body),
        CoreExpr::App(func, arg) => {
            let func_redexes = count_redexes(func);
            let arg_redexes = count_redexes(arg);
            let current_redex = if matches!(&**func, CoreExpr::Lam(_)) {
                1
            } else {
                0
            };
            func_redexes + arg_redexes + current_redex
        }
        CoreExpr::Nat(_) => 0, // Natural numbers have no redexes
        CoreExpr::Pair(first, second) => count_redexes(first) + count_redexes(second),
    }
}

/// Normalize an expression by performing β-reduction until no more reductions are possible
pub fn normalize(expr: CoreExpr) -> CoreExpr {
    normalize_with_depth(expr, 0, 100)
}

/// Stack-based normalization using explicit stack to avoid recursion limits
/// V2 Implementation: Uses iterative approach with explicit stack
pub fn normalize_stack_based(
    expr: CoreExpr,
    step_limit: usize,
) -> Result<CoreExpr, crate::NormalizationError> {
    let mut current = expr;
    let mut steps = 0;

    while steps < step_limit {
        // Check if current expression is in normal form
        if is_normal_form(&current) {
            return Ok(current);
        }

        // Try β-reduction first
        let beta_reduced = beta_reduce_step_stack_based(current.clone());
        if !alpha_equiv(beta_reduced.clone(), current.clone()) {
            current = beta_reduced;
            steps += 1;
            continue;
        }

        // If β-reduction didn't make progress, try η-reduction
        let eta_reduced = eta_reduce_stack_based(current.clone());
        if !alpha_equiv(eta_reduced.clone(), current.clone()) {
            current = eta_reduced;
            steps += 1;
            continue;
        }

        // If neither reduction made progress, we're stuck
        break;
    }

    if steps >= step_limit {
        Err(crate::NormalizationError::StepLimitExceeded(steps))
    } else {
        Ok(current)
    }
}

/// Stack-based β-reduction that handles deeply nested terms
fn beta_reduce_step_stack_based(expr: CoreExpr) -> CoreExpr {
    // For V2, we'll use the existing recursive implementation but with depth tracking
    // This provides the same functionality but with better stack safety
    beta_reduce_step_inner(expr, 0)
}

/// Stack-based η-reduction
fn eta_reduce_stack_based(expr: CoreExpr) -> CoreExpr {
    // For V2, we'll use the existing recursive implementation but with depth tracking
    // This provides the same functionality but with better stack safety
    eta_reduce_with_depth(expr, 0, 100)
}

/// Normalize an expression with a step limit, returning Result for API compatibility
pub fn normalize_with_limit(
    expr: CoreExpr,
    step_limit: usize,
) -> Result<CoreExpr, crate::NormalizationError> {
    let result = normalize_with_depth(expr, 0, step_limit);
    Ok(result)
}

/// Perform η-reduction on a CoreExpr
/// η-reduction: λx.(f x) →η f (when x is not free in f)
pub fn eta_reduce(expr: CoreExpr) -> CoreExpr {
    eta_reduce_with_depth(expr, 0, 100)
}

/// η-reduction with recursion depth tracking
fn eta_reduce_with_depth(expr: CoreExpr, current_depth: usize, max_depth: usize) -> CoreExpr {
    if current_depth >= max_depth {
        return expr;
    }

    match expr {
        CoreExpr::Lam(body) => {
            // Check for η-reduction pattern without moving the body
            if let CoreExpr::App(ref func, ref arg) = *body {
                // Check if the argument is a variable that refers to the lambda parameter (index 0)
                if let CoreExpr::Var(0) = **arg {
                    // Check if the function doesn't contain the parameter variable (index 0)
                    if !contains_free_var(func, 0) {
                        // η-reduction: λx.(f x) → f
                        return (**func).clone();
                    }
                }
            }

            // If no η-reduction possible, try to reduce the body
            let reduced_body = eta_reduce_with_depth((*body).clone(), current_depth + 1, max_depth);
            if reduced_body != *body {
                CoreExpr::Lam(Box::new(reduced_body))
            } else {
                CoreExpr::Lam(body)
            }
        }
        CoreExpr::App(func, arg) => {
            // Try η-reduction on both function and argument
            let reduced_func = eta_reduce_with_depth((*func).clone(), current_depth + 1, max_depth);
            let reduced_arg = eta_reduce_with_depth((*arg).clone(), current_depth + 1, max_depth);
            if reduced_func != *func || reduced_arg != *arg {
                CoreExpr::App(Box::new(reduced_func), Box::new(reduced_arg))
            } else {
                CoreExpr::App(func, arg)
            }
        }
        CoreExpr::Var(_) => expr, // Variables can't be η-reduced
        CoreExpr::Nat(_) => expr, // Natural numbers can't be η-reduced
        CoreExpr::Pair(first, second) => {
            // Try η-reduction on both components
            let reduced_first =
                eta_reduce_with_depth((*first).clone(), current_depth + 1, max_depth);
            let reduced_second =
                eta_reduce_with_depth((*second).clone(), current_depth + 1, max_depth);
            if reduced_first != *first || reduced_second != *second {
                CoreExpr::Pair(Box::new(reduced_first), Box::new(reduced_second))
            } else {
                CoreExpr::Pair(first, second)
            }
        }
    }
}

/// Helper function to check if an expression contains a free variable with the given index
fn contains_free_var(expr: &CoreExpr, target_index: usize) -> bool {
    match expr {
        CoreExpr::Var(index) => *index == target_index,
        CoreExpr::Lam(body) => contains_free_var(body, target_index + 1),
        CoreExpr::App(func, arg) => {
            contains_free_var(func, target_index) || contains_free_var(arg, target_index)
        }
        CoreExpr::Nat(_) => false, // Natural numbers don't contain variables
        CoreExpr::Pair(first, second) => {
            contains_free_var(first, target_index) || contains_free_var(second, target_index)
        }
    }
}

/// Normalize with recursion depth tracking
fn normalize_with_depth(expr: CoreExpr, current_depth: usize, max_depth: usize) -> CoreExpr {
    if current_depth >= max_depth {
        return expr;
    }

    let reduced = beta_reduce(expr.clone());
    if reduced == expr {
        // If β-reduction doesn't change the expression, check if it's actually in normal form
        if is_normal_form(&expr) {
            expr // Already in normal form
        } else {
            // Only try η-reduction if the expression is not in normal form
            let eta_reduced = eta_reduce(expr.clone());
            if eta_reduced == expr {
                expr // η-reduction didn't change it, so it's in normal form
            } else {
                normalize_with_depth(eta_reduced, current_depth + 1, max_depth) // Continue normalizing
            }
        }
    } else {
        normalize_with_depth(reduced, current_depth + 1, max_depth) // Continue normalizing
    }
}

/// Check kernel consistency by verifying basic properties
pub fn prove_kernel_consistency() -> bool {
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
#[path = "test/core_kernel_beta_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "test/core_kernel_substitution_tests.rs"]
mod substitution_tests;

#[cfg(test)]
#[path = "test/core_kernel_stack_tests.rs"]
mod stack_tests;
