/// Comparison opcode handlers - Eq, Lt, Gt, Lte, Gte, Ne
use crate::types::Value;
use crate::vm::state::VmError;
use crate::vm::state::VmState;

/// Handles Eq opcode
pub fn handle_eq(vm: &mut VmState) -> Result<(), VmError> {
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    vm.stack.push(Value::Bool(a == b));
    Ok(())
}

/// Handles Lt opcode
pub fn handle_lt(vm: &mut VmState) -> Result<(), VmError> {
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match (a, b) {
        (Value::Int(x), Value::Int(y)) => {
            vm.stack.push(Value::Bool(x < y));
        }
        _ => return Err(VmError::TypeMismatch),
    }
    Ok(())
}

/// Handles Gt opcode
pub fn handle_gt(vm: &mut VmState) -> Result<(), VmError> {
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match (a, b) {
        (Value::Int(x), Value::Int(y)) => {
            vm.stack.push(Value::Bool(x > y));
        }
        _ => return Err(VmError::TypeMismatch),
    }
    Ok(())
}

/// Handles Lte opcode (<=)
pub fn handle_lte(vm: &mut VmState) -> Result<(), VmError> {
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match (a, b) {
        (Value::Int(x), Value::Int(y)) => {
            vm.stack.push(Value::Bool(x <= y));
        }
        _ => return Err(VmError::TypeMismatch),
    }
    Ok(())
}

/// Handles Gte opcode (>=)
pub fn handle_gte(vm: &mut VmState) -> Result<(), VmError> {
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match (a, b) {
        (Value::Int(x), Value::Int(y)) => {
            vm.stack.push(Value::Bool(x >= y));
        }
        _ => return Err(VmError::TypeMismatch),
    }
    Ok(())
}

/// Handles Ne opcode (!=)
pub fn handle_ne(vm: &mut VmState) -> Result<(), VmError> {
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    vm.stack.push(Value::Bool(a != b));
    Ok(())
}
