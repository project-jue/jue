use physics_world::types::{Capability, HostFunction, OpCode, Value};
use serde_json;

#[test]
fn test_capability_enum_comprehensive() {
    // Test all capability variants
    let capabilities = vec![
        Capability::MetaSelfModify,
        Capability::MetaGrant,
        Capability::MacroHygienic,
        Capability::MacroUnsafe,
        Capability::ComptimeEval,
        Capability::IoReadSensor,
        Capability::IoWriteActuator,
        Capability::IoNetwork,
        Capability::IoPersist,
        Capability::SysCreateActor,
        Capability::SysTerminateActor,
        Capability::SysClock,
        Capability::ResourceExtraMemory(1024),
        Capability::ResourceExtraTime(5000),
    ];

    // Test that all capabilities can be serialized and deserialized
    for cap in &capabilities {
        let serialized = serde_json::to_string(cap).unwrap();
        let deserialized: Capability = serde_json::from_str(&serialized).unwrap();
        assert_eq!(*cap, deserialized);
    }

    // Test that capabilities can be used in Value enum
    let cap_value = Value::Capability(Capability::MetaSelfModify);
    assert!(cap_value.is_truthy());

    // Test that capabilities can be used in constant pool
    let constant_pool = vec![
        Value::Capability(Capability::MetaGrant),
        Value::Capability(Capability::IoNetwork),
    ];

    assert_eq!(constant_pool.len(), 2);
}

#[test]
fn test_host_function_enum_comprehensive() {
    // Test all host functions
    let functions = vec![
        HostFunction::ReadSensor,
        HostFunction::WriteActuator,
        HostFunction::GetWallClockNs,
        HostFunction::SpawnActor,
        HostFunction::TerminateActor,
        HostFunction::NetworkSend,
        HostFunction::NetworkReceive,
        HostFunction::PersistWrite,
        HostFunction::PersistRead,
    ];

    // Test that all functions have unique values
    let mut values = std::collections::HashSet::new();
    for func in &functions {
        let value = *func as u8;
        assert!(
            !values.contains(&value),
            "Duplicate host function value: {}",
            value
        );
        values.insert(value);
    }

    // Test serialization
    for func in &functions {
        let serialized = serde_json::to_string(func).unwrap();
        let deserialized: HostFunction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(*func, deserialized);
    }
}

#[test]
fn test_capability_opcodes() {
    // Test that capability opcodes can be created and have correct sizes
    let opcodes = vec![
        OpCode::HasCap(0),
        OpCode::RequestCap(0, 1),
        OpCode::GrantCap(1, 0),
        OpCode::RevokeCap(2, 0),
        OpCode::HostCall {
            cap_idx: 0,
            func_id: HostFunction::ReadSensor as u16,
            args: 1,
        },
    ];

    // Test sizes
    assert_eq!(opcodes[0].size_bytes(), 5); // HasCap
    assert_eq!(opcodes[1].size_bytes(), 9); // RequestCap
    assert_eq!(opcodes[2].size_bytes(), 6); // GrantCap
    assert_eq!(opcodes[3].size_bytes(), 6); // RevokeCap
    assert_eq!(opcodes[4].size_bytes(), 7); // HostCall

    // Test serialization
    for opcode in &opcodes {
        let serialized = serde_json::to_string(opcode).unwrap();
        let deserialized: OpCode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(*opcode, deserialized);
    }
}

#[test]
fn test_capability_hashing() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(Capability::MetaSelfModify);
    set.insert(Capability::MetaGrant);
    set.insert(Capability::ResourceExtraMemory(1024));

    // Test containment
    assert!(set.contains(&Capability::MetaSelfModify));
    assert!(set.contains(&Capability::MetaGrant));
    assert!(set.contains(&Capability::ResourceExtraMemory(1024)));
    assert!(!set.contains(&Capability::MacroHygienic));

    // Test that different resource capabilities are different
    assert_ne!(
        Capability::ResourceExtraMemory(1024),
        Capability::ResourceExtraMemory(2048)
    );
    assert_ne!(
        Capability::ResourceExtraTime(1000),
        Capability::ResourceExtraTime(2000)
    );
}

#[test]
fn test_capability_resource_edge_cases() {
    // Test resource capabilities with edge case values
    let edge_cases = vec![
        (Capability::ResourceExtraMemory(0), 0),
        (Capability::ResourceExtraMemory(1), 1),
        (Capability::ResourceExtraMemory(u64::MAX), u64::MAX),
        (Capability::ResourceExtraTime(0), 0),
        (Capability::ResourceExtraTime(1), 1),
        (Capability::ResourceExtraTime(u64::MAX), u64::MAX),
    ];

    for (cap, expected_value) in edge_cases {
        match cap {
            Capability::ResourceExtraMemory(val) => assert_eq!(val, expected_value),
            Capability::ResourceExtraTime(val) => assert_eq!(val, expected_value),
            _ => panic!("Expected resource capability"),
        }
    }
}

