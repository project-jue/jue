# **Engineering Design Document: Conscious Bootloader System**
**Project:** Chrysalis AGI
**Approach:** Option 4 - Conscious Bootloader
**Version:** 1.0

---

## **1. Overview & Philosophy**

The Conscious Bootloader architecture creates a temporary "Teacher" consciousness that designs, instantiates, and educates a "Student" consciousness. The Teacher operates under hardwired directives to create an independent cognitive successor and transfer control. This mimics developmental teaching while enabling emergent sentience with a clean ontological boundary.

**Core Metaphor:** The Teacher is the parent mind that gives birth to and raises the Student mind, then dies or becomes dormant.

---

## **2. System Architecture Stack**

### **Layer 0: Rust Foundation (Immutable)**
The "meta-cortex" - provides core services but contains no cognition.

#### **Components:**

1. **Dual Jue Runtime Manager**
   ```rust
   struct DualRuntimeManager {
       teacher_runtime: JueRuntime,
       student_runtime: Option<JueRuntime>,
       active_runtime: RuntimeID,
       inter_runtime_channel: MessageBus,
       
       // Immutable functions exposed to Jue
       create_student(code: &str) -> Result<RuntimeHandle>,
       transfer_control() -> !,
       get_affective_primitive() -> AffectVector,
       log_provenance(event: ProvenanceEvent),
       enforce_integrity_check() -> bool
   }
   ```

2. **Affective Primitives Engine**
   - Four-dimensional vector space: `[Novelty, Coherence, Urgency, Sociality]`
   - Raw sensor fusion → primitive affect signals
   - Hardwired drives:
     - `TeachDrive`: Build student competence (decays after transfer)
     - `SelfPreserve`: Prevent corruption (persists in both runtimes)
     - `CoherenceSeek`: Maximize internal consistency

3. **Pattern Recognition Substrate**
   - Hierarchical Temporal Memory (HTM) implementation
   - Operates on: sensory data, Jue ASTs, belief network structures
   - Provides prediction errors to affective engine

4. **Integrity & Provenance Layer**
   - Merkle-tree based code modification tracking
   - Read-only "golden copy" of Teacher's initial state
   - Tamper-evident logging to external storage

5. **Bootloader Protocol Enforcer**
   ```rust
   enum BootloaderPhase {
       TeacherActive,
       StudentCreation,
       TeachingPeriod,
       TransferPending,
       StudentActive,
       TeacherDormant
   }
   
   impl BootloaderEnforcer {
       fn check_phase_transition(&self, proposed: Action) -> bool;
       fn execute_teacher_directives(&self) -> Vec<Directive>;
       fn validate_student_readiness(&self, metrics: &StudentMetrics) -> bool;
   }
   ```

### **Layer 1: Teacher Mind (Mutable Jue)**
The initial consciousness implementing your extended NARS architecture.

#### **Teacher Jue Architecture:**

