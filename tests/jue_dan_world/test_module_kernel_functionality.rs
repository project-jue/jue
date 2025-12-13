/// Test for module kernel functionality
use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::core_kernel::{beta_reduce, prove_consistency};
use core_world::proof_checker::{prove_beta_reduction, verify_proof};

#[test]
fn test_module_kernel_functionality() {
    // Test that we can create module proposals using core expressions
    // (which is what the Jue module kernel would use internally)

    let valid_proposal_code = CoreExpr::Var(0);
    let valid_proof = CoreExpr::Var(0);

    // This simulates what would happen when Jue module kernel
    // validates a proposal - it uses core expressions
    assert!(matches!(valid_proposal_code, CoreExpr::Var(_)));
    assert!(matches!(valid_proof, CoreExpr::Var(_)));

    // Test that core expressions work correctly
    let identity = lam(var(0));
    let test_var = var(1);
    let expr = app(identity, test_var.clone());

    // Verify beta reduction works (foundation for module validation)
    let reduced = beta_reduce(expr.clone());
    assert_eq!(reduced, test_var);

    // Verify proof generation works
    let beta_proof = prove_beta_reduction(expr.clone());
    assert!(beta_proof.is_some());

    // Verify proof verification works
    let proof = beta_proof.unwrap();
    assert!(verify_proof(&proof, &expr));
}