#[test]
fn test_capability_equality_comprehensive() {
    // Test equality for all capability variants
    let caps = vec![
        (Capability::MetaSelfModify, Capability::MetaSelfModify),
        (Capability::MetaGrant, Capability::MetaGrant),
        (Capability::MacroHygienic, Capability::MacroHygienic),
        (Capability::MacroUnsafe, Capability::MacroUnsafe),
        (Capability::ComptimeEval, Capability::ComptimeEval),
        (Capability::IoReadSensor, Capability::IoReadSensor),
        (Capability::IoWriteActuator, Capability::IoWriteActuator),
        (Capability::IoNetwork, Capability::IoNetwork),
        (Capability::IoPersist, Capability::IoPersist),
        (Capability::SysCreateActor, Capability::SysCreateActor),
        (Capability::SysTerminateActor, Capability::SysTerminateActor),
        (Capability::SysClock, Capability::SysClock),
        (
            Capability::ResourceExtraMemory(1024),
            Capability::ResourceExtraMemory(1024),
        ),
        (
            Capability::ResourceExtraTime(5000),
            Capability::ResourceExtraTime(5000),
        ),
    ];

    for (cap1, cap2) in caps {
        assert_eq!(cap1, cap2, "Capabilities should be equal: {:?}", cap1);
    }
}

#[test]
fn test_capability_inequality_comprehensive() {
    // Test that different capabilities are not equal
    assert_ne!(Capability::MetaSelfModify, Capability::MetaGrant);
    assert_ne!(Capability::MetaGrant, Capability::MacroHygienic);
    assert_ne!(Capability::IoReadSensor, Capability::IoWriteActuator);
    assert_ne!(Capability::SysCreateActor, Capability::SysTerminateActor);

    // Test resource capabilities with different values
    assert_ne!(
        Capability::ResourceExtraMemory(1024),
        Capability::ResourceExtraMemory(2048)
    );
    assert_ne!(
        Capability::ResourceExtraTime(1000),
        Capability::ResourceExtraTime(2000)
    );
    assert_ne!(
        Capability::ResourceExtraMemory(1024),
        Capability::ResourceExtraTime(1024)
    );
}

#[test]
fn test_capability_serialization_roundtrip() {
    // Test serialization round-trip for all capability variants
    let capabilities = vec![
        Capability::MetaSelfModify,
        Capability::MetaGrant,
        Capability::MacroHygienic,
        Capability::MacroUnsafe,
        Capability::ComptimeEval,
        Capability::IoReadSensor,
        Capability::IoWriteActuator,
        Capability::IoNetwork,
        Capability::IoPersist,
        Capability::SysCreateActor,
        Capability::SysTerminateActor,
        Capability::SysClock,
        Capability::ResourceExtraMemory(u64::MAX),
        Capability::ResourceExtraTime(u64::MIN),
    ];

    for cap in capabilities {
        let serialized = serde_json::to_string(&cap).unwrap();
        let deserialized: Capability = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cap, deserialized, "Serialization failed for {:?}", cap);
    }
}

#[test]
fn test_host_function_numeric_values() {
    // Test that host functions have expected numeric values
    assert_eq!(HostFunction::ReadSensor as u8, 0);
    assert_eq!(HostFunction::WriteActuator as u8, 1);
    assert_eq!(HostFunction::GetWallClockNs as u8, 2);
    assert_eq!(HostFunction::SpawnActor as u8, 3);
    assert_eq!(HostFunction::TerminateActor as u8, 4);
    assert_eq!(HostFunction::NetworkSend as u8, 5);
    assert_eq!(HostFunction::NetworkReceive as u8, 6);
    assert_eq!(HostFunction::PersistWrite as u8, 7);
    assert_eq!(HostFunction::PersistRead as u8, 8);
}

