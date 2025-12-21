# Engineering Plan: Phase 2 - Optimization
**Duration**: 3 weeks
**Objective**: Implement full escape analysis, complete tail call optimization, and closure optimizations

## 📋 Overview
This phase builds on Phase 1's foundation to implement advanced optimizations including full escape analysis, complete tail call optimization with frame reuse, and closure optimizations.

## 🗂️ File Modifications

### 1. **Physics World VM Enhancements**

#### `physics_world/src/vm/state.rs`
**Enhanced Modifications**:
```rust
// Complete tail call implementation with frame reuse
pub fn handle_tail_call(&mut VmState, function_ptr: u16, args: Vec<Value>) -> Result<(), VmError> {
    if vm.call_stack.is_empty() {
        return Err(VmError::stack_underflow(
            vm.create_error_context(),
            "TailCall",
            1,
            0
        ));
    }

    // Get current frame for reuse
    let current_frame = vm.call_stack.last_mut().unwrap();

    // Verify this is a valid tail call position
    if !vm.is_current_position_tail() {
        return Err(VmError::invalid_opcode(
            vm.create_error_context(),
            "TailCall not in tail position"
        ));
    }

    // Check recursion depth even for tail calls
    let new_depth = current_frame.recursion_depth + 1;
    if new_depth > vm.max_recursion_depth {
        return Err(VmError::recursion_limit_exceeded(
            vm.create_error_context(),
            vm.max_recursion_depth,
            new_depth
        ));
    }

    // Reuse the current frame - this is the key TCO optimization
    current_frame.return_address = vm.ip + 2; // Update return address
    current_frame.recursion_depth = new_depth;
    current_frame.is_tail_call = true;

    // Clear locals for reuse
    current_frame.locals.clear();
    current_frame.closed_over.clear();

    // Push arguments as new locals
    current_frame.locals = args;

    // Jump to function instead of calling
    vm.ip = function_ptr as usize;
    Ok(())
}

// Enhanced handle_call with full escape analysis integration
pub fn handle_call(&mut VmState, function_ptr: u16) -> Result<(), VmError> {
    let is_tail_position = vm.is_current_position_tail();
    let arg_count = vm.read_u16()?;
    let args: Vec<Value> = vm.stack.drain((vm.stack.len() - arg_count as usize)..).collect();

    if is_tail_position {
        handle_tail_call(vm, function_ptr, args)?
    } else {
        // Regular call with escape analysis integration
        let current_depth = if let Some(last_frame) = vm.call_stack.last() {
            last_frame.recursion_depth + 1
        } else {
            1
        };

        if current_depth > vm.max_recursion_depth {
            return Err(VmError::recursion_limit_exceeded(
                vm.create_error_context(),
                vm.max_recursion_depth,
                current_depth
            ));
        }

        // Get escape analysis info from function metadata
        let function_info = vm.get_function_info(function_ptr)?;
        let local_count = function_info.local_count;
        let escape_info = function_info.escape_info;

        let call_frame = CallFrame {
            return_address: vm.ip + 2,
            stack_start: vm.stack.len() - arg_count as usize,
            locals: vec![Value::Nil; local_count], // Pre-allocate locals
            closed_over: HashMap::new(),
            recursion_depth: current_depth,
            is_tail_call: false,
            frame_id: vm.next_frame_id(),
        };

        // Process closed-over variables based on escape analysis
        for (var_index, escape_status) in escape_info {
            if escape_status == EscapeStatus::Escaping {
                // This variable escapes - needs to be in closed_over
                let value = vm.get_local_var(var_index)?;
                call_frame.closed_over.insert(var_index, value);
            }
        }

        vm.call_stack.push(call_frame);
        vm.ip = function_ptr as usize;
        Ok(())
    }
}
```

#### `physics_world/src/vm/opcodes/closure.rs`
**New File**:
```rust
// Optimized closure creation based on escape analysis
pub fn handle_make_closure(vm: &mut VmState, function_ptr: u16) -> Result<(), VmError> {
    let function_info = vm.get_function_info(function_ptr)?;
    let free_vars = &function_info.free_variables;
    let escape_info = &function_info.escape_info;

    // Create closure with only escaping variables
    let mut closure = Closure {
        function_ptr,
        environment: Environment::new(),
    };

    for (var_index, escape_status) in escape_info.iter().enumerate() {
        if *escape_status == EscapeStatus::Escaping {
            let value = if let Some(var) = free_vars.get(var_index) {
                vm.get_local_var(*var)?
            } else {
                Value::Nil // Shouldn't happen with proper analysis
            };
            closure.environment.set(var_index as usize, value);
        }
    }

    // Push closure to stack
    vm.stack.push(Value::Closure(closure));
    Ok(())
}
```

