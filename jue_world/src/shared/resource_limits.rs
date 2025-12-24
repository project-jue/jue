use crate::error::CompilationError;
use physics_world::types::{OpCode, Value};
use physics_world::vm::state::{VmError, VmState};

/// Resource limit configuration for Jue-World execution
#[derive(Debug, Clone)]
pub struct ResourceLimits {
    /// Maximum execution steps allowed
    pub step_limit: u64,
    /// Maximum memory usage allowed in bytes
    pub memory_limit: usize,
    /// Maximum call stack depth allowed
    pub call_stack_limit: usize,
    /// Maximum heap allocations allowed
    pub heap_allocation_limit: usize,
}

/// Resource limit enforcer that works with Physics-World VM
pub struct ResourceLimitEnforcer {
    limits: ResourceLimits,
}

impl ResourceLimitEnforcer {
    /// Create a new resource limit enforcer with the given configuration
    pub fn new(limits: ResourceLimits) -> Self {
        Self { limits }
    }

    /// Apply resource limits to a VM state
    pub fn apply_limits_to_vm(&self, vm: &mut VmState) {
        // Set step limit
        vm.steps_remaining = self.limits.step_limit;

        // Ensure memory limit is respected
        if vm.memory.capacity() > self.limits.memory_limit as u32 {
            // If current memory capacity exceeds limit, we need to create a new arena
            // This is a simplified approach - in a real implementation, we would
            // need to handle memory migration or error conditions
            vm.memory = physics_world::memory::arena::ObjectArena::with_capacity(
                self.limits.memory_limit as u32,
            );
        }
    }

    /// Validate bytecode against resource limits
    pub fn validate_bytecode(
        &self,
        bytecode: &[OpCode],
        constants: &[Value],
    ) -> Result<(), CompilationError> {
        // Estimate resource usage from bytecode
        let estimated_steps = self.estimate_step_count(bytecode);
        let estimated_memory = self.estimate_memory_usage(bytecode, constants);

        if estimated_steps > self.limits.step_limit {
            return Err(CompilationError::ProofGenerationFailed(format!(
                "Estimated step count {} exceeds limit {}",
                estimated_steps, self.limits.step_limit
            )));
        }

        if estimated_memory > self.limits.memory_limit {
            return Err(CompilationError::ProofGenerationFailed(format!(
                "Estimated memory usage {} exceeds limit {}",
                estimated_memory, self.limits.memory_limit
            )));
        }

        Ok(())
    }

    /// Estimate step count based on bytecode complexity
    fn estimate_step_count(&self, bytecode: &[OpCode]) -> u64 {
        // Simple estimation: each instruction takes at least 1 step
        // Complex instructions (loops, function calls) would take more
        bytecode.len() as u64
    }

    /// Estimate memory usage based on bytecode and constants
    fn estimate_memory_usage(&self, bytecode: &[OpCode], constants: &[Value]) -> usize {
        // Estimate memory usage:
        // 1. Constants pool
        // 2. Stack usage
        // 3. Heap allocations (Cons, MakeClosure operations)

        let mut estimated_memory = 0;

        // Constants pool
        for constant in constants {
            estimated_memory += match constant {
                Value::Nil => 1,
                Value::Bool(_) => 1,
                Value::Int(_) => 8,
                Value::Float(_) => 8,
                Value::String(_) => 8, // String storage (pointer + length)
                Value::Symbol(_) => 4,
                Value::Pair(_) => 8,
                Value::Closure(_) => 16,
                Value::ActorId(_) => 4,
                Value::Capability(_) => 8,
                &Value::GcPtr(_) => 4,
            };
        }

        // Heap allocations from bytecode
        for opcode in bytecode {
            match opcode {
                OpCode::Cons => estimated_memory += 8, // Pair allocation
                OpCode::MakeClosure(_, capture_count) => {
                    estimated_memory += 4 + (capture_count * 4); // Closure allocation
                }
                _ => {}
            }
        }

        // Add some overhead for stack and VM state
        estimated_memory += 1024; // 1KB overhead

        estimated_memory
    }

    /// Check if a VM error is related to resource limits
    pub fn is_resource_limit_error(&self, error: &VmError) -> bool {
        matches!(
            error,
            VmError::CpuLimitExceeded | VmError::MemoryLimitExceeded
        )
    }

    /// Get resource limit configuration
    pub fn get_limits(&self) -> &ResourceLimits {
        &self.limits
    }
}

/// Resource limit builder for configuring execution constraints
pub struct ResourceLimitBuilder {
    limits: ResourceLimits,
}

impl ResourceLimitBuilder {
    /// Create a new resource limit builder with default settings
    pub fn new() -> Self {
        Self {
            limits: ResourceLimits {
                step_limit: 1000,
                memory_limit: 1024 * 1024, // 1MB
                call_stack_limit: 100,
                heap_allocation_limit: 1000,
            },
        }
    }

    /// Set the maximum execution steps
    pub fn with_step_limit(mut self, limit: u64) -> Self {
        self.limits.step_limit = limit;
        self
    }

    /// Set the maximum memory usage
    pub fn with_memory_limit(mut self, limit: usize) -> Self {
        self.limits.memory_limit = limit;
        self
    }

    /// Set the maximum call stack depth
    pub fn with_call_stack_limit(mut self, limit: usize) -> Self {
        self.limits.call_stack_limit = limit;
        self
    }

    /// Set the maximum heap allocations
    pub fn with_heap_allocation_limit(mut self, limit: usize) -> Self {
        self.limits.heap_allocation_limit = limit;
        self
    }

