/// Arithmetic opcode handlers - Add, Sub, Mul, Div, Mod
use crate::types::Value;
use crate::vm::state::VmError;
use crate::vm::state::VmState;

/// Handles Add opcode
pub fn handle_add(vm: &mut VmState) -> Result<(), VmError> {
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match (a, b) {
        (Value::Int(x), Value::Int(y)) => {
            let result = x.checked_add(y).ok_or(VmError::ArithmeticOverflow)?;
            vm.stack.push(Value::Int(result));
        }
        _ => return Err(VmError::TypeMismatch),
    }
    Ok(())
}

/// Handles Sub opcode
pub fn handle_sub(vm: &mut VmState) -> Result<(), VmError> {
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match (a, b) {
        (Value::Int(x), Value::Int(y)) => {
            let result = x.checked_sub(y).ok_or(VmError::ArithmeticOverflow)?;
            vm.stack.push(Value::Int(result));
        }
        _ => return Err(VmError::TypeMismatch),
    }
    Ok(())
}

/// Handles Mul opcode
pub fn handle_mul(vm: &mut VmState) -> Result<(), VmError> {
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match (a, b) {
        (Value::Int(x), Value::Int(y)) => {
            let result = x.checked_mul(y).ok_or(VmError::ArithmeticOverflow)?;
            vm.stack.push(Value::Int(result));
        }
        _ => return Err(VmError::TypeMismatch),
    }
    Ok(())
}

/// Handles Div opcode
pub fn handle_div(vm: &mut VmState) -> Result<(), VmError> {
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match (a, b) {
        (Value::Int(_), Value::Int(0)) => return Err(VmError::DivisionByZero),
        (Value::Int(x), Value::Int(y)) => {
            let result = x.checked_div(y).ok_or(VmError::ArithmeticOverflow)?;
            vm.stack.push(Value::Int(result));
        }
        _ => return Err(VmError::TypeMismatch),
    }
    Ok(())
}

/// Handles Mod opcode
pub fn handle_mod(vm: &mut VmState) -> Result<(), VmError> {
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match (a, b) {
        (Value::Int(x), Value::Int(y)) => {
            if y == 0 {
                return Err(VmError::DivisionByZero);
            }
            let result = x.checked_rem(y).ok_or(VmError::ArithmeticOverflow)?;
            vm.stack.push(Value::Int(result));
        }
        _ => return Err(VmError::TypeMismatch),
    }
    Ok(())
}
