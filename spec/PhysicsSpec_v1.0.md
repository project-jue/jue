# **Project Jue: Physics-World Specification v1.0**

## **1. Overview & Guarantees**

The Physics-World is a **deterministic, introspectable, single-threaded virtual machine** that provides the computational substrate for Dan-World actors. It enforces the Assumption of Insufficient Knowledge and Resources (AIKR) through precise resource limits.

**Core Guarantees:**
*   **Determinism:** Identical initial `VmState` and input message queue produce identical final state and output.
*   **Introspection:** The entire `VmState` is serializable at any instruction boundary for time-travel debugging.
*   **Isolation:** Actors cannot share mutable state; communication is only via immutable message copying.
*   **Enforcement:** All resource limits (computation, memory) are enforced, resulting in structured errors, not undefined behavior.

## **2. Data Model**

### **2.1. The `Value` Enum**
The fundamental unit of data, designed for efficient storage and deterministic behavior.

```rust
#[derive(Clone, Debug, Serialize)]
pub enum Value {
    /// Represents nil/empty.
    Nil,
    /// Boolean primitive.
    Bool(bool),
    /// 64-bit signed integer. Primary numeric type for deterministic math.
    Int(i64),
    /// Symbol (interned string). Index into the constant table.
    Symbol(usize),
    /// Cons pair (heap-allocated).
    Pair(HeapPtr),
    /// Closure (captured code and environment).
    Closure(HeapPtr),
    /// Unique identifier for an actor.
    ActorId(usize),
}
```

### **2.2. The `HeapPtr` and Object Model**
A `HeapPtr` is an index into a contiguous `ObjectArena`. It points to a header followed by data.

```rust
pub struct HeapPtr(usize); // Index into the arena's vector.

#[repr(u8)]
pub enum ObjectType { Pair, Closure, // ... future types
}

struct ObjectHeader {
    type_tag: ObjectType,
    is_marked: bool, // For garbage collection (future use).
    size: usize,     // Total size of this object (header + data).
}

// A Pair stores two Values.
struct PairData {
    car: Value,
    cdr: Value,
}

// A Closure stores a reference to its code and its captured environment.
struct ClosureData {
    code_index: usize, // Index into the bytecode's constant pool for the function.
    env: Vec<Value>,   // Captured values at the time of closure creation.
}
```

## **3. Instruction Set (`OpCode`)**

A stack-based bytecode. Each variant's comment defines its precise semantic effect.

```rust
#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    // === Constants & Values ===
    /// Push `Nil` onto the stack.
    Nil,
    /// Push boolean `b` onto the stack.
    Bool(bool),
    /// Push integer `i` onto the stack.
    Int(i64),
    /// Push the Symbol at constant pool index `idx` onto the stack.
    Sym(usize),

    // === Stack Manipulation ===
    /// Duplicate the value at stack[-1].
    Dup,
    /// Pop the top value from the stack.
    Pop,
    /// Swap the two topmost values on the stack.
    Swap,

    // === Heap Operations ===
    /// Pop two values (cdr, car). Allocate a Pair, push `HeapPtr`.
    Cons,
    /// Pop a `HeapPtr` to a Pair, push its `car` value.
    Car,
    /// Pop a `HeapPtr` to a Pair, push its `cdr` value.
    Cdr,

    // === Closure Operations ===
    /// Create a closure. `code_idx` is an index into the constant pool for the function's bytecode.
    /// Pops `n` values from the stack to form the captured environment.
    /// Pushes a `Closure(HeapPtr)`.
    MakeClosure(usize /* code_idx */, usize /* capture_count */),
    /// Call a closure. Pops `arg_count` arguments, then the closure.
    /// Sets up a new call frame and jumps. Pushes the result.
    Call(usize /* arg_count */),
    /// Return from the current call. Pops the return value, restores the previous frame.
    Ret,

    // === Control Flow ===
    /// Unconditional jump to `ip` offset.
    Jmp(i32),
    /// Pop a boolean. If false, jump to `ip` offset.
    JmpIfFalse(i32),

    // === Actor & Resource Operations ===
    /// Yield execution back to the scheduler. The actor can be resumed.
    Yield,
    /// Check resource limits. Throws `CpuLimitExceeded` if `step_counter > step_limit`.
    CheckStepLimit,
    /// Send a message. Pops the message (value) and target `ActorId`.
    Send,
}
```

