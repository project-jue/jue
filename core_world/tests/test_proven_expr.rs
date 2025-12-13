/// Test for proven expression
use core_world::core_expr::{lam, var};
use core_world::proof_checker::{attach_proof, prove_evaluation};

#[test]
fn test_proven_expr() {
    // Test proven expression creation and verification
    let expr = lam(var(0));
    let proof = prove_evaluation(expr.clone());
    let proven_expr = attach_proof(expr.clone(), proof);

    assert!(proven_expr.verify());
    assert_eq!(proven_expr.expr, expr);
}
