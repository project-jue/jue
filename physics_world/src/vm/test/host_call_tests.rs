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
