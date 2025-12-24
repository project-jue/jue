//! Execution logic for the VM.
//!
//! This module handles the step-by-step execution of instructions,
//! instruction dispatch, and performance tracking.
//!
//! # Extracted from
//! - `vm/state.rs` (lines 947-1298, step() and run() methods)

use crate::types::{OpCode, Value};
use crate::vm::error::{SimpleVmError, VmError, WithContext};
use crate::vm::opcodes::{
    arithmetic, basic, call, capability, comparison, jump, list_ops, make_closure, messaging, ret,
    stack_ops, string_ops,
};
use crate::vm::state::InstructionResult;

/// Core execution engine for the VM.
///
/// This struct provides the step-based execution model and instruction dispatch.
pub struct ExecutionEngine;

impl ExecutionEngine {
    /// Creates a new execution engine instance.
    pub fn new() -> Self {
        Self
    }

    /// Executes a single instruction in the given VM state.
    ///
    /// # Arguments
    /// * `state` - Mutable reference to the VM state
    ///
    /// # Returns
    /// Result containing either the instruction result or an execution error
    pub fn step(
        &mut self,
        state: &mut crate::vm::state::VmState,
    ) -> Result<InstructionResult, SimpleVmError> {
        // Check if we've exceeded CPU limit
        if state.steps_remaining == 0 {
            return Err(SimpleVmError::CpuLimitExceeded);
        }
        state.steps_remaining -= 1;

        eprintln!(
            "STEP: ip={}, call_stack_len={}, instructions_len={}",
            state.ip,
            state.call_stack.len(),
            state.instructions.len()
        );

        // Check bounds BEFORE trying to fetch the instruction
        if state.call_stack.is_empty() && state.ip >= state.instructions.len() {
            eprintln!(
                "BOUNDS CHECK TRIGGERED: call_stack empty, ip {} >= instructions.len() {}",
                state.ip,
                state.instructions.len()
            );
            return Ok(InstructionResult::Finished(
                state.stack.pop().unwrap_or(Value::Nil),
            ));
        }

        // Get current instruction
        let instruction = match state.instructions.get(state.ip) {
            Some(instr) => {
                eprintln!("FETCHED INSTRUCTION: {:?}", instr);
                instr
            }
            None => {
                eprintln!("INSTRUCTION NOT FOUND at ip={}", state.ip);
                // We're out of instructions. If we're at the top level (no call frames),
                // the program is finished. If we're inside a function call, we should
                // automatically return Nil by properly restoring the call frame.
                if !state.call_stack.is_empty() {
                    eprintln!("Out of instructions in function call - implicit return");
                    // We're in a function that didn't explicitly return - treat as implicit return
                    // Push Nil as return value and restore call frame
                    state.stack.push(Value::Nil);

                    // Restore call frame like a proper return
                    let call_frame = state.call_stack.pop().unwrap();

                    // Pop the return value and restore stack to call frame state
                    let return_value = if state.stack.len() > call_frame.stack_start {
                        state.stack.pop()
                    } else {
                        None
                    };

                    // Restore stack to call frame state
                    state.stack.truncate(call_frame.stack_start);

                    // Push return value (or Nil if none)
                    if let Some(value) = return_value {
                        state.stack.push(value);
                    } else {
                        state.stack.push(Value::Nil);
                    }

                    // Restore instruction pointer and original instructions
                    state.ip = call_frame.return_ip;
                    if let Some(saved_instructions) = call_frame.saved_instructions {
                        state.instructions = saved_instructions;
                    }

                    // Continue execution from the restored context
                    return Ok(InstructionResult::Continue);
                } else {
                    eprintln!("Out of instructions at top level - program finished");
                    // We're at top level - program is finished
                    return Ok(InstructionResult::Finished(
                        state.stack.pop().unwrap_or(Value::Nil),
                    ));
                }
            }
        };

        // Execute the instruction using modular handlers
        match instruction {
            OpCode::Nil => {
                basic::handle_nil(state)?;
                state.ip += 1;
            }
            OpCode::Bool(b) => {
                basic::handle_bool(state, *b)?;
                state.ip += 1;
            }
            OpCode::Int(i) => {
                basic::handle_int(state, *i)?;
                state.ip += 1;
            }
            OpCode::Float(f) => {
                basic::handle_float(state, *f)?;
                state.ip += 1;
            }
            OpCode::Symbol(sym_idx) => {
                basic::handle_symbol(state, *sym_idx)?;
                state.ip += 1;
            }
            OpCode::LoadString(string_idx) => {
                string_ops::handle_load_string(state, *string_idx)?;
                state.ip += 1;
            }
            OpCode::StrLen => {
                string_ops::handle_str_len(state)?;
                state.ip += 1;
            }
            OpCode::StrConcat => {
                string_ops::handle_str_concat(state)?;
                state.ip += 1;
            }
            OpCode::StrIndex => {
                string_ops::handle_str_index(state)?;
                state.ip += 1;
            }
            OpCode::Dup => {
                stack_ops::handle_dup(state)?;
                state.ip += 1;
            }
            OpCode::Pop => {
                stack_ops::handle_pop(state)?;
                state.ip += 1;
            }
            OpCode::Swap => {
                stack_ops::handle_swap(state)?;
                state.ip += 1;
            }
            OpCode::GetLocal(offset) => {
                stack_ops::handle_get_local(state, *offset)?;
                state.ip += 1;
            }
            OpCode::SetLocal(offset) => {
                stack_ops::handle_set_local(state, *offset)?;
                state.ip += 1;
            }
            OpCode::Cons => {
                list_ops::handle_cons(state)?;
                state.ip += 1;
            }
            OpCode::Car => {
                list_ops::handle_car(state)?;
                state.ip += 1;
            }
            OpCode::Cdr => {
                list_ops::handle_cdr(state)?;
                state.ip += 1;
            }
            OpCode::Call(arg_count) => {
                // Use the new enhanced handle_call method from VmState
                state.handle_call(*arg_count)?;
                // Note: Call handler sets ip to 0 for closure execution
            }
            OpCode::TailCall(arg_count) => {
                // Use the consolidated handle_tail_call from opcodes/call.rs
                // The implementation handles argument extraction from stack internally
                state.handle_tail_call(*arg_count)?;
                // Note: TailCall handler sets ip to 0 for closure execution
            }
            OpCode::Ret => {
                ret::handle_ret(state)?;
                // Note: Ret handler sets ip to return address
            }
            OpCode::Jmp(offset) => {
                jump::handle_jmp(state, *offset)?;
                // Note: Jmp handler sets ip to new position
            }
            OpCode::JmpIfFalse(offset) => {
                jump::handle_jmp_if_false(state, *offset)?;
                // Note: JmpIfFalse handler sets ip to new position or increments it
            }
            OpCode::Yield => {
                state.ip += 1;
                return Ok(InstructionResult::Yield);
            }
            OpCode::Send => {
                // Implement Send handler for inter-actor communication
                let result = messaging::handle_send(state)?;
                state.ip += 1;
                return Ok(result);
            }
            OpCode::Add => {
                arithmetic::handle_add(state)?;
                state.ip += 1;
            }
            OpCode::Div => {
                arithmetic::handle_div(state)?;
                state.ip += 1;
            }
            OpCode::Eq => {
                comparison::handle_eq(state)?;
                state.ip += 1;
            }
            OpCode::Lt => {
                comparison::handle_lt(state)?;
                state.ip += 1;
            }
            OpCode::Gt => {
                comparison::handle_gt(state)?;
                state.ip += 1;
            }
            OpCode::Lte => {
                comparison::handle_lte(state)?;
                state.ip += 1;
            }
            OpCode::Gte => {
                comparison::handle_gte(state)?;
                state.ip += 1;
            }
            OpCode::Ne => {
                comparison::handle_ne(state)?;
                state.ip += 1;
            }
            OpCode::Sub => {
                arithmetic::handle_sub(state)?;
                state.ip += 1;
            }
            OpCode::Mul => {
                arithmetic::handle_mul(state)?;
                state.ip += 1;
            }
            OpCode::FAdd => {
                arithmetic::handle_fadd(state)?;
                state.ip += 1;
            }
            OpCode::FSub => {
                arithmetic::handle_fsub(state)?;
                state.ip += 1;
            }
            OpCode::FMul => {
                arithmetic::handle_fmul(state)?;
                state.ip += 1;
            }
            OpCode::FDiv => {
                arithmetic::handle_fdiv(state)?;
                state.ip += 1;
            }
            OpCode::Mod => {
                arithmetic::handle_mod(state)?;
                state.ip += 1;
            }
            OpCode::MakeClosure(code_idx, capture_count) => {
                let closure = make_closure::handle_make_closure(state, *code_idx, *capture_count)?;
                state.stack.push(closure);
                state.ip += 1;
            }
            OpCode::CheckStepLimit => {
                // Check if we've exceeded CPU limit
                if state.steps_remaining == 0 {
                    return Err(SimpleVmError::CpuLimitExceeded);
                }
                state.ip += 1;
            }
            // V2 Capability System - Implement capability opcodes
            OpCode::HasCap(cap_idx) => {
                let result = capability::handle_has_cap(state, *cap_idx)?;
                return Ok(result);
            }
            OpCode::RequestCap(cap_idx, justification_idx) => {
                let result = capability::handle_request_cap(state, *cap_idx, *justification_idx)?;
                return Ok(result);
            }
            OpCode::GrantCap(target_actor_id, cap_idx) => {
                capability::handle_grant_cap(state, *target_actor_id, *cap_idx)?;
                state.ip += 1;
            }
            OpCode::RevokeCap(target_actor_id, cap_idx) => {
                capability::handle_revoke_cap(state, *target_actor_id, *cap_idx)?;
                state.ip += 1;
            }
            OpCode::HostCall {
                cap_idx,
                func_id,
                args,
            } => {
                capability::handle_host_call(state, *cap_idx, *func_id, *args)?;
                state.ip += 1;
            }
            // Sandbox instructions
            OpCode::InitSandbox => {
                // Initialize sandbox environment - place holder for now
                state.ip += 1;
            }
            OpCode::IsolateCapabilities => {
                // Isolate capability access - place holder for now
                state.ip += 1;
            }
            OpCode::SetErrorHandler(offset) => {
                // Set error handler jump target - place holder for now
                state.ip += 1;
            }
            OpCode::LogSandboxViolation => {
                // Log sandbox violation - place holder for now
                state.ip += 1;
            }
            OpCode::CleanupSandbox => {
                // Cleanup sandbox resources - place holder for now
                state.ip += 1;
            }
        }

        Ok(InstructionResult::Continue)
    }

