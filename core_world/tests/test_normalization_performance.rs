use core_world::core_kernel::normalize;

use core_world::core_expr::var;

use core_world::core_expr::lam;

use core_world::core_expr::app;

use std::time::Instant;

#[test]
pub(crate) fn test_normalization_performance() {
    let start_time = Instant::now();

    // Test many normalizations
    for _i in 0..500 {
        let expr = app(app(lam(lam(var(1))), var(0)), var(1));
        let _normalized = normalize(expr);
    }

    let duration = start_time.elapsed();
    println!("500 normalizations completed in {:?}", duration);
}
