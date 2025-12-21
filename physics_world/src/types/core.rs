/// Core types for Physics World
use serde::{Deserialize, Serialize};
use std::fmt;

/// A strongly-typed index into an ObjectArena's storage vector.
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

/// Represents a compiled instruction.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OpCode {
    // Constants
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64), // NEW: Direct float constant
    Symbol(usize),
    // String Operations
    LoadString(usize), // Load string from constant pool by index
    StrLen,            // Get string length
    StrConcat,         // Concatenate two strings
    StrIndex,          // Get character at index
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
    Call(u16),     // Argument count
    TailCall(u16), // NEW: Tail call (reuses stack frame)
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

    // NEW: Float Arithmetic Operations
    FAdd, // Float addition
    FSub, // Float subtraction
    FMul, // Float multiplication
    FDiv, // Float division

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

    // --- SANDBOX INSTRUCTIONS ---
    /// Initialize sandbox environment with resource limits
    InitSandbox,

    /// Isolate capability access for untrusted code
    IsolateCapabilities,

    /// Set error handler jump target for sandbox violations
    SetErrorHandler(i16),

    /// Log sandbox violation and transition to error state
    LogSandboxViolation,

    /// Cleanup sandbox resources and restore previous state
    CleanupSandbox,
}

impl OpCode {
    pub fn size_bytes(&self) -> usize {
        match self {
            OpCode::Nil => 1,
            OpCode::Bool(_) => 2,
            OpCode::Int(_) => 9,
            OpCode::Float(_) => 9, // f64 (8 bytes) + opcode tag (1 byte)
            OpCode::Symbol(_) => 5,
            OpCode::LoadString(_) => 5, // usize (4 bytes) + opcode tag (1 byte)
            OpCode::StrLen => 1,
            OpCode::StrConcat => 1,
            OpCode::StrIndex => 1,
            OpCode::Dup => 1,
            OpCode::Pop => 1,
            OpCode::Swap => 1,
            OpCode::GetLocal(_) => 3, // u16 (2 bytes) + opcode tag (1 byte)
            OpCode::SetLocal(_) => 3, // u16 (2 bytes) + opcode tag (1 byte)
            OpCode::Cons => 1,
            OpCode::Car => 1,
            OpCode::Cdr => 1,
            OpCode::Call(_) => 3,
            OpCode::TailCall(_) => 3,
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
            OpCode::FAdd => 1, // NEW: Float addition
            OpCode::FSub => 1, // NEW: Float subtraction
            OpCode::FMul => 1, // NEW: Float multiplication
            OpCode::FDiv => 1, // NEW: Float division
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
            // Sandbox instructions
            OpCode::InitSandbox => 1,
            OpCode::IsolateCapabilities => 1,
            OpCode::SetErrorHandler(_) => 3, // i16 (2 bytes) + opcode tag (1 byte)
            OpCode::LogSandboxViolation => 1,
            OpCode::CleanupSandbox => 1,
        }
    }
}

/// Represents a value in the VM
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Nil,
    Bool(bool),
    Int(i64),       // Primary deterministic number type.
    Float(f64),     // NEW: Float value type
    String(String), // NEW: String value type
    Symbol(usize),  // Index into a constant table.
    Pair(HeapPtr),  // HeapPtr is a u32 index into an ObjectArena.
    Closure(HeapPtr),
    ActorId(u32),
    Capability(crate::types::capability::Capability),
    GcPtr(crate::vm::gc::GcPtr), // GC-managed pointer
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(fl) => write!(f, "{}", fl),
            Value::String(s) => write!(f, "\"{}\"", s),
            Value::Symbol(idx) => write!(f, "Symbol({})", idx),
            Value::Pair(ptr) => write!(f, "Pair({})", ptr),
            Value::Closure(ptr) => write!(f, "Closure({})", ptr),
            Value::ActorId(id) => write!(f, "Actor({})", id),
            Value::Capability(cap) => write!(f, "Capability({:?})", cap),
            Value::GcPtr(ptr) => write!(f, "GcPtr({})", ptr.0),
        }
    }
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Nil => false,
            Value::Bool(b) => *b,
            Value::Int(i) => *i != 0,
            Value::Float(fl) => *fl != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Symbol(_) => true,
            Value::Pair(_) => true,
            Value::Closure(_) => true,
            Value::ActorId(_) => true,
            Value::Capability(_) => true,
            Value::GcPtr(_) => true,
        }
    }
}
