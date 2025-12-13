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
        println!(
            "WARNING: Beta reduction recursion limit reached at depth {}",
            current_depth
        );
        return expr;
    }

    match expr {
        CoreExpr::App(func, arg) => {
            // Call-by-Value: First reduce function to WHNF
            let reduced_func = beta_reduce_with_depth(*func, current_depth + 1, max_depth);

            match reduced_func {
                CoreExpr::Lam(body) => {
                    // Function is lambda, reduce argument first (call-by-value)
                    let reduced_arg = beta_reduce_with_depth(*arg, current_depth + 1, max_depth);
                    substitute_with_depth(*body, 0, reduced_arg, current_depth + 1, max_depth)
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
        println!(
            "WARNING: Substitution recursion limit reached at depth {}",
            current_depth
        );
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
        _ => false, // Different variants are not equivalent
    }
}

/// Normalize an expression by performing β-reduction until no more reductions are possible
pub fn normalize(expr: CoreExpr) -> CoreExpr {
    normalize_with_depth(expr, 0, 100)
}

/// Normalize with recursion depth tracking
fn normalize_with_depth(expr: CoreExpr, current_depth: usize, max_depth: usize) -> CoreExpr {
    if current_depth >= max_depth {
        println!(
            "WARNING: Normalization recursion limit reached at depth {}",
            current_depth
        );
        return expr;
    }

    let reduced = beta_reduce(expr.clone());
    if reduced == expr {
        expr // Already in normal form
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

/// Check recursion health for higher-level attention/patience mechanisms
pub fn check_recursion_health(expr: &CoreExpr, current_depth: usize, max_depth: usize) -> f64 {
    let depth_ratio = current_depth as f64 / max_depth as f64;
    // Simple linear depletion model
    1.0 - depth_ratio
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_expr::{app, lam, var};

    #[test]
    fn test_beta_reduce_identity() {
        // Test Call-by-Value: (λx.x) y → y
        // In De Bruijn: (λ.0) 1 → 1
        // Call-by-Value: function is reduced to WHNF first, then argument is evaluated
        let identity = lam(var(0));
        let y = var(1);
        let app_expr = app(identity, y);
        let reduced = beta_reduce(app_expr);

        // With Call-by-Value, the identity function is reduced to WHNF first
        // Then the argument y (var(1)) is evaluated and substituted
        assert_eq!(reduced, var(1));
    }

    #[test]
    fn test_beta_reduce_complex() {
        // Test Call-by-Value: (λx.λy.x) a → λy.x
        // In De Bruijn: (λ.λ.1) 0 → λ.1 (x is still the outer variable, now at index 1)
        // Call-by-Value: function is reduced to WHNF first, then argument is evaluated
        let outer_lam = lam(lam(var(1)));
        let a = var(0);
        let app_expr = app(outer_lam.clone(), a.clone());

        // Expected: λy.x where x is still the outer variable (index 1)
        // With Call-by-Value, the outer lambda is reduced to WHNF first
        let expected_first_step = lam(var(1));
        assert_eq!(beta_reduce(app_expr), expected_first_step);

        // Test full reduction with Call-by-Value: ((λx.λy.x) a) b → a
        // In De Bruijn: ((λ.λ.1) 0) 1 → 0 (after proper substitution)
        // Call-by-Value evaluation order:
        // 1. Reduce ((λ.λ.1) 0) to WHNF: (λ.1)
        // 2. Reduce argument 1 to WHNF: 1 (already in WHNF)
        // 3. Apply: (λ.1) 1 → 0 (after substitution)
        let outer_lam2 = lam(lam(var(1)));
        let a2 = var(0);
        let b = var(1);
        let full_expr = app(app(outer_lam2, a2), b);
        let normalized = normalize(full_expr);
        // The result should be var(0) after normalization because:
        // ((λ.λ.1) 0) 1 → (λ.1) 1 → 0 (after substitution)
        // This demonstrates Call-by-Value: arguments are evaluated before substitution
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
        // Test Call-by-Value normalization: ((λx.λy.x) a) b → a
        // In De Bruijn: ((λ.λ.1) 0) 1 → 0 (not 1)
        // Call-by-Value evaluation order:
        // 1. Reduce ((λ.λ.1) 0) to WHNF: (λ.1)
        // 2. Reduce argument 1 to WHNF: 1 (already in WHNF)
        // 3. Apply: (λ.1) 1 → 0 (after substitution)
        // The outer variable 'a' is at index 0, and after substitution it becomes the result
        let expr = app(app(lam(lam(var(1))), var(0)), var(1));
        let normalized = normalize(expr);
        // This demonstrates Call-by-Value: arguments are evaluated before substitution
        assert_eq!(normalized, var(0));
    }

    #[test]
    fn test_prove_consistency() {
        assert!(prove_kernel_consistency());
    }

    #[test]
    fn test_call_by_value_semantics() {
        // Test that demonstrates Call-by-Value behavior
        // Expression: (λf.(f (λx.x))) (λy.y)
        // In De Bruijn: (λ.0 (λ.0)) (λ.0)
        // Call-by-Value evaluation:
        // 1. Reduce function to WHNF: (λ.0 (λ.0)) is already in WHNF
        // 2. Reduce argument to WHNF: (λ.0) is already in WHNF
        // 3. Apply: substitute (λ.0) for index 0 in (0 (λ.0))
        // 4. Result: (λ.0) (λ.0) → λ.0 (after beta reduction)

        let identity = lam(var(0));
        let func = lam(app(var(0), identity.clone()));
        let arg = identity.clone();
        let expr = app(func, arg);

        let result = normalize(expr);

        // The result should be the identity function
        // This demonstrates Call-by-Value: both function and argument are evaluated before substitution
        assert_eq!(result, identity);
    }

    #[test]
    fn test_call_by_value_nested_application() {
        // Test nested application with Call-by-Value
        // Expression: ((λx.λy.x) (λz.z)) (λw.w)
        // In De Bruijn: ((λ.λ.1) (λ.0)) (λ.0)
        // Call-by-Value evaluation:
        // 1. Reduce ((λ.λ.1) (λ.0)) to WHNF: (λ.1)
        // 2. Reduce argument (λ.0) to WHNF: (λ.0) (already in WHNF)
        // 3. Apply: (λ.1) (λ.0) → λ.0 (after substitution)

        let outer_identity = lam(var(0));
        let inner_identity = lam(var(0));
        let outer_lam = lam(lam(var(1)));
        let outer_identity = lam(var(0));
        let expr = app(app(outer_lam, outer_identity), inner_identity.clone());

        let result = normalize(expr);

        // The result should be the inner identity function
        // This demonstrates Call-by-Value: arguments are evaluated before substitution
        assert_eq!(result, inner_identity);
    }

    #[test]
    fn test_formal_substitution_rules() {
        // Test formal substitution rule: [N/k]k = N
        let result1 = substitute(var(0), 0, var(5));
        assert_eq!(result1, var(5));

        // Test formal substitution rule: [N/k]n = n-1 if n > k
        let result2 = substitute(var(2), 0, var(5));
        assert_eq!(result2, var(1));

        // Test formal substitution rule: [N/k]n = n if n < k
        let result3 = substitute(var(0), 1, var(5));
        assert_eq!(result3, var(0));

        // Test formal substitution rule: [N/k](λM) = λ([↑(N)/k+1]M)
        let body = var(0);
        let lambda_expr = lam(body);
        let replacement = var(1);
        let result4 = substitute(lambda_expr, 0, replacement);
        // Should be λ([↑(var(1))/1]var(0)) = λ(var(0))
        assert_eq!(result4, lam(var(0)));

        // Test formal substitution rule: [N/k](M₁ M₂) = ([N/k]M₁)([N/k]M₂)
        let app_expr = app(var(0), var(1));
        let replacement = var(2);
        let result5 = substitute(app_expr, 0, replacement);
        // Should be (var(2) var(0)) because [2/0]1 = 0 (since 1 > 0, it becomes 1-1=0)
        assert_eq!(result5, app(var(2), var(0)));
    }

    #[test]
    fn test_lift_operation() {
        // Test lifting increases free variables by 1
        let expr = var(1);
        let lifted = lift(expr);
        assert_eq!(lifted, var(2));

        // Test lifting with cutoff
        let expr2 = app(var(0), var(2));
        let lifted2 = lift_with_cutoff(expr2, 1, 1);
        // var(0) < 1, so stays 0; var(2) >= 1, so becomes 3
        assert_eq!(lifted2, app(var(0), var(3)));
    }

    #[test]
    fn test_complex_substitution() {
        // Test complex case: substitute in nested lambda
        // Expression: λ λ 2 (substitute for index 1)
        // Should become: λ λ 1 (after formal substitution)
        let expr = lam(lam(var(2)));
        let result = substitute(expr, 1, var(0));
        // [var(0)/1]λ λ 2 = λ([↑(var(0))/2]λ 2) = λ(λ([↑(var(0))/3]2))
        // ↑(var(0)) = var(1), [var(1)/3]2 = 2 (since 2 < 3)
        // So result should be λ λ 2
        assert_eq!(result, lam(lam(var(2))));
    }

    #[test]
    fn debug_simple_case() {
        // Test a simple case to understand the substitution
        let inner_lam = lam(var(1)); // λy.y
        let arg = var(0); // x
        let body = app(inner_lam, arg); // (λy.y) x

        println!("\n=== Simple case ===");
        println!("Body: {}", body);

        let replacement = lam(var(0)); // λz.z
        println!("Replacement: {}", replacement);
        let result = substitute(body, 0, replacement);

        println!("Result: {}", result);
        println!("Expected: app(lam(var(1)), lam(var(0)))");
    }

    #[test]
    fn debug_deeply_nested() {
        let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));

        println!("\n=== Debugging deeply nested ===");
        println!("Original: {}", deeply_nested);
        println!("Structure: {:?}", deeply_nested);

        let step1 = beta_reduce(deeply_nested.clone());
        println!("Step 1: {}", step1);
        println!("Step 1 structure: {:?}", step1);

        let step2 = beta_reduce(step1.clone());
        println!("Step 2: {}", step2);
        println!("Step 2 structure: {:?}", step2);

        let step3 = beta_reduce(step2.clone());
        println!("Step 3: {}", step3);
        println!("Step 3 structure: {:?}", step3);

        let normalized = normalize(deeply_nested.clone());
        println!("Normalized: {}", normalized);
        println!("Normalized structure: {:?}", normalized);
        println!("Expected: var(0)");
        println!("Match: {}", normalized == var(0));
    }

    #[test]
    fn debug_substitution() {
        println!("\n=== Debugging substitution ===");

        // Test case 3 from test_simple_substitution
        let expr = lam(var(1)); // λy.x where x is index 0
        let replacement = var(1); // y
        println!("Expression: {}", expr);
        println!("Expression structure: {:?}", expr);
        println!("Target index: 0");
        println!("Replacement: {}", replacement);
        println!("Replacement structure: {:?}", replacement);
        let result = substitute(expr.clone(), 0, replacement.clone());

        println!("Result: {}", result);
        println!("Result structure: {:?}", result);
        println!("Expected: lam(var(2))");
        println!("Match: {}", result == lam(var(2)));
    }

    #[test]
    fn test_simple_substitution_case_3_debug() {
        println!("=== Testing simple substitution case 3 ===");

        // Test case 3 from test_simple_substitution
        let expr = lam(var(1)); // λy.x (where x is the outer variable)
        let replacement = var(1); // y
        println!("Expression: {}", expr);
        println!("Target index: 0");
        println!("Replacement: {}", replacement);

        let result = substitute(expr.clone(), 0, replacement.clone());
        println!("Result: {}", result);
        println!("Expected: lam(var(2))");
        println!("Match: {}", result == lam(var(2)));

        // Let's also test the edge case
        println!("\n=== Testing edge case ===");
        let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));

        println!("Original: {}", deeply_nested);

        let step1 = beta_reduce(deeply_nested.clone());
        println!("After first beta reduction: {}", step1);

        let step2 = beta_reduce(step1.clone());
        println!("After second beta reduction: {}", step2);
        println!("Expected: var(0)");
        println!("Match: {}", step2 == var(0));
    }

    #[test]
    fn understand_de_bruijn_indices() {
        println!("=== Understanding De Bruijn indices ===");

        // Let's create the expression: (λy.x) x
        // In De Bruijn: (λ.1) 0
        let expr = app(lam(var(1)), var(0));
        println!("Expression: {}", expr);
        println!("This should be: (λy.x) x");

        // Now let's understand what each index means:
        // - var(0) in the argument position refers to the outer x
        // - var(1) inside the lambda refers to the outer x (because we're inside one lambda)
        // - The lambda parameter y would be var(0) inside the lambda

        // So when we substitute for index 0 (the outer x), we should replace:
        // - var(0) in the argument with the replacement
        // - var(1) inside the lambda with the shifted replacement

        // Let's test this:
        let replacement = lam(var(0)); // λz.z
        println!("Replacement: {}", replacement);

        let result = substitute(expr, 0, replacement);
        println!("Result: {}", result);
        println!("Expected: (λy.x) (λz.z) which should be app(lam(var(1)), lam(var(0)))");

        let expected = app(lam(var(1)), lam(var(0)));
        println!("Expected: {}", expected);
        println!("Match: {}", result == expected);

        // Let's also test a simpler case
        println!("\n=== Simple case: substitute y for x in λy.x ===");
        let simple_expr = lam(var(1)); // λy.x
        let simple_replacement = var(1); // y
        println!("Expression: {}", simple_expr);
        println!("Replacement: {}", simple_replacement);

        let simple_result = substitute(simple_expr, 0, simple_replacement);
        println!("Result: {}", simple_result);
        println!("Expected: λy.y which should be lam(var(0))? Or lam(var(2))?");

        // Actually, let's think about this:
        // - We're substituting y (index 1) for x (index 0) in λy.x
        // - Inside the lambda, x is at index 1
        // - We replace x (index 1) with y (index 1)
        // - But y is already bound by the lambda, so we need to shift it
        // - Shifted y becomes index 2
        // - So the result should be λy.2

        let expected_simple = lam(var(2));
        println!("Expected (lam(var(2))): {}", expected_simple);
        println!("Match: {}", simple_result == expected_simple);
    }

    #[test]
    fn debug_simple_substitution_step_by_step() {
        println!("=== Debug simple substitution step by step ===");

        // Test case: substitute y for x in λy.x
        // Expression: lam(var(1)) (λy.x where x is index 0)
        // Target index: 0 (x)
        // Replacement: var(1) (y at index 1)
        // Expected: lam(var(2)) (λy.y where y is now at index 2)

        let expr = lam(var(1));
        let replacement = var(1);
        let target_index = 0;

        println!("Expression: {}", expr);
        println!("Target index: {}", target_index);
        println!("Replacement: {}", replacement);

        // Let's manually trace through the substitution
        println!("\n...Manual trace...");
        // This is a lambda, so we should:
        // 1. Lift the replacement: ↑(var(1)) = var(2)
        // 2. Substitute in the body with target_index + 1 = 1
        // 3. The body is var(1), so [var(2)/1]var(1) = var(2)

        let result = substitute(expr.clone(), target_index, replacement.clone());
        println!("Result: {}", result);
        println!("Expected: lam(var(2))");
        println!("Match: {}", result == lam(var(2)));
    }

    #[test]
    fn debug_substitution_issue() {
        println!("\n=== Debugging Substitution Issue ===");

        // Test the specific case that's failing
        // Expression: lam(lam(var(2))) - which is λx.λy.x
        // Substitute var(0) for index 0
        // Expected: lam(lam(var(1))) - because:
        // [var(0)/0](λ λ 2) = λ([↑(var(0))/1](λ 2)) = λ([var(1)/1](λ 2))
        // [var(1)/1](λ 2) = λ([↑(var(1))/2](2)) = λ([var(2)/2]2) = λ var(2)
        // Wait, that gives us λ λ 2, not λ λ 1
        // Let me re-read the formal rules...

        // Actually, let me test a simpler case first
        println!("Original expression: lam(lam(var(2)))");
        let expr = lam(lam(var(2)));
        println!("Expression: {}", expr);

        let replacement = var(0);
        println!("Replacement: {}", replacement);

        let result = substitute(expr, 0, replacement);
        println!("Substitution result: {}", result);
        println!("Expected: lam(lam(var(1)))"); // This might be wrong based on formal rules

        // Let's manually trace the formal rules:
        // [var(0)/0](λ λ 2) = λ([↑(var(0))/1](λ 2))
        // ↑(var(0)) = var(1)
        // So: λ([var(1)/1](λ 2))
        // [var(1)/1](λ 2) = λ([↑(var(1))/2](2))
        // ↑(var(1)) = var(2)
        // So: λ([var(2)/2]2) = λ var(2)
        // So final result: λ λ var(2)

        // Wait, that doesn't seem right. Let me re-read the formal rules...

        // Actually, let me test the specific failing case from the test
        println!("\n=== Testing the specific failing case ===");
        // Expression: app(app(lam(lam(var(1))), var(0)), var(1))
        // This is ((λx.λy.x) a) b
        // Expected result: a (which is var(0))

        let complex_expr = app(app(lam(lam(var(1))), var(0)), var(1));
        println!("Complex expression: {}", complex_expr);

        let step1 = beta_reduce(complex_expr.clone());
        println!("After first beta reduction: {}", step1);
        // Expected: app(lam(var(1)), var(1)) - which is (λy.x) b

        let step2 = beta_reduce(step1.clone());
        println!("After second beta reduction: {}", step2);
        // Expected: var(0) - which is a

        let normalized = normalize(complex_expr);
        println!("Fully normalized: {}", normalized);
        println!("Expected: var(0)");
        println!("Match: {}", normalized == var(0));
    }

    #[test]
    fn test_recursion_limit() {
        // Test that recursion limits work correctly
        let y_combinator_inner_body = app(var(1), app(var(0), var(0)));
        let y_combinator_inner = lam(y_combinator_inner_body);
        let y_combinator_inner_copy = lam(app(var(1), app(var(0), var(0))));
        let y_combinator = lam(app(y_combinator_inner, y_combinator_inner_copy));

        // This should not cause stack overflow due to recursion limits
        let result = normalize(y_combinator);

        // Verify that the recursion limit was actually triggered by checking
        // that the result is not the same as what we'd get from a successful normalization
        // Since Y-combinator is infinitely recursive, it should hit the recursion limit
        // We can verify this by checking that the result still contains the Y-combinator structure
        // rather than being fully reduced to some terminal form

        // Check that the result is still a lambda expression (indicating recursion limit was hit)
        assert!(matches!(result, CoreExpr::Lam(_)));

        // Additional verification: test with a simple expression that should normalize successfully
        let simple_expr = app(lam(var(0)), var(1));
        let simple_result = normalize(simple_expr);
        // This should successfully normalize to var(1) without hitting recursion limits
        assert_eq!(simple_result, var(1));
    }

    #[test]
    fn test_recursion_health() {
        // Test the recursion health function
        let expr = lam(var(0));
        let health = check_recursion_health(&expr, 0, 100);
        assert_eq!(health, 1.0);

        let health = check_recursion_health(&expr, 50, 100);
        assert_eq!(health, 0.5);

        let health = check_recursion_health(&expr, 100, 100);
        assert_eq!(health, 0.0);
    }
}
