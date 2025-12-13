/// Reusable test utilities for the JUE project
///
/// This module provides shared helper functions, common test patterns,
/// and utilities for testing across the JUE codebase.
use core_world::core_expr::{app, lam, var, CoreExpr};
use core_world::core_kernel::{beta_reduce, normalize, prove_kernel_consistency};
use core_world::eval_relation::{eval_empty, is_normal_form, Env, EvalResult};
use core_world::proof_checker::{
    prove_beta_reduction, prove_consistency as prove_consistency_proof, prove_evaluation,
    prove_normalization, verify_proof, Proof,
};
use std::time::Instant;

/// Expression Construction Helpers
pub mod expression_helpers {
    use super::*;

    /// Creates an identity function: λx.x
    pub fn identity_function() -> CoreExpr {
        lam(var(0))
    }

    /// Creates a nested lambda expression: λx.λy.x
    pub fn nested_lambda() -> CoreExpr {
        lam(lam(var(1)))
    }

    /// Creates a complex application: (λx.x) y
    pub fn identity_application() -> CoreExpr {
        app(identity_function(), var(1))
    }

    /// Creates a deeply nested expression: (λx.(λy.1) 0) (λz.0)
    pub fn deeply_nested_expression() -> CoreExpr {
        app(lam(app(lam(var(1)), var(0))), lam(var(0)))
    }

    /// Creates a standard test expression for beta reduction: (λx.x) y
    pub fn standard_beta_reduction_expr() -> CoreExpr {
        app(lam(var(0)), var(1))
    }

    /// Creates a standard test expression for normalization: (λx.λy.x) a b
    pub fn standard_normalization_expr() -> CoreExpr {
        app(app(lam(lam(var(1))), var(0)), var(1))
    }

    /// Creates a complex application expression: (λx.(x y)) (λz.z)
    pub fn complex_application_expr() -> CoreExpr {
        app(lam(app(var(0), var(1))), lam(var(0)))
    }
}

/// Environment Setup Utilities
pub mod environment_utils {
    use super::*;

    /// Creates a standard empty environment for testing
    pub fn create_empty_env() -> Env {
        Env::new()
    }

    /// Creates a standard environment with basic variable bindings
    pub fn create_standard_env() -> Env {
        let mut env = Env::new();
        env.insert(0, var(5));
        env.insert(1, var(10));
        env
    }

    /// Creates an environment with custom variable bindings
    pub fn create_custom_env(vars: Vec<(usize, CoreExpr)>) -> Env {
        let mut env = Env::new();
        for (index, expr) in vars {
            env.insert(index, expr);
        }
        env
    }
}

/// Proof Verification Patterns
pub mod proof_utils {
    use super::*;

    /// Generates and verifies a standard beta reduction proof
    pub fn verify_beta_reduction(expr: CoreExpr) -> bool {
        let proof = prove_beta_reduction(expr.clone());
        proof.map_or(false, |p| verify_proof(&p, &expr))
    }

    /// Generates and verifies a standard evaluation proof
    pub fn verify_evaluation(expr: CoreExpr) -> bool {
        let proof = prove_evaluation(expr.clone());
        verify_proof(&proof, &expr)
    }

    /// Generates and verifies a standard normalization proof
    pub fn verify_normalization(expr: CoreExpr) -> bool {
        let proof = prove_normalization(expr.clone());
        verify_proof(&proof, &expr)
    }

    /// Creates a composite proof from multiple proof types
    pub fn create_composite_proof(expr: CoreExpr, conclusion: &str) -> Proof {
        let beta_proof = prove_beta_reduction(expr.clone()).unwrap_or_else(|| {
            panic!(
                "Failed to create beta reduction proof for expression: {}",
                expr
            )
        });
        let eval_proof = prove_evaluation(expr.clone());
        let norm_proof = prove_normalization(expr.clone());

        Proof::Composite {
            proofs: vec![beta_proof, eval_proof, norm_proof],
            conclusion: conclusion.to_string(),
        }
    }

    /// Verifies a composite proof
    pub fn verify_composite_proof(proof: &Proof, expr: &CoreExpr) -> bool {
        verify_proof(proof, expr)
    }
}

/// Performance Measurement Utilities
pub mod performance_utils {
    use super::*;

    /// Measures the time taken to perform beta reductions
    pub fn measure_beta_reduction_performance(
        iterations: usize,
        expr_fn: impl Fn() -> CoreExpr,
    ) -> std::time::Duration {
        let start_time = Instant::now();

        for _ in 0..iterations {
            let expr = expr_fn();
            let _reduced = beta_reduce(expr);
        }

        start_time.elapsed()
    }

    /// Measures the time taken to perform normalizations
    pub fn measure_normalization_performance(
        iterations: usize,
        expr_fn: impl Fn() -> CoreExpr,
    ) -> std::time::Duration {
        let start_time = Instant::now();

        for _ in 0..iterations {
            let expr = expr_fn();
            let _normalized = normalize(expr);
        }

        start_time.elapsed()
    }

    /// Measures the time taken to generate proofs
    pub fn measure_proof_generation_performance(
        iterations: usize,
        expr_fn: impl Fn() -> CoreExpr,
    ) -> std::time::Duration {
        let start_time = Instant::now();

        for _ in 0..iterations {
            let expr = expr_fn();
            let _beta_proof = prove_beta_reduction(expr.clone());
            let _eval_proof = prove_evaluation(expr.clone());
            let _norm_proof = prove_normalization(expr);
        }

        start_time.elapsed()
    }

