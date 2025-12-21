#[test]
fn debug_core_expr_error_handling() {
    // Test the specific failing case from test_core_expr_error_handling
    let func = core_world::core_expr::lam(core_world::core_expr::var(0));
    let func_serialized = core_world::serialize_core_expr(&func);
    let mut incomplete_app = vec![0x03]; // App tag
    incomplete_app.extend_from_slice(&func_serialized);
    // Missing argument bytes

    println!("Func serialized length: {}", func_serialized.len());
    println!("Incomplete app length: {}", incomplete_app.len());

    let result = core_world::deserialize_core_expr(&incomplete_app);
    println!("Deserialization result: {:?}", result);
}

#[test]
fn debug_proof_error_handling() {
    // Test the specific failing case from test_proof_error_handling
    let proof1 = core_world::proof_checker::prove_beta(core_world::core_expr::app(
        core_world::core_expr::lam(core_world::core_expr::var(0)),
        core_world::core_expr::var(1),
    ));
    let proof1_serialized = core_world::serialize_proof(&proof1);
    let mut incomplete_trans = vec![0x05]; // Trans tag
    incomplete_trans.extend_from_slice(&proof1_serialized);
    // Missing second proof bytes

    println!("Proof1 serialized length: {}", proof1_serialized.len());
    println!("Incomplete trans length: {}", incomplete_trans.len());

    let result = core_world::deserialize_proof(&incomplete_trans);
    println!("Deserialization result: {:?}", result);
}
