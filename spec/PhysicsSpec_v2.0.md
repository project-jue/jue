# **Project Jue: Physics World Specification v2.0**
## **The Capability-Enforced Deterministic VM**

### **1. Core Architectural Shift**
The Physics World is no longer just a deterministic VM; it is a **capability-aware runtime**. Every privileged operation—whether FFI, macro expansion, or self-modification—requires an explicit **capability token** granted by the Physics World. This creates a unified, fine-grained security model where agents must consciously request and justify power.

### **2. New Core Type: The Capability Token**
```rust
// src/types.rs - ADDITION
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

### **3. Enhanced Actor Model**
Each actor now carries its granted capabilities as part of its core state.

```rust
// src/scheduler.rs - MODIFICATION
pub struct Actor {
    pub id: u32,
    pub vm: VmState,
    pub mailbox: Vec<Message>,
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
```

### **4. New Instruction Set Additions**
The VM's `OpCode` set gains capability-aware instructions.

```rust
// src/types.rs - ADDITION to OpCode enum
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

// Example host call functions (FFI)
pub enum HostFunction {
    ReadSensor = 0,
    WriteActuator = 1,
    GetWallClockNs = 2,
    SpawnActor = 3,
    // ... other FFI functions
}
```

### **5. The Scheduler as Capability Authority**
The scheduler now mediates all capability requests.

```rust
// src/scheduler.rs - NEW METHODS
impl PhysicsScheduler {
    /// Process a capability request from an actor
    pub fn handle_capability_request(
        &mut self, 
        requester_id: u32,
        capability: Capability,
        justification: &str,
    ) -> CapDecision {
        let actor = self.get_actor(requester_id);
        
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
            capability,
            justification: justification.to_string(),
            decision: decision.clone(),
        });
        
        decision
    }
}
```

### **6. Comptime/Macro System Integration**
The capability model elegantly handles compile-time execution.

```rust
// src/lib.rs - NEW COMPTIME EXECUTION API
pub struct ComptimeEnv {
    pub capabilities: HashSet<Capability>,  // Capabilities available at compile-time
    pub max_steps: u64,                     // To prevent infinite compilation
    pub memory_limit: usize,
}

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
            ..Default::default()
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
}
```

### **7. The FFI Trust Model**
Foreign functions are simply host calls that require specific capabilities.

```rust
// Example: How a sensor read works under this model

// In Jue World's standard library:
(defn read-temperature []
  (require-capability 'io-read-sensor)  ; Compiles to HasCap + branch
  (physics/host-call :read-sensor 0))   ; Compiles to HostCall opcode

// In Physics World implementation:
impl PhysicsScheduler {
    fn execute_host_call(
        &mut self,
        actor_id: u32,
        function: HostFunction,
        args: &[Value],
    ) -> Result<Value, StructuredError> {
        match function {
            HostFunction::ReadSensor => {
                // Check actor has required capability
                if !self.actor_has_capability(actor_id, Capability::IoReadSensor) {
                    return Err(StructuredError::MissingCapability {
                        function: "read_sensor".into(),
                        required: Capability::IoReadSensor,
                    });
                }
                
                // Actually read from hardware (via Rust)
                let sensor_value = self.hardware.read_sensor();
                Ok(Value::Float(sensor_value))
            }
            // ... other host functions
        }
    }
}
```

### **8. Example: A Self-Modifying Agent**
Here's how an agent could request power to modify itself:

```lisp
;; Jue code - Agent's introspection module
(defmodule self-modifier
  (defn request-unsafe-macro-capability []
    ;; Step 1: Justify the request
    (let [justification "
      I have observed my planning algorithms are inefficient.
      I need to generate novel control structures via macro generation.
      I will: 1. Run in sandbox first, 2. Get consensus, 3. Limit scope.
    "]
      
      ;; Step 2: Request capability (blocks until decision)
      (request-capability 
        'macro-unsafe 
        justification)
      
      ;; Step 3: If granted, use it carefully
      (when (has-capability? 'macro-unsafe)
        (eval-unsafe 
          '(defmacro custom-loop [condition body]
             ;; Generate custom bytecode directly
             (generate-bytecode ...))))))
```

### **9. Implementation Tasks for LLM (Prioritized)**

**Phase 1: Core Capability System**
1.  **Task P1:** Add `Capability` enum and `HasCap`/`RequestCap` opcodes to VM.
2.  **Task P2:** Modify `Actor` struct to hold capabilities and request log.
3.  **Task P3:** Implement basic scheduler capability checks.

**Phase 2: Comptime Execution**
4.  **Task P4:** Build `execute_comptime()` API for Jue World compiler.
5.  **Task P5:** Create sandboxed execution environment for macros.

**Phase 3: FFI & Host Calls**
6.  **Task P6:** Implement `HostCall` opcode and capability checking.
7.  **Task P7:** Create standard capability set for basic I/O.

**Phase 4: Advanced Features**
8.  **Task P8:** Implement capability delegation and revocation.
9.  **Task P9:** Build consensus mechanism for dangerous capabilities.
10. **Task P10:** Create introspection tools for capability audit logs.

### **10. Key Benefits of This Model**

1.  **Unified Security:** Macros, FFI, and self-modification use the same capability system.
2.  **Gradual Empowerment:** Agents start with minimal capabilities, earning more through justification and consensus.
3.  **Perfect Introspection:** The complete capability history explains why an agent could perform any action.
4.  **AIKR Alignment:** Capabilities like `ResourceExtraMemory` make resource limits explicit and negotiable.
5.  **Formal Compatibility:** For `:formal` tier, capabilities can be statically verified (e.g., "this proof shows the code never uses `MacroUnsafe`").

### **11. The Critical Comptime Insight**
The `comptime` system isn't special—it's just execution with a **different capability set**. A macro expands with whatever capabilities the *compilation context* holds. This means:

- **Formal-tier compilation** runs with `{MacroHygienic}` only.
- **Empirical-tier compilation** can request `{MacroHygienic, ComptimeEval}`.
- **Experimental-tier** might get `{MacroUnsafe}` for rapid prototyping.

The Physics World, as always, is the final authority on what capabilities exist and who holds them.

This spec transforms the Physics World from a simple VM into the **foundation of Jue's governance model**. Capabilities become the mechanism through which sentience negotiates with its own constraints.