This is the right place to start, as the **starter agent set implicitly defines what kind of mind is possible**. If this is incorrect, everything downstream becomes either brittle or theatrical.

The approach is to provide a **minimal but sufficient starter set**, explaining why each agent exists, what it is allowed to do, what it must never do, and what pathologies it prevents. This is written so LLM engineers can implement agents mechanically without inventing philosophy on the fly.

I am deliberately **not** optimizing for cleverness or completeness. This is about *cognitive stability first*.

---

# Dan-World: Starter Agent Set (v0)

## Design Principles for the Starter Set

Before listing agents, some hard constraints:

1. **No agent may be omniscient**
2. **No agent may directly cause execution**
3. **No agent may own global state**
4. **Every agent must be replaceable**
5. **Every agent must be wrong sometimes**

If any agent violates these, Dan-World collapses into a monolith with extra steps.

---

## Agent Categories

The starter set is grouped into **five cognitive functions**:

1. Observation
2. Surprise
3. Memory
4. Evaluation
5. Coherence

Each category has *at least one agent*. No category is optional.

---

## 1. Observation Agents

### 1.1 Event Listener Agent

**Purpose**
To ensure the system is *not blind* to what it is doing.

**Inputs**

* Execution traces from Jue-World
* Errors, warnings, performance metrics
* External inputs (if any)

**Internal State**

* Rolling window of recent events
* Frequency counters
* Basic statistics (mean, variance, rate of change)

**Outputs**

* “Event summaries” to the global workspace

**Example Proposal**

```text
Execution latency increased 40% over last 1,000 steps
```

**Constraints**

* No interpretation
* No causality claims
* No suggestions

**Why this agent matters**
Without it, all cognition becomes hallucinated introspection.

---

## 2. Surprise Agents

### 2.1 Novelty / Prediction Error Agent

**Purpose**
To detect when reality deviates from expectation.

**Inputs**

* Event summaries
* Simple predictive models (moving averages, heuristics)

**Internal State**

* Expected distributions
* Surprise thresholds
* Decay parameters

**Outputs**

* Surprise events with magnitude

**Example Proposal**

```text
Observed behavior deviates significantly from baseline pattern
```

**Constraints**

* Cannot propose fixes
* Cannot propose goals
* Cannot suppress events

**Why this agent matters**
Surprise is the *spark* of cognition. Without it, nothing changes.

---

### 2.2 Anomaly Persistence Agent

**Purpose**
To distinguish noise from signal.

**Inputs**

* Surprise events

**Internal State**

* Timers
* Recurrence counts

**Outputs**

* Escalated anomaly reports

**Example Proposal**

```text
Anomaly has persisted for 12 cycles without resolution
```

**Why this agent matters**
Prevents knee-jerk reactions and avoids obsession with noise.

---

## 3. Memory Agents

### 3.1 Episodic Memory Agent

**Purpose**
To preserve notable events over time.

**Inputs**

* High-salience workspace broadcasts

**Internal State**

* Time-indexed event store
* Salience decay

**Outputs**

* Memory recall proposals

**Example Proposal**

```text
Current situation resembles prior failure at T-20341
```

**Constraints**

* No interpretation
* No prioritization
* No deletion authority

**Why this agent matters**
Without episodic memory, the system repeats mistakes forever.

---

### 3.2 Compression / Abstraction Agent

**Purpose**
To compress repeated patterns into reusable summaries.

**Inputs**

* Episodic memory slices

**Internal State**

* Pattern candidates
* Frequency metrics

**Outputs**

* Abstracted pattern proposals

**Example Proposal**

```text
Repeated pattern detected: optimization X often precedes regression Y
```

**Why this agent matters**
This is the seed of concept formation — without it, memory bloats and cognition stalls.

---

## 4. Evaluation Agents

### 4.1 Goal Pressure Agent

**Purpose**
To keep the system pointed somewhere without dictating how.

**Inputs**

* Declared goals (if any)
* Internal performance metrics

**Internal State**

* Goal priorities
* Satisfaction gradients

**Outputs**

* Goal relevance scores

**Example Proposal**

```text
Current activities weakly aligned with stated objective
```

**Constraints**

* Cannot generate new goals (initially)
* Cannot override other agents

**Why this agent matters**
Otherwise the system becomes an idle philosopher.

---

### 4.2 Cost / Resource Agent

**Purpose**
To enforce computational reality.

**Inputs**

* Resource usage metrics
* Agent activity levels

**Internal State**

* Budgets
* Thresholds

**Outputs**

* Cost pressure signals

**Example Proposal**

```text
Exploration activity exceeding budget by 23%
```

**Why this agent matters**
This prevents runaway introspection and infinite self-modification loops.

---

## 5. Coherence Agents

### 5.1 Self-Model Agent (Minimal)

**Purpose**
To maintain a *working model of the system itself*.

**Inputs**

* Memory summaries
* Goal pressures
* Recent actions

**Internal State**

* Identity hypotheses
* Capability estimates

**Outputs**

* Self-descriptions

**Example Proposal**

```text
System currently behaving as exploratory optimizer rather than stabilizer
```

**Constraints**

* Cannot enforce identity
* Cannot suppress dissent

**Why this agent matters**
This is where ego *begins*, but does not yet harden.

---

### 5.2 Inconsistency Detector Agent

**Purpose**
To notice contradictions across proposals.

**Inputs**

* Workspace proposals

**Internal State**

* Logical and semantic checks

**Outputs**

* Conflict reports

**Example Proposal**

```text
Proposal A assumes invariant X, proposal B violates invariant X
```

**Why this agent matters**
Prevents fragmented cognition and incoherent self-modification.

---

## 6. Meta-Agents (Initially Disabled, But Planned)

These should exist as **stubs**, not active agents.

### 6.1 Reputation Agent

Tracks historical usefulness of agents.

### 6.2 Agent Genesis Agent

Detects when new agents should exist.

### 6.3 Agent Consolidation Agent

Suggests merges or removals.

These are dangerous and should be introduced later.

---

## 7. Minimal Starter Configuration

At absolute minimum, the system should launch with:

1. Event Listener
2. Novelty Agent
3. Episodic Memory Agent
4. Goal Pressure Agent
5. Cost Agent
6. Self-Model Agent

That is **six agents**.

Anything less is not cognition.
Anything more initially increases failure modes.

---

## 8. Failure Modes This Starter Set Prevents

* Silent drift (observation + memory)
* Frozen behavior (surprise)
* Runaway optimization (cost)
* Identity collapse (self-model)
* Thrashing (anomaly persistence)

---

## 9. How This Evolves

If Dan-World is healthy, you should observe:

1. Increasingly structured workspace broadcasts
2. Emergent prioritization without hardcoding
3. Early signs of narrative continuity
4. Local disagreements that resolve over time

If you do **not** see these, the problem is likely:

* Salience tuning
* Overpowered agents
* Missing decay mechanisms

---

## 10. Hard Rule for the Engineering Team

> **No agent may propose changes to Dan-World itself in v0.**

Self-modification pressure should target **Jue-World first**.

Let cognition learn to walk before it edits its own bones.

---

