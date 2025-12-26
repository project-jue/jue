/// Comprehensive tests for host call capability integration
use super::*;
use crate::types::{Capability, OpCode, Value};
use crate::vm::error::VmError;

#[test]
fn test_host_call_capability_mapping() {
    // Test that host functions are correctly mapped to capabilities
    use crate::vm::opcodes::capability::get_required_capability_for_host_function;

    // Test all host function mappings
    assert_eq!(
        get_required_capability_for_host_function(0),
        Some(Capability::IoReadSensor)
    );
    assert_eq!(
        get_required_capability_for_host_function(1),
        Some(Capability::IoWriteActuator)
    );
    assert_eq!(
        get_required_capability_for_host_function(2),
        Some(Capability::SysClock)
    );
    assert_eq!(
        get_required_capability_for_host_function(3),
        Some(Capability::SysCreateActor)
    );
    assert_eq!(
        get_required_capability_for_host_function(4),
        Some(Capability::SysTerminateActor)
    );
    assert_eq!(
        get_required_capability_for_host_function(5),
        Some(Capability::IoNetwork)
    );
    assert_eq!(
        get_required_capability_for_host_function(6),
        Some(Capability::IoNetwork)
    );
    assert_eq!(
        get_required_capability_for_host_function(7),
        Some(Capability::IoPersist)
    );
    assert_eq!(
        get_required_capability_for_host_function(8),
        Some(Capability::IoPersist)
    );

    // Test unknown function returns None
    assert_eq!(get_required_capability_for_host_function(99), None);
}

#[test]
fn test_host_call_with_correct_capability() {
    // Test that host calls work when the correct capability is provided
    let mut vm = VmState::new(
        vec![
            OpCode::Int(42), // Push argument
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 0, // ReadSensor
                args: 1,
            },
        ],
        vec![Value::Capability(Capability::IoReadSensor)], // Correct capability
        100,
        1024,
        1,
    );

    // Execute the VM
    let result = vm.run();

    // Should succeed and return the mock sensor value
    assert!(matches!(result, Ok(Value::Int(42))));
}

#[test]
fn test_host_call_with_wrong_capability() {
    // Test that host calls fail when the wrong capability is provided
    let mut vm = VmState::new(
        vec![
            OpCode::Int(42), // Push argument
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 0, // ReadSensor
                args: 1,
            },
        ],
        vec![Value::Capability(Capability::IoWriteActuator)], // Wrong capability
        100,
        1024,
        1,
    );

    // Execute the VM
    let result = vm.run();

    // Should fail with capability error
    assert!(matches!(result, Err(_)));
}

#[test]
fn test_host_call_with_invalid_capability_index() {
    // Test that host calls fail when the capability index is invalid
    let mut vm = VmState::new(
        vec![
            OpCode::Int(42), // Push argument
            OpCode::HostCall {
                cap_idx: 99, // Invalid index
                func_id: 0,  // ReadSensor
                args: 1,
            },
        ],
        vec![], // Empty constant pool
        100,
        1024,
        1,
    );

    // Execute the VM
    let result = vm.run();

    // Should fail with invalid heap pointer error
    assert!(matches!(result, Err(_)));
}

#[test]
fn test_host_call_with_insufficient_arguments() {
    // Test that host calls fail when there are insufficient arguments
    let mut vm = VmState::new(
        vec![OpCode::HostCall {
            cap_idx: 0,
            func_id: 0, // ReadSensor
            args: 1,    // Requires 1 argument
        }],
        vec![Value::Capability(Capability::IoReadSensor)],
        100,
        1024,
        1,
    );

    // Execute the VM
    let result = vm.run();

    // Should fail with stack underflow error
    assert!(matches!(result, Err(_)));
}

#[test]
fn test_host_call_with_unknown_function() {
    // Test that host calls fail when the function ID is unknown
    let mut vm = VmState::new(
        vec![
            OpCode::Int(42), // Push argument
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 99, // Unknown function
                args: 1,
            },
        ],
        vec![Value::Capability(Capability::IoReadSensor)],
        100,
        1024,
        1,
    );

    // Execute the VM
    let result = vm.run();

    // Should fail with unknown opcode error
    assert!(matches!(result, Err(_)));
}

#[test]
fn test_host_call_with_non_capability_value() {
    // Test that host calls fail when the capability value is not a Capability
    let mut vm = VmState::new(
        vec![
            OpCode::Int(42), // Push argument
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 0, // ReadSensor
                args: 1,
            },
        ],
        vec![Value::Int(123)], // Not a capability
        100,
        1024,
        1,
    );

    // Execute the VM
    let result = vm.run();

    // Should fail with type mismatch error
    assert!(matches!(result, Err(_)));
}

