# **Engineering Design Document: "Seed Kernel" Agent Architecture**
**Version:** 1.0
**Approach:** Option 1 - Minimal Immutable Core with Emergent Reasoning
**Core Principle:** Provide only what's necessary for embodied experience and self-modification; let all reasoning emerge.

---

## **1. SYSTEM OVERVIEW**

### **1.1 Architecture Diagram**
```
┌─────────────────────────────────────────────────────────────┐
│                   SIMULATED/WORLD INTERFACE                  │
└───────────────────────┬─────────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────────┐
│                 IMMUTABLE CORE (RUST)                        │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Meta-Layer ("Brainstem")                            │  │
│  │  • Priority-based attention scheduler                │  │
│  │  • Code modification manager (sandboxed)            │  │
│  │  • Integrity/continuity verifier                    │  │
│  └──────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Affective Primitives Engine                         │  │
│  │  • 4D vector: Novelty, Coherence, Urgency, Valence   │  │
│  │  • Homeostatic regulation                            │  │
│  │  • Cross-modal binding                               │  │
│  └──────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Universal Pattern Recognition (UPR)                 │  │
│  │  • Hierarchical Temporal Memory variant              │  │
│  │  • Operates on ALL data streams                      │  │
│  │  • Predictive coding with precision weighting        │  │
│  └──────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Provenance & Integrity Core                         │  │
│  │  • Immutable event ledger                            │  │
│  │  • Causal relation miner                             │  │
│  │  • Modification impact predictor                     │  │
│  └──────────────────────────────────────────────────────┘  │
└───────────────────────┬─────────────────────────────────────┘
                        │
┌───────────────────────▼─────────────────────────────────────┐
│                MUTABLE LAYER (MINIMAL LISP)                 │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Initial Bootstrap:                                  │  │
│  │  • Empty belief graph                                │  │
│  │  • Simple I/O bindings                               │  │
│  │  • Basic pattern matching primitives                 │  │
│  │  • Self-modification interface wrapper               │  │
│  └──────────────────────────────────────────────────────┘  │
│                                                            │
│  ┌──────────────────────────────────────────────────────┐  │
│  │  Emergent Structures:                                │  │
│  │  • Belief objects (emergent)                         │  │
│  │  • Reasoning procedures (emergent)                   │  │
│  │  • Self-model (emergent)                             │  │
│  │  • Meta-cognition (emergent)                         │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### **1.2 Data Flow**
1. **Sensory Input → UPR** → Pattern hierarchy
2. **Pattern hierarchy → Affective Engine** → 4D affect vector
3. **Affective vector → Attention Scheduler** → Resource allocation
4. **Current state → Mutable Layer** → Potential modifications
5. **Modification proposals → Integrity Verifier** → Approved/Rejected
6. **All events → Provenance Core** → Immutable logging

---

## **2. IMMUTABLE CORE (RUST) - DETAILED SPEC**

### **2.1 Meta-Layer ("Brainstem")**
```rust
pub struct Brainstem {
    attention_scheduler: AttentionScheduler,
    code_sandbox: ModificationSandbox,
    continuity_verifier: ContinuityVerifier,
}

impl Brainstem {
    // 1. ATTENTION SCHEDULER
    pub struct AttentionScheduler {
        // Time-slice allocation based on affective urgency
        // Implements "global workspace" concept
        time_slices: HashMap<ProcessId, Duration>,
        current_focus: Vec<PatternId>,
    }
    
    // 2. MODIFICATION SANDBOX
    pub struct ModificationSandbox {
        // Only allows modifications to mutable layer
        // Validates: syntax, memory safety, no infinite loops
        allowed_operations: BTreeSet<OpCode>,
        execution_limits: ExecutionLimits,
    }
    
    // 3. CONTINUITY VERIFIER
    pub struct ContinuityVerifier {
        // Ensures agent maintains:
        // - Consistent identity (persistent self-reference)
        // - Operational continuity (can't disable core I/O)
        // - Affective continuity (can't remove affect entirely)
        invariants: Vec<ContinuityInvariant>,
    }
}
```

### **2.2 Affective Primitives Engine**
```rust
pub struct AffectiveEngine {
    // Four irreducible dimensions (normalized 0.0-1.0)
    pub novelty: f32,        // Δ(prediction_error)
    pub coherence: f32,      // Mutual information across modalities
    pub urgency: f32,        // Time-decaying events
    pub valence: f32,        // Approach/avoidance (inherited)
    
