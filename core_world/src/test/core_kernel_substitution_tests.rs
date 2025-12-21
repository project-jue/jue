use super::*;
use crate::core_expr::{app, lam, var};

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
    let result4 = substitute(lambda_expr, 0, replacement.clone());
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
    // [var(0)/1]λ λ 2 = λ([↑(var(0))/2]λ 2) = λ([var(1)/2]λ 2)
    // [var(1)/2]λ 2 = λ([↑(var(1))/3]2)) = λ([var(2)/3]2)
    // [var(2)/3]2 = 2 (since 2 < 3)
    // So result should be λ λ 2
    assert_eq!(result, lam(lam(var(2))));
}
