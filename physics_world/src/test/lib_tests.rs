use super::*;
use std::collections::HashSet;
use types::OpCode;

#[test]
fn test_physics_world_new() {
    let world = PhysicsWorld::new();
    assert!(world.scheduler.actors.is_empty());
}

#[test]
fn test_simple_execution() {
    let mut world = PhysicsWorld::new();

    // Create a simple program that pushes 42 and finishes
    let result = world.execute_actor(1, vec![OpCode::Int(42)], vec![], 1000, 1024);

    assert_eq!(result.output, Some(Value::Int(42)));
    assert!(result.messages_sent.is_empty());
    assert!(result.error.is_none());
    assert!(result.metrics.steps_used > 0);
}

#[test]
fn test_cpu_limit_exceeded() {
    let mut world = PhysicsWorld::new();

    // Create a program that will exceed CPU limit
    let result = world.execute_actor(
        1,
        vec![OpCode::Int(1), OpCode::Int(2)],
        vec![],
        1, // Very low limit
        1024,
    );

    assert!(result.output.is_none());
    assert!(matches!(
        result.error,
        Some(StructuredError::CpuLimitExceeded { .. })
    ));
}

#[test]
fn test_message_delivery() {
    let mut world = PhysicsWorld::new();

    // Add an actor first
    let actor = Actor {
        id: 1,
        vm: VmState::new(vec![], vec![], 100, 1024, 1),
        mailbox: Vec::new(),
        is_waiting: false,
        capabilities: HashSet::new(),
        capability_requests: Vec::new(),
        parent_id: None,
        priority: 128, // Default priority
        priority_boost: None,
    };
    world.scheduler.add_actor(actor);

    // Deliver messages
    world.deliver_messages(1, vec![Value::Int(42), Value::Bool(true)]);

    // Check that messages were delivered
    let actor = &world.scheduler.actors[0];
    assert_eq!(actor.mailbox.len(), 3); // Includes the placeholder Nil message
}

#[test]
fn test_integration_simple_program() {
    let mut world = PhysicsWorld::new();

    // Integration test from Section 7 of the specification
    let result = world.execute_actor(1, vec![OpCode::Int(42)], vec![], 1000, 1024);

    assert_eq!(result.output, Some(Value::Int(42)));
    assert!(result.messages_sent.is_empty());
    assert!(result.error.is_none());
}

#[test]
fn test_comptime_execution_basic() {
    let mut world = PhysicsWorld::new();

    // Create comptime environment with basic capabilities
    let mut capabilities = HashSet::new();
    capabilities.insert(Capability::ComptimeEval);
    capabilities.insert(Capability::MacroHygienic);

    let env = ComptimeEnv {
        capabilities,
        max_steps: 1000,
        memory_limit: 1024,
    };

    // Test comptime execution with simple arithmetic
    let result = world.execute_comptime(
        vec![OpCode::Int(10), OpCode::Int(20), OpCode::Add],
        vec![],
        env,
    );

    assert!(result.is_ok());
    let comptime_result = result.unwrap();
    assert_eq!(comptime_result.output, Some(Value::Int(30)));
    assert!(comptime_result.metrics.steps_used > 0);
}

#[test]
fn test_comptime_execution_without_capabilities() {
    let mut world = PhysicsWorld::new();

    // Create comptime environment with no capabilities
    let env = ComptimeEnv {
        capabilities: HashSet::new(),
        max_steps: 1000,
        memory_limit: 1024,
    };

    // Test comptime execution with simple arithmetic (should still work)
    let result = world.execute_comptime(
        vec![OpCode::Int(10), OpCode::Int(20), OpCode::Add],
        vec![],
        env,
    );

    assert!(result.is_ok());
    let comptime_result = result.unwrap();
    assert_eq!(comptime_result.output, Some(Value::Int(30)));
}

#[test]
fn test_comptime_cpu_limit_exceeded() {
    let mut world = PhysicsWorld::new();

    // Create comptime environment
    let mut capabilities = HashSet::new();
    capabilities.insert(Capability::ComptimeEval);

    let env = ComptimeEnv {
        capabilities,
        max_steps: 1, // Very low limit
        memory_limit: 1024,
    };

    // Test comptime execution that exceeds CPU limit
    let result = world.execute_comptime(
        vec![OpCode::Int(10), OpCode::Int(20), OpCode::Add],
        vec![],
        env,
    );

    assert!(result.is_err());
    if let Err(ComptimeError::CpuLimitExceeded { limit, attempted }) = result {
        assert_eq!(limit, 1);
        assert!(attempted >= 1);
    } else {
        panic!("Expected CpuLimitExceeded error");
    }
}

#[test]
fn test_comptime_capability_restrictions() {
    let mut world = PhysicsWorld::new();

    // Create comptime environment with basic capabilities
    let mut capabilities = HashSet::new();
    capabilities.insert(Capability::ComptimeEval);
    capabilities.insert(Capability::MacroHygienic);

    let env = ComptimeEnv {
        capabilities,
        max_steps: 1000,
        memory_limit: 1024,
    };

    // Test that comptime execution cannot request additional capabilities
    // This would require a RequestCap opcode, which should fail in comptime context
    let result = world.execute_comptime(
        vec![
            OpCode::Int(0), // Placeholder for capability index
            OpCode::Int(0), // Placeholder for justification index
            OpCode::RequestCap(0, 0),
        ],
        vec![],
        env,
    );

    // This should fail because comptime actors cannot request additional capabilities
    assert!(result.is_err());
    if let Err(ComptimeError::CapabilityError(msg)) = result {
        assert!(msg.contains("Comptime execution cannot request additional capabilities"));
    } else {
        panic!("Expected CapabilityError for comptime capability request");
    }
}

#[test]
fn test_comptime_with_macro_hygienic() {
    let mut world = PhysicsWorld::new();

    // Create comptime environment with macro capabilities
    let mut capabilities = HashSet::new();
    capabilities.insert(Capability::ComptimeEval);
    capabilities.insert(Capability::MacroHygienic);
    capabilities.insert(Capability::MacroUnsafe);

    let env = ComptimeEnv {
        capabilities,
        max_steps: 1000,
        memory_limit: 1024,
    };

    // Test comptime execution with macro capabilities
    let result = world.execute_comptime(vec![OpCode::Int(42)], vec![], env);

    assert!(result.is_ok());
    let comptime_result = result.unwrap();
    assert_eq!(comptime_result.output, Some(Value::Int(42)));
}

#[test]
fn test_comptime_resource_limits() {
    let mut world = PhysicsWorld::new();

    // Create comptime environment with very restrictive limits
    let mut capabilities = HashSet::new();
    capabilities.insert(Capability::ComptimeEval);

    let env = ComptimeEnv {
        capabilities,
        max_steps: 5,
        memory_limit: 64, // Very small memory limit
    };

    // Test comptime execution with restrictive limits
    let result = world.execute_comptime(
        vec![OpCode::Int(10), OpCode::Int(20), OpCode::Add],
        vec![],
        env,
    );

    // This should either succeed or fail with resource limits
    // depending on the actual resource usage
    match result {
        Ok(comptime_result) => {
            assert_eq!(comptime_result.output, Some(Value::Int(30)));
            assert!(comptime_result.metrics.steps_used <= 5);
        }
        Err(ComptimeError::CpuLimitExceeded { .. }) => {
            // This is also acceptable if the execution exceeds the step limit
        }
        Err(e) => {
            panic!("Unexpected error: {:?}", e);
        }
    }
}
