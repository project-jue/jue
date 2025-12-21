use crate::scheduler::{CapDecision, PhysicsScheduler};
/// Capability enforcement system for the Physics World VM
///
/// This module provides the core capability checking and enforcement logic
/// that integrates with the scheduler to provide a complete capability system.
use crate::types::{Capability, Value};
use crate::vm::error::VmError;
use crate::vm::state::VmState;

/// Capability enforcement system that integrates with the scheduler
pub struct CapabilityEnforcer;

impl CapabilityEnforcer {
    /// Check if an actor has a specific capability
    pub fn actor_has_capability(
        scheduler: &PhysicsScheduler,
        actor_id: u32,
        capability: &Capability,
    ) -> bool {
        scheduler.actor_has_capability(actor_id, capability)
    }

    /// Request a capability on behalf of an actor
    pub fn request_capability(
        scheduler: &mut PhysicsScheduler,
        actor_id: u32,
        capability: Capability,
        justification: &str,
    ) -> CapDecision {
        scheduler.handle_capability_request(actor_id, capability, justification)
    }

    /// Grant a capability to an actor
    pub fn grant_capability(
        scheduler: &mut PhysicsScheduler,
        requester_id: u32,
        target_id: u32,
        capability: Capability,
    ) -> Result<(), VmError> {
        // Check if requester has MetaGrant capability
        if !Self::actor_has_capability(scheduler, requester_id, &Capability::MetaGrant) {
            return Err(VmError::CapabilityDenied);
        }

        scheduler
            .grant_capability(target_id, capability)
            .map_err(|_| VmError::CapabilityDenied)
    }

    /// Revoke a capability from an actor
    pub fn revoke_capability(
        scheduler: &mut PhysicsScheduler,
        requester_id: u32,
        target_id: u32,
        capability: &Capability,
    ) -> Result<(), VmError> {
        // Check if requester has MetaGrant capability or is revoking from self
        if requester_id != target_id
            && !Self::actor_has_capability(scheduler, requester_id, &Capability::MetaGrant)
        {
            return Err(VmError::CapabilityDenied);
        }

        scheduler
            .revoke_capability(target_id, capability)
            .map_err(|_| VmError::CapabilityDenied)
    }

    /// Check if an actor can perform a host call with the required capability
    pub fn check_host_call_permission(
        scheduler: &PhysicsScheduler,
        actor_id: u32,
        required_capability: &Capability,
    ) -> Result<(), VmError> {
        if Self::actor_has_capability(scheduler, actor_id, required_capability) {
            Ok(())
        } else {
            Err(VmError::CapabilityDenied)
        }
    }

    /// Get the capability required for a specific host function
    pub fn get_required_capability_for_host_function(func_id: u16) -> Option<Capability> {
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
            _ => None,
        }
    }
}

/// Host function execution system
pub struct HostFunctionExecutor;

impl HostFunctionExecutor {
    /// Execute a host function call
    pub fn execute_host_function(
        func_id: u16,
        args: Vec<Value>,
        scheduler: &mut PhysicsScheduler,
        actor_id: u32,
    ) -> Result<Value, VmError> {
        // Check if the actor has the required capability
        if let Some(required_cap) =
            CapabilityEnforcer::get_required_capability_for_host_function(func_id)
        {
            CapabilityEnforcer::check_host_call_permission(scheduler, actor_id, &required_cap)?;
        }

        // Execute the host function
        match func_id {
            0 => Self::read_sensor(args),
            1 => Self::write_actuator(args),
            2 => Self::get_wall_clock_ns(args),
            3 => Self::spawn_actor(args, scheduler),
            4 => Self::terminate_actor(args, scheduler, actor_id),
            5 => Self::network_send(args),
            6 => Self::network_receive(args),
            7 => Self::persist_write(args),
            8 => Self::persist_read(args),
            _ => Err(VmError::UnknownOpCode),
        }
    }

    fn read_sensor(_args: Vec<Value>) -> Result<Value, VmError> {
        // In a real implementation, this would read from virtual sensors
        Ok(Value::Int(0))
    }

    fn write_actuator(_args: Vec<Value>) -> Result<Value, VmError> {
        // In a real implementation, this would write to virtual actuators
        Ok(Value::Nil)
    }

    fn get_wall_clock_ns(_args: Vec<Value>) -> Result<Value, VmError> {
        // In a real implementation, this would get the current time
        Ok(Value::Int(0))
    }

    fn spawn_actor(_args: Vec<Value>, _scheduler: &mut PhysicsScheduler) -> Result<Value, VmError> {
        // In a real implementation, this would spawn a new actor
        Ok(Value::ActorId(0))
    }

    fn terminate_actor(
        args: Vec<Value>,
        scheduler: &mut PhysicsScheduler,
        requester_id: u32,
    ) -> Result<Value, VmError> {
        if let Some(Value::ActorId(target_id)) = args.first() {
            // Check if requester can terminate the target actor
            // In a real implementation, this would have more sophisticated logic
            if *target_id == requester_id {
                // Allow self-termination
                Ok(Value::Nil)
            } else {
                // Check if requester has the capability to terminate others
                if CapabilityEnforcer::actor_has_capability(
                    scheduler,
                    requester_id,
                    &Capability::SysTerminateActor,
                ) {
                    Ok(Value::Nil)
                } else {
                    Err(VmError::CapabilityDenied)
                }
            }
        } else {
            Err(VmError::TypeMismatch)
        }
    }

    fn network_send(_args: Vec<Value>) -> Result<Value, VmError> {
        // In a real implementation, this would send network data
        Ok(Value::Nil)
    }

    fn network_receive(_args: Vec<Value>) -> Result<Value, VmError> {
        // In a real implementation, this would receive network data
        Ok(Value::Nil)
    }

    fn persist_write(_args: Vec<Value>) -> Result<Value, VmError> {
        // In a real implementation, this would write to persistent storage
        Ok(Value::Nil)
    }

    fn persist_read(_args: Vec<Value>) -> Result<Value, VmError> {
        // In a real implementation, this would read from persistent storage
        Ok(Value::Nil)
    }
}