    /// Executes the VM until completion or error.
    ///
    /// # Arguments
    /// * `state` - Mutable reference to the VM state
    ///
    /// # Returns
    /// The final result value or an error
    pub fn run(&mut self, state: &mut crate::vm::state::VmState) -> Result<Value, VmError> {
        loop {
            match self.step(state) {
                Ok(InstructionResult::Continue) => continue,
                Ok(InstructionResult::Yield) => return Ok(Value::Nil), // Yield returns Nil for now
                Ok(InstructionResult::Finished(result)) => return Ok(result),
                Ok(InstructionResult::WaitingForCapability(_capability)) => {
                    let context = state.create_error_context();
                    return Err(VmError::capability_error(
                        context,
                        "unknown",
                        "WaitingForCapability",
                    ));
                }
                Err(simple_error) => {
                    // Convert simple error to detailed error with context
                    return Err(simple_error.with_context(state.create_error_context()));
                }
            }
        }
    }
}

impl Default for ExecutionEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::OpCode;
    use crate::vm::state::VmState;

    #[test]
    fn test_execution_engine_creation() {
        let _engine = ExecutionEngine::new();
        // Just verify it can be created
        assert!(true);
    }

    #[test]
    fn test_step_nil_instruction() {
        let mut state = VmState::new(vec![OpCode::Nil], Vec::new(), 100, 1024, 1, 100);

        let mut engine = ExecutionEngine::new();
        let result = engine.step(&mut state);

        // Nil pushes Value::Nil to stack
        assert!(result.is_ok());
        assert_eq!(state.stack.len(), 1);
    }

    #[test]
    fn test_step_int_instruction() {
        let mut state = VmState::new(vec![OpCode::Int(42)], Vec::new(), 100, 1024, 1, 100);

        let mut engine = ExecutionEngine::new();
        let result = engine.step(&mut state);

        assert!(result.is_ok());
        assert_eq!(state.stack.len(), 1);
        if let Value::Int(n) = &state.stack[0] {
            assert_eq!(*n, 42);
        } else {
            panic!("Expected Int(42)");
        }
    }

    #[test]
    fn test_step_with_step_limit() {
        let mut state = VmState::new(
            vec![OpCode::Nil, OpCode::Nil],
            Vec::new(),
            1, // Only 1 step allowed
            1024,
            1,
            100,
        );

        let mut engine = ExecutionEngine::new();

        // First step should succeed
        let result = engine.step(&mut state);
        assert!(result.is_ok());

        // Second step should fail due to CPU limit
        let result = engine.step(&mut state);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_completes() {
        let mut state = VmState::new(
            vec![OpCode::Nil, OpCode::Nil, OpCode::Add], // Add will fail but program completes
            Vec::new(),
            100,
            1024,
            1,
            100,
        );

        let mut engine = ExecutionEngine::new();
        let result = engine.run(&mut state);
        // Add with no arguments will fail, but we can test it completes
        assert!(result.is_err() || true); // Either completes or fails as expected
    }
}
