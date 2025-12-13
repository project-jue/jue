use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::{beta_reduce, normalize, substitute};

fn main() {
    println!("=== Debugging substitution step by step ===");

    // Test case from test_formal_substitution_rules_compliance
    println!("\n=== Test case 1: (λx. λy. (x y)) (λz. z) ===");
    let lambda_x_y_xy = lam(lam(app(var(1), var(0))));
    let lambda_z_z = lam(var(0));
    let expr = app(lambda_x_y_xy.clone(), lambda_z_z.clone());

    println!("Original expression: {}", expr);
    println!("Lambda x y xy: {}", lambda_x_y_xy);
    println!("Lambda z z: {}", lambda_z_z);

    // Step 1: First beta reduction
    let step1 = beta_reduce(expr.clone());
    println!("After first beta reduction: {}", step1);

    // Step 2: Full normalization
    let normalized = normalize(expr.clone());
    println!("Fully normalized: {}", normalized);

    // Let's also test the substitution directly
    println!("\n=== Testing substitution directly ===");
    let body = lam(app(var(1), var(0))); // λy. (x y)
    let body_clone = body.clone();
    let result = substitute(body, 0, lambda_z_z.clone());
    println!(
        "Substituting {} for index 0 in {} gives: {}",
        lambda_z_z, body_clone, result
    );

    // Test case from test_variable_capture_scenarios
    println!("\n=== Test case 2: (λx. λy. x y) (λz. z) ===");
    let lambda_x_y_xy2 = lam(lam(app(var(1), var(0))));
    let lambda_z_z2 = lam(var(0));
    let expr2 = app(lambda_x_y_xy2.clone(), lambda_z_z2.clone());

    println!("Original expression: {}", expr2);

    let step2_1 = beta_reduce(expr2.clone());
    println!("After first beta reduction: {}", step2_1);

    let normalized2 = normalize(expr2.clone());
    println!("Fully normalized: {}", normalized2);

    // Test case from test_self_application_scenarios
    println!("\n=== Test case 3: (λx. λy. x y y) (λx. x x) ===");
    let complex_lambda = lam(lam(app(app(var(1), var(0)), var(0))));
    let self_app_body = app(var(0), var(0));
    let self_app = lam(self_app_body);
    let expr3 = app(complex_lambda.clone(), self_app.clone());

    println!("Original expression: {}", expr3);

    let step3_1 = beta_reduce(expr3.clone());
    println!("After first beta reduction: {}", step3_1);

    let step3_2 = beta_reduce(step3_1.clone());
    println!("After second beta reduction: {}", step3_2);

    let normalized3 = normalize(expr3.clone());
    println!("Fully normalized: {}", normalized3);
}
