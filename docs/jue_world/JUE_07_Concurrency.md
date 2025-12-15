

## **4. The Concurrency Model: Determinism vs. Parallelism**

**The Problem:**
- Physics-World is **deterministic single-threaded**
- Cognition needs **concurrent modules**
- Real time requires **responsiveness**

**Options:**

**A. Cooperative Multitasking:** Modules yield explicitly.
```jue
-- Like early event loops
module perception = {
  loop {
    events = await(sensors);
    broadcast(events);
    yield;  // Let other modules run
  }
}
```

**B. Software Transactional Memory:** Atomic blocks.
```jue
-- Concurrent updates with rollback
atomically {
  account1.balance -= amount;
  account2.balance += amount;
}
```

**C. Message Passing:** Isolated processes (like Erlang).
```jue
-- Modules as isolated processes
spawn module -> pid;
send(pid, message);
receive(pid) -> response;
```

**My Advice:** **Message passing with deterministic scheduler.**
1. Each Dan module runs as a separate "process"
2. Processes communicate via immutable messages
3. Scheduler is deterministic (round-robin)
4. Physics-World enforces fairness

```jue
-- Jue's concurrency primitives
spawn :: Code -> ProcessId
send :: ProcessId -> Message -> ()
receive :: Timeout -> Maybe Message

-- Implementation ensures:
-- 1. Deterministic message ordering
-- 2. No shared mutable state
-- 3. Process isolation
```