## **4. Execution Engine & VM State**

### **4.1. The `VmState` Structure**
Encapsulates the complete state of a single actor's execution.

```rust
pub struct VmState {
    // --- Execution Context ---
    pub instructions: Vec<OpCode>,
    pub ip: usize, // Instruction Pointer
    pub stack: Vec<Value>,
    pub constants: Vec<Value>, // Constant pool (symbols, etc.)

    // --- Call Frame ---
    pub frames: Vec<CallFrame>, // For `Call`/`Ret`

    // --- Memory ---
    pub heap: ObjectArena, // Manages `HeapPtr` allocation

    // --- Resources & AIKR ---
    pub step_counter: u64,
    pub step_limit: u64,
    pub memory_limit: usize,

    // --- Communication ---
    pub mailbox: Vec<Value>, // Incoming messages for *this* actor
}
```

### **4.2. The Interpreter Loop**
The core deterministic engine.

```rust
impl VmState {
    pub fn run(&mut self) -> Result<Value, VmError> {
        loop {
            self.step_counter += 1;

            // 1. AIKR ENFORCEMENT: Check before fetching the instruction.
            if self.step_counter > self.step_limit {
                return Err(VmError::CpuLimitExceeded {
                    limit: self.step_limit,
                    attempted: self.step_counter,
                });
            }
            if self.heap.used() > self.memory_limit {
                return Err(VmError::MemoryLimitExceeded {
                    limit: self.memory_limit,
                    attempted: self.heap.used(),
                });
            }

            // 2. INSTRUCTION FETCH & DISPATCH
            let op = self.instructions[self.ip];
            match op {
                OpCode::Int(i) => self.stack.push(Value::Int(i)),
                OpCode::Cons => self.op_cons()?,
                OpCode::Call(arg_count) => self.op_call(arg_count)?,
                OpCode::Yield => return Ok(Value::YieldFlag),
                // ... handle all OpCode variants
                _ => return Err(VmError::UnknownOpCode),
            }
            self.ip += 1; // Default IP increment, modified by jumps.
        }
    }
}
```

## **5. Memory Management: The `ObjectArena`**

A simple, deterministic arena allocator. For v1.0, we use a **"per-thought" arena** that is wiped clean when an actor finishes a cognitive cycle.

```rust
pub struct ObjectArena {
    // Single contiguous allocation. A `HeapPtr` is an index into this.
    memory: Vec<u8>,
    // Bump pointer.
    next_free: usize,
}

impl ObjectArena {
    /// Allocate space for an object. Returns a `HeapPtr`.
    pub fn allocate(&mut self, header: ObjectHeader, data: &[u8]) -> Result<HeapPtr, VmError> {
        let total_size = std::mem::size_of::<ObjectHeader>() + data.len();
        if self.next_free + total_size > self.memory.len() {
            return Err(VmError::ArenaFull);
        }
        let ptr = HeapPtr(self.next_free);
        // Write header, then copy data.
        self.next_free += total_size;
        Ok(ptr)
    }

    /// Reset the arena. O(1). Called by the scheduler after an actor yields or finishes.
    pub fn reset(&mut self) {
        self.next_free = 0;
    }

    /// Given a `HeapPtr`, return a reference to its `ObjectHeader`.
    pub fn get_header(&self, ptr: HeapPtr) -> &ObjectHeader { /* ... */ }

    /// Given a `HeapPtr` and its expected type, return a reference to its data.
    pub fn get_data<T>(&self, ptr: HeapPtr) -> &T { /* ... */ }
}
```

## **6. Concurrency & The Scheduler**

The Physics-World runs a **single OS thread**. Concurrency is cooperative via the `Yield` opcode.