### 2. **Jue World Compiler Enhancements**

#### `jue_world/src/compiler/compiler.rs`
**Enhanced Modifications**:
```rust
// Complete escape analysis implementation
pub struct EscapeAnalysis {
    escaping_vars: HashSet<usize>,
    current_function: Option<FunctionId>,
    variable_environments: Vec<HashSet<usize>>, // Track variables in each scope
    function_info: HashMap<FunctionId, FunctionInfo>,
}

impl EscapeAnalysis {
    pub fn new() -> Self {
        Self {
            escaping_vars: HashSet::new(),
            current_function: None,
            variable_environments: Vec::new(),
            function_info: HashMap::new(),
        }
    }

    pub fn analyze_expression(&mut self, expr: &Expr, context: &mut AnalysisContext) {
        match expr {
            Expr::Variable(var) => {
                self.analyze_variable(*var, context);
            }
            Expr::Lambda(params, body) => {
                self.analyze_lambda(params, body, context);
            }
            Expr::Let(bindings, body) => {
                self.analyze_let(bindings, body, context);
            }
            // Other expression types...
        }
    }

    fn analyze_variable(&mut self, var: usize, context: &mut AnalysisContext) {
        // Check if this variable is defined in current or outer scopes
        for (scope_index, vars) in self.variable_environments.iter().rev() {
            if vars.contains(&var) {
                // Variable is in scope - check if it's used in a way that requires escaping
                if context.is_captured() {
                    self.escaping_vars.insert(var);
                }
                return;
            }
        }

        // Variable not found - error condition
        context.report_error(CompileError::VariableNotFound(var));
    }

    fn analyze_lambda(&mut self, params: &[Expr], body: &Expr, context: &mut AnalysisContext) {
        // Push new scope for lambda parameters
        let mut param_vars = HashSet::new();
        for (index, param) in params.iter().enumerate() {
            if let Expr::Variable(var) = param {
                param_vars.insert(*var);
            }
        }
        self.variable_environments.push(param_vars);

        // Analyze lambda body
        self.analyze_expression(body, context);

        // Pop parameter scope
        self.variable_environments.pop();

        // All free variables in this lambda escape
        let free_vars = self.find_free_variables(body);
        for var in free_vars {
            self.escaping_vars.insert(var);
        }

        // Store escape info for this function
        if let Some(func_id) = context.current_function {
            let mut escape_info = HashMap::new();
            for var in 0..context.max_variable {
                escape_info.insert(var, self.escaping_vars.contains(&var));
            }
            self.function_info.insert(func_id, escape_info);
        }
    }
}

// Enhanced tail call detection with control flow analysis
pub fn is_tail_position(expr: &Expr, context: &CompilationContext) -> bool {
    match context.position {
        ExpressionPosition::FunctionBodyLast => true,
        ExpressionPosition::BlockLast => true,
        ExpressionPosition::IfConsequent => {
            // In if expressions, both branches must be tail positions
            // for the whole if to be in tail position
            is_tail_position(expr, context)
        },
        ExpressionPosition::IfAlternative => {
            is_tail_position(expr, context)
        },
        ExpressionPosition::MatchArm => {
            // In match expressions, all arms must be tail positions
            is_tail_position(expr, context)
        },
        _ => false,
    }
}
```

### 3. **New Files**

