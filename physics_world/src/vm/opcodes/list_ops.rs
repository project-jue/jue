/// List operation handlers - Cons, Car, Cdr
use crate::types::Value;
use crate::vm::state::{VmError, VmState};

/// Create a new pair (cons cell) from two values
pub fn handle_cons(vm: &mut VmState) -> Result<(), VmError> {
    let cdr = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let car = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    // Allocate space for pair: [car_value][cdr_value]
    let car_size = std::mem::size_of::<Value>();
    let cdr_size = std::mem::size_of::<Value>();
    let total_size = car_size + cdr_size;

    // Allocate memory for the pair (tag 3 for pairs)
    let pair_ptr = vm
        .memory
        .allocate(total_size as u32, 3)
        .map_err(|_| VmError::MemoryLimitExceeded)?;

    // Store car and cdr values
    unsafe {
        let data = vm.memory.get_data_mut(pair_ptr);

        // Convert values to u32 representation
        let car_bytes = value_to_u32(&car);
        let cdr_bytes = value_to_u32(&cdr);

        // Write car value (first 4 bytes)
        data[0..4].copy_from_slice(&car_bytes.to_le_bytes());

        // Write cdr value (next 4 bytes)
        data[4..8].copy_from_slice(&cdr_bytes.to_le_bytes());
    }

    // Push the pair pointer
    vm.stack.push(Value::Pair(pair_ptr));
    Ok(())
}

/// Get the car (first element) of a pair
pub fn handle_car(vm: &mut VmState) -> Result<(), VmError> {
    let pair = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match pair {
        Value::Pair(ptr) => {
            // Read car value from the pair
            unsafe {
                let data = vm.memory.get_data(ptr);
                let car_bytes = &data[0..4];
                let car_value = u32::from_le_bytes(car_bytes.try_into().unwrap());
                let car = u32_to_value(car_value);
                vm.stack.push(car);
            }
            Ok(())
        }
        _ => Err(VmError::TypeMismatch),
    }
}

/// Get the cdr (rest of the list) of a pair
pub fn handle_cdr(vm: &mut VmState) -> Result<(), VmError> {
    let pair = vm.stack.pop().ok_or(VmError::StackUnderflow)?;

    match pair {
        Value::Pair(ptr) => {
            // Read cdr value from the pair
            unsafe {
                let data = vm.memory.get_data(ptr);
                let cdr_bytes = &data[4..8];
                let cdr_value = u32::from_le_bytes(cdr_bytes.try_into().unwrap());
                let cdr = u32_to_value(cdr_value);
                vm.stack.push(cdr);
            }
            Ok(())
        }
        _ => Err(VmError::TypeMismatch),
    }
}

/// Convert a Value to a u32 representation (simplified for basic types)
fn value_to_u32(value: &Value) -> u32 {
    match value {
        Value::Int(n) => (*n as u32),
        Value::Float(f) => (*f as u32), // Convert float to u32 for storage
        Value::Bool(b) => {
            if *b {
                1
            } else {
                0
            }
        }
        Value::Nil => 0,
        Value::String(_) => 0, // Strings handled through constant pool, not stored directly
        Value::Symbol(s) => *s as u32,
        Value::ActorId(id) => *id,
        Value::Pair(ptr) => ptr.get(),
        Value::Closure(ptr) => ptr.get(),
        Value::Capability(_) => 0, // Placeholder
        Value::GcPtr(ptr) => ptr.0 as u32,
    }
}

/// Convert a u32 back to a Value (simplified)
fn u32_to_value(value: u32) -> Value {
    // This is a simplified conversion - in a real implementation,
    // you'd need type information to properly reconstruct values
    if value == 0 {
        Value::Nil
    } else {
        Value::Int(value as i64)
    }
}