```rust
pub struct Actor {
    pub id: usize,
    pub vm_state: VmState,
    pub is_waiting: bool,
}

pub struct PhysicsScheduler {
    pub actors: Vec<Actor>,
    pub current_actor_index: usize,
    pub global_message_queues: HashMap<usize, Vec<Value>>, // Map from ActorId to mailbox
}

impl PhysicsScheduler {
    /// The main tick. Runs one actor until it yields or hits a limit.
    pub fn tick(&mut self) -> Result<(), PhysicsWorldError> {
        let actor = &mut self.actors[self.current_actor_index];
        // 1. Load any pending messages into the actor's VmState.mailbox.
        // 2. Execute:
        let result = actor.vm_state.run();
        // 3. Handle result:
        match result {
            Ok(Value::YieldFlag) => { /* Actor cooperatively yielded */ }
            Ok(_) => { /* Actor finished computation; store result */ }
            Err(VmError::CpuLimitExceeded { .. }) => { /* Enforced AIKR; schedule next */ }
            Err(e) => { /* Other error */ }
        }
        // 4. Round-robin to next actor.
        self.current_actor_index = (self.current_actor_index + 1) % self.actors.len();
        Ok(())
    }
}
```

## **7. Physics-World External API**

The clean interface that Jue-World will call. This is the **only public API** of the Physics-World.

```rust
// ============ PUBLIC API ============
pub struct PhysicsWorld {
    scheduler: PhysicsScheduler,
}

#[derive(Debug, Serialize)]
pub enum StructuredError {
    CpuLimitExceeded { limit: u64, attempted: u64 },
    MemoryLimitExceeded { limit: usize, attempted: usize },
    IllegalOperation { opcode: String },
    ActorNotFound(usize),
}

#[derive(Debug, Serialize)]
pub struct ExecutionResult {
    pub output: Option<Value>,              // The final result on the actor's stack.
    pub messages_sent: Vec<(usize, Value)>, // List of (target_actor_id, message).
    pub error: Option<StructuredError>,     // If execution failed.
    pub final_state_serialized: Vec<u8>,    // Snapshot of VmState for introspection.
    pub metrics: ResourceMetrics,           // Steps taken, memory used, etc.
}

impl PhysicsWorld {
    /// Primary API: Execute an actor's code with strict limits.
    pub fn execute_actor(
        &mut self,
        actor_id: usize,
        bytecode: Vec<OpCode>,
        constants: Vec<Value>,
        step_limit: u64,
        memory_limit: usize,
    ) -> ExecutionResult {
        // 1. Locate or create the actor's `VmState`.
        // 2. Configure it with limits and load bytecode.
        // 3. Run the scheduler for this actor until it yields/finishes/errors.
        // 4. Package the result into `ExecutionResult`.
    }

    /// Deliver messages to an actor's mailbox (called by Jue-World between executions).
    pub fn deliver_messages(&mut self, actor_id: usize, messages: Vec<Value>) -> Result<(), StructuredError> { /* ... */ }

    /// Get a serializable snapshot of an actor's state (for debugging/rollback).
    pub fn snapshot_actor_state(&self, actor_id: usize) -> Result<Vec<u8>, StructuredError> { /* ... */ }
}
```

---

### **Next Steps for LLM-Assisted Implementation**

With this spec, you can break implementation into parallel, testable units. I suggest this order:

1.  **Unit 1: Data Model & Arena (`value.rs`, `arena.rs`)**
    *   Implement the `Value` enum, `HeapPtr`, `ObjectHeader`, `ObjectArena`.
    *   Write tests: allocate a `Pair`, read it back.

2.  **Unit 2: Core Interpreter (`opcode.rs`, `vm_state.rs`)**
    *   Implement the `OpCode` enum and the `VmState` struct (skeleton).
    *   Implement the `match` arm for 5 core instructions (e.g., `Int`, `Cons`, `Car`, `Cdr`, `Add`).

3.  **Unit 3: Control Flow & Calls (`frame.rs`, `call.rs`)**
    *   Implement `CallFrame`, then the `Call` and `Ret` instructions. This is the most complex part.

4.  **Unit 4: Scheduler & API (`scheduler.rs`, `api.rs`)**
    *   Implement the round-robin `PhysicsScheduler` and the public `PhysicsWorld` API.

5.  **Unit 5: Integration & Testing**
    *   Connect Jue-World's compiler to generate this bytecode.
    *   Write an end-to-end test: a Jue program `(+ 1 2)` compiles, runs in the VM, and returns `3`.

This spec provides the **what** and the **why**. An LLM can now generate the **how**—the precise Rust code for each `match` arm, allocation routine, and state transition. The path is clear.