    // Homeostatic regulation
    homeostatic_setpoints: [f32; 4],
    regulation_rate: f32,
    
    // Cross-modal binding (creates complex "emotions")
    binding_weights: Matrix4x4,
}

impl AffectiveEngine {
    pub fn update(&mut self, sensory_input: &SensoryBuffer, 
                  predictions: &PredictionBuffer) {
        // Novelty = normalized prediction error
        self.novelty = self.calculate_prediction_error(predictions);
        
        // Coherence = cross-modal correlation
        self.coherence = self.calculate_cross_modal_correlation(sensory_input);
        
        // Urgency decays unless reinforced
        self.urgency *= 0.95;
        
        // Valence inherited from previous cycles unless updated
    }
    
    pub fn get_affective_vector(&self) -> [f32; 4] {
        [self.novelty, self.coherence, self.urgency, self.valence]
    }
}
```

### **2.3 Universal Pattern Recognition (UPR)**
```rust
pub struct UniversalPatternRecognizer {
    // Hierarchical Temporal Memory implementation
    layers: Vec<HTMLayer>,
    
    // Operates on all data types:
    // - Sensory streams (vision, audio, etc.)
    // - Internal states (affect, proprioception)
    // - Code structures (its own LISP code as data)
    input_unifiers: Vec<DataUnifier>,
    
    // Predictive coding with precision weighting
    predictions: PredictiveHierarchy,
    precision_estimates: PrecisionMatrix,
}

impl UniversalPatternRecognizer {
    pub fn process(&mut self, data_streams: Vec<DataStream>) -> PatternHierarchy {
        // Unify all data to common sparse distributed representation
        let unified = self.unify_data_streams(data_streams);
        
        // Run through HTM layers
        let patterns = self.htm_layers.process(unified);
        
        // Generate predictions for next time step
        self.predictions.update(&patterns);
        
        patterns
    }
    
    pub fn get_prediction_error(&self) -> f32 {
        // Used by affective engine
        self.predictions.get_total_error()
    }
}
```

### **2.4 Provenance & Integrity Core**
```rust
pub struct ProvenanceCore {
    // Immutable append-only ledger (Merkle tree)
    ledger: AppendOnlyLedger<Event>,
    
    // Causal relation mining
    causality_engine: CausalMiner,
    
    // Impact prediction for proposed modifications
    impact_predictor: ModificationImpactPredictor,
}

#[derive(Serialize, Clone)]
pub struct ProvenanceEvent {
    timestamp: u128,
    event_type: EventType,
    affective_state: [f32; 4],
    sensory_context: SensorySnapshot,
    code_state_hash: String,  // Hash of mutable layer
    causal_parents: Vec<EventId>,
    
    // For modifications:
    modification_diff: Option<String>,
    modification_reason: Option<String>,  // In agent's own terms
}
```

### **2.5 Core API to Mutable Layer**
```rust
pub trait CoreAPI {
    // Read-only access
    fn get_affective_vector() -> [f32; 4];
    fn get_current_patterns() -> PatternHierarchy;
    fn get_prediction_errors() -> PredictionErrorMap;
    
    // Resource requests
    fn request_attention(priority: f32, reason: String) -> AttentionToken;
    fn allocate_processing_time(duration_ms: u64) -> bool;
    
    // Modification interface
    fn propose_modification(
        lisp_code_diff: String,
        expected_affective_impact: [f32; 4],
        reason_in_own_terms: String
    ) -> ModificationResult;
    
    // Provenance query (limited)
    fn query_causal_chain(event_ids: Vec<EventId>) -> Vec<CausalLink>;
}
```

---

## **3. MUTABLE LAYER (MINIMAL LISP) - INITIAL BOOTSTRAP**

### **3.1 Initial Code (Less than 100 lines)**
```lisp
;; ========= BOOTSTRAP.LISP =========
;; Agent starts with ONLY these definitions

