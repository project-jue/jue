
use core_world::proof_checker::prove_evaluation;

use core_world::core_kernel::normalize;

use core_world::core_expr::var;

use core_world::core_expr::app;

use core_world::core_expr::lam;

/// Test Core-World ↔ Dan-World Integration
#[test]
pub(crate) fn test_core_dan_integration() {
    // This test would involve:
    // 1. Creating Dan-World module proposals with CoreExpr
    // 2. Verifying the core expressions in the proposals
    // 3. Testing mutation protocols with core proofs

    // For now, we'll test core components that Dan would use
    let module_code = lam(app(var(0), var(1)));
    let proof_code = lam(var(0));

    // Test that the expressions are valid
    assert!(matches!(module_code, CoreExpr::Lam(_)));
    assert!(matches!(proof_code, CoreExpr::Lam(_)));

    // Test normalization
    let normalized_module = normalize(module_code.clone());
    let normalized_proof = normalize(proof_code.clone());

    assert_eq!(normalized_module, module_code);
    assert_eq!(normalized_proof, proof_code);

    // Test proof verification
    let eval_proof = prove_evaluation(module_code.clone());
    assert!(verify_proof(&eval_proof, &module_code));
}
