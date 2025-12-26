use crate::types::{Capability, Value};
use crate::vm::state::VmError;
/// Capability-related opcode handlers for the Physics World VM
use crate::vm::state::{InstructionResult, VmState};

/// Helper function to push an error value onto the stack
fn push_error(vm: &mut VmState, message: &str) {
    vm.stack.push(Value::Error(message.to_string()));
}

/// Helper function to pop two integer arguments, checking for type mismatches
fn pop_int_args(vm: &mut VmState, args: u8) -> Result<(i64, i64), VmError> {
    if args != 2 {
        return Err(VmError::TypeMismatch);
    }
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    match (a, b) {
        (Value::Int(x), Value::Int(y)) => Ok((x, y)),
        _ => {
            push_error(vm, "expected integers");
            Err(VmError::TypeMismatch)
        }
    }
}

/// Helper function to pop two float arguments, checking for type mismatches
fn pop_float_args(vm: &mut VmState, args: u8) -> Result<(f64, f64), VmError> {
    if args != 2 {
        return Err(VmError::TypeMismatch);
    }
    let b = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    match (a, b) {
        (Value::Float(x), Value::Float(y)) => Ok((x, y)),
        _ => {
            push_error(vm, "expected floats");
            Err(VmError::TypeMismatch)
        }
    }
}

/// Helper function to pop a single integer argument
fn pop_int_arg(vm: &mut VmState) -> Result<i64, VmError> {
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    match a {
        Value::Int(x) => Ok(x),
        _ => {
            push_error(vm, "expected integer");
            Err(VmError::TypeMismatch)
        }
    }
}

/// Helper function to pop a single float argument
fn pop_float_arg(vm: &mut VmState) -> Result<f64, VmError> {
    let a = vm.stack.pop().ok_or(VmError::StackUnderflow)?;
    match a {
        Value::Float(x) => Ok(x),
        _ => {
            push_error(vm, "expected float");
            Err(VmError::TypeMismatch)
        }
    }
}

/// Handles the HasCap opcode - checks if the actor has a specific capability
pub fn handle_has_cap(vm: &mut VmState, cap_idx: usize) -> Result<InstructionResult, VmError> {
    // Get the capability from the constant pool
    let capability_value = match vm.constant_pool.get(cap_idx) {
        Some(val) => val,
        None => return Err(VmError::InvalidHeapPtr),
    };

    // Extract the capability from the Value enum
    let capability = match capability_value {
        Value::Capability(cap) => cap,
        _ => return Err(VmError::TypeMismatch),
    };

    // Check if the actor has this capability
    // This requires access to the scheduler, which we don't have directly in the VM
    // For now, we'll push false as a placeholder
    // In a real implementation, this would call the scheduler's capability system
    vm.stack.push(Value::Bool(false));

    Ok(InstructionResult::Continue)
}

/// Handles the RequestCap opcode - requests a capability from the scheduler
pub fn handle_request_cap(
    vm: &mut VmState,
    cap_idx: usize,
    justification_idx: usize,
) -> Result<InstructionResult, VmError> {
    // Get the capability from the constant pool
    let capability_value = match vm.constant_pool.get(cap_idx) {
        Some(val) => val,
        None => return Err(VmError::InvalidHeapPtr),
    };

    // Get the justification string from the constant pool
    let justification_value = match vm.constant_pool.get(justification_idx) {
        Some(val) => val,
        None => return Err(VmError::InvalidHeapPtr),
    };

    // Extract the capability and justification
    let capability = match capability_value {
        Value::Capability(cap) => cap,
        _ => return Err(VmError::TypeMismatch),
    };

    let justification = match justification_value {
        Value::Symbol(sym_idx) => {
            // In a real implementation, we'd look up the string from a symbol table
            // For now, we'll use a placeholder
            format!("Request for {:?}", capability)
        }
        _ => return Err(VmError::TypeMismatch),
    };

    // Return WaitingForCapability to indicate the actor is waiting for a decision
    Ok(InstructionResult::WaitingForCapability(capability.clone()))
}

/// Handles the GrantCap opcode - grants a capability to another actor
pub fn handle_grant_cap(
    vm: &mut VmState,
    target_actor_id: u32,
    cap_idx: usize,
) -> Result<(), VmError> {
    // Check if the current actor has MetaGrant capability
    // This would be checked against the scheduler's capability system
    // For now, we'll assume it doesn't have the capability

    // Get the capability from the constant pool
    let capability_value = match vm.constant_pool.get(cap_idx) {
        Some(val) => val,
        None => return Err(VmError::InvalidHeapPtr),
    };

    let capability = match capability_value {
        Value::Capability(cap) => cap,
        _ => return Err(VmError::TypeMismatch),
    };

    // In a real implementation, this would call the scheduler to grant the capability
    // For now, we'll just continue execution

    Ok(())
}

