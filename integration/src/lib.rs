/// Integration bridge module
/// Provides conversion functions between Core-World and Physics-World types
use core_world::core_expr::CoreExpr;
use physics_world::types::Value;

/// Convert a Physics-World Value to a Core-World CoreExpr
/// This function provides the bridge between the two layers
pub fn core_expr_from_value(value: &Value) -> CoreExpr {
    match value {
        Value::Nil => CoreExpr::Nat(0), // Represent nil as zero for simplicity
        Value::Bool(true) => CoreExpr::Nat(1),
        Value::Bool(false) => CoreExpr::Nat(0),
        Value::Int(n) => CoreExpr::Nat(*n as u64),
        Value::Symbol(usize) => CoreExpr::Var(*usize), // Use symbol index as variable index
        Value::Pair(_) => {
            // For pairs, we'll create a placeholder representation
            // In a full implementation, this would need proper pair handling
            CoreExpr::Nat(42) // Placeholder for pair representation
        }
        Value::Closure(_) => {
            // For closures, we'll create a placeholder representation
            // In a full implementation, this would need proper closure handling
            CoreExpr::Nat(43) // Placeholder for closure representation
        }
        Value::ActorId(id) => CoreExpr::Nat(*id as u64),
        Value::Capability(_) => {
            // For capabilities, we'll create a placeholder representation
            // In a full implementation, this would need proper capability handling
            CoreExpr::Nat(44) // Placeholder for capability representation
        }
        Value::GcPtr(_) => {
            // For GC pointers, we'll create a placeholder representation
            // In a full implementation, this would need proper GC pointer handling
            CoreExpr::Nat(45) // Placeholder for GC pointer representation
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use physics_world::types::{Capability, HostFunction, OpCode, Value};
    use serde_json;

    #[test]
    fn test_core_expr_from_value_conversions() {
        // Test basic value conversions
        assert_eq!(core_expr_from_value(&Value::Nil), CoreExpr::Nat(0));
        assert_eq!(core_expr_from_value(&Value::Bool(true)), CoreExpr::Nat(1));
        assert_eq!(core_expr_from_value(&Value::Bool(false)), CoreExpr::Nat(0));
        assert_eq!(core_expr_from_value(&Value::Int(42)), CoreExpr::Nat(42));
        assert_eq!(core_expr_from_value(&Value::Symbol(5)), CoreExpr::Var(5));
        assert_eq!(
            core_expr_from_value(&Value::ActorId(123)),
            CoreExpr::Nat(123)
        );
    }

    #[test]
    fn test_capability_conversion_to_core_expr() {
        // Test capability value conversions
        let meta_self_modify = Value::Capability(Capability::MetaSelfModify);
        let meta_grant = Value::Capability(Capability::MetaGrant);
        let io_network = Value::Capability(Capability::IoNetwork);
        let resource_mem = Value::Capability(Capability::ResourceExtraMemory(1024));
        let resource_time = Value::Capability(Capability::ResourceExtraTime(5000));

        // All capabilities should convert to the same placeholder for now
        assert_eq!(core_expr_from_value(&meta_self_modify), CoreExpr::Nat(44));
        assert_eq!(core_expr_from_value(&meta_grant), CoreExpr::Nat(44));
        assert_eq!(core_expr_from_value(&io_network), CoreExpr::Nat(44));
        assert_eq!(core_expr_from_value(&resource_mem), CoreExpr::Nat(44));
        assert_eq!(core_expr_from_value(&resource_time), CoreExpr::Nat(44));
    }

    #[test]
    fn test_capability_enum_comprehensive() {
        // Test all capability variants can be converted
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

        for cap in capabilities {
            let value = Value::Capability(cap);
            let core_expr = core_expr_from_value(&value);
            // All capabilities should convert to the placeholder value
            assert_eq!(core_expr, CoreExpr::Nat(44));
        }
    }

    #[test]
    fn test_capability_opcode_integration() {
        // Test that capability opcodes can be used in integration context
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

        // Test that all capability opcodes can be serialized and deserialized
        for opcode in &opcodes {
            let serialized = serde_json::to_string(opcode).unwrap();
            let deserialized: OpCode = serde_json::from_str(&serialized).unwrap();
            assert_eq!(*opcode, deserialized);
        }
    }

    #[test]
    fn test_host_function_integration() {
        // Test that host functions can be used in integration context
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

        // Test that all host functions have unique values
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

        // Test that all host functions can be serialized and deserialized
        for func in &functions {
            let serialized = serde_json::to_string(func).unwrap();
            let deserialized: HostFunction = serde_json::from_str(&serialized).unwrap();
            assert_eq!(*func, deserialized);
        }
    }

    #[test]
    fn test_capability_constant_pool_integration() {
        // Test that capabilities can be used in constant pools for integration
        let constant_pool = vec![
            Value::Capability(Capability::MetaGrant),
            Value::Capability(Capability::IoNetwork),
            Value::Capability(Capability::ResourceExtraMemory(2048)),
            Value::Capability(Capability::ResourceExtraTime(10000)),
        ];

        // Test that we can retrieve capabilities from the pool
        assert_eq!(constant_pool.len(), 4);

        // Test that all capabilities can be converted to core expressions
        for value in &constant_pool {
            let core_expr = core_expr_from_value(value);
            assert_eq!(core_expr, CoreExpr::Nat(44));
        }
    }

    #[test]
    fn test_capability_value_equality_integration() {
        // Test equality of capability values in integration context
        let cap1 = Value::Capability(Capability::MetaSelfModify);
        let cap2 = Value::Capability(Capability::MetaSelfModify);
        let cap3 = Value::Capability(Capability::MetaGrant);

        assert_eq!(cap1, cap2);
        assert_ne!(cap1, cap3);

        // Test that different resource capabilities are not equal
        let mem1 = Value::Capability(Capability::ResourceExtraMemory(1024));
        let mem2 = Value::Capability(Capability::ResourceExtraMemory(2048));
        assert_ne!(mem1, mem2);

        // Test that all capability values convert to the same core expression
        let core1 = core_expr_from_value(&cap1);
        let core2 = core_expr_from_value(&cap2);
        let core3 = core_expr_from_value(&cap3);
        assert_eq!(core1, core2);
        assert_eq!(core2, core3);
    }

    #[test]
    fn test_capability_resource_edge_cases_integration() {
        // Test resource capabilities with edge case values in integration context
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

            // Test that all resource capabilities convert to the same core expression
            let value = Value::Capability(cap);
            let core_expr = core_expr_from_value(&value);
            assert_eq!(core_expr, CoreExpr::Nat(44));
        }
    }

    #[test]
    fn test_capability_opcode_size_integration() {
        // Test size calculations for capability opcodes in integration context
        assert_eq!(OpCode::HasCap(0).size_bytes(), 5);
        assert_eq!(OpCode::RequestCap(0, 0).size_bytes(), 9);
        assert_eq!(OpCode::GrantCap(0, 0).size_bytes(), 6);
        assert_eq!(OpCode::RevokeCap(0, 0).size_bytes(), 6);

        assert_eq!(
            OpCode::HostCall {
                cap_idx: 0,
                func_id: 0,
                args: 0
            }
            .size_bytes(),
            7
        );

        // Test with maximum values
        assert_eq!(OpCode::HasCap(u32::MAX as usize).size_bytes(), 5);
        assert_eq!(
            OpCode::RequestCap(u32::MAX as usize, u32::MAX as usize).size_bytes(),
            9
        );
        assert_eq!(
            OpCode::GrantCap(u32::MAX, u32::MAX as usize).size_bytes(),
            6
        );
        assert_eq!(
            OpCode::RevokeCap(u32::MAX, u32::MAX as usize).size_bytes(),
            6
        );
    }

    #[test]
    fn test_capability_enum_exhaustiveness_integration() {
        // This test helps ensure we don't miss any capability variants in integration testing
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

            // Test that all variants can be converted to core expressions
            let value = Value::Capability(cap.clone());
            let core_expr = core_expr_from_value(&value);
            assert_eq!(core_expr, CoreExpr::Nat(44));
        }
    }
}