#### `physics_world/src/vm/optimization.rs`
```rust
// Optimization analysis and metrics
pub struct OptimizationMetrics {
    pub tail_calls: u32,
    pub escaped_variables: u32,
    pub heap_allocations: u32,
    pub stack_reuses: u32,
}

impl VmState {
    pub fn get_optimization_metrics(&self) -> OptimizationMetrics {
        let tail_calls = self.call_stack.iter()
            .filter(|frame| frame.is_tail_call)
            .count() as u32;

        let escaped_variables = self.call_stack.iter()
            .map(|frame| frame.closed_over.len() as u32)
            .sum();

        OptimizationMetrics {
            tail_calls,
            escaped_variables,
            heap_allocations: 0, // TODO: Track heap allocations
            stack_reuses: tail_calls, // Each tail call reuses a frame
        }
    }

    pub fn log_optimization_event(&mut self, event: OptimizationEvent) {
        // Log optimization events for analysis
        self.optimization_log.push(event);
    }
}

#[derive(Debug, Clone)]
pub enum OptimizationEvent {
    TailCallOptimized,
    VariableEscaped(usize),
    ClosureOptimized,
    FrameReused,
}

#[derive(Debug, Clone)]
pub struct OptimizationAnalysis {
    pub metrics: OptimizationMetrics,
    pub events: Vec<OptimizationEvent>,
}

impl OptimizationAnalysis {
    pub fn new() -> Self {
        Self {
            metrics: OptimizationMetrics::default(),
            events: Vec::new(),
        }
    }

    pub fn analyze(&mut self, vm: &VmState) {
        self.metrics = vm.get_optimization_metrics();
        // Additional analysis logic...
    }
}
```

#### `physics_world/src/vm/test/optimization_tests.rs`
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::state::VmState;
    use crate::types::Value;

    #[test]
    fn test_tail_call_optimization() {
        let mut vm = VmState::new(Vec::new(), Vec::new(), 1000, 1024, 1);

        // Set up a tail call scenario
        vm.stack.push(Value::Int(5));
        vm.stack.push(Value::Int(1));

        // Simulate tail call
        let result = vm.handle_tail_call(0); // Assuming function at 0

        assert!(result.is_ok());
        assert_eq!(vm.call_stack.len(), 1); // Frame should be reused
        assert!(vm.call_stack[0].is_tail_call);
    }

    #[test]
    fn test_escape_analysis() {
        let mut analysis = EscapeAnalysis::new();
        let mut context = AnalysisContext::new();

        // Test lambda with escaping variable
        let lambda = Expr::Lambda(
            vec![Expr::Variable(0)],
            Box::new(Expr::Variable(0))
        );

        analysis.analyze_expression(&lambda, &mut context);

        // Variable 0 should be marked as escaping
        assert!(analysis.is_escaping(0));
    }

    #[test]
    fn test_optimization_metrics() {
        let mut vm = VmState::new(Vec::new(), Vec::new(), 1000, 1024, 1);

        // Create some tail calls
        vm.call_stack.push(CallFrame {
            return_address: 0,
            stack_start: 0,
            locals: Vec::new(),
            closed_over: HashMap::new(),
            recursion_depth: 1,
            is_tail_call: true,
            frame_id: 1,
        });

        let metrics = vm.get_optimization_metrics();
        assert_eq!(metrics.tail_calls, 1);
        assert_eq!(metrics.stack_reuses, 1);
    }
}
```

## 📋 Implementation Plan

### Week 1: Complete Tail Call Optimization
```markdown
[ ] Implement full frame reuse in handle_tail_call
[ ] Add recursion depth checking to tail calls
[ ] Update all call sites to use new optimization
[ ] Create comprehensive tail call test suite
[ ] Benchmark tail call performance improvements
[ ] Analyze memory usage with TCO
```

### Week 2: Full Escape Analysis
```markdown
[ ] Implement complete escape analysis in compiler
[ ] Integrate escape analysis with closure creation
[ ] Update MakeClosure opcode to use escape info
[ ] Add escape analysis to all expression types
[ ] Create escape analysis test cases
[ ] Verify escape analysis correctness
```

### Week 3: Closure Optimization and Integration
```markdown
[ ] Implement optimized closure creation
[ ] Integrate with existing closure mechanism
[ ] Add closure optimization tests
[ ] Benchmark closure performance
[ ] Create optimization metrics system
[ ] Final integration testing
```

## 🎯 Success Criteria
- ✅ Complete tail call optimization with frame reuse
- ✅ Full escape analysis implementation
- ✅ Optimized closure creation
- ✅ 95% memory reduction for non-escaping variables
- ✅ O(1) space complexity for tail recursion
- ✅ All Phase 1 tests still passing