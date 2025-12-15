use core_world::core_expr::{app, lam, nat, pair, var};
use core_world::core_kernel::{beta_reduce, normalize};

#[test]
fn test_nat_not_reducible() {
    // Test that Nat values are not reducible by beta-reduction
    let nat_expr = nat(42);
    let reduced = beta_reduce(nat_expr.clone());
    assert_eq!(reduced, nat_expr);
}

#[test]
fn test_pair_not_reducible() {
    // Test that Pair values are not reducible by beta-reduction
    let pair_expr = pair(nat(1), nat(2));
    let reduced = beta_reduce(pair_expr.clone());
    assert_eq!(reduced, pair_expr);
}

#[test]
fn test_nat_in_lambda() {
    // Test that Nat values work correctly inside lambda expressions
    let expr = lam(nat(42));
    let reduced = beta_reduce(expr.clone());
    assert_eq!(reduced, expr);
}

#[test]
fn test_pair_in_application() {
    // Test that Pair values work correctly in applications
    let identity = lam(var(0));
    let pair_expr = pair(nat(1), nat(2));
    let app_expr = app(identity, pair_expr.clone());

    // The identity function should reduce to the pair
    let reduced = beta_reduce(app_expr);
    assert_eq!(reduced, pair_expr);
}

#[test]
fn test_normalize_with_nat_and_pair() {
    // Test normalization with expressions containing Nat and Pair
    let identity = lam(var(0));
    let pair_expr = pair(nat(1), nat(2));
    let app_expr = app(identity, pair_expr.clone());

    // Normalization should also work correctly
    let normalized = normalize(app_expr);
    assert_eq!(normalized, pair_expr);
}
