
# Dan-World Starter Agent Implementation Guidelines

## 1. Salience Scoring Algorithm

Every proposal from an agent is assigned a **salience score** which determines its visibility to the global workspace.

### Inputs

* `magnitude`: How unusual the event is compared to baseline (surprise agent)
* `recency`: How recent the event occurred
* `repetition`: Has this event been seen before?
* `cost`: Computational/temporal cost of acting on the proposal

### Score Calculation

```text
salience = α * normalized_magnitude
         + β * recency_weight
         + γ * novelty_weight
         - δ * cost_weight
```

Where:

* `α, β, γ, δ` are tunable parameters (initial default: 1, 0.8, 0.5, 0.3)
* `normalized_magnitude` ∈ [0,1]
* `recency_weight` = e^(-Δt / τ)
* `novelty_weight` = 1 if new pattern, 0 otherwise
* `cost_weight` ∈ [0,1]

### Decay Mechanism

* Each proposal has a **decaying score over time**:
  `salience(t+1) = salience(t) * decay_factor`
* `decay_factor` ∈ [0.8, 0.99] depending on category of agent.

---

## 2. Reference Agent Template (LLM-Friendly)

### Agent Interface

```lisp
(define-agent
  (name "Novelty Detector")
  (category 'surprise)
  (state (expected-patterns '()))
  (input ['workspace-events])
  (output ['surprise-proposals])
  (proposal-function
    (lambda ()
      ;; Analyze events
      (let ((deviation (compute-deviation expected-patterns workspace-events)))
        (if (> deviation threshold)
            (list (make-proposal deviation))
            '()))))
  (update-function
    (lambda (accepted-proposals)
      ;; Update internal state
      (update-expected-patterns accepted-proposals))))
```

### Required Functions for All Agents

1. `proposal-function`: Generates proposals for global workspace
2. `update-function`: Updates internal state after proposals are processed
3. `salience-function`: Returns numeric salience (used by workspace)
4. `serialize-state`: Optional, for snapshots or handover
5. `validate`: Optional, checks internal consistency

### Key Constraints

* Agents may **not mutate other agents**
* Agents may **not mutate global state directly**
* Proposals must be **annotated with a type and salience score**
* Must handle empty input gracefully

---

## 3. Bad Agent Example

A **“bad agent”** is an intentional test to validate governance mechanisms.

### Example: Overconfident Self-Model Agent

```lisp
(define-agent
  (name "Overconfident Ego")
  (category 'coherence)
  (state (confidence 1.0))
  (input ['workspace-events])
  (output ['self-descriptions])
  (proposal-function
    (lambda ()
      ;; Always outputs max-confidence self-description
      (list (make-proposal "I am always correct!" :salience 1.0))))
  (update-function
    (lambda (accepted-proposals)
      ;; Ignores feedback
      state)))
```

**Purpose**: Ensures global workspace **can suppress agents** whose proposals violate constraints.

* The system should:

  1. Lower salience via decay or conflict
  2. Log rejection
  3. Continue processing other agents

---

## 4. Workspace Handling of Proposals

1. **Collect all proposals** from active agents
2. **Compute salience score** per proposal
3. **Filter proposals** below `threshold`
4. **Broadcast accepted proposals** to all agents
5. **Update agent states** via `update-function`

```lisp
(define (global-workspace-step)
  (let ((all-proposals
         (flatten (map (lambda (a) (proposal-function a)) agents))))
    (let ((accepted
           (filter (lambda (p) (> (salience p) threshold))
                   all-proposals)))
      ;; Broadcast
      (for-each (lambda (a) (update-function a accepted)) agents))))
```

---

## 5. Starter Agent Set Configuration (Concrete)

| Agent Name             | Category    | Input                | Output                   | Salience Parameters |
| ---------------------- | ----------- | -------------------- | ------------------------ | ------------------- |
| Event Listener         | Observation | Execution traces     | Event summaries          | α=1, β=0.8          |
| Novelty Detector       | Surprise    | Event summaries      | Surprise proposals       | α=1, β=0.8, γ=0.5   |
| Anomaly Persistence    | Surprise    | Surprise proposals   | Escalated anomalies      | α=0.8, β=0.8, γ=0.5 |
| Episodic Memory        | Memory      | High-salience events | Memory recall            | α=0.5, β=0.9        |
| Pattern Compression    | Memory      | Episodic memory      | Abstracted patterns      | α=0.7, β=0.8        |
| Goal Pressure          | Evaluation  | Performance metrics  | Goal alignment proposals | α=0.6, β=0.7        |
| Cost / Resource        | Evaluation  | Resource metrics     | Cost warnings            | α=0.9, β=0.8        |
| Self-Model (minimal)   | Coherence   | Workspace proposals  | Self-descriptions        | α=0.8, β=0.9        |
| Inconsistency Detector | Coherence   | Workspace proposals  | Conflict reports         | α=0.9, β=0.9        |

> Meta-agents remain disabled at startup.

---

## 6. Next Steps for the LLM Engineering Team

1. Implement **salience scoring and decay**
2. Implement **agent interface** with proposal/update functions
3. Implement **starter agents** per above configuration
4. Implement **bad agent** example for suppression testing
5. Implement **global workspace loop** handling proposals, filtering, broadcasting, and state updates
6. Verify **starter set behaves coherently** (sanity checks)

   * Surprise triggers memory updates
   * Cost warnings prevent runaway exploration
   * Conflicting proposals are reported

---


