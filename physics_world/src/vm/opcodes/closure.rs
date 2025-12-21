/// Optimized closure creation based on escape analysis
use crate::types::{HeapPtr, OpCode, Value};
use crate::vm::state::{EscapeStatus, VmError, VmState};
use bincode;
use std::collections::HashMap;

/// Handles the MakeClosure opcode with escape analysis optimization
///
/// # Arguments
/// * `vm` - The VM state
/// * `function_ptr` - Index in constant pool where closure body is stored
///
/// # Returns
/// Result containing the created closure or error
pub fn handle_make_closure(vm: &mut VmState, function_ptr: u16) -> Result<Value, VmError> {
    // Get function info from escape analysis
    let function_info = vm.get_function_info(function_ptr)?;
    let free_vars = &function_info.free_variables;
    let escape_info = &function_info.escape_info;

    // Create closure with only escaping variables
    let mut closure = Closure {
        function_ptr,
        environment: Environment::new(),
    };

    for (var_index, escape_status) in escape_info {
        if *escape_status == EscapeStatus::Escaping {
            let value = if let Some(var) = free_vars.get(*var_index) {
                vm.get_local_var(*var)?
            } else {
                Value::Nil // Shouldn't happen with proper analysis
            };
            closure.environment.set(*var_index, value);
        }
    }

    // Create closure in memory and push to stack
    let closure_ptr = create_closure_in_memory(vm, closure)?;
    vm.stack.push(Value::Closure(closure_ptr));
    Ok(Value::Closure(closure_ptr))
}

/// Closure structure with optimized environment
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Closure {
    pub function_ptr: u16,
    pub environment: Environment,
}

/// Environment for closure variables
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Environment {
    variables: HashMap<usize, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, index: usize, value: Value) {
        self.variables.insert(index, value);
    }

    pub fn get(&self, index: usize) -> Option<&Value> {
        self.variables.get(&index)
    }
}

/// Helper function to create closure bodies in memory
pub fn create_closure_body(vm: &mut VmState, bytecode: Vec<OpCode>) -> Result<HeapPtr, VmError> {
    // 1. Serialize the bytecode
    let serialized = bincode::serialize(&bytecode).map_err(|_| VmError::TypeMismatch)?;

    // 2. Calculate size (4 bytes length + serialized data)
    let size = 4 + serialized.len() as u32;

    // 3. Allocate memory for closure body
    let body_ptr = vm
        .memory
        .allocate(size, 2) // Tag 2 for closure bodies
        .map_err(|_| VmError::MemoryLimitExceeded)?;

    // 4. Store size and bytecode
    let data = unsafe { vm.memory.get_data_mut(body_ptr) };
    let size_bytes = (serialized.len() as u32).to_le_bytes();
    data[0..4].copy_from_slice(&size_bytes);
    data[4..4 + serialized.len()].copy_from_slice(&serialized);

    Ok(body_ptr)
}

/// Helper function to create closure in memory
fn create_closure_in_memory(vm: &mut VmState, closure: Closure) -> Result<HeapPtr, VmError> {
    // Serialize the closure data
    let serialized = bincode::serialize(&closure).map_err(|_| VmError::TypeMismatch)?;

    // Allocate memory for closure
    let size = serialized.len() as u32;
    let closure_ptr = vm
        .memory
        .allocate(size, 2) // Tag 2 for closures
        .map_err(|_| VmError::MemoryLimitExceeded)?;

    // Store closure data
    let data = unsafe { vm.memory.get_data_mut(closure_ptr) };
    data[0..serialized.len()].copy_from_slice(&serialized);

    Ok(closure_ptr)
}
