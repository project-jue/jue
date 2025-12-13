use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize};

fn main() {
    println!("=== Debugging the failing test case ===");

    // Test case from test_variable_capture_scenarios
    // Expression: (λx. λy. x y) (λz. z) → λy. ((λz. z) y)
    // De Bruijn: (λ λ 1 0) (λ 0) → λ ((λ 0) 0)

    let lambda_x_y_xy = lam(lam(app(var(1), var(0))));
    let lambda_z_z = lam(var(0));
    let expr = app(lambda_x_y_xy, lambda_z_z);

    println!("Original expression: {}", expr);
    println!("Expected: lam(app(lam(var(0)), var(0)))");

    let step1 = beta_reduce(expr.clone());
    println!("After first beta reduction: {}", step1);

    let normalized = normalize(expr.clone());
    println!("Normalized: {}", normalized);

    let expected = lam(app(lam(var(0)), var(0)));
    println!("Expected: {}", expected);
    println!("Match: {}", normalized == expected);
}
