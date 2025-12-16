#[cfg(test)]
mod tests {
    use super::*;
    use crate::comptime::{execute_comptime, ComptimeEnv, ComptimeExecutor};
    use crate::error::CompilationError;
    use crate::trust_tier::TrustTier;
    use physics_world::types::{Capability, OpCode, Value};

    #[test]
    fn test_comptime_env_creation() {
        let env = ComptimeEnv::new(TrustTier::Empirical, 1000, 1024);

        assert!(env.has_capability(&Capability::MacroHygienic));
        assert!(env.has_capability(&Capability::IoReadSensor));
        assert_eq!(env.max_steps, 1000);
        assert_eq!(env.memory_limit, 1024);
        assert_eq!(env.step_count, 0);
        assert_eq!(env.memory_usage, 0);
    }

    #[test]
    fn test_comptime_env_capabilities() {
        let env = ComptimeEnv::new(TrustTier::Formal, 1000, 1024);

        assert!(env.has_capability(&Capability::MacroHygienic));
        assert!(!env.has_capability(&Capability::ComptimeEval));
        assert!(!env.has_capability(&Capability::IoReadSensor));
    }

    #[test]
    fn test_comptime_env_limits() {
        let mut env = ComptimeEnv::new(TrustTier::Empirical, 5, 10);

        assert!(env.can_continue());

        for _ in 0..5 {
            env.increment_step().unwrap();
        }

        assert!(!env.can_continue());
        assert!(env.increment_step().is_err());
    }

    #[test]
    fn test_comptime_executor_creation() {
        let executor = ComptimeExecutor::new(TrustTier::Empirical, 1000, 1024);

        assert_eq!(executor.stack.len(), 0);
        assert_eq!(executor.constants.len(), 0);
        assert!(executor.env.has_capability(&Capability::MacroHygienic));
    }

    #[test]
    fn test_simple_comptime_execution() {
        let mut executor = ComptimeExecutor::new(TrustTier::Empirical, 100, 1024);

        // Execute: push 5, push 3, add
        let bytecode = vec![OpCode::Int(5), OpCode::Int(3), OpCode::Add];

        let result = executor.execute(bytecode);

        assert!(result.is_ok());
        let result = result.unwrap();

        assert!(matches!(result.value, Value::Int(8)));
        assert_eq!(result.steps_used, 3);
    }

    #[test]
    fn test_comptime_arithmetic() {
        let mut executor = ComptimeExecutor::new(TrustTier::Empirical, 100, 1024);

        // Execute: push 10, push 3, sub
        let bytecode = vec![OpCode::Int(10), OpCode::Int(3), OpCode::Sub];

        let result = executor.execute(bytecode);

        assert!(result.is_ok());
        assert!(matches!(result.unwrap().value, Value::Int(7)));
    }

    #[test]
    fn test_comptime_comparison() {
        let mut executor = ComptimeExecutor::new(TrustTier::Empirical, 100, 1024);

        // Execute: push 5, push 3, lt
        let bytecode = vec![OpCode::Int(5), OpCode::Int(3), OpCode::Lt];

        let result = executor.execute(bytecode);

        assert!(result.is_ok());
        assert!(matches!(result.unwrap().value, Value::Bool(false)));
    }

    #[test]
    fn test_comptime_stack_operations() {
        let mut executor = ComptimeExecutor::new(TrustTier::Empirical, 100, 1024);

        // Execute: push 1, push 2, swap, pop, dup
        let bytecode = vec![
            OpCode::Int(1),
            OpCode::Int(2),
            OpCode::Swap,
            OpCode::Pop,
            OpCode::Dup,
        ];

        let result = executor.execute(bytecode);

        assert!(result.is_ok());
        assert!(matches!(result.unwrap().value, Value::Int(2)));
    }

    #[test]
    fn test_comptime_step_limit() {
        let mut executor = ComptimeExecutor::new(TrustTier::Empirical, 2, 1024);

        // Execute more steps than allowed
        let bytecode = vec![OpCode::Int(1), OpCode::Int(2), OpCode::Add];

        let result = executor.execute(bytecode);

        assert!(result.is_err());
        match result {
            Err(CompilationError::ComptimeError(_)) => assert!(true),
            _ => panic!("Expected ComptimeError"),
        }
    }

    #[test]
    fn test_comptime_stack_underflow() {
        let mut executor = ComptimeExecutor::new(TrustTier::Empirical, 100, 1024);

        // Execute: pop from empty stack
        let bytecode = vec![OpCode::Pop];

        let result = executor.execute(bytecode);

        assert!(result.is_err());
        match result {
            Err(CompilationError::ComptimeError(_)) => assert!(true),
            _ => panic!("Expected ComptimeError"),
        }
    }

    #[test]
    fn test_comptime_capability_check() {
        let mut executor = ComptimeExecutor::new(TrustTier::Empirical, 100, 1024);

        // Add capability constant
        executor
            .constants
            .push(Value::Capability(Capability::IoReadSensor));

        // Execute: check if we have IoReadSensor capability
        let bytecode = vec![OpCode::HasCap(0)];

        let result = executor.execute(bytecode);

        assert!(result.is_ok());
        assert!(matches!(result.unwrap().value, Value::Bool(true)));
    }

    #[test]
    fn test_comptime_unsupported_opcode() {
        let mut executor = ComptimeExecutor::new(TrustTier::Empirical, 100, 1024);

        // Execute: jump opcode (not supported in comptime)
        let bytecode = vec![OpCode::Jmp(10)];

        let result = executor.execute(bytecode);

        assert!(result.is_err());
        match result {
            Err(CompilationError::ComptimeError(_)) => assert!(true),
            _ => panic!("Expected ComptimeError"),
        }
    }

    #[test]
    fn test_comptime_function() {
        let result = execute_comptime(
            vec![OpCode::Int(5), OpCode::Int(3), OpCode::Add],
            TrustTier::Empirical,
            100,
            1024,
        );

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(matches!(result.value, Value::Int(8)));
        assert_eq!(result.steps_used, 3);
    }
}
