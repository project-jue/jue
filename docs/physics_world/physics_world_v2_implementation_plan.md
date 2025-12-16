# Physics World V2 Capability System: Implementation Plan

## Table of Contents
1. [Executive Summary](#executive-summary)
2. [Current Implementation Analysis](#current-implementation-analysis)
3. [Detailed Implementation Plan](#detailed-implementation-plan)
4. [Implementation Phases](#implementation-phases)
5. [Open Questions and Assumptions](#open-questions-and-assumptions)
6. [Testing Strategy](#testing-strategy)
7. [Validation Criteria](#validation-criteria)

## Executive Summary

The Physics World V2 capability system represents a fundamental architectural shift from a simple deterministic VM to a **capability-enforced runtime**. This transformation introduces a unified security model where all privileged operations—FFI, macro expansion, self-modification, and resource access—require explicit capability tokens granted by the Physics World.

### Key Objectives
1. **Unified Security Model**: Create a single mechanism for managing all privileged operations
2. **Gradual Empowerment**: Enable agents to earn capabilities through justification and consensus
3. **Perfect Introspection**: Maintain complete audit trails of capability grants and usage
4. **AIKR Alignment**: Make resource limits explicit and negotiable through capabilities
5. **Formal Compatibility**: Support static verification of capability usage for formal-tier code

### Critical Success Factors
- Capability system must be operational before Jue-World can be updated
- All capability requests must flow through the scheduler
- The system must maintain deterministic execution while adding capability checks
- Backward compatibility with existing V1 code must be preserved during transition

## Current Implementation Analysis

### Existing Architecture
The current Physics World V1 implementation consists of:

1. **Core Components**:
   - `VmState`: Stack-based VM with instruction execution
   - `PhysicsScheduler`: Round-robin actor scheduler
   - `Actor`: Basic actor model with mailbox and VM state
   - `OpCode`: 24 basic operations (arithmetic, control flow, stack ops)

2. **Current Limitations**:
   - No capability system or security model
   - Actors have unrestricted access to all operations
   - No mechanism for privilege escalation or restriction
   - Limited introspection into actor behavior
   - No support for compile-time execution with different privilege levels

3. **Strengths to Preserve**:
   - Deterministic execution model
   - Clean separation between VM and scheduler
   - Resource limit enforcement (CPU, memory)
   - Message passing between actors

### Gap Analysis
| Feature             | V1 Status        | V2 Requirement              | Implementation Need      |
| ------------------- | ---------------- | --------------------------- | ------------------------ |
| Capability Tokens   | Not implemented  | Full capability enum        | New `Capability` type    |
| Capability Checks   | None             | Per-instruction checks      | New opcodes and VM logic |
| Actor Capabilities  | None             | Per-actor capability sets   | Actor struct extension   |
| Scheduler Authority | Basic scheduling | Capability mediation        | Scheduler enhancement    |
| Comptime Execution  | Not supported    | Sandboxed execution         | New execution API        |
| Host Calls (FFI)    | Not implemented  | Capability-gated FFI        | New host call system     |
| Audit Logging       | None             | Complete capability history | New audit system         |

## Detailed Implementation Plan

### 1. Core Types and Enums (`src/types.rs`)

#### 1.1 Capability Enum Addition
```rust
// Add to src/types.rs
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub enum Capability {
    // Meta-capabilities
    MetaSelfModify,     // Can modify own non-core code
    MetaGrant,          // Can grant capabilities to others (dangerous)
    
    // Macro & Compile-time capabilities
    MacroHygienic,      // Can expand hygienic macros
    MacroUnsafe,        // Can generate arbitrary syntax
    ComptimeEval,       // Can execute code at compile-time
    
    // I/O & External World
    IoReadSensor,       // Read from virtual sensors
    IoWriteActuator,    // Write to virtual actuators
    IoNetwork,          // Network access
    IoPersist,          // Write to persistent storage
    
    // System
    SysCreateActor,     // Can spawn new actors
    SysTerminateActor,  // Can terminate actors (including self)
    SysClock,           // Access non-deterministic time
    
    // Resource privileges
    ResourceExtraMemory(u64), // Additional memory quota
    ResourceExtraTime(u64),   // Additional time quota
}
```

#### 1.2 Capability-Related Opcodes
```rust
// Add to OpCode enum in src/types.rs
pub enum OpCode {
    // ... existing opcodes ...
    
    // --- CAPABILITY INSTRUCTIONS ---
    /// Check if actor has capability. Pushes bool to stack.
    /// Operand: index into constant pool where Capability is stored
    HasCap(usize),
    
    /// Request a capability from scheduler. Blocks until decision.
    /// Operand: capability index, justification string index
    RequestCap(usize, usize),
    
    /// Grant a capability to another actor.
    /// Requires MetaGrant capability.
    /// Operand: target actor ID, capability index
    GrantCap(u32, usize),
    
    /// Revoke a capability (from self or other with MetaGrant).
    /// Operand: target actor ID, capability index
    RevokeCap(u32, usize),
    
    /// Execute a privileged host call.
    /// Requires specific capability (encoded in constant pool).
    /// Format: HostCall { cap_index, function_id, arg_count }
    HostCall { cap_idx: usize, func_id: u16, args: u8 },
}
```

#### 1.3 Host Function Enum
```rust
// Add to src/types.rs
pub enum HostFunction {
    ReadSensor = 0,
    WriteActuator = 1,
    GetWallClockNs = 2,
    SpawnActor = 3,
    TerminateActor = 4,
    NetworkSend = 5,
    NetworkReceive = 6,
    PersistWrite = 7,
    PersistRead = 8,
}
```

### 2. Actor Model Enhancement (`src/scheduler.rs`)

#### 2.1 Actor Struct Extension
```rust
// Modify Actor struct in src/scheduler.rs
use std::collections::HashSet;
use crate::types::Capability;

pub struct Actor {
    pub id: u32,
    pub vm: VmState,
    pub mailbox: Vec<Value>,
    pub is_waiting: bool,
    
    // --- CAPABILITY FIELDS ---
    pub capabilities: HashSet<Capability>,      // Currently held capabilities
    pub capability_requests: Vec<CapRequest>,   // Pending requests (audit log)
    pub parent_id: Option<u32>,                 // Who granted existence?
}

pub struct CapRequest {
    pub capability: Capability,
    pub justification: String,      // In Jue-like DSL for introspection
    pub requested_at: u64,          // Global step count
    pub granted: Option<bool>,      // None = pending, Some(bool) = decision
}

impl Actor {
    pub fn has_cap(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }
    
    pub fn grant_cap(&mut self, capability: Capability) {
        self.capabilities.insert(capability);
    }
    
    pub fn revoke_cap(&mut self, capability: &Capability) {
        self.capabilities.remove(capability);
    }
}
```

#### 2.2 Capability Decision Types
```rust
// Add to src/scheduler.rs
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CapDecision {
    Granted,
    Denied(String), // Reason for denial
    Pending,       // Awaiting consensus/parent decision
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CapabilityAudit {
    pub step: u64,
    pub requester: u32,
    pub capability: Capability,
    pub justification: String,
    pub decision: CapDecision,
}
```

### 3. Scheduler as Capability Authority (`src/scheduler.rs`)

#### 3.1 Scheduler Enhancement
```rust
// Modify PhysicsScheduler struct in src/scheduler.rs
pub struct PhysicsScheduler {
    pub actors: Vec<Actor>,
    pub current_actor_index: usize,
    pub message_queues: HashMap<u32, Vec<Value>>,
    
    // --- CAPABILITY FIELDS ---
    pub global_step_count: u64,
    pub audit_log: Vec<CapabilityAudit>,
    pub memory_pool: u64,        // Available memory for resource caps
    pub time_pool: u64,          // Available time for resource caps
}
```

#### 3.2 Capability Request Handling
```rust
// Add to impl PhysicsScheduler in src/scheduler.rs
impl PhysicsScheduler {
    /// Process a capability request from an actor
    pub fn handle_capability_request(
        &mut self, 
        requester_id: u32,
        capability: Capability,
        justification: &str,
    ) -> CapDecision {
        let actor = self.get_actor_mut(requester_id);
        
        // Decision matrix (v1.0 simple rules)
        let decision = match capability {
            // Auto-grant to self if non-destructive
            Capability::MacroHygienic => CapDecision::Granted,
            
            // Requires MetaGrant capability to grant to others
            Capability::MetaGrant if actor.has_cap(&Capability::MetaGrant) => {
                CapDecision::Granted
            }
            
            // Dangerous capabilities require consensus
            Capability::SysTerminateActor |
            Capability::MacroUnsafe => {
                // Query other actors (simplified v1.0: 75% majority)
                let votes = self.take_vote(requester_id, &capability);
                if votes.approve >= votes.total * 3 / 4 {
                    CapDecision::Granted
                } else {
                    CapDecision::Denied("Insufficient consensus".into())
                }
            }
            
            // Resource capabilities check available pool
            Capability::ResourceExtraMemory(amount) => {
                if self.memory_pool >= amount {
                    self.memory_pool -= amount;
                    CapDecision::Granted
                } else {
                    CapDecision::Denied("Insufficient system memory".into())
                }
            }
            
            // Default: require parent approval
            _ => match actor.parent_id {
                Some(parent) => {
                    // Forward to parent actor for decision
                    self.forward_to_parent(parent, requester_id, capability)
                }
                None => CapDecision::Denied("No authority to grant".into()),
            }
        };
        
        // Log decision for introspection
        self.audit_log.push(CapabilityAudit {
            step: self.global_step_count,
            requester: requester_id,
            capability: capability.clone(),
            justification: justification.to_string(),
            decision: decision.clone(),
        });
        
        decision
    }
    
    fn get_actor_mut(&mut self, id: u32) -> &mut Actor {
        self.actors.iter_mut().find(|a| a.id == id)
            .expect("Actor not found")
    }
    
    fn take_vote(&mut self, requester_id: u32, capability: &Capability) -> VoteResult {
        let mut approve = 0;
        let mut total = 0;
        
        for actor in &self.actors {
            if actor.id != requester_id {
                total += 1;
                // Simple voting logic - in real implementation would be more sophisticated
                if actor.has_cap(&Capability::MetaGrant) {
                    approve += 1;
                }
            }
        }
        
        VoteResult { approve, total }
    }
    
    fn forward_to_parent(&mut self, parent_id: u32, requester_id: u32, capability: Capability) -> CapDecision {
        // Simplified - in real implementation would send message to parent
        CapDecision::Pending
    }
}

pub struct VoteResult {
    pub approve: u32,
    pub total: u32,
}
```

### 4. VM Capability Integration (`src/vm/state.rs`)

#### 4.1 Capability Instruction Implementation
```rust
// Add to the match statement in VmState::step() in src/vm/state.rs
// Add these cases to handle the new capability opcodes

OpCode::HasCap(cap_idx) => {
    // Get capability from constant pool
    let capability = match self.constant_pool.get(*cap_idx) {
        Some(Value::Capability(cap)) => cap.clone(),
        _ => return Err(VmError::InvalidHeapPtr),
    };
    
    // Check if current actor has this capability
    // Note: VM doesn't directly know about capabilities, this is handled by scheduler
    // For now, we'll push a placeholder that the scheduler will replace
    self.stack.push(Value::Bool(false)); // Placeholder
    self.ip += 1;
}

OpCode::RequestCap(cap_idx, justification_idx) => {
    // Get capability and justification from constant pool
    let capability = match self.constant_pool.get(*cap_idx) {
        Some(Value::Capability(cap)) => cap.clone(),
        _ => return Err(VmError::InvalidHeapPtr),
    };
    
    let justification = match self.constant_pool.get(*justification_idx) {
        Some(Value::String(just)) => just.clone(),
        _ => return Err(VmError::InvalidHeapPtr),
    };
    
    // Signal to scheduler that we're requesting a capability
    // This will cause the VM to yield and let scheduler handle the request
    return Ok(InstructionResult::RequestCapability {
        capability,
        justification,
    });
}

OpCode::GrantCap(target_actor_id, cap_idx) => {
    // Get capability from constant pool
    let capability = match self.constant_pool.get(*cap_idx) {
        Some(Value::Capability(cap)) => cap.clone(),
        _ => return Err(VmError::InvalidHeapPtr),
    };
    
    // Signal to scheduler to grant capability
    return Ok(InstructionResult::GrantCapability {
        target_actor: *target_actor_id,
        capability,
    });
}

OpCode::RevokeCap(target_actor_id, cap_idx) => {
    // Get capability from constant pool
    let capability = match self.constant_pool.get(*cap_idx) {
        Some(Value::Capability(cap)) => cap.clone(),
        _ => return Err(VmError::InvalidHeapPtr),
    };
    
    // Signal to scheduler to revoke capability
    return Ok(InstructionResult::RevokeCapability {
        target_actor: *target_actor_id,
        capability,
    });
}

OpCode::HostCall { cap_idx, func_id, args } => {
    // Get required capability from constant pool
    let required_cap = match self.constant_pool.get(*cap_idx) {
        Some(Value::Capability(cap)) => cap.clone(),
        _ => return Err(VmError::InvalidHeapPtr),
    };
    
    // Get arguments from stack
    if self.stack.len() < *args as usize {
        return Err(VmError::StackUnderflow);
    }
    
    let mut host_args = Vec::new();
    for _ in 0..*args {
        host_args.push(self.stack.pop().unwrap());
    }
    host_args.reverse(); // Reverse to get correct order
    
    // Signal to scheduler to execute host call
    return Ok(InstructionResult::HostCall {
        required_capability: required_cap,
        function_id: *func_id,
        arguments: host_args,
    });
}
```

#### 4.2 Instruction Result Extension
```rust
// Modify InstructionResult enum in src/vm/state.rs
pub enum InstructionResult {
    Continue,
    Yield,
    Finished(Value), // Final value on the stack
    
    // --- CAPABILITY RESULTS ---
    RequestCapability {
        capability: Capability,
        justification: String,
    },
    GrantCapability {
        target_actor: u32,
        capability: Capability,
    },
    RevokeCapability {
        target_actor: u32,
        capability: Capability,
    },
    HostCall {
        required_capability: Capability,
        function_id: u16,
        arguments: Vec<Value>,
    },
}
```

### 5. Comptime Execution API (`src/lib.rs`)

#### 5.1 Comptime Environment
```rust
// Add to src/lib.rs
use std::collections::HashSet;
use crate::types::Capability;

pub struct ComptimeEnv {
    pub capabilities: HashSet<Capability>,  // Capabilities available at compile-time
    pub max_steps: u64,                     // To prevent infinite compilation
    pub memory_limit: usize,
}

pub struct ComptimeResult {
    pub expanded_bytecode: Vec<OpCode>,
    pub computed_constants: Vec<Value>,
    pub proof_obligations: Vec<String>, // For formal tier
}

#[derive(Debug)]
pub enum ComptimeError {
    CapabilityDenied(Capability),
    StepLimitExceeded,
    MemoryLimitExceeded,
    CompilationError(String),
}
```

#### 5.2 Comptime Execution Implementation
```rust
// Add to impl PhysicsWorld in src/lib.rs
impl PhysicsWorld {
    /// Execute code at compile-time (for macro expansion)
    /// The Jue World compiler calls this
    pub fn execute_comptime(
        &mut self,
        bytecode: Vec<OpCode>,
        constants: Vec<Value>,
        env: ComptimeEnv,                    // Capabilities granted by compiler
    ) -> Result<ComptimeResult, ComptimeError> {
        // Create a temporary actor with COMPTIME capabilities only
        let mut temp_actor = Actor {
            id: COMPTIME_ACTOR_ID,
            vm: VmState::new(bytecode, constants, env.max_steps, env.memory_limit),
            capabilities: env.capabilities,
            capability_requests: Vec::new(),
            parent_id: None,
            mailbox: Vec::new(),
            is_waiting: false,
        };
        
        // Run with special comptime scheduler
        let result = self.run_comptime_actor(&mut temp_actor);
        
        // Comptime execution can't send messages or affect real actors
        // But it can return:
        // 1. Expanded code (new bytecode)
        // 2. Computed constants
        // 3. Proof obligations (for formal tier)
        result
    }
    
    fn run_comptime_actor(&mut self, actor: &mut Actor) -> Result<ComptimeResult, ComptimeError> {
        // Execute until completion or error
        loop {
            match actor.vm.step() {
                Ok(InstructionResult::Continue) => continue,
                Ok(InstructionResult::Finished(value)) => {
                    // Extract results from VM state
                    return Ok(ComptimeResult {
                        expanded_bytecode: extract_bytecode(&actor.vm),
                        computed_constants: extract_constants(&actor.vm),
                        proof_obligations: extract_proof_obligations(&actor.vm),
                    });
                }
                Ok(InstructionResult::RequestCapability { capability, .. }) => {
                    return Err(ComptimeError::CapabilityDenied(capability));
                }
                Err(vm_error) => {
                    return Err(ComptimeError::CompilationError(format!("{:?}", vm_error)));
                }
                _ => {
                    // Other instruction results shouldn't occur in comptime
                    return Err(ComptimeError::CompilationError("Invalid instruction in comptime".to_string()));
                }
            }
        }
    }
}

const COMPTIME_ACTOR_ID: u32 = 0xFFFFFFFF; // Special ID for comptime execution
```

### 6. Host Call Implementation (`src/host.rs` - New File)

#### 6.1 Host Call Module Creation
```rust
// Create new file: src/host.rs
use crate::types::{Capability, HostFunction, Value, VmError};
use crate::scheduler::StructuredError;

pub struct HostInterface {
    // Hardware interfaces would go here
    sensors: SensorManager,
    actuators: ActuatorManager,
    network: NetworkManager,
    storage: StorageManager,
}

impl HostInterface {
    pub fn new() -> Self {
        Self {
            sensors: SensorManager::new(),
            actuators: ActuatorManager::new(),
            network: NetworkManager::new(),
            storage: StorageManager::new(),
        }
    }
    
    pub fn execute_host_call(
        &mut self,
        function: HostFunction,
        args: &[Value],
    ) -> Result<Value, StructuredError> {
        match function {
            HostFunction::ReadSensor => {
                let sensor_id = args.get(0).ok_or(StructuredError::TypeMismatch)?;
                let sensor_value = self.sensors.read_sensor(sensor_id)?;
                Ok(Value::Float(sensor_value))
            }
            HostFunction::WriteActuator => {
                let actuator_id = args.get(0).ok_or(StructuredError::TypeMismatch)?;
                let value = args.get(1).ok_or(StructuredError::TypeMismatch)?;
                self.actuators.write_actuator(actuator_id, value)?;
                Ok(Value::Bool(true))
            }
            HostFunction::GetWallClockNs => {
                let time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos() as u64;
                Ok(Value::Int(time as i64))
            }
            HostFunction::SpawnActor => {
                // Implementation would create new actor
                Ok(Value::ActorId(0)) // Placeholder
            }
            HostFunction::TerminateActor => {
                // Implementation would terminate actor
                Ok(Value::Bool(true)) // Placeholder
            }
            HostFunction::NetworkSend => {
                let target = args.get(0).ok_or(StructuredError::TypeMismatch)?;
                let message = args.get(1).ok_or(StructuredError::TypeMismatch)?;
                self.network.send(target, message)?;
                Ok(Value::Bool(true))
            }
            HostFunction::NetworkReceive => {
                let message = self.network.receive()?;
                Ok(message)
            }
            HostFunction::PersistWrite => {
                let key = args.get(0).ok_or(StructuredError::TypeMismatch)?;
                let value = args.get(1).ok_or(StructuredError::TypeMismatch)?;
                self.storage.write(key, value)?;
                Ok(Value::Bool(true))
            }
            HostFunction::PersistRead => {
                let key = args.get(0).ok_or(StructuredError::TypeMismatch)?;
                let value = self.storage.read(key)?;
                Ok(value)
            }
        }
    }
}

// Placeholder implementations for hardware interfaces
pub struct SensorManager;
impl SensorManager {
    pub fn new() -> Self { Self }
    pub fn read_sensor(&self, _sensor_id: &Value) -> Result<f64, StructuredError> {
        Ok(42.0) // Placeholder
    }
}

pub struct ActuatorManager;
impl ActuatorManager {
    pub fn new() -> Self { Self }
    pub fn write_actuator(&mut self, _actuator_id: &Value, _value: &Value) -> Result<(), StructuredError> {
        Ok(()) // Placeholder
    }
}

pub struct NetworkManager;
impl NetworkManager {
    pub fn new() -> Self { Self }
    pub fn send(&mut self, _target: &Value, _message: &Value) -> Result<(), StructuredError> {
        Ok(()) // Placeholder
    }
    pub fn receive(&mut self) -> Result<Value, StructuredError> {
        Ok(Value::Nil) // Placeholder
    }
}

pub struct StorageManager;
impl StorageManager {
    pub fn new() -> Self { Self }
    pub fn write(&mut self, _key: &Value, _value: &Value) -> Result<(), StructuredError> {
        Ok(()) // Placeholder
    }
    pub fn read(&mut self, _key: &Value) -> Result<Value, StructuredError> {
        Ok(Value::Nil) // Placeholder
    }
}
```

#### 6.2 Module Integration
```rust
// Add to src/lib.rs
pub mod host;

// And use it in PhysicsWorld
use crate::host::HostInterface;

pub struct PhysicsWorld {
    scheduler: PhysicsScheduler,
    host_interface: HostInterface,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            scheduler: PhysicsScheduler::new(),
            host_interface: HostInterface::new(),
        }
    }
}
```

## Implementation Phases

### Phase 1: Core Capability System (Week 1-2)

#### Objectives
- Implement basic capability types and storage
- Add capability-aware opcodes to VM
- Extend actor model with capability fields
- Implement basic scheduler capability checks

#### Deliverables
1. **Enhanced Type System** (`src/types.rs`)
   - `Capability` enum with all basic capabilities
   - New capability-related opcodes
   - `HostFunction` enum for FFI

2. **Enhanced Actor Model** (`src/scheduler.rs`)
   - Actor struct with capability fields
   - `CapRequest` and `CapDecision` types
   - Basic capability checking methods

3. **Basic Scheduler Authority** (`src/scheduler.rs`)
   - Scheduler with capability audit log
   - Simple decision matrix for common capabilities
   - Memory and time pool management

4. **VM Integration** (`src/vm/state.rs`)
   - Instruction result extension for capability operations
   - Basic capability instruction handling
   - Error handling for capability violations

#### Validation Criteria
- All new types compile without errors
- Basic capability checking works in unit tests
- Scheduler can grant/deny simple capabilities
- VM can execute capability-related opcodes

### Phase 2: Comptime Execution (Week 3)

#### Objectives
- Implement sandboxed compile-time execution
- Create comptime environment with restricted capabilities
- Integrate with Jue-World compiler interface

#### Deliverables
1. **Comptime API** (`src/lib.rs`)
   - `ComptimeEnv` struct for compile-time capabilities
   - `execute_comptime()` method for sandboxed execution
   - `ComptimeResult` and `ComptimeError` types

2. **Comptime Execution Engine**
   - Specialized comptime actor execution
   - Capability enforcement during compilation
   - Result extraction (bytecode, constants, proofs)

#### Validation Criteria
- Comptime execution works with restricted capabilities
- Compiler can expand macros with appropriate capabilities
- Comptime errors are properly reported
- No side effects from comptime execution

### Phase 3: FFI and Host Calls (Week 4)

#### Objectives
- Implement capability-gated foreign function interface
- Create host call execution system
- Add hardware abstraction layer

#### Deliverables
1. **Host Interface Module** (`src/host.rs`)
   - `HostInterface` struct with hardware managers
   - Implementation of all host functions
   - Capability checking for each host call

2. **VM Host Call Integration**
   - `HostCall` opcode implementation
   - Argument validation and extraction
   - Result handling and error propagation

3. **Hardware Abstraction**
   - Sensor manager for input devices
   - Actuator manager for output devices
   - Network and storage managers

#### Validation Criteria
- Host calls require appropriate capabilities
- Hardware interfaces work correctly
- Errors are properly handled and reported
- No unauthorized access to system resources

### Phase 4: Advanced Features (Week 5-6)

#### Objectives
- Implement capability delegation and revocation
- Add consensus mechanism for dangerous capabilities
- Create introspection tools for capability audit

#### Deliverables
1. **Capability Delegation**
   - `GrantCap` and `RevokeCap` opcode implementations
   - Delegation rules and restrictions
   - Revocation propagation

2. **Consensus Mechanism**
   - Voting system for dangerous capabilities
   - Consensus calculation and enforcement
   - Timeout and fallback mechanisms

3. **Introspection Tools**
   - Audit log query interface
   - Capability history visualization
   - Debug tools for capability debugging

#### Validation Criteria
- Capabilities can be safely delegated and revoked
- Consensus mechanism works for dangerous operations
- Complete audit trail is maintained
- Introspection tools provide useful debugging information

### Phase 5: Integration and Testing (Week 7-8)

#### Objectives
- Integrate capability system with Jue-World
- Comprehensive testing of all components
- Performance optimization and bug fixes

#### Deliverables
1. **Jue-World Integration**
   - Updated compiler backend for capability generation
   - Runtime integration for capability requests
   - Standard library with capability-aware functions

2. **Comprehensive Test Suite**
   - Unit tests for all capability operations
   - Integration tests for cross-layer functionality
   - Performance benchmarks for capability checks

3. **Documentation and Examples**
   - Complete API documentation
   - Usage examples for common patterns
   - Migration guide from V1 to V2

#### Validation Criteria
- All tests pass without regressions
- Performance impact is minimal (<5% overhead)
- Documentation is complete and accurate
- Migration path is clear and tested

## Open Questions and Assumptions

### Open Questions

1. **Capability Serialization Format**
   - Question: How should capabilities be serialized in the constant pool?
   - Impact: Affects bytecode format and VM implementation
   - Decision needed: Before Phase 1 completion

2. **Consensus Algorithm Complexity**
   - Question: How sophisticated should the voting mechanism be?
   - Impact: Affects security vs. performance trade-off
   - Decision needed: Before Phase 4 implementation

3. **Hardware Abstraction Level**
   - Question: How much hardware detail should be exposed through host calls?
   - Impact: Affects portability and security
   - Decision needed: Before Phase 3 implementation

4. **Backward Compatibility Strategy**
   - Question: How should V1 code be handled in V2 system?
   - Impact: Affects migration complexity and user experience
   - Decision needed: Before Phase 1 completion

5. **Capability Granularity**
   - Question: Are the current capabilities too coarse or too fine-grained?
   - Impact: Affects usability and security
   - Decision needed: After Phase 1 testing

### Assumptions

1. **Performance Impact**
   - Assumption: Capability checks will add <5% performance overhead
   - Validation: Performance benchmarks in Phase 5

2. **Memory Usage**
   - Assumption: Capability storage will require <10% additional memory
   - Validation: Memory profiling in Phase 5

3. **Security Model**
   - Assumption: Current capability set covers all critical operations
   - Validation: Security review in Phase 4

4. **Developer Adoption**
   - Assumption: Developers will understand and use capability system correctly
   - Validation: User testing in Phase 5

5. **Determinism Preservation**
   - Assumption: Capability system won't break deterministic execution
   - Validation: Determinism tests in Phase 2

## Testing Strategy

### Unit Testing

#### 1. Capability Type Tests
```rust
// tests/capability_types.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_capability_serialization() {
        // Test that all capabilities can be serialized/deserialized
    }
    
    #[test]
    fn test_capability_equality() {
        // Test capability comparison and hashing
    }
    
    #[test]
    fn test_opcode_sizes() {
        // Test that new opcodes have correct sizes
    }
}
```

#### 2. Actor Model Tests
```rust
// tests/actor_capabilities.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_actor_capability_grant() {
        // Test granting capabilities to actors
    }
    
    #[test]
    fn test_actor_capability_revocation() {
        // Test revoking capabilities from actors
    }
    
    #[test]
    fn test_capability_inheritance() {
        // Test parent-child capability relationships
    }
}
```

#### 3. Scheduler Tests
```rust
// tests/scheduler_authority.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_capability_request_handling() {
        // Test scheduler processes requests correctly
    }
    
    #[test]
    fn test_consensus_mechanism() {
        // Test voting for dangerous capabilities
    }
    
    #[test]
    fn test_audit_logging() {
        // Test that all capability actions are logged
    }
}
```

### Integration Testing

#### 1. End-to-End Capability Flow
```rust
// tests/integration/capability_flow.rs
#[test]
fn test_complete_capability_request_flow() {
    // Test: Actor requests capability → scheduler processes → decision made
    // Verify: Audit log updated, capability granted/denied appropriately
}

#[test]
fn test_host_call_with_capabilities() {
    // Test: Host call requires capability → check → execute/deny
    // Verify: Only authorized actors can access hardware
}

#[test]
fn test_comptime_execution() {
    // Test: Compile-time execution with restricted capabilities
    // Verify: No side effects, proper capability enforcement
}
```

#### 2. Cross-Layer Integration
```rust
// tests/integration/jue_world_integration.rs
#[test]
fn test_jue_compiler_capability_generation() {
    // Test: Jue compiler generates appropriate capability requests
}

#[test]
fn test_dan_world_capability_usage() {
    // Test: Dan-World agents can request and use capabilities
}
```

### Performance Testing

#### 1. Capability Check Overhead
```rust
// tests/performance/capability_overhead.rs
#[test]
fn benchmark_capability_checks() {
    // Measure overhead of capability checking vs. no checks
}

#[test]
fn benchmark_audit_log_performance() {
    // Measure impact of audit logging on performance
}
```

#### 2. Memory Usage
```rust
// tests/performance/memory_usage.rs
#[test]
fn benchmark_capability_memory_usage() {
    // Measure additional memory usage for capability system
}
```

### Property-Based Testing

#### 1. Capability System Properties
```rust
// tests/properties/capability_properties.rs
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_capability_monotonicity(
        // Property: Once granted, capabilities aren't automatically revoked
    ) {
        // Test implementation
    }
    
    #[test]
    fn test_audit_log_completeness(
        // Property: Every capability action is logged
    ) {
        // Test implementation
    }
}
```

## Validation Criteria

### Phase 1 Validation

#### Functional Criteria
- [ ] All capability types compile and serialize correctly
- [ ] Actors can store and retrieve capabilities
- [ ] Scheduler can process basic capability requests
- [ ] VM can execute capability-related opcodes
- [ ] Audit log records all capability actions

#### Performance Criteria
- [ ] Capability checks add <2% performance overhead
- [ ] Memory usage increases by <5%
- [ ] No regressions in existing functionality

#### Security Criteria
- [ ] Actors cannot access capabilities without explicit grants
- [ ] Capability requests are properly validated
- [ ] Audit log cannot be tampered with

### Phase 2 Validation

#### Functional Criteria
- [ ] Comptime execution works with restricted capabilities
- [ ] Compiler can expand macros with appropriate capabilities
- [ ] Comptime errors are properly reported
- [ ] No side effects from comptime execution

#### Performance Criteria
- [ ] Comptime execution adds <10% compilation time
- [ ] Memory usage during comptime is bounded

#### Security Criteria
- [ ] Comptime execution cannot affect runtime state
- [ ] Capability restrictions are enforced during compilation

### Phase 3 Validation

#### Functional Criteria
- [ ] All host functions require appropriate capabilities
- [ ] Hardware interfaces work correctly
- [ ] Errors are properly handled and reported
- [ ] No unauthorized access to system resources

#### Performance Criteria
- [ ] Host call overhead is minimal
- [ ] Hardware abstraction doesn't add significant latency

#### Security Criteria
- [ ] Host calls cannot bypass capability checks
- [ ] Hardware access is properly restricted

### Phase 4 Validation

#### Functional Criteria
- [ ] Capabilities can be safely delegated and revoked
- [ ] Consensus mechanism works for dangerous operations
- [ ] Complete audit trail is maintained
- [ ] Introspection tools provide useful debugging information

#### Performance Criteria
- [ ] Consensus mechanism completes in reasonable time
- [ ] Audit log queries are efficient

#### Security Criteria
- [ ] Delegation follows proper rules
- [ ] Consensus cannot be easily manipulated
- [ ] Revocation propagates correctly

### Phase 5 Validation

#### Functional Criteria
- [ ] Jue-World integration works seamlessly
- [ ] All tests pass without regressions
- [ ] Documentation is complete and accurate
- [ ] Migration path is clear and tested

#### Performance Criteria
- [ ] Overall system performance impact is <5%
- [ ] Memory usage is within acceptable limits
- [ ] No performance regressions in existing code

#### Security Criteria
- [ ] No security vulnerabilities introduced
- [ ] All capability paths are properly tested
- [ ] Security review passes

## Conclusion

This implementation plan provides a comprehensive roadmap for transforming the Physics World from a simple deterministic VM into a capability-enforced runtime. The phased approach ensures that each component is thoroughly tested before integration, while the validation criteria provide clear checkpoints for measuring progress.

The capability system represents a fundamental shift in how the Physics World manages security and privilege, creating a unified model that can handle everything from macro expansion to hardware access. By following this plan, we can ensure that the implementation is robust, secure, and performant while maintaining the deterministic guarantees that are core to the Physics World's design.

The success of this implementation will enable the broader V2 architecture to function as intended, providing the foundation for safe self-modification, formal verification, and emergent cognition in the Dan-World layer.