    /// Measures the time taken for complete expression processing
    pub fn measure_complete_processing_performance(
        iterations: usize,
        expr_fn: impl Fn() -> CoreExpr,
    ) -> std::time::Duration {
        let start_time = Instant::now();

        for _ in 0..iterations {
            let expr = expr_fn();
            let _eval_result = eval_empty(expr.clone());
            let _beta_proof = prove_beta_reduction(expr.clone());
            let _eval_proof = prove_evaluation(expr.clone());
            let _norm_proof = prove_normalization(expr);
        }

        start_time.elapsed()
    }
}

/// Error Handling Test Patterns
pub mod error_utils {
    use super::*;

    /// Tests that an expression evaluates without panicking
    pub fn test_safe_evaluation(expr: CoreExpr) -> bool {
        let result = eval_empty(expr);
        match result {
            EvalResult::Value(_) => true,
            EvalResult::Closure(_) => true,
        }
    }

    /// Tests that proof generation doesn't panic for an expression
    pub fn test_safe_proof_generation(expr: CoreExpr) -> bool {
        let beta_proof = prove_beta_reduction(expr.clone());
        let eval_proof = prove_evaluation(expr.clone());
        let norm_proof = prove_normalization(expr.clone());

        beta_proof.is_some() && verify_proof(&eval_proof, &expr) && verify_proof(&norm_proof, &expr)
    }

    /// Tests that an expression can be normalized without errors
    pub fn test_safe_normalization(expr: CoreExpr) -> bool {
        let expr_for_normalization = expr.clone();
        let normalized = normalize(expr_for_normalization);
        let expr_for_eval = expr.clone();
        let eval_result = eval_empty(expr_for_eval);
        is_normal_form(&eval_result) || normalized != expr
    }
}

/// Test Data Constants
pub mod test_data {
    use super::*;

    /// Standard test expressions that are commonly used
    pub const STANDARD_TEST_EXPRESSIONS: [fn() -> CoreExpr; 6] = [
        || app(lam(var(0)), var(1)),                   // (λx.x) y
        || app(app(lam(lam(var(1))), var(0)), var(1)), // (λx.λy.x) a b
        || app(lam(app(var(0), var(1))), lam(var(0))), // (λx.(x y)) (λz.z)
        || lam(var(0)),                                // λx.x
        || app(lam(var(0)), lam(var(0))),              // (λx.x) (λy.y)
        || app(app(lam(lam(var(2))), var(0)), var(1)), // (λx.λy.y) a b
    ];

    /// Complex test expressions for more thorough testing
    pub const COMPLEX_TEST_EXPRESSIONS: [fn() -> CoreExpr; 4] = [
        || app(lam(app(lam(var(1)), var(0))), lam(var(0))), // (λx.((λy.1) 0)) (λz.0)
        || app(app(lam(lam(lam(var(2)))), var(0)), app(var(1), var(2))), // (λx.λy.λz.z) a (b c)
        || lam(app(lam(var(1)), var(0))),                   // λx.(λy.y) x
        || app(lam(lam(app(var(1), var(0)))), lam(var(0))), // (λx.λy.(y x)) (λz.z)
    ];

    /// Creates all standard test expressions
    pub fn create_standard_expressions() -> Vec<CoreExpr> {
        STANDARD_TEST_EXPRESSIONS.iter().map(|f| f()).collect()
    }

    /// Creates all complex test expressions
    pub fn create_complex_expressions() -> Vec<CoreExpr> {
        COMPLEX_TEST_EXPRESSIONS.iter().map(|f| f()).collect()
    }

    /// Creates all test expressions (standard + complex)
    pub fn create_all_expressions() -> Vec<CoreExpr> {
        let mut all = create_standard_expressions();
        all.extend(create_complex_expressions());
        all
    }
}

/// Comprehensive Test Utilities
pub mod comprehensive_utils {
    use super::*;

    /// Runs a complete test workflow on an expression:
    /// 1. Evaluation
    /// 2. Beta reduction
    /// 3. Normalization
    /// 4. Proof generation and verification
    pub fn run_complete_workflow(expr: CoreExpr) -> bool {
        // 1. Evaluation
        let eval_result = eval_empty(expr.clone());
        let eval_success = match eval_result {
            EvalResult::Value(_) => true,
            EvalResult::Closure(_) => true,
        };

        // 2. Beta reduction
        let _beta_reduced = beta_reduce(expr.clone());
        let beta_success = true; // beta_reduce doesn't fail

        // 3. Normalization
        let _normalized = normalize(expr.clone());
        let norm_success = true; // normalize doesn't fail

        // 4. Proof generation and verification
        let proof_success = proof_utils::verify_beta_reduction(expr.clone())
            && proof_utils::verify_evaluation(expr.clone())
            && proof_utils::verify_normalization(expr.clone());

        eval_success && beta_success && norm_success && proof_success
    }

    /// Tests that all standard expressions work correctly
    pub fn test_all_standard_expressions() -> bool {
        let expressions = test_data::create_standard_expressions();
        expressions
            .iter()
            .all(|expr| run_complete_workflow(expr.clone()))
    }

    /// Tests that all complex expressions work correctly
    pub fn test_all_complex_expressions() -> bool {
        let expressions = test_data::create_complex_expressions();
        expressions
            .iter()
            .all(|expr| run_complete_workflow(expr.clone()))
    }

    /// Tests the complete system consistency
    pub fn test_system_consistency() -> bool {
        prove_kernel_consistency() && verify_proof(&prove_consistency_proof(), &lam(var(0)))
    }
}