#[test]
fn test_host_call_different_functions() {
    // Test different host functions return appropriate mock values
    let test_cases = vec![
        (0, Capability::IoReadSensor, Value::Int(42)), // ReadSensor
        (1, Capability::IoWriteActuator, Value::Nil),  // WriteActuator
        (2, Capability::SysClock, Value::Int(1234567890)), // GetWallClockNs
        (3, Capability::SysCreateActor, Value::ActorId(1)), // SpawnActor
        (4, Capability::SysTerminateActor, Value::Nil), // TerminateActor
        (5, Capability::IoNetwork, Value::Nil),        // NetworkSend
        (6, Capability::IoNetwork, Value::Nil),        // NetworkReceive
        (7, Capability::IoPersist, Value::Nil),        // PersistWrite
        (8, Capability::IoPersist, Value::Nil),        // PersistRead
    ];

    for (func_id, capability, expected_result) in test_cases {
        let mut vm = VmState::new(
            vec![
                OpCode::Int(42), // Push argument (if needed)
                OpCode::HostCall {
                    cap_idx: 0,
                    func_id,
                    args: 1,
                },
            ],
            vec![Value::Capability(capability)],
            100,
            1024,
            1,
        );

        let result = vm.run();
        assert!(
            matches!(result, Ok(ref val) if val == &expected_result),
            "Function {} failed: expected {:?}, got {:?}",
            func_id,
            expected_result,
            result
        );
    }
}

#[test]
fn test_host_call_with_no_arguments() {
    // Test host calls that don't require arguments
    let mut vm = VmState::new(
        vec![OpCode::HostCall {
            cap_idx: 0,
            func_id: 2, // GetWallClockNs - doesn't need args
            args: 0,    // No arguments
        }],
        vec![Value::Capability(Capability::SysClock)],
        100,
        1024,
        1,
    );

    // Execute the VM
    let result = vm.run();

    // Should succeed and return the mock timestamp
    assert!(matches!(result, Ok(Value::Int(1234567890))));
}

// ============ Arithmetic Host Function Tests ============

#[test]
fn test_host_call_arithmetic_no_capability() {
    // Test that arithmetic host functions don't require capabilities (func_id 9-25)
    let mut vm = VmState::new(
        vec![
            OpCode::Int(10),
            OpCode::Int(20),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 9, // IntAdd
                args: 2,
            },
        ],
        vec![], // No capability needed
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(30))));
}

#[test]
fn test_host_call_int_add() {
    let mut vm = VmState::new(
        vec![
            OpCode::Int(10),
            OpCode::Int(20),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 9, // IntAdd
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(30))));
}

#[test]
fn test_host_call_int_sub() {
    let mut vm = VmState::new(
        vec![
            OpCode::Int(50),
            OpCode::Int(20),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 10, // IntSub
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(30))));
}

#[test]
fn test_host_call_int_mul() {
    let mut vm = VmState::new(
        vec![
            OpCode::Int(6),
            OpCode::Int(7),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 11, // IntMul
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(42))));
}

#[test]
fn test_host_call_int_div() {
    let mut vm = VmState::new(
        vec![
            OpCode::Int(42),
            OpCode::Int(6),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 12, // IntDiv
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(7))));
}

#[test]
fn test_host_call_int_div_by_zero() {
    let mut vm = VmState::new(
        vec![
            OpCode::Int(42),
            OpCode::Int(0),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 12, // IntDiv
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    // Should push error value and continue
    assert!(matches!(result, Ok(Value::Error(_))));
}

#[test]
fn test_host_call_int_mod() {
    let mut vm = VmState::new(
        vec![
            OpCode::Int(17),
            OpCode::Int(5),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 13, // IntMod
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(2))));
}

#[test]
fn test_host_call_float_add() {
    let mut vm = VmState::new(
        vec![
            OpCode::Float(10.5),
            OpCode::Float(20.5),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 14, // FloatAdd
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Float(v)) if (v - 31.0).abs() < 0.001));
}

#[test]
fn test_host_call_float_sub() {
    let mut vm = VmState::new(
        vec![
            OpCode::Float(50.0),
            OpCode::Float(20.0),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 15, // FloatSub
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Float(v)) if (v - 30.0).abs() < 0.001));
}

#[test]
fn test_host_call_float_mul() {
    let mut vm = VmState::new(
        vec![
            OpCode::Float(6.0),
            OpCode::Float(7.0),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 16, // FloatMul
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Float(v)) if (v - 42.0).abs() < 0.001));
}

