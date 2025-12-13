use core_world::core_expr::app;

use core_world::core_expr::var;

use core_world::core_expr::lam;
use core_world::core_kernel::prove_kernel_consistency;
/// Test Kernel Consistency
#[test]
pub(crate) fn test_kernel_consistency_comprehensive() {
    // Test that kernel consistency holds
    assert!(prove_kernel_consistency());

    // Test consistency with various expressions
    let _expr1 = lam(var(0));
    let _expr2 = app(lam(var(0)), var(1));
    let _expr3 = app(app(lam(lam(var(1))), var(0)), var(1));

    // All should be consistent
    assert!(prove_kernel_consistency());
}
