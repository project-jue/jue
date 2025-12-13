use core_world::core_kernel::beta_reduce;

use core_world::core_expr::app;

use core_world::core_expr::var;

use core_world::core_expr::lam;

use std::time::Instant;

#[test]
pub(crate) fn test_beta_reduction_performance() {
    let start_time = Instant::now();

    // Test many beta reductions
    for i in 0..1000 {
        let identity = lam(var(0));
        let v = var(i % 10);
        let expr = app(identity, v);
        let _reduced = beta_reduce(expr);
    }

    let duration = start_time.elapsed();
    println!("1,000 beta reductions completed in {:?}", duration);
}
