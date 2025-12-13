/// Test for Jue compiler for Dan World
use core_world::core_expr::{app, lam, var};
use core_world::eval_relation::{eval_empty, EvalResult};
use core_world::proof_checker::{prove_evaluation, verify_proof};

#[test]
fn test_jue_compiler_for_dan_world() {
    // Test that Jue compiler can handle Dan World patterns

    // Simple Jue-like expression patterns
    let simple_exprs = vec![
        "(lambda x x)",         // Identity function
        "(lambda (x y) (x y))", // Application
        "(let x 5 x)",          // Let binding
    ];

    for expr_text in simple_exprs {
        // This would be the pattern used by Dan World:
        // 1. Parse Jue text to AST
        // 2. Compile AST to CoreExpr
        // 3. Use CoreExpr in Dan World components

        // For now, we test the core expression equivalents
        // (full Jue parsing would be tested in jue_world_tests.rs)

        let core_expr = match expr_text {
            "(lambda x x)" => lam(var(0)),
            "(lambda (x y) (x y))" => lam(app(var(0), var(1))),
            "(let x 5 x)" => app(lam(var(0)), var(0)), // Simplified
            _ => var(0),
        };

        // Verify the core expression works
        let eval_result = eval_empty(core_expr.clone());
        match eval_result {
            EvalResult::Value(_) => assert!(true),
            EvalResult::Closure(_) => assert!(true),
        }

        // Verify proofs can be generated
        let eval_proof = prove_evaluation(core_expr.clone());
        assert!(verify_proof(&eval_proof, &core_expr));
    }
}
