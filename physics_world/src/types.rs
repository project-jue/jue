use serde::{Deserialize, Serialize};
use std::fmt;

// A strongly-typed index into an ObjectArena's storage vector.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HeapPtr(pub u32);

impl HeapPtr {
    pub fn new(ptr: u32) -> Self {
        Self(ptr)
    }

    pub fn get(&self) -> u32 {
        self.0
    }
}

impl fmt::Display for HeapPtr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HeapPtr({})", self.0)
    }
}

// Represents a compiled instruction.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpCode {
    // Constants
    Nil,
    Bool(bool),
    Int(i64),
    Symbol(usize),
    // Primitive Stack Operations
    Swap, // Swap top two stack values
    Dup,
    Pop,
    // Heap
    Cons,
    Car,
    Cdr,
    // Control
    Call(u16), // Argument count
    Ret,
    Jmp(i16),
    JmpIfFalse(i16),
    // Actors
    Yield,
    Send,
    // Closure Operations
    MakeClosure(usize /* code_idx */, usize /* capture_count */),
    // Resource Management
    CheckStepLimit,

    // Primitive Arithmetic (Int64)
    Add, // TOS = TOS + TOS-1
    Sub, // TOS = TOS - TOS-1
    Mul,
    Div,
    Mod,

    // Primitive Comparisons (result is Bool)
    Eq, // TOS == TOS-1 ?
    Lt, // TOS < TOS-1 ?
    Gt,

    // --- CAPABILITY INSTRUCTIONS ---
    /// Check if actor has capability. Pushes bool to stack.
    /// Operand: index into constant pool where Capability is stored
    HasCap(usize),

    /// Request a capability from scheduler. Blocks until decision.
    /// Operand: capability index, justification string index
    RequestCap(usize, usize),

    /// Grant a capability to another actor.
    /// Requires MetaGrant capability.
    /// Operand: target actor ID, capability index
    GrantCap(u32, usize),

    /// Revoke a capability (from self or other with MetaGrant).
    /// Operand: target actor ID, capability index
    RevokeCap(u32, usize),

    /// Execute a privileged host call.
    /// Requires specific capability (encoded in constant pool).
    /// Format: HostCall { cap_index, function_id, arg_count }
    HostCall {
        cap_idx: usize,
        func_id: u16,
        args: u8,
    },
}

impl OpCode {
    pub fn size_bytes(&self) -> usize {
        match self {
            OpCode::Nil => 1,
            OpCode::Bool(_) => 2,
            OpCode::Int(_) => 9,
            OpCode::Symbol(_) => 5,
            OpCode::Dup => 1,
            OpCode::Pop => 1,
            OpCode::Swap => 1,
            OpCode::Cons => 1,
            OpCode::Car => 1,
            OpCode::Cdr => 1,
            OpCode::Call(_) => 3,
            OpCode::Ret => 1,
            OpCode::Jmp(_) => 3,
            OpCode::JmpIfFalse(_) => 3,
            OpCode::Yield => 1,
            OpCode::Send => 1,
            OpCode::Add => 1,
            OpCode::Sub => 1,
            OpCode::Mul => 1,
            OpCode::Div => 1,
            OpCode::Mod => 1,
            OpCode::Eq => 1,
            OpCode::Lt => 1,
            OpCode::Gt => 1,
            OpCode::MakeClosure(_, _) => 9, // 4 bytes for each usize
            OpCode::CheckStepLimit => 1,
            // Capability instructions
            OpCode::HasCap(_) => 5, // usize (4 bytes) + opcode tag (1 byte)
            OpCode::RequestCap(_, _) => 9, // 2 x usize (8 bytes) + opcode tag (1 byte)
            OpCode::GrantCap(_, _) => 6, // u32 (4 bytes) + usize (4 bytes) + opcode tag (1 byte) = 9 bytes
            OpCode::RevokeCap(_, _) => 6, // u32 (4 bytes) + usize (4 bytes) + opcode tag (1 byte) = 9 bytes
            OpCode::HostCall {
                cap_idx: _,
                func_id: _,
                args: _,
            } => 7, // usize (4) + u16 (2) + u8 (1) + opcode tag (1) = 8 bytes
        }
    }
}

// OpCode Size Analysis: u8 vs u16 Decision
//
// Analysis: OpCode CANNOT fit in u8 and REQUIRES u16 for efficient storage.
//
// Reasoning:
// 1. OpCode variants like Call(u16), Jmp(i16), JmpIfFalse(i16) require 2-byte parameters
// 2. Individual OpCode instruction size ranges from 1-9 bytes as calculated by size_bytes()
// 3. The enum has 16 distinct variants, requiring at least 4 bits for discrimination
// 4. For efficient binary representation, u16 (2 bytes) is the minimum safe choice
// 5. Using u8 would require complex variable-length encoding and hurt performance
// 6. u16 provides sufficient range while maintaining word-alignment for fast access
//
// Decision: Use u16 for OpCode representation in bytecode format for optimal performance.

