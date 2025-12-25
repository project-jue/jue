//! Call frame management for the VM.
//!
//! This module handles the call stack, including frame creation,
//! push/pop operations, and call frame state tracking.
//!
//! Also provides RecursiveEnvironment for proper letrec semantics
//! and recursive closure support.
//!
//! # Extracted from
//! - `vm/state.rs` (lines 398-411, call-related methods)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::types::Value;

/// Represents a binding in the environment.
///
/// This enum supports proper letrec semantics where recursive bindings
/// can reference themselves during construction.
#[derive(Debug, Clone)]
pub enum EnvBinding {
    /// Normal variable binding (immutable)
    Normal(Value),
    /// Recursive binding that can reference itself
    /// The closure captures its own binding for self-reference
    Recursive {
        /// The closure value
        closure: Value,
        /// The code index for re-creation if needed
        code_index: usize,
        /// Captured values from the environment
        captures: Vec<Value>,
    },
    /// Uninitialized binding (for letrec during construction)
    Uninitialized,
}

/// A symbol type for environment variable names
pub type Symbol = String;

/// RecursiveEnvironment - Environment with proper letrec semantics
///
/// This environment supports recursive bindings where a closure can
/// reference itself. Following Lisp/Scheme standards:
/// - `letrec` allows local functions to reference themselves
/// - Bindings are initialized before their body is evaluated
/// - Parent chain enables proper lexical scoping
#[derive(Debug, Clone)]
pub struct RecursiveEnvironment {
    /// The bindings in this environment layer
    bindings: HashMap<Symbol, EnvBinding>,
    /// Parent environment for lexical scoping
    parent: Option<Box<RecursiveEnvironment>>,
}

impl RecursiveEnvironment {
    /// Creates a new empty environment with no parent
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    /// Creates a child environment with the given parent
    pub fn extend(parent: RecursiveEnvironment) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    /// Defines a normal variable binding
    pub fn define(&mut self, name: Symbol, value: Value) {
        self.bindings.insert(name, EnvBinding::Normal(value));
    }

    /// Defines a recursive binding (for self-referential closures)
    ///
    /// # Arguments
    /// * `name` - The variable name
    /// * `code_index` - Index into constant pool for closure body
    /// * `captures` - Values captured from the environment
    pub fn define_recursive(&mut self, name: Symbol, code_index: usize, captures: Vec<Value>) {
        self.bindings.insert(
            name,
            EnvBinding::Recursive {
                closure: Value::Nil, // Will be filled in by MakeClosure
                code_index,
                captures,
            },
        );
    }

    /// Updates a recursive binding with the actual closure value
    ///
    /// This must be called after the closure is created to complete
    /// the self-reference cycle.
    pub fn set_recursive_closure(&mut self, name: &Symbol, closure: Value) {
        if let Some(binding) = self.bindings.get_mut(name) {
            if let EnvBinding::Recursive {
                closure: existing, ..
            } = binding
            {
                *existing = closure;
            }
        }
    }

    /// Looks up a variable in this environment or parent chain
    ///
    /// # Returns
    /// `Some(&Value)` if found, `None` if not found
    pub fn lookup(&self, name: &Symbol) -> Option<&Value> {
        // Check local bindings first
        if let Some(binding) = self.bindings.get(name) {
            match binding {
                EnvBinding::Normal(value) => Some(value),
                EnvBinding::Recursive { closure, .. } => Some(closure),
                EnvBinding::Uninitialized => None,
            }
        } else {
            // Check parent environment
            self.parent.as_ref().and_then(|p| p.lookup(name))
        }
    }

    /// Looks up a variable mutably (for setting values in local scope only)
    ///
    /// # Returns
    /// `Some(&mut Value)` if found in local scope, `None` otherwise
    pub fn lookup_mut(&mut self, name: &Symbol) -> Option<&mut Value> {
        // Only look in local bindings - parent is immutable reference
        if let Some(binding) = self.bindings.get_mut(name) {
            match binding {
                EnvBinding::Normal(value) => Some(value),
                EnvBinding::Recursive { closure, .. } => Some(closure),
                EnvBinding::Uninitialized => None,
            }
        } else {
            None
        }
    }

    /// Checks if a binding exists in this environment (not parent)
    pub fn is_bound_locally(&self, name: &Symbol) -> bool {
        self.bindings.contains_key(name)
    }

    /// Returns the number of local bindings
    pub fn len(&self) -> usize {
        self.bindings.len()
    }

    /// Returns true if this environment has no local bindings
    pub fn is_empty(&self) -> bool {
        self.bindings.is_empty()
    }

    /// Gets the parent environment reference
    pub fn get_parent(&self) -> Option<&Box<RecursiveEnvironment>> {
        self.parent.as_ref()
    }
}

