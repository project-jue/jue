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
    // Variable Access
    GetLocal(u16), // Get local variable at stack offset
    SetLocal(u16), // Set local variable at stack offset
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
            OpCode::GetLocal(_) => 3, // u16 (2 bytes) + opcode tag (1 byte)
            OpCode::SetLocal(_) => 3, // u16 (2 bytes) + opcode tag (1 byte)
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

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Symbol(idx) => write!(f, "Symbol({})", idx),
            Value::Pair(ptr) => write!(f, "Pair({})", ptr),
            Value::Closure(ptr) => write!(f, "Closure({})", ptr),
            Value::ActorId(id) => write!(f, "Actor({})", id),
            Value::Capability(cap) => write!(f, "Capability({:?})", cap),
        }
    }
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

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Capability::MetaSelfModify => write!(f, "MetaSelfModify"),
            Capability::MetaGrant => write!(f, "MetaGrant"),
            Capability::MacroHygienic => write!(f, "MacroHygienic"),
            Capability::MacroUnsafe => write!(f, "MacroUnsafe"),
            Capability::ComptimeEval => write!(f, "ComptimeEval"),
            Capability::IoReadSensor => write!(f, "IoReadSensor"),
            Capability::IoWriteActuator => write!(f, "IoWriteActuator"),
            Capability::IoNetwork => write!(f, "IoNetwork"),
            Capability::IoPersist => write!(f, "IoPersist"),
            Capability::SysCreateActor => write!(f, "SysCreateActor"),
            Capability::SysTerminateActor => write!(f, "SysTerminateActor"),
            Capability::SysClock => write!(f, "SysClock"),
            Capability::ResourceExtraMemory(bytes) => write!(f, "ResourceExtraMemory({})", bytes),
            Capability::ResourceExtraTime(ms) => write!(f, "ResourceExtraTime({})", ms),
        }
    }
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

// Rest of the file remains the same...