#[test]
fn test_capability_opcode_creation() {
    // Test creation of all capability-related opcodes
    let has_cap = OpCode::HasCap(42);
    let request_cap = OpCode::RequestCap(1, 2);
    let grant_cap = OpCode::GrantCap(100, 3);
    let revoke_cap = OpCode::RevokeCap(200, 4);
    let host_call = OpCode::HostCall {
        cap_idx: 5,
        func_id: HostFunction::ReadSensor as u16,
        args: 2,
    };

    // Verify opcode variants
    match has_cap {
        OpCode::HasCap(idx) => assert_eq!(idx, 42),
        _ => panic!("Expected HasCap opcode"),
    }

    match request_cap {
        OpCode::RequestCap(cap_idx, just_idx) => {
            assert_eq!(cap_idx, 1);
            assert_eq!(just_idx, 2);
        }
        _ => panic!("Expected RequestCap opcode"),
    }

    match grant_cap {
        OpCode::GrantCap(actor_id, cap_idx) => {
            assert_eq!(actor_id, 100);
            assert_eq!(cap_idx, 3);
        }
        _ => panic!("Expected GrantCap opcode"),
    }

    match revoke_cap {
        OpCode::RevokeCap(actor_id, cap_idx) => {
            assert_eq!(actor_id, 200);
            assert_eq!(cap_idx, 4);
        }
        _ => panic!("Expected RevokeCap opcode"),
    }

    match host_call {
        OpCode::HostCall {
            cap_idx,
            func_id,
            args,
        } => {
            assert_eq!(cap_idx, 5);
            assert_eq!(func_id, HostFunction::ReadSensor as u16);
            assert_eq!(args, 2);
        }
        _ => panic!("Expected HostCall opcode"),
    }
}

#[test]
fn test_capability_opcode_size_calculations() {
    // Test size calculations for capability opcodes with various parameters
    assert_eq!(OpCode::HasCap(0).size_bytes(), 5);
    assert_eq!(OpCode::HasCap(u32::MAX as usize).size_bytes(), 5);

    assert_eq!(OpCode::RequestCap(0, 0).size_bytes(), 9);
    assert_eq!(
        OpCode::RequestCap(u32::MAX as usize, u32::MAX as usize).size_bytes(),
        9
    );

    assert_eq!(OpCode::GrantCap(0, 0).size_bytes(), 6);
    assert_eq!(
        OpCode::GrantCap(u32::MAX, u32::MAX as usize).size_bytes(),
        6
    );

    assert_eq!(OpCode::RevokeCap(0, 0).size_bytes(), 6);
    assert_eq!(
        OpCode::RevokeCap(u32::MAX, u32::MAX as usize).size_bytes(),
        6
    );

    assert_eq!(
        OpCode::HostCall {
            cap_idx: 0,
            func_id: 0,
            args: 0
        }
        .size_bytes(),
        7
    );
    assert_eq!(
        OpCode::HostCall {
            cap_idx: u32::MAX as usize,
            func_id: u16::MAX,
            args: u8::MAX
        }
        .size_bytes(),
        7
    );
}

#[test]
fn test_value_capability_truthiness() {
    // Test that all capability values are truthy
    let capabilities = vec![
        Capability::MetaSelfModify,
        Capability::MetaGrant,
        Capability::MacroHygienic,
        Capability::MacroUnsafe,
        Capability::ComptimeEval,
        Capability::IoReadSensor,
        Capability::IoWriteActuator,
        Capability::IoNetwork,
        Capability::IoPersist,
        Capability::SysCreateActor,
        Capability::SysTerminateActor,
        Capability::SysClock,
        Capability::ResourceExtraMemory(0),
        Capability::ResourceExtraTime(0),
    ];

    for cap in &capabilities {
        let value = Value::Capability(cap.clone());
        assert!(value.is_truthy(), "Capability should be truthy: {:?}", cap);
    }
}

#[test]
fn test_capability_enum_exhaustiveness() {
    // This test helps ensure we don't miss any capability variants in testing
    // We'll test that we can create and serialize all variants
    let all_capabilities = vec![
        Capability::MetaSelfModify,
        Capability::MetaGrant,
        Capability::MacroHygienic,
        Capability::MacroUnsafe,
        Capability::ComptimeEval,
        Capability::IoReadSensor,
        Capability::IoWriteActuator,
        Capability::IoNetwork,
        Capability::IoPersist,
        Capability::SysCreateActor,
        Capability::SysTerminateActor,
        Capability::SysClock,
        Capability::ResourceExtraMemory(1024),
        Capability::ResourceExtraTime(5000),
    ];

    // Test that we can serialize and deserialize all variants
    for cap in &all_capabilities {
        let serialized = serde_json::to_string(&cap).unwrap();
        let deserialized: Capability = serde_json::from_str(&serialized).unwrap();
        assert_eq!(cap, &deserialized);
    }

    // Test that all variants are truthy when wrapped in Value
    for cap in &all_capabilities {
        let value = Value::Capability(cap.clone());
        assert!(value.is_truthy());
    }
}