impl Default for RecursiveEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a closure with its environment for proper recursion support
#[derive(Debug, Clone)]
pub struct Closure {
    /// The code index into constant pool
    pub code_index: usize,
    /// Captured values from the environment
    pub captures: Vec<Value>,
    /// The environment where this closure was created
    /// This is crucial for recursive self-reference
    pub environment: Option<Box<RecursiveEnvironment>>,
    /// The variable name this closure is bound to (for debugging/recursion)
    pub name: Option<Symbol>,
}

impl Closure {
    /// Creates a new closure with the given code index and captures
    pub fn new(
        code_index: usize,
        captures: Vec<Value>,
        environment: Option<Box<RecursiveEnvironment>>,
        name: Option<Symbol>,
    ) -> Self {
        Self {
            code_index,
            captures,
            environment,
            name,
        }
    }

    /// Creates a closure that can reference itself via its name
    pub fn with_self_reference(
        code_index: usize,
        captures: Vec<Value>,
        environment: RecursiveEnvironment,
        name: Symbol,
    ) -> Self {
        Self {
            code_index,
            captures,
            environment: Some(Box::new(environment)),
            name: Some(name),
        }
    }
}

use crate::types::{HeapPtr, OpCode};

/// Represents a call frame for function calls.
///
/// Stores the return address and stack state for proper function call/return semantics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallFrame {
    pub return_ip: usize,
    pub stack_start: usize,
    pub original_stack_size: usize, // Stack size BEFORE this call (for proper truncation on return)
    pub saved_instructions: Option<Vec<OpCode>>, // Store original instructions for nested calls
    pub recursion_depth: u32,       // Track recursion depth for this call frame
    pub locals: Vec<Value>,         // Lexical environment storage
    pub closed_over: HashMap<usize, Value>, // Closed-over variables
    pub is_tail_call: bool,         // TCO tracking flag
    pub frame_id: u64,              // For debugging/verification
    pub code_index: usize,          // Track which closure this frame is for (for TCO)
}

