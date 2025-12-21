/// Actor management for the Physics World scheduler
use crate::types::Value;
use crate::vm::state::VmState;
use std::collections::HashSet;

/// Represents a capability request from an actor
#[derive(Debug, Clone)]
pub struct CapRequest {
    pub capability: crate::types::Capability,
    pub justification: String,
    pub requested_at: u64,
    pub granted: Option<bool>,
}

/// Represents an actor in the Physics World.
pub struct Actor {
    pub id: u32,
    pub vm: VmState,
    pub mailbox: Vec<Value>, // Incoming messages
    pub is_waiting: bool,
    // V2 Capability System - Added capability state
    pub capabilities: HashSet<crate::types::Capability>,
    pub capability_requests: Vec<CapRequest>,
    pub parent_id: Option<u32>,
    // V2 Priority Scheduling - Added priority fields
    pub priority: u8,                // 0-255 range, higher = more important
    pub priority_boost: Option<u32>, // Temporary priority boost (step count)
}
