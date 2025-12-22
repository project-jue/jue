use jue_world::ast::{AstNode, Literal};
use jue_world::integration::physics::PhysicsWorldCompiler;
use jue_world::trust_tier::TrustTier;
/// Simple validation test for Physics-World TODO features
/// This test validates that all the TODO features have been implemented
use physics_world::types::{OpCode, Value};
use physics_world::vm::VmState;

#[test]
fn test_physics_world_todo_features_implemented() {
    // Test 1: Float literal support
    let ast = AstNode::Literal(Literal::Float(3.14));
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
    let bytecode = compiler.compile_to_physics(&ast).unwrap();

    // Should generate OpCode::Float, not convert to Int
    assert_eq!(bytecode, vec![OpCode::Float(3.14)]);

    // Test 2: String literal support
    let ast = AstNode::Literal(Literal::String("test".to_string()));
    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
    let bytecode = compiler.compile_to_physics(&ast).unwrap();

    // Should generate LoadString opcode
    assert_eq!(bytecode.len(), 1);
    if let OpCode::LoadString(_) = bytecode[0] {
        // String literal compiled successfully
    } else {
        panic!("Expected LoadString opcode for string literal");
    }

    // Test 3: Environment management
    let ast = AstNode::Let {
        bindings: vec![("x".to_string(), AstNode::Literal(Literal::Int(42)))],
        body: Box::new(AstNode::Variable("x".to_string())),
        location: Default::default(),
    };

    let mut compiler = PhysicsWorldCompiler::new(TrustTier::Formal);
    let bytecode = compiler.compile_to_physics(&ast).unwrap();

    // Should contain SetLocal and GetLocal operations
    assert!(bytecode.contains(&OpCode::SetLocal(0)));
    assert!(bytecode.contains(&OpCode::GetLocal(0)));

    // Test 4: VM handles new OpCodes
    let bytecode = vec![OpCode::Float(2.5), OpCode::Float(1.5), OpCode::FAdd];
    let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
    let result = vm.run().unwrap();

    assert_eq!(result, Value::Float(4.0));

    // Test 5: Sandbox wrapper generation
    let ast = AstNode::Literal(Literal::Int(1));
    // Use compile_to_physics_world which applies tier-specific processing
    let (bytecode, _) =
        jue_world::integration::physics::compile_to_physics_world(&ast, TrustTier::Experimental)
            .unwrap();

    // Should contain sandbox wrapper operations
    assert!(bytecode.contains(&OpCode::InitSandbox));
    assert!(bytecode.contains(&OpCode::CleanupSandbox));

    println!("✅ All Physics-World TODO features are implemented and working!");
}

#[test]
fn test_vm_opcode_handling() {
    // Test that the VM can handle all the new OpCodes we implemented

    // Test Float operations
    let bytecode = vec![OpCode::Float(10.0), OpCode::Float(5.0), OpCode::FAdd];
    let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
    let result = vm.run().unwrap();
    assert_eq!(result, Value::Float(15.0));

    // Test String operations
    let bytecode = vec![
        OpCode::LoadString(0),
        OpCode::LoadString(1),
        OpCode::StrConcat,
    ];
    let mut vm = VmState::new(
        bytecode,
        vec![
            Value::String("Hello".to_string()),
            Value::String(" World".to_string()),
        ],
        100,
        1024,
        1,
        100,
    );
    let result = vm.run().unwrap();
    assert_eq!(result, Value::String("Hello World".to_string()));

    // Test Variable operations
    let bytecode = vec![OpCode::Int(42), OpCode::SetLocal(0), OpCode::GetLocal(0)];
    let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
    let result = vm.run().unwrap();
    assert_eq!(result, Value::Int(42));

    // Test Sandbox operations
    let bytecode = vec![OpCode::InitSandbox, OpCode::Int(1), OpCode::CleanupSandbox];
    let mut vm = VmState::new(bytecode, vec![], 100, 1024, 1, 100);
    let result = vm.run().unwrap();
    assert_eq!(result, Value::Int(1));

    println!("✅ VM successfully handles all new OpCodes!");
}