/// Handles the RevokeCap opcode - revokes a capability from an actor
pub fn handle_revoke_cap(
    vm: &mut VmState,
    target_actor_id: u32,
    cap_idx: usize,
) -> Result<(), VmError> {
    // Get the capability from the constant pool
    let capability_value = match vm.constant_pool.get(cap_idx) {
        Some(val) => val,
        None => return Err(VmError::InvalidHeapPtr),
    };

    let capability = match capability_value {
        Value::Capability(cap) => cap,
        _ => return Err(VmError::TypeMismatch),
    };

    // In a real implementation, this would call the scheduler to revoke the capability
    // For now, we'll just continue execution

    Ok(())
}

/// Get the capability required for a specific host function
/// Arithmetic operations (func_id 9-25) don't require special capabilities
fn get_required_capability_for_host_function(func_id: u16) -> Option<Capability> {
    match func_id {
        0 => Some(Capability::IoReadSensor),      // ReadSensor
        1 => Some(Capability::IoWriteActuator),   // WriteActuator
        2 => Some(Capability::SysClock),          // GetWallClockNs
        3 => Some(Capability::SysCreateActor),    // SpawnActor
        4 => Some(Capability::SysTerminateActor), // TerminateActor
        5 => Some(Capability::IoNetwork),         // NetworkSend
        6 => Some(Capability::IoNetwork),         // NetworkReceive
        7 => Some(Capability::IoPersist),         // PersistWrite
        8 => Some(Capability::IoPersist),         // PersistRead
        // Arithmetic operations (9-25) don't require special capabilities
        9..=25 => None,                            // IntAdd through FloatGt
        _ => None,
    }
}

