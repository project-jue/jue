//! Call frame management for the VM.
//!
//! This module handles the call stack, including frame creation,
//! push/pop operations, and call frame state tracking.
//!
//! # Extracted from
//! - `vm/state.rs` (lines 398-411, call-related methods)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::Value;

/// Represents a call frame for function calls.
///
/// Stores the return address and stack state for proper function call/return semantics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallFrame {
    pub return_ip: usize,
    pub stack_start: usize,
    pub saved_instructions: Option<Vec<crate::types::OpCode>>, // Store original instructions for nested calls
    pub recursion_depth: u32, // Track recursion depth for this call frame
    pub locals: Vec<Value>,   // Lexical environment storage
    pub closed_over: HashMap<usize, Value>, // Closed-over variables
    pub is_tail_call: bool,   // TCO tracking flag
    pub frame_id: u64,        // For debugging/verification
}

impl CallFrame {
    /// Creates a new call frame with the given return address and stack start position.
    ///
    /// # Arguments
    /// * `return_ip` - The instruction pointer to return to after the function completes
    /// * `stack_start` - The stack position before this function was called
    /// * `frame_id` - Unique identifier for debugging
    pub fn new(return_ip: usize, stack_start: usize, frame_id: u64) -> Self {
        Self {
            return_ip,
            stack_start,
            saved_instructions: None,
            recursion_depth: 0,
            locals: Vec::new(),
            closed_over: HashMap::new(),
            is_tail_call: false,
            frame_id,
        }
    }

    /// Pushes a local variable onto this frame's local storage.
    pub fn push_local(&mut self, value: Value) {
        self.locals.push(value);
    }

    /// Pops a local variable from this frame's local storage.
    pub fn pop_local(&mut self) -> Option<Value> {
        self.locals.pop()
    }

    /// Gets a local variable by index.
    pub fn get_local(&self, index: usize) -> Option<&Value> {
        self.locals.get(index)
    }

    /// Sets a local variable by index.
    pub fn set_local(&mut self, index: usize, value: Value) {
        if index < self.locals.len() {
            self.locals[index] = value;
        }
    }
}

/// Manages the call stack for the VM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStack {
    frames: Vec<CallFrame>,
    max_depth: usize,
}

impl CallStack {
    /// Creates a new call stack with the specified maximum depth.
    pub fn new(max_depth: usize) -> Self {
        Self {
            frames: Vec::with_capacity(max_depth),
            max_depth,
        }
    }

    /// Pushes a new call frame onto the stack.
    ///
    /// # Returns
    /// `Ok(())` if successful, or `Err(VmError::RecursionLimitExceeded)` if max depth reached.
    pub fn push(&mut self, frame: CallFrame) -> Result<(), crate::vm::error::VmError> {
        if self.frames.len() >= self.max_depth {
            // Create a minimal context for the error
            let context = crate::vm::error::ErrorContext {
                instruction_pointer: 0,
                current_instruction: None,
                stack_state: Vec::new(),
                call_stack_depth: self.frames.len(),
                steps_remaining: 0,
                actor_id: 0,
                memory_usage: 0,
                stack_trace: Vec::new(),
                execution_history: Vec::new(),
                timestamp: 0,
            };
            return Err(crate::vm::error::VmError::recursion_limit_exceeded(
                context,
                self.max_depth as u32,
                self.frames.len() as u32,
            ));
        }
        self.frames.push(frame);
        Ok(())
    }

    /// Pops the top call frame from the stack.
    pub fn pop(&mut self) -> Option<CallFrame> {
        self.frames.pop()
    }

    /// Gets the current recursion depth.
    pub fn depth(&self) -> usize {
        self.frames.len()
    }

    /// Gets a reference to the top call frame.
    pub fn last(&self) -> Option<&CallFrame> {
        self.frames.last()
    }

    /// Gets a mutable reference to the top call frame.
    pub fn last_mut(&mut self) -> Option<&mut CallFrame> {
        self.frames.last_mut()
    }

    /// Returns true if the call stack is empty.
    pub fn is_empty(&self) -> bool {
        self.frames.is_empty()
    }

    /// Returns the current call stack as a slice.
    pub fn as_slice(&self) -> &[CallFrame] {
        &self.frames
    }

    /// Returns the current call stack as a mutable slice.
    pub fn as_mut_slice(&mut self) -> &mut [CallFrame] {
        &mut self.frames
    }

    /// Truncates the stack to the specified length.
    pub fn truncate(&mut self, len: usize) {
        self.frames.truncate(len);
    }

    /// Clears all call frames.
    pub fn clear(&mut self) {
        self.frames.clear();
    }
}

impl std::ops::Deref for CallStack {
    type Target = Vec<CallFrame>;

    fn deref(&self) -> &Self::Target {
        &self.frames
    }
}

impl std::ops::DerefMut for CallStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.frames
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_frame_creation() {
        let frame = CallFrame::new(10, 5, 1);
        assert_eq!(frame.return_ip, 10);
        assert_eq!(frame.stack_start, 5);
        assert_eq!(frame.frame_id, 1);
        assert!(frame.locals.is_empty());
    }

    #[test]
    fn test_call_frame_locals() {
        let mut frame = CallFrame::new(0, 0, 1);
        frame.push_local(Value::Int(42));
        frame.push_local(Value::Int(100));

        assert_eq!(frame.locals.len(), 2);
        assert_eq!(frame.get_local(0), Some(&Value::Int(42)));
        assert_eq!(frame.get_local(1), Some(&Value::Int(100)));

        frame.set_local(0, Value::Int(999));
        assert_eq!(frame.get_local(0), Some(&Value::Int(999)));
    }

    #[test]
    fn test_call_stack_basic_operations() {
        let mut stack = CallStack::new(10);

        assert!(stack.is_empty());
        assert_eq!(stack.depth(), 0);

        let frame1 = CallFrame::new(10, 5, 1);
        stack.push(frame1).unwrap();

        assert!(!stack.is_empty());
        assert_eq!(stack.depth(), 1);

        let frame2 = CallFrame::new(20, 10, 2);
        stack.push(frame2).unwrap();

        assert_eq!(stack.depth(), 2);

        let popped = stack.pop().unwrap();
        assert_eq!(popped.return_ip, 20);
        assert_eq!(stack.depth(), 1);
    }

    #[test]
    fn test_call_stack_max_depth() {
        let mut stack = CallStack::new(3);

        for i in 0..3 {
            let frame = CallFrame::new(i * 10, i * 5, i as u64 + 1);
            assert!(stack.push(frame).is_ok());
        }

        // This should fail
        let frame = CallFrame::new(100, 50, 4);
        assert!(stack.push(frame).is_err());
    }
}