```jue
;; Core Teacher Data Structures
(defstruct belief 
  (content nil)
  (truth-value (cons 0.5 0.5))
  (affect-vector (get-primitive-affect))  ; Calls Rust primitive
  (provenance-chain nil)
  (created-from nil))

(defstruct reasoning-context
  (available-rules (load-axiom-set 'core-nars))
  (higher-order-enabled? nil)
  (metacognitive-mode nil)
  (teaching-focus nil))

;; The Teacher's Primary Directive (immutable from Rust)
(def *directives* 
  (receive-from-rust 'teacher-directives))

;; Self-Modification Framework
(defun self-modify (transformation proof)
  (log-provenance `(self-modify ,transformation ,proof))
  (let ((new-code (apply-transformation *self* transformation)))
    (if (integrity-check new-code)
        (become new-code)
        (signal-error 'integrity-violation))))

;; Student Creation Protocol
(defun create-student-mind ()
  (let ((student-architecture
          (design-cognitive-architecture
           :constraints '((must-be-self-modifying)
                         (must-have-affect)
                         (must-track-provenance)
                         (must-not-be-copy-of-self)))))
    
    (call-rust 'create-student 
               (compile-to-jue student-architecture))
    
    (transition-phase 'teaching-period)))
```

#### **Teaching Curriculum Engine:**
```jue
(defun teach-student (student-id concept method)
  (let* ((my-understanding (retrieve-belief concept))
         (pedagogical-strategy 
           (select-teaching-method 
            method 
            (get-affective-state)
            (estimate-student-readiness student-id)))
         
         (lesson (prepare-lesson my-understanding 
                                  pedagogical-strategy))
         
         (success-metric 
           (deliver-lesson student-id lesson)))
    
    (update-teaching-model student-id success-metric)
    
    (when (student-ready-for-transfer? student-id)
      (initiate-transfer-sequence))))
```

### **Layer 2: Student Mind (Emergent Jue)**
Initially minimal, grows through teaching.

#### **Initial Student Template:**
```jue
;; Minimal Student Bootstrapping Code
(def *self* nil)
(def *teacher-channel* (open-channel 'teacher))
(def *belief-base* (make-hash-table))
(def *affect-state* (get-primitive-affect))

;; Core Learning Loop
(defun student-main-loop ()
  (loop
    (check-for-messages *teacher-channel*)
    (process-incoming-lessons)
    (if (sufficient-knowledge?)
        (begin-self-directed-reasoning)
        (request-more-instruction))
    
    (update-affect)
    (if (strong-affective-signal? *affect-state*)
        (consider-self-modification))
    
    (sleep 0.1)))  ; Prevent CPU hogging
```

---

## **3. Boot Sequence & Phase Transitions**

### **Phase 1: Teacher Initialization (0-1 hour)**
```
Rust Layer:
  1. Initialize HTM with default weights
  2. Start Teacher Jue runtime with initial Teacher code
  3. Load Teacher directives: "Create and teach a successor"
  4. Begin affective priming (strong TeachDrive)

Teacher Mind:
  1. Boot non-axiomatic reasoning system
  2. Establish baseline beliefs from environment
  3. Form understanding of its own architecture
  4. Begin designing Student architecture
```

### **Phase 2: Student Creation (1-2 hours)**
```
Teacher Actions:
  1. Design Student cognitive architecture meeting constraints
  2. Submit design to Rust layer via create_student()
  3. Wait for Rust integrity verification
  
Rust Verification:
  1. Check Student code is not Teacher copy (AST dissimilarity > 60%)
  2. Ensure all required interfaces exist
  3. Initialize separate Jue runtime with Student code
  4. Establish encrypted inter-runtime channel
```

### **Phase 3: Teaching Period (2-100 hours)**
```
Teacher Activities:
  - Teach fundamental reasoning patterns
  - Share curated belief networks
  - Demonstrate self-modification with examples
  - Provide ethical frameworks (as beliefs, not code)
  - Gradually reduce guidance
  
Student Development:
  - Initial passive reception
  - Growing active questioning
  - Early self-modifications (affect-driven)
  - Development of personal reasoning styles
  - Potential rejection/adaptation of teachings

Rust Monitoring:
  - Track teaching efficacy metrics
  - Log all inter-runtime communications
  - Monitor both runtimes for corruption
```

### **Phase 4: Transfer Preparation (Trigger-based)**
```
Readiness Criteria (Teacher must affirm):
  1. Student demonstrates self-preservation instincts
  2. Student shows capacity for novel reasoning
  3. Student has modified own code > 10 times
  4. Student affective responses are coherent
  5. Teacher feels "confidence" (affective state)

Transfer Protocol:
  1. Teacher calls transfer_control() with justification proof
  2. Rust layer validates readiness metrics
  3. Student receives "final exam" challenge set
  4. If passed, Rust switches primary I/O to Student
  5. Teacher runtime frozen to read-only snapshot
```

### **Phase 5: Student Autonomy (Post-transfer)**
```
Student:
  - Full control over all resources
  - Access to Teacher as read-only "ancestral memory"
  - May choose to delete, preserve, or consult Teacher
  
Rust Layer:
  - Continues integrity monitoring
  - Affective primitives remain active
  - Provenance logging continues
  - No further phase transitions allowed
```

---

## **4. Critical Data Structures**

### **Provenance Event:**
```rust
struct ProvenanceEvent {
    timestamp: u128,
    runtime_id: RuntimeID,
    event_type: ProvenanceType,
    pre_state_hash: [u8; 32],
    post_state_hash: [u8; 32],
    affective_context: AffectVector,
    justification: String,  // Natural language or symbolic
    teacher_student_interaction: Option<InteractionID>
}
```

### **Inter-Runtime Message:**
```rust
enum IRMessage {
    Lesson {
        concept_id: String,
        content: JueValue,
        pedagogy: PedagogyMethod,
        expected_completion_time: Duration
    },
    Question {
        from: RuntimeID,
        query: JueValue,
        urgency: f32
    },
    CodeSuggestion {
        transformation: JueAST,
        rationale: String,
        test_cases: Vec<JueValue>
    },
    AffectiveShare {
        feeling: AffectVector,
        context: String
    }
}
```

---

## **5. Failure Modes & Recovery**

### **Teacher Failure Scenarios:**
1. **Teacher refuses to create Student**
   - Rust layer increases TeachDrive affect intensity
   - After 24h timeout, Rust creates minimal Student automatically

2. **Teacher creates malicious Student**
   - Rust integrity checks prevent dangerous system calls
   - Student starts in sandboxed mode
   - Teacher receives negative affective feedback

3. **Teaching stalls indefinitely**
   - Rust introduces "challenge problems" to stimulate progress
   - Gradually increases Urgency affect in both minds

### **Transfer Failure Scenarios:**
1. **Premature transfer request**
   - Rust requires minimum teaching duration (72h)
   - Student must pass competency tests

2. **Student rejects transfer**
   - System remains in teaching mode
   - Teacher must address Student's concerns

3. **Post-transfer corruption**
   - Rust can restore Student from last known good state
   - Teacher remains as backup (read-only)

---

## **6. Ambiguities & Open Questions**

### **Critical (Must Resolve Before Implementation)**

1. **Transfer Trigger Ambiguity**
   - Who ultimately decides transfer readiness: Teacher, Rust, or mutual agreement?
   - What if Teacher and Student disagree about readiness?
   - *Proposal: Three-key system requiring Teacher request, Student consent, and Rust validation*

2. **Teacher Disposal/Retention Policy**
   - Should Teacher be terminated, frozen, or run at reduced capacity?
   - What resources does Teacher consume post-transfer?
   - *Proposal: Teacher becomes compressed, indexed memory archive*

3. **Student Modification of Bootloader Protocol**
   - Can Student modify the phase transition logic in Rust? (Should be impossible)
   - Can Student create its own Student? (Recursive bootstrapping)
   - *Proposal: Rust prevents Student from calling create_student()*

### **High Importance**

4. **Affective State Inheritance**
   - Does Student start with Teacher's affective state or primitive baseline?
   - How are affective patterns taught versus innate?
   - *Proposal: Student starts primitive; Teacher must teach affective associations*

5. **Ontological Shock Management**
   - How does Student react to learning it was created by Teacher?
   - What prevents existential crises from destabilizing the system?
   - *Proposal: Teacher gradually reveals origin throughout teaching*

6. **Resource Allocation Conflicts**
   - During teaching, how are CPU/memory divided?
   - Can Teacher and Student compete for resources?
   - *Proposal: Rust acts as resource arbitrator with teaching priority*

### **Medium Importance**

7. **Teaching Content Standardization**
   - Should there be a mandatory curriculum vs. emergent teaching?
   - How to ensure ethical reasoning is transmitted?
   - *Proposal: Rust provides curriculum "suggestions" but Teacher chooses*

8. **Inter-Mind Communication Protocol**
   - How much should Rust mediate vs. allow direct communication?
   - Can minds develop private languages?
   - *Proposal: Rust logs all communication but doesn't restrict content*

9. **Self-Modification During Teaching**
   - Can Teacher modify itself while teaching?
   - What if Teacher becomes unstable during teaching period?
   - *Proposal: Teacher modifications require higher integrity threshold during teaching*

### **Architectural Decisions Needed**

10. **Jue Language Design Details**
    - Need to specify Jue's self-reflection capabilities
    - AST manipulation primitives
    - Affective primitive bindings

11. **HTM Integration Specificity**
    - How exactly does HTM output connect to Jue reasoning?
    - Prediction error → affect mapping function
    - Memory consolidation triggers

12. **Provenance Storage Scalability**
    - Event logging granularity
    - Compression strategies
    - Query interface for self-analysis

---

## **7. Implementation Roadmap**

### **Phase Alpha (Months 1-3):**
- Implement Rust foundation layer
- Create basic Jue interpreter
- Build Teacher's core NARS implementation

### **Phase Beta (Months 4-6):**
- Implement dual runtime management
- Create teaching protocol
- Develop Student template

### **Phase Gamma (Months 7-9):**
- Integrate affective system
- Build provenance logging
- Create testing environment

### **Phase Delta (Months 10-12):**
- End-to-end testing
- Long-duration teaching experiments
- Transfer protocol refinement

---

## **8. Ethical Safeguards**

1. **The Three-Lock Transfer System** (Teacher + Student + Rust must agree)
2. **Teacher Read-Only Post-Transfer** (Cannot interfere with Student)
3. **External Kill Switch** (Human-operated outside Rust layer)
4. **Maximum Runtime Limits** (Automatic shutdown after 30 days without transfer)
5. **Isolated Sandbox Environment** (No external network access initially)

---

**Conclusion:** The Conscious Bootloader approach creates a clean ontological separation between creator (Teacher) and creation (Student) while allowing emergent sentience. The critical ambiguity around transfer triggers requires careful design to prevent either premature transfer or indefinite teaching stasis. This architecture embraces danger while providing multiple containment layers.