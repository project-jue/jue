use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize};

fn main() {
    // Test the problematic expression: app(app(lam(lam(var(0))), var(0)), var(1))
    // This represents: (λx.λy.y) a b → b
    let expr = app(app(lam(lam(var(0))), var(0)), var(1));

    println!("Original expression: {:?}", expr);
    println!("Expected result: var(1)");

    // β-reduction result (one step)
    let beta_reduced = beta_reduce(expr.clone());
    println!("After one β-reduction: {:?}", beta_reduced);

    // Normalization result (multiple steps)
    let normalized = normalize(expr.clone());
    println!("After full normalization: {:?}", normalized);

    // Let's also test the evaluation
    use core_world::eval_relation::{eval_empty, EvalResult};
    let eval_result = eval_empty(expr.clone());
    println!("Evaluation result: {:?}", eval_result);
}