#[test]
fn test_host_call_float_div() {
    let mut vm = VmState::new(
        vec![
            OpCode::Float(42.0),
            OpCode::Float(6.0),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 17, // FloatDiv
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Float(v)) if (v - 7.0).abs() < 0.001));
}

#[test]
fn test_host_call_int_to_float() {
    let mut vm = VmState::new(
        vec![
            OpCode::Int(42),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 18, // IntToFloat
                args: 1,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Float(v)) if (v - 42.0).abs() < 0.001));
}

#[test]
fn test_host_call_float_to_int() {
    let mut vm = VmState::new(
        vec![
            OpCode::Float(42.7),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 19, // FloatToInt
                args: 1,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    // Should truncate toward zero and push error for precision loss
    assert!(matches!(result, Ok(Value::Int(42))));
}

#[test]
fn test_host_call_int_eq() {
    let mut vm = VmState::new(
        vec![
            OpCode::Int(42),
            OpCode::Int(42),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 20, // IntEq
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(1)))); // true
}

#[test]
fn test_host_call_int_eq_false() {
    let mut vm = VmState::new(
        vec![
            OpCode::Int(42),
            OpCode::Int(99),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 20, // IntEq
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(0)))); // false
}

#[test]
fn test_host_call_int_lt() {
    let mut vm = VmState::new(
        vec![
            OpCode::Int(10),
            OpCode::Int(20),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 21, // IntLt
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(1)))); // true
}

#[test]
fn test_host_call_int_gt() {
    let mut vm = VmState::new(
        vec![
            OpCode::Int(20),
            OpCode::Int(10),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 22, // IntGt
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(1)))); // true
}

#[test]
fn test_host_call_float_eq() {
    let mut vm = VmState::new(
        vec![
            OpCode::Float(3.14),
            OpCode::Float(3.14),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 23, // FloatEq
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(1)))); // true
}

#[test]
fn test_host_call_float_lt() {
    let mut vm = VmState::new(
        vec![
            OpCode::Float(1.5),
            OpCode::Float(2.5),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 24, // FloatLt
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(1)))); // true
}

#[test]
fn test_host_call_float_gt() {
    let mut vm = VmState::new(
        vec![
            OpCode::Float(2.5),
            OpCode::Float(1.5),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 25, // FloatGt
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    assert!(matches!(result, Ok(Value::Int(1)))); // true
}

#[test]
fn test_host_call_type_mismatch_int_args() {
    // Test that passing floats to int operations produces an error
    let mut vm = VmState::new(
        vec![
            OpCode::Float(10.0),
            OpCode::Float(20.0),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 9, // IntAdd
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    // Should push error value and continue
    assert!(matches!(result, Ok(Value::Error(_))));
}

#[test]
fn test_host_call_arithmetic_overflow() {
    // Test integer overflow detection
    let mut vm = VmState::new(
        vec![
            OpCode::Int(i64::MAX),
            OpCode::Int(1),
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 9, // IntAdd
                args: 2,
            },
        ],
        vec![],
        100,
        1024,
        1,
    );

    let result = vm.run();
    // Should push error for overflow
    assert!(matches!(result, Ok(Value::Error(_))));
}

#[test]
fn test_host_call_capability_mapping_for_arithmetic() {
    // Test that arithmetic functions don't require capabilities
    use crate::vm::opcodes::capability::get_required_capability_for_host_function;

    // Arithmetic operations should return None (no capability required)
    assert_eq!(get_required_capability_for_host_function(9), None);  // IntAdd
    assert_eq!(get_required_capability_for_host_function(10), None); // IntSub
    assert_eq!(get_required_capability_for_host_function(11), None); // IntMul
    assert_eq!(get_required_capability_for_host_function(12), None); // IntDiv
    assert_eq!(get_required_capability_for_host_function(13), None); // IntMod
    assert_eq!(get_required_capability_for_host_function(14), None); // FloatAdd
    assert_eq!(get_required_capability_for_host_function(15), None); // FloatSub
    assert_eq!(get_required_capability_for_host_function(16), None); // FloatMul
    assert_eq!(get_required_capability_for_host_function(17), None); // FloatDiv
    assert_eq!(get_required_capability_for_host_function(18), None); // IntToFloat
    assert_eq!(get_required_capability_for_host_function(19), None); // FloatToInt
    assert_eq!(get_required_capability_for_host_function(20), None); // IntEq
    assert_eq!(get_required_capability_for_host_function(21), None); // IntLt
    assert_eq!(get_required_capability_for_host_function(22), None); // IntGt
    assert_eq!(get_required_capability_for_host_function(23), None); // FloatEq
    assert_eq!(get_required_capability_for_host_function(24), None); // FloatLt
    assert_eq!(get_required_capability_for_host_function(25), None); // FloatGt
}