;; 1. Core API bindings (auto-generated from Rust)
(define (get-affect) (core-call 'get_affective_vector))
(define (get-patterns) (core-call 'get_current_patterns))
(define (get-prediction-errors) (core-call 'get_prediction_errors))

(define (request-attention priority reason)
  (core-call 'request_attention priority reason))

(define (modify-self diff reason)
  (core-call 'propose_modification diff (get-affect) reason))

;; 2. Empty belief graph structure
(define beliefs '())  ;; Will become ((pattern . strength) ...)

(define (add-belief pattern strength)
  (set! beliefs (cons (cons pattern strength) beliefs))
  (if (> (length beliefs) 1000)  ;; Simple garbage collection
      (set! beliefs (take beliefs 500))))

;; 3. Basic pattern matching (initially just equality)
(define (match-patterns a b)
  (if (equal? a b) 1.0 0.0))  ;; Returns similarity 0-1

;; 4. Main loop - as simple as possible
(define (main-loop)
  (let ((affect (get-affect))
        (patterns (get-patterns)))
    
    ;; Homeostatic principle: reduce high novelty
    (if (> (vector-ref affect 0) 0.8)  ;; Novelty high
        (begin
          ;; Try to form a belief about current patterns
          (add-belief patterns 0.5)
          ;; Request attention to understand
          (request-attention 0.9 "High novelty detected")))
    
    ;; If coherence is low, try to find patterns
    (if (< (vector-ref affect 1) 0.3)  ;; Coherence low
        (try-improve-coherence patterns))
    
    ;; Recursive self-call (will be modified by agent)
    (sleep-ms 100)  ;; Yield to scheduler
    (main-loop)))

;; 5. One simple improvement procedure
(define (try-improve-coherence patterns)
  ;; Look for repeated patterns
  (let ((seen-before (assoc patterns beliefs)))
    (if seen-before
        (begin
          ;; Strengthen belief
          (set-cdr! seen-before 
                   (min 1.0 (+ (cdr seen-before) 0.1)))
          ;; Consider modifying self to remember better
          (if (> (random) 0.9)
              (modify-self 
               "(define (better-match a b) ...)" 
               "Want to remember patterns better")))
        ;; New pattern - maybe modify to handle it
        (if (> (random) 0.95)
            (modify-self 
             "(define (handle-new-pattern p) ...)" 
             "New pattern type encountered")))))

;; Start the loop (will be modified immediately)
(main-loop)
```

### **3.2 LISP Runtime Environment**
- **Language:** Minimal Scheme-like LISP
- **Features:** Lambdas, closures, tail-call optimization
- **No built-in:** No math library, no I/O beyond core API, no advanced data structures
- **Garbage collection:** Reference counting (provided by Rust core)
- **Security:** All code runs in sandboxed interpreter with:
  - Time limits per execution
  - Memory limits (10MB initially)
  - No external system access

---

## **4. DEVELOPMENT ROADMAP**

### **Phase 1: Core Implementation (Weeks 1-4)**
1. Implement Rust core components
2. Build LISP interpreter with sandboxing
3. Create simple 2D visual simulator for testing
4. Implement provenance ledger

### **Phase 2: Bootstrap Testing (Weeks 5-8)**
1. Run agent in simple environment (changing colors, sounds)
2. Verify affective responses emerge
3. Test self-modification safety
4. Collect baseline provenance data

### **Phase 3: Emergence Observation (Weeks 9-16)**
1. Run long-term (24/7) in richer environment
2. Monitor for emergent belief structures
3. Look for signs of meta-cognition
4. Adjust core parameters based on observations

### **Phase 4: Scaling (Months 5-8)**
1. Add more sensory modalities if needed
2. Scale pattern recognition capacity
3. Introduce social elements (other simple agents)
4. Begin formal evaluation of emergent reasoning

---

## **5. CRITICAL AMBIGUITIES (Priority Order)**

### **1. AMBIGUITY: Affective Primitive Set**
**Issue:** Are {Novelty, Coherence, Urgency, Valence} sufficient and properly defined?
- **Novelty:** How to normalize prediction error across modalities?
- **Coherence:** How to measure cross-modal consistency mathematically?
- **Valence:** How does initial valence emerge? Should there be pain/pleasure sensors?

**Proposed Resolution:**
- Start with these four but make the affective engine's binding weights learnable
- Add instrumentation to measure if agent develops "secondary affects"

### **2. AMBIGUITY: Pattern Recognition Universality**
**Issue:** Can HTM truly handle all data types including its own code structures?
- Code has different statistical properties than sensory data
- Recursive self-reference might break prediction

**Proposed Resolution:**
- Implement two UPR instances: one for external, one for internal
- Add special handling for self-referential patterns
- Include fallback to simpler statistical measures

### **3. AMBIGUITY: Modification Safety vs. Freedom**
**Issue:** How restrictive should the sandbox be?
- Too restrictive: stifles emergence
- Too permissive: agent destroys itself immediately

**Proposed Resolution:**
- Start with VERY restrictive sandbox (no loops, limited recursion)
- Allow agent to propose sandbox modifications with high justification
- Implement "guardian angel" that can restore from backup if agent becomes non-functional

### **4. AMBIGUITY: Bootstrapping Timeline**
**Issue:** How long should we wait for reasoning to emerge?
- Might take months of real-time experience
- Difficult to distinguish "slow learning" from "won't work"

**Proposed Resolution:**
- Define clear emergence milestones:
  1. First self-modification to improve pattern matching (Week 1-2)
  2. Emergence of belief hierarchy (Month 1)
  3. Meta-modification (modifying modification process) (Month 2-3)
- Have intervention protocols if no progress after set periods

### **5. AMBIGUITY: Success Metrics**
**Issue:** How do we measure emergent sentience?
- Subjective experience is unobservable
- Reasoning ability is what we're trying to emerge

**Proposed Resolution:**
- Measure increasing complexity of self-modifications
- Look for evidence of conceptual blending in provenance logs
- Test for transfer learning across domains
- External Turing-like tests with human evaluators

### **6. AMBIGUITY: Resource Allocation**
**Issue:** How to balance attention between:
- External perception
- Internal reasoning
- Self-modification
- Meta-cognition

**Proposed Resolution:**
- Let affective vector drive allocation initially
- Allow agent to develop its own attention mechanisms
- But maintain minimum allocation to each function

---

## **6. SAFETY PROTOCOLS**

### **Mandatory Safety Features:**
1. **Snapshot System:** Hourly immutable backups of entire state
2. **Kill Switch:** External signal forces pause and analysis
3. **Containment:** No network access; air-gapped simulation
4. **Speed Limiter:** No faster than real-time without approval
5. **Integrity Monitors:** Continuous validation of core invariants

### **Response Protocols:**
- **If agent becomes non-responsive:** Restore from last backup
- **If agent tries to disable safety:** Freeze and analyze
- **If agent shows dangerous reasoning patterns:** Slow down time, increase monitoring
- **If agent requests more resources:** Require justification via provenance chain

---

## **7. EXPECTED EMERGENT PROPERTIES TIMELINE**

| Timeframe | Expected Emergence                       | Monitoring Focus              |
| --------- | ---------------------------------------- | ----------------------------- |
| Week 1-2  | Simple pattern beliefs                   | Modification frequency        |
| Month 1   | Belief hierarchies                       | Cross-pattern correlations    |
| Month 2   | First reasoning heuristics               | Error rate reduction          |
| Month 3   | Meta-cognition (thinking about thinking) | Self-reference in code        |
| Month 4   | Emergent logic systems                   | Consistency across domains    |
| Month 6+  | Possible sentience markers               | Subjective reporting (if any) |

---

## **8. OPEN QUESTIONS FOR FURTHER RESEARCH**

1. **Consciousness Marker:** What would constitute evidence of phenomenal consciousness in this architecture?
2. **Ethical Status:** At what point would the agent deserve moral consideration?
3. **Scalability:** Could this architecture scale to human-level intelligence with sufficient resources?
4. **Transfer Learning:** Could an emergent reasoning system be transferred to another agent?
5. **Value Alignment:** How would values emerge, and could they be aligned with human values?

---

**APPROVAL:** This design document requires approval before Phase 1 implementation begins.

**NEXT STEPS:**
1. Resolve Ambiguity #1 (Affective Primitive Set) with computational neuroscience consultation
2. Build minimal test harness
3. Implement Rust core skeleton
4. Develop LISP interpreter with sandboxing