#[test]
fn test_capability_constant_pool_integration() {
    // Test that capabilities can be used in constant pools
    let constant_pool = vec![
        Value::Capability(Capability::MetaGrant),
        Value::Capability(Capability::IoNetwork),
        Value::Capability(Capability::ResourceExtraMemory(2048)),
        Value::Capability(Capability::ResourceExtraTime(10000)),
    ];

    // Test that we can retrieve capabilities from the pool
    assert_eq!(constant_pool.len(), 4);

    match &constant_pool[0] {
        Value::Capability(Capability::MetaGrant) => {}
        _ => panic!("Expected MetaGrant capability"),
    }

    match &constant_pool[1] {
        Value::Capability(Capability::IoNetwork) => {}
        _ => panic!("Expected IoNetwork capability"),
    }

    match &constant_pool[2] {
        Value::Capability(Capability::ResourceExtraMemory(2048)) => {}
        _ => panic!("Expected ResourceExtraMemory capability"),
    }

    match &constant_pool[3] {
        Value::Capability(Capability::ResourceExtraTime(10000)) => {}
        _ => panic!("Expected ResourceExtraTime capability"),
    }
}

#[test]
fn test_host_function_serialization_roundtrip() {
    let functions = vec![
        HostFunction::ReadSensor,
        HostFunction::WriteActuator,
        HostFunction::GetWallClockNs,
        HostFunction::SpawnActor,
        HostFunction::TerminateActor,
        HostFunction::NetworkSend,
        HostFunction::NetworkReceive,
        HostFunction::PersistWrite,
        HostFunction::PersistRead,
    ];

    for func in functions {
        let serialized = serde_json::to_string(&func).unwrap();
        let deserialized: HostFunction = serde_json::from_str(&serialized).unwrap();
        assert_eq!(func, deserialized);
    }
}

#[test]
fn test_capability_opcode_serialization() {
    // Test serialization of all capability-related opcodes
    let opcodes = vec![
        OpCode::HasCap(42),
        OpCode::RequestCap(1, 2),
        OpCode::GrantCap(100, 3),
        OpCode::RevokeCap(200, 4),
        OpCode::HostCall {
            cap_idx: 5,
            func_id: HostFunction::ReadSensor as u16,
            args: 2,
        },
    ];

    for opcode in &opcodes {
        let serialized = serde_json::to_string(opcode).unwrap();
        let deserialized: OpCode = serde_json::from_str(&serialized).unwrap();
        assert_eq!(*opcode, deserialized);
    }
}

#[test]
fn test_capability_value_equality() {
    // Test equality of capability values
    let cap1 = Value::Capability(Capability::MetaSelfModify);
    let cap2 = Value::Capability(Capability::MetaSelfModify);
    let cap3 = Value::Capability(Capability::MetaGrant);

    assert_eq!(cap1, cap2);
    assert_ne!(cap1, cap3);

    // Test that different resource capabilities are not equal
    let mem1 = Value::Capability(Capability::ResourceExtraMemory(1024));
    let mem2 = Value::Capability(Capability::ResourceExtraMemory(2048));
    assert_ne!(mem1, mem2);
}

#[test]
fn test_capability_hashing_comprehensive() {
    use std::collections::HashSet;

    let mut set = HashSet::new();
    set.insert(Capability::MetaSelfModify);
    set.insert(Capability::MetaGrant);
    set.insert(Capability::MacroHygienic);
    set.insert(Capability::MacroUnsafe);
    set.insert(Capability::ComptimeEval);
    set.insert(Capability::IoReadSensor);
    set.insert(Capability::IoWriteActuator);
    set.insert(Capability::IoNetwork);
    set.insert(Capability::IoPersist);
    set.insert(Capability::SysCreateActor);
    set.insert(Capability::SysTerminateActor);
    set.insert(Capability::SysClock);
    set.insert(Capability::ResourceExtraMemory(1024));
    set.insert(Capability::ResourceExtraTime(5000));

    // Test that all inserted capabilities are in the set
    assert!(set.contains(&Capability::MetaSelfModify));
    assert!(set.contains(&Capability::MetaGrant));
    assert!(set.contains(&Capability::MacroHygienic));
    assert!(set.contains(&Capability::MacroUnsafe));
    assert!(set.contains(&Capability::ComptimeEval));
    assert!(set.contains(&Capability::IoReadSensor));
    assert!(set.contains(&Capability::IoWriteActuator));
    assert!(set.contains(&Capability::IoNetwork));
    assert!(set.contains(&Capability::IoPersist));
    assert!(set.contains(&Capability::SysCreateActor));
    assert!(set.contains(&Capability::SysTerminateActor));
    assert!(set.contains(&Capability::SysClock));
    assert!(set.contains(&Capability::ResourceExtraMemory(1024)));
    assert!(set.contains(&Capability::ResourceExtraTime(5000)));

    // Test that capabilities not in the set are not found
    assert!(!set.contains(&Capability::ResourceExtraMemory(2048)));
    assert!(!set.contains(&Capability::ResourceExtraTime(10000)));

    // Test set size
    assert_eq!(set.len(), 14);
}
