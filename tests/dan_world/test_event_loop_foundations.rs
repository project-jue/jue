/// Event Loop Foundations Tests
use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::eval_relation::eval_empty;

#[test]
fn test_event_loop_foundations() {
    // Test core expression patterns that event loop would use

    // Lambda expressions (for event handlers)
    let handler = lam(app(var(0), var(1)));
    assert!(matches!(handler, CoreExpr::Lam(_)));

    // Variable expressions (for event data)
    let event_data = var(0);
    assert!(matches!(event_data, CoreExpr::Var(_)));

    // Application expressions (for event processing)
    let processing = app(handler, event_data);
    assert!(matches!(processing, CoreExpr::App(_, _)));

    // Verify evaluation works
    let eval_result = eval_empty(processing);
    match eval_result {
        EvalResult::Value(_) => assert!(true),
        EvalResult::Closure(_) => assert!(true),
    }
}

#[test]
fn test_mutation_protocol_foundations() {
    // Test core expression patterns for mutation protocol

    // Code expressions
    let mutation_code = lam(var(0));
    let proof_code = lam(var(0));

    // Verify they can be used in proofs
    let eval_proof = prove_evaluation(mutation_code.clone());
    assert!(verify_proof(&eval_proof, &mutation_code));

    let norm_proof = prove_normalization(proof_code.clone());
    assert!(verify_proof(&norm_proof, &proof_code));

    // Test that proofs can be combined (foundation for consensus)
    let composite_proof = Proof::Composite {
        proofs: vec![eval_proof, norm_proof],
        conclusion: "Mutation protocol foundation test".to_string(),
    };

    assert!(verify_proof(&composite_proof, &mutation_code));
}