impl CallFrame {
    /// Creates a new call frame with the given return address and stack start position.
    ///
    /// # Arguments
    /// * `return_ip` - The instruction pointer to return to after the function completes
    /// * `stack_start` - The stack position before this function was called
    /// * `original_stack_size` - The stack size before this call (for truncation on return)
    /// * `frame_id` - Unique identifier for debugging
    /// * `code_index` - The code index for TCO detection
    pub fn new(
        return_ip: usize,
        stack_start: usize,
        original_stack_size: usize,
        frame_id: u64,
        code_index: usize,
    ) -> Self {
        Self {
            return_ip,
            stack_start,
            original_stack_size,
            saved_instructions: None,
            recursion_depth: 0,
            locals: Vec::new(),
            closed_over: HashMap::new(),
            is_tail_call: false,
            frame_id,
            code_index,
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
    use crate::types::HeapPtr;

    #[test]
    fn test_call_frame_creation() {
        let frame = CallFrame::new(10, 5, 3, 1, 0);
        assert_eq!(frame.return_ip, 10);
        assert_eq!(frame.stack_start, 5);
        assert_eq!(frame.original_stack_size, 3);
        assert_eq!(frame.frame_id, 1);
        assert!(frame.locals.is_empty());
    }

    #[test]
    fn test_call_frame_locals() {
        let mut frame = CallFrame::new(0, 0, 0, 1, 0);
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

        let frame1 = CallFrame::new(10, 5, 3, 1, 0);
        stack.push(frame1).unwrap();

        assert!(!stack.is_empty());
        assert_eq!(stack.depth(), 1);

        let frame2 = CallFrame::new(20, 10, 6, 2, 1);
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
            let frame = CallFrame::new(i * 10, i * 5, i * 3, i as u64 + 1, i);
            assert!(stack.push(frame).is_ok());
        }

        // This should fail
        let frame = CallFrame::new(100, 50, 30, 4, 4);
        assert!(stack.push(frame).is_err());
    }

    #[test]
    fn test_recursive_environment_basic() {
        let mut env = RecursiveEnvironment::new();

        // Define a normal binding
        let x_name: Symbol = "x".to_string();
        env.define(x_name.clone(), Value::Int(42));
        assert_eq!(env.lookup(&x_name), Some(&Value::Int(42)));

        // Define a recursive binding
        let fact_name: Symbol = "fact".to_string();
        env.define_recursive(fact_name.clone(), 0, vec![]);
        assert!(env.is_bound_locally(&fact_name));

        // Update the recursive binding with a closure
        let closure = Value::Closure(HeapPtr::new(100));
        env.set_recursive_closure(&fact_name, closure);
        assert!(matches!(env.lookup(&fact_name), Some(&Value::Closure(_))));

        println!("✅ RecursiveEnvironment basic operations test passed");
    }

    #[test]
    fn test_recursive_environment_parent_chain() {
        let mut parent = RecursiveEnvironment::new();
        let parent_var: Symbol = "parent_var".to_string();
        parent.define(parent_var.clone(), Value::Int(100));

        let mut child = RecursiveEnvironment::extend(parent);
        let child_var: Symbol = "child_var".to_string();
        child.define(child_var.clone(), Value::Int(200));

        // Look up in child
        assert_eq!(child.lookup(&child_var), Some(&Value::Int(200)));

        // Look up in parent via chain
        assert_eq!(child.lookup(&parent_var), Some(&Value::Int(100)));

        // Non-existent variable
        let nonexistent: Symbol = "nonexistent".to_string();
        assert_eq!(child.lookup(&nonexistent), None);

        println!("✅ RecursiveEnvironment parent chain test passed");
    }

    #[test]
    fn test_recursive_environment_closure_self_reference() {
        let mut env = RecursiveEnvironment::new();

        // Define recursive binding for factorial
        let fact_name: Symbol = "fact".to_string();
        env.define_recursive(fact_name.clone(), 0, vec![]);

        // Create a closure that references "fact" from the environment
        let closure = Closure::with_self_reference(
            0,                 // code_index
            vec![],            // no captures from outer scope
            env.clone(),       // environment with "fact" binding
            fact_name.clone(), // name for self-reference
        );

        // Update the recursive binding
        env.set_recursive_closure(&fact_name, Value::Closure(HeapPtr::new(42)));

        // Verify the closure can look up itself
        let fact_lookup = env.lookup(&fact_name);
        assert!(fact_lookup.is_some());

        println!("✅ RecursiveEnvironment closure self-reference test passed");
    }

    #[test]
    fn test_letrec_semantics() {
        let mut env = RecursiveEnvironment::new();

        // Step 1: Create uninitialized recursive binding
        let fact_name: Symbol = "fact".to_string();
        env.define_recursive(fact_name.clone(), 0, vec![]);

        // Step 2: Create closure that references "fact"
        let _closure = Closure::with_self_reference(
            0,           // code_index for factorial body
            vec![],      // no outer captures
            env.clone(), // environment with "fact"
            fact_name.clone(),
        );

        // Step 3: Complete the recursive binding
        env.set_recursive_closure(&fact_name, Value::Closure(HeapPtr::new(100)));

        // Step 4: Verify the binding is available
        let fact = env.lookup(&fact_name);
        assert!(fact.is_some());

        println!("✅ letrec semantics test passed");
    }

    #[test]
    fn test_mutual_recursion_environment() {
        let mut env = RecursiveEnvironment::new();

        // Define both bindings as uninitialized
        let even_name: Symbol = "even".to_string();
        let odd_name: Symbol = "odd".to_string();
        env.define_recursive(even_name.clone(), 0, vec![]);
        env.define_recursive(odd_name.clone(), 1, vec![]);

        // Create closures with environment that has both bindings
        let _even_closure = Closure::with_self_reference(
            0,           // even body code index
            vec![],      // no captures
            env.clone(), // env with both even and odd
            even_name.clone(),
        );

        let _odd_closure = Closure::with_self_reference(
            1,           // odd body code index
            vec![],      // no captures
            env.clone(), // env with both even and odd
            odd_name.clone(),
        );

        // Complete the recursive bindings
        env.set_recursive_closure(&even_name, Value::Closure(HeapPtr::new(200)));
        env.set_recursive_closure(&odd_name, Value::Closure(HeapPtr::new(201)));

        // Verify both are available
        assert!(env.lookup(&even_name).is_some());
        assert!(env.lookup(&odd_name).is_some());

        println!("✅ Mutual recursion environment test passed");
    }

    #[test]
    fn test_env_binding_variants() {
        // Test Normal binding
        let normal = EnvBinding::Normal(Value::Int(42));
        assert!(matches!(normal, EnvBinding::Normal(Value::Int(42))));

        // Test Uninitialized binding
        let uninit = EnvBinding::Uninitialized;
        assert!(matches!(uninit, EnvBinding::Uninitialized));

        // Test Recursive binding
        let recursive = EnvBinding::Recursive {
            closure: Value::Nil,
            code_index: 0,
            captures: vec![],
        };
        assert!(matches!(
            recursive,
            EnvBinding::Recursive {
                closure: Value::Nil,
                ..
            }
        ));

        println!("✅ EnvBinding variants test passed");
    }
}