    /// Build the resource limit enforcer
    pub fn build(self) -> ResourceLimitEnforcer {
        ResourceLimitEnforcer::new(self.limits)
    }
}

/// Resource monitoring and reporting
pub struct ResourceMonitor {
    initial_steps: u64,
    initial_memory: u32,
    step_count: u64,
    memory_usage: u32,
    heap_allocations: usize,
}

impl ResourceMonitor {
    /// Create a new resource monitor
    pub fn new() -> Self {
        Self {
            initial_steps: 0,
            initial_memory: 0,
            step_count: 0,
            memory_usage: 0,
            heap_allocations: 0,
        }
    }

    /// Start monitoring a VM
    pub fn start_monitoring(&mut self, vm: &VmState) {
        self.initial_steps = vm.steps_remaining;
        self.initial_memory = vm.memory.next_free();
        self.step_count = 0;
        self.memory_usage = 0;
        self.heap_allocations = 0;
    }

    /// Update monitoring data after VM execution
    pub fn update_after_execution(&mut self, vm: &VmState) {
        self.step_count = self.initial_steps - vm.steps_remaining;
        self.memory_usage = vm.memory.next_free() - self.initial_memory;
    }

    /// Record a heap allocation
    pub fn record_heap_allocation(&mut self, size: usize) {
        self.heap_allocations += 1;
        self.memory_usage += size as u32;
    }

    /// Get resource usage report
    pub fn get_report(&self) -> ResourceUsageReport {
        ResourceUsageReport {
            steps_used: self.step_count,
            memory_used: self.memory_usage as usize,
            heap_allocations: self.heap_allocations,
        }
    }
}

/// Resource usage report
#[derive(Debug, Clone)]
pub struct ResourceUsageReport {
    /// Number of execution steps used
    pub steps_used: u64,
    /// Memory used in bytes
    pub memory_used: usize,
    /// Number of heap allocations
    pub heap_allocations: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use physics_world::types::OpCode;

    #[test]
    fn test_resource_limit_creation() {
        let limits = ResourceLimits {
            step_limit: 100,
            memory_limit: 1024,
            call_stack_limit: 50,
            heap_allocation_limit: 100,
        };

        let enforcer = ResourceLimitEnforcer::new(limits);
        assert!(enforcer.limits.step_limit == 100);
        assert!(enforcer.limits.memory_limit == 1024);
        assert!(enforcer.limits.call_stack_limit == 50);
        assert!(enforcer.limits.heap_allocation_limit == 100);
    }

    #[test]
    fn test_resource_limit_builder() {
        let enforcer = ResourceLimitBuilder::new()
            .with_step_limit(200)
            .with_memory_limit(2048)
            .with_call_stack_limit(100)
            .with_heap_allocation_limit(200)
            .build();

        assert!(enforcer.limits.step_limit == 200);
        assert!(enforcer.limits.memory_limit == 2048);
        assert!(enforcer.limits.call_stack_limit == 100);
        assert!(enforcer.limits.heap_allocation_limit == 200);
    }

    #[test]
    fn test_bytecode_validation() {
        let enforcer = ResourceLimitBuilder::new().build();

        // Test that simple bytecode passes validation
        let simple_bytecode = vec![OpCode::Int(42), OpCode::Int(1), OpCode::Add];
        let result = enforcer.validate_bytecode(&simple_bytecode, &[]);
        assert!(result.is_ok());

        // Test that complex bytecode with many operations might exceed limits
        let complex_bytecode = vec![OpCode::Int(42); 10000]; // 10k operations
        let result = enforcer.validate_bytecode(&complex_bytecode, &[]);
        // This should fail because we estimate 10k steps but limit is 1k
        assert!(result.is_err());
    }

    #[test]
    fn test_step_count_estimation() {
        let enforcer = ResourceLimitBuilder::new().build();

        let bytecode = vec![OpCode::Int(1), OpCode::Int(2), OpCode::Add];
        let estimated_steps = enforcer.estimate_step_count(&bytecode);

        assert!(estimated_steps == 3);
    }

    #[test]
    fn test_memory_usage_estimation() {
        let enforcer = ResourceLimitBuilder::new().build();

        let bytecode = vec![OpCode::Cons, OpCode::MakeClosure(0, 2)];
        let constants = vec![];

        let estimated_memory = enforcer.estimate_memory_usage(&bytecode, &constants);

        // Should account for Cons (8 bytes) + MakeClosure (4 + 2*4 = 12 bytes) + overhead (1024)
        assert!(estimated_memory > 1000);
    }

    #[test]
    fn test_resource_limit_error_detection() {
        let enforcer = ResourceLimitBuilder::new().build();

        assert!(enforcer.is_resource_limit_error(&VmError::CpuLimitExceeded));
        assert!(enforcer.is_resource_limit_error(&VmError::MemoryLimitExceeded));
        assert!(!enforcer.is_resource_limit_error(&VmError::StackUnderflow));
    }

    #[test]
    fn test_resource_monitoring() {
        let mut monitor = ResourceMonitor::new();
        let mut vm = VmState::new(vec![OpCode::Int(42)], vec![], 100, 1024, 1, 100);

        monitor.start_monitoring(&vm);
        assert!(monitor.initial_steps == 100);
        assert!(monitor.initial_memory == 0);

        // Simulate some execution
        vm.steps_remaining = 95;
        vm.memory.allocate(16, 1).unwrap(); // Allocate some memory

        monitor.update_after_execution(&vm);
        let report = monitor.get_report();

        assert!(report.steps_used == 5);
        assert!(report.memory_used > 0);
    }
}
