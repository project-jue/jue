/// String opcode handlers - LoadString, StrLen, StrConcat, StrIndex
use crate::types::Value;
use crate::vm::state::VmError;
use crate::vm::state::VmState;

/// Handles LoadString opcode - loads a string from the constant pool
pub fn handle_load_string(vm: &mut VmState, string_idx: usize) -> Result<(), VmError> {
    let string_value = match vm.constant_pool.get(string_idx) {
        Some(Value::String(s)) => s.clone(),
        Some(_) => return Err(VmError::TypeMismatch),
        None => return Err(VmError::InvalidHeapPtr),
    };
    vm.stack.push(Value::String(string_value));
    Ok(())
}

/// Handles StrLen opcode - gets the length of a string
pub fn handle_str_len(vm: &mut VmState) -> Result<(), VmError> {
    let string_value = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    match string_value {
        Value::String(s) => {
            let len = s.len() as i64;
            vm.stack.push(Value::Int(len));
            Ok(())
        }
        _ => Err(VmError::TypeMismatch),
    }
}

/// Handles StrConcat opcode - concatenates two strings
pub fn handle_str_concat(vm: &mut VmState) -> Result<(), VmError> {
    let right = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let left = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match (left, right) {
        (Value::String(mut left_str), Value::String(right_str)) => {
            left_str.push_str(&right_str);
            vm.stack.push(Value::String(left_str));
            Ok(())
        }
        _ => Err(VmError::TypeMismatch),
    }
}

/// Handles StrIndex opcode - gets character at index
pub fn handle_str_index(vm: &mut VmState) -> Result<(), VmError> {
    let index_value = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let string_value = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match (string_value, index_value) {
        (Value::String(s), Value::Int(i)) => {
            if i < 0 || i as usize >= s.len() {
                vm.stack.push(Value::Nil);
            } else {
                let char_at_index = s.chars().nth(i as usize).unwrap();
                vm.stack.push(Value::String(char_at_index.to_string()));
            }
            Ok(())
        }
        _ => Err(VmError::TypeMismatch),
    }
}