/// Handles the HostCall opcode - executes a privileged host function call
pub fn handle_host_call(
    vm: &mut VmState,
    cap_idx: usize,
    func_id: u16,
    args: u8,
) -> Result<(), VmError> {
    // For arithmetic operations, we don't require a capability check
    // Arithmetic operations use func_id 9-25
    let requires_capability = matches!(func_id, 0..=8);

    if requires_capability {
        // Get the required capability from the constant pool
        let capability_value = match vm.constant_pool.get(cap_idx) {
            Some(val) => val,
            None => return Err(VmError::InvalidHeapPtr),
        };

        let required_capability = match capability_value {
            Value::Capability(cap) => cap,
            _ => return Err(VmError::TypeMismatch),
        };

        // Get the required capability for this host function
        let expected_capability = get_required_capability_for_host_function(func_id);

        // Verify that the provided capability matches the expected capability
        if let Some(expected_cap) = expected_capability {
            if expected_cap != *required_capability {
                return Err(VmError::CapabilityDenied);
            }
        }

        // Get the arguments from the stack for system operations
        if vm.stack.len() < args as usize {
            return Err(VmError::StackUnderflow);
        }

        let _call_args: Vec<Value> = vm.stack.drain((vm.stack.len() - args as usize)..).collect();
    }

    // Execute the host function based on function ID
    // Arithmetic operations (func_id 9-25) use helper functions that pop their own arguments
    let result = match func_id {
        // System operations (require capability)
        0 => Value::Int(42),         // ReadSensor - return mock sensor value
        1 => Value::Nil,             // WriteActuator - return nil
        2 => Value::Int(1234567890), // GetWallClockNs - return mock timestamp
        3 => Value::ActorId(1),      // SpawnActor - return mock actor ID
        4 => Value::Nil,             // TerminateActor - return nil
        5 => Value::Nil,             // NetworkSend - return nil
        6 => Value::Nil,             // NetworkReceive - return nil
        7 => Value::Nil,             // PersistWrite - return nil
        8 => Value::Nil,             // PersistRead - return nil
        
        // Integer arithmetic operations
        9 => {  // IntAdd
            match pop_int_args(vm, args) {
                Ok((x, y)) => {
                    match x.checked_add(y) {
                        Some(result) => Value::Int(result),
                        None => { push_error(vm, "integer overflow"); Value::Int(0) }
                    }
                }
                Err(_) => Value::Int(0), // Error already pushed
            }
        }
        10 => { // IntSub
            match pop_int_args(vm, args) {
                Ok((x, y)) => {
                    match x.checked_sub(y) {
                        Some(result) => Value::Int(result),
                        None => { push_error(vm, "integer overflow"); Value::Int(0) }
                    }
                }
                Err(_) => Value::Int(0),
            }
        }
        11 => { // IntMul
            match pop_int_args(vm, args) {
                Ok((x, y)) => {
                    match x.checked_mul(y) {
                        Some(result) => Value::Int(result),
                        None => { push_error(vm, "integer overflow"); Value::Int(0) }
                    }
                }
                Err(_) => Value::Int(0),
            }
        }
        12 => { // IntDiv
            match pop_int_args(vm, args) {
                Ok((x, y)) => {
                    if y == 0 {
                        push_error(vm, "division by zero");
                        Value::Int(0)
                    } else {
                        match x.checked_div(y) {
                            Some(result) => Value::Int(result),
                            None => { push_error(vm, "integer overflow"); Value::Int(0) }
                        }
                    }
                }
                Err(_) => Value::Int(0),
            }
        }
        13 => { // IntMod
            match pop_int_args(vm, args) {
                Ok((x, y)) => {
                    if y == 0 {
                        push_error(vm, "division by zero");
                        Value::Int(0)
                    } else {
                        match x.checked_rem(y) {
                            Some(result) => Value::Int(result),
                            None => { push_error(vm, "integer overflow"); Value::Int(0) }
                        }
                    }
                }
                Err(_) => Value::Int(0),
            }
        }
        
        // Float arithmetic operations
        14 => { // FloatAdd
            match pop_float_args(vm, args) {
                Ok((x, y)) => Value::Float(x + y),
                Err(_) => Value::Float(0.0),
            }
        }
        15 => { // FloatSub
            match pop_float_args(vm, args) {
                Ok((x, y)) => Value::Float(x - y),
                Err(_) => Value::Float(0.0),
            }
        }
        16 => { // FloatMul
            match pop_float_args(vm, args) {
                Ok((x, y)) => Value::Float(x * y),
                Err(_) => Value::Float(0.0),
            }
        }
        17 => { // FloatDiv
            match pop_float_args(vm, args) {
                Ok((x, y)) => Value::Float(x / y), // IEEE 754: returns Inf for div by zero
                Err(_) => Value::Float(0.0),
            }
        }
        
        // Type conversions
        18 => { // IntToFloat
            match pop_int_arg(vm) {
                Ok(x) => Value::Float(x as f64),
                Err(_) => Value::Float(0.0),
            }
        }
        19 => { // FloatToInt
            match pop_float_arg(vm) {
                Ok(x) => {
                    // Check for potential precision loss
                    if x.fract() != 0.0 {
                        push_error(vm, "potential precision loss in float to int conversion");
                    }
                    Value::Int(x as i64)
                }
                Err(_) => Value::Int(0),
            }
        }
        
        // Integer comparison operations
        20 => { // IntEq
            match pop_int_args(vm, args) {
                Ok((x, y)) => Value::Int(if x == y { 1 } else { 0 }),
                Err(_) => Value::Int(0),
            }
        }
        21 => { // IntLt
            match pop_int_args(vm, args) {
                Ok((x, y)) => Value::Int(if x < y { 1 } else { 0 }),
                Err(_) => Value::Int(0),
            }
        }
        22 => { // IntGt
            match pop_int_args(vm, args) {
                Ok((x, y)) => Value::Int(if x > y { 1 } else { 0 }),
                Err(_) => Value::Int(0),
            }
        }
        
        // Float comparison operations
        23 => { // FloatEq
            match pop_float_args(vm, args) {
                Ok((x, y)) => Value::Int(if x == y { 1 } else { 0 }),
                Err(_) => Value::Int(0),
            }
        }
        24 => { // FloatLt
            match pop_float_args(vm, args) {
                Ok((x, y)) => Value::Int(if x < y { 1 } else { 0 }),
                Err(_) => Value::Int(0),
            }
        }
        25 => { // FloatGt
            match pop_float_args(vm, args) {
                Ok((x, y)) => Value::Int(if x > y { 1 } else { 0 }),
                Err(_) => Value::Int(0),
            }
        }
        
        _ => return Err(VmError::UnknownOpCode),
    };

    // Push the result onto the stack
    vm.stack.push(result);

    Ok(())
}
