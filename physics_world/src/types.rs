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

        assert!(!nil.is_truthy());
        assert!(bool_val.is_truthy());
        assert!(int_val.is_truthy());
        assert!(symbol_val.is_truthy());
        assert!(pair_val.is_truthy());
        assert!(closure_val.is_truthy());
        assert!(actor_val.is_truthy());

        assert_eq!(Value::Int(0).is_truthy(), false);
    }
}
