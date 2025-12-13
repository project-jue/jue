use core_world::core_expr::{app, lam, var};
use core_world::core_kernel::normalize;

fn main() {
    // Test the deeply nested expression that's failing
    // Expression: (λx.(λy.x) x) (λz.z)
    // In De Bruijn: (λ.(λ.1) 0) (λ.0)
    let deeply_nested = app(lam(app(lam(var(1)), var(0))), lam(var(0)));

    println!("Original expression: {:?}", deeply_nested);

    let normalized = normalize(deeply_nested.clone());
    println!("Normalized result: {:?}", normalized);

    // Test the multi-level expression too
    // Expression: ((λx.λy.λz.x) a) (b c)
    // In De Bruijn: ((λ.λ.λ.2) 0) (1 2)
    let multi_level = app(app(lam(lam(var(2))), var(0)), app(var(1), var(2)));

    println!("Multi-level original: {:?}", multi_level);

    let normalized_multi = normalize(multi_level.clone());
    println!("Multi-level normalized: {:?}", normalized_multi);
}
