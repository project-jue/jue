#[test]
fn test_debug_debruijn() {
    use core_world::core_expr::{app, lam, var};
    use core_world::core_kernel::normalize;

    // Test the deeply nested normalization
    let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));
    println!("Original expression: {:?}", deeply_nested);

    let normalized = normalize(deeply_nested);
    println!("Normalized: {:?}", normalized);

    // Let's also test a simpler case
    let simple = app(lam(var(0)), var(1));
    println!("Simple app: {:?}", simple);
    let simple_norm = normalize(simple);
    println!("Simple normalized: {:?}", simple_norm);
}