// Represents a value in the VM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),      // Primary deterministic number type.
    Symbol(usize), // Index into a constant table.
    Pair(HeapPtr), // HeapPtr is a u32 index into an ObjectArena.
    Closure(HeapPtr),
    ActorId(u32),
    Capability(Capability),
}

// Capability enum for the capability system
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum Capability {
    // Meta-capabilities
    MetaSelfModify, // Can modify own non-core code
    MetaGrant,      // Can grant capabilities to others (dangerous)

    // Macro & Compile-time capabilities
    MacroHygienic, // Can expand hygienic macros
    MacroUnsafe,   // Can generate arbitrary syntax
    ComptimeEval,  // Can execute code at compile-time

    // I/O & External World
    IoReadSensor,    // Read from virtual sensors
    IoWriteActuator, // Write to virtual actuators
    IoNetwork,       // Network access
    IoPersist,       // Write to persistent storage

    // System
    SysCreateActor,    // Can spawn new actors
    SysTerminateActor, // Can terminate actors (including self)
    SysClock,          // Access non-deterministic time

    // Resource privileges
    ResourceExtraMemory(u64), // Additional memory quota
    ResourceExtraTime(u64),   // Additional time quota
}

// Host function enum for FFI operations
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostFunction {
    ReadSensor = 0,
    WriteActuator = 1,
    GetWallClockNs = 2,
    SpawnActor = 3,
    TerminateActor = 4,
    NetworkSend = 5,
    NetworkReceive = 6,
    PersistWrite = 7,
    PersistRead = 8,
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Symbol(_) => true,
            Value::Pair(_) => true,
            Value::Closure(_) => true,
            Value::ActorId(_) => true,
            Value::Capability(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_ptr() {
        let ptr = HeapPtr::new(42);
        assert_eq!(ptr.get(), 42);
        assert_eq!(format!("{}", ptr), "HeapPtr(42)");
    }

    #[test]
    fn test_opcode_sizes() {
        assert_eq!(OpCode::Nil.size_bytes(), 1);
        assert_eq!(OpCode::Bool(true).size_bytes(), 2);
        assert_eq!(OpCode::Int(42).size_bytes(), 9);
        assert_eq!(OpCode::Symbol(0).size_bytes(), 5);
        assert_eq!(OpCode::Call(2).size_bytes(), 3);
        // Test capability instruction sizes
        assert_eq!(OpCode::HasCap(0).size_bytes(), 5);
        assert_eq!(OpCode::RequestCap(0, 1).size_bytes(), 9);
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
    }

    #[test]
    fn test_value_variants() {
        let nil = Value::Nil;
        let bool_val = Value::Bool(true);
        let int_val = Value::Int(42);
        let symbol_val = Value::Symbol(0);
        let pair_val = Value::Pair(HeapPtr::new(1));
        let closure_val = Value::Closure(HeapPtr::new(2));
        let actor_val = Value::ActorId(3);
        let cap_val = Value::Capability(Capability::MetaSelfModify);

        assert!(!nil.is_truthy());
        assert!(bool_val.is_truthy());
        assert!(int_val.is_truthy());
        assert!(symbol_val.is_truthy());
        assert!(pair_val.is_truthy());
        assert!(closure_val.is_truthy());
        assert!(actor_val.is_truthy());
        assert!(cap_val.is_truthy());

        assert_eq!(Value::Int(0).is_truthy(), false);
    }

    #[test]
    fn test_capability_serialization() {
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
            let serialized = serde_json::to_string(&cap).unwrap();
            let deserialized: Capability = serde_json::from_str(&serialized).unwrap();
            assert_eq!(cap, deserialized);
        }
    }

    #[test]
    fn test_capability_equality_and_hashing() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(Capability::MetaSelfModify);
        set.insert(Capability::MetaGrant);

        assert!(set.contains(&Capability::MetaSelfModify));
        assert!(set.contains(&Capability::MetaGrant));
        assert!(!set.contains(&Capability::MacroHygienic));

        // Test that different resource capabilities with same value are equal
        assert_eq!(
            Capability::ResourceExtraMemory(1024),
            Capability::ResourceExtraMemory(1024)
        );
        assert_ne!(
            Capability::ResourceExtraMemory(1024),
            Capability::ResourceExtraMemory(2048)
        );
    }

    #[test]
    fn test_host_function_enum() {
        // Test that all host functions have unique values
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

        let mut values = std::collections::HashSet::new();
        for &func in &functions {
            let value = func as u8;
            assert!(
                !values.contains(&value),
                "Duplicate host function value: {}",
                value
            );
            values.insert(value);
        }

        // Test serialization
        for &func in &functions {
            let serialized = serde_json::to_string(&func).unwrap();
            let deserialized: HostFunction = serde_json::from_str(&serialized).unwrap();
            assert_eq!(func, deserialized);
        }
    }

    #[test]
    fn test_capability_equality_edge_cases() {
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
    fn test_capability_inequality() {
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
    fn test_capability_serialization_edge_cases() {
        // Test serialization of all capability variants
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
}
