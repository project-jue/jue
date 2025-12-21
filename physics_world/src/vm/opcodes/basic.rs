/// Basic opcode handlers - Nil, Bool, Int, Symbol
use crate::types::Value;
use crate::vm::state::VmError;
use crate::vm::state::VmState;

/// Handles Nil opcode
pub fn handle_nil(vm: &mut VmState) -> Result<(), VmError> {
    vm.stack.push(Value::Nil);
    Ok(())
}

/// Handles Bool opcode
pub fn handle_bool(vm: &mut VmState, value: bool) -> Result<(), VmError> {
    vm.stack.push(Value::Bool(value));
    Ok(())
}

/// Handles Int opcode
pub fn handle_int(vm: &mut VmState, value: i64) -> Result<(), VmError> {
    vm.stack.push(Value::Int(value));
    Ok(())
}

/// Handles Symbol opcode
pub fn handle_symbol(vm: &mut VmState, sym_idx: usize) -> Result<(), VmError> {
    let symbol = match vm.constant_pool.get(sym_idx) {
        Some(val) => val.clone(),
        None => return Err(VmError::InvalidHeapPtr),
    };
    vm.stack.push(symbol);
    Ok(())
}
