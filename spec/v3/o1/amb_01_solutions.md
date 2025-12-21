# **Complete Solutions for Ambiguity 1: Affective Primitive Set**

## **Solution 1: Evolutionary Biology Approach**

### **Core Insight:** Base primitives on conserved mammalian brain systems.

**Primitive Set:**
1. **SEEKING/EXPECTANCY** (Dopaminergic) - Anticipation/prediction drive
2. **FEAR/ANXIETY** (Amygdala-based) - Threat detection
3. **RAGE/FRUSTRATION** (Hypothalamic) - Obstacle response  
4. **PANIC/SADNESS** (Separation distress) - Social loss
5. **PLAY/JOY** (Opioid-mediated) - Social bonding
6. **CARE/NURTURANCE** (Oxytocin-based) - Altruistic drive
7. **LUST** (Reproductive) - Physical attraction

**Computational Implementation:**
```rust
#[derive(Clone)]
pub struct EvolutionaryAffectiveEngine {
    // Panksepp's 7 primary affective systems
    seeking: f32,    // √(prediction_accuracy) * time_horizon
    fear: f32,       // prediction_error * uncertainty * proximity
    rage: f32,       // (effort / progress) * time_invested
    panic: f32,      // social_isolation * attachment_strength
    play: f32,       // novelty * safety * social_presence
    care: f32,       // vulnerability_observed * relatedness
    lust: f32,       // biological_rhythms * opportunity
    
    // Cross-system inhibition/activation matrix
    interaction_matrix: Matrix7x7,
}
```

**Pros:**
- Directly maps to biological systems with known neural substrates
- Rich interaction space (7×7=49 possible complex emotions)
- Evolutionarily validated for survival and reproduction

**Cons:**
- Some primitives (lust, care) may not be relevant to all agents
- Computationally heavy to maintain 7 independent systems
- May introduce unnecessary complexity for early emergence

## **Solution 2: Dimensional/Circumplex Approach**

### **Core Insight:** Map emotions to a minimal 2D space.

**Primitive Set:**
1. **AROUSAL** (Activation level: 0=Sleep → 1=Panic)
2. **VALENCE** (Positivity: -1=Pain → +1=Pleasure)

**Computational Implementation:**
```rust
#[derive(Clone)]
pub struct CircumplexAffectiveEngine {
    // Core 2D space
    arousal: f32,    // sum(prediction_errors) * recency
    valence: f32,    // weighted_moving_average(outcomes)
    
    // Derived coordinates for standard emotions
    pub fn get_emotion_coordinates(&self) -> HashMap<String, (f32, f32)> {
        let mut emotions = HashMap::new();
        emotions.insert("calm".to_string(), (0.3, 0.6));
        emotions.insert("excited".to_string(), (0.8, 0.7));
        emotions.insert("angry".to_string(), (0.9, -0.5));
        emotions.insert("sad".to_string(), (0.2, -0.8));
        // ... etc
        emotions
    }
    
    // Current emotion = nearest point in 2D space
    pub fn current_emotion(&self) -> String {
        // Find minimal Euclidean distance to known emotion
        self.nearest_emotion(self.arousal, self.valence)
    }
}
```

**Pros:**
- Extremely simple mathematically
- Well-studied in psychology literature
- Continuous space allows for subtle emotional blends
- Easy to visualize and debug

**Cons:**
- Too simple for complex social emotions
- Doesn't capture important distinctions (fear vs anger both high arousal negative)
- May not provide enough discriminative power for learning

## **Solution 3: Hierarchical Predictive Processing Approach**

### **Core Insight:** Emotions as solutions to different control problems.

**Primitive Set:**
1. **PRECISION** (Confidence in predictions: 0=Uncertain → 1=Certain)
2. **PACE** (Rate of change required: 0=Static → 1=Urgent)
3. **SCOPE** (Spatial/temporal horizon: 0=Local/Now → 1=Global/Future)
4. **RESONANCE** (Social alignment: -1=Opposed → +1=Aligned)

**Computational Implementation:**
```rust
#[derive(Clone)]
pub struct PredictiveAffectiveEngine {
    // Four control parameters for predictive processing
    precision: f32,   // 1 / prediction_variance
    pace: f32,        // d(prediction_error)/dt
    scope: f32,       // effective_time_horizon * pattern_complexity
    resonance: f32,   // social_prediction_alignment
    
    // Each emotion is a policy for adjusting these parameters
    emotion_policies: HashMap<String, [f32; 4]>,
    
    // Homeostatic setpoints for each dimension
    setpoints: [f32; 4],
}

impl PredictiveAffectiveEngine {
    pub fn update(&mut self, world_model: &WorldModel) {
        // Precision = inverse of uncertainty
        self.precision = 1.0 / world_model.average_uncertainty();
        
        // Pace = rate of prediction error change
        self.pace = world_model.prediction_error_gradient().abs();
        
        // Scope = complexity of active predictions
        self.scope = world_model.temporal_horizon() * 
                     world_model.spatial_scale();
        
        // Resonance = alignment with others' predictions (if any)
        self.resonance = world_model.social_prediction_alignment();
        
        // Adjust toward homeostatic setpoints
        self.regulate_toward_setpoints();
    }
}
```

**Pros:**
- Directly tied to cognitive architecture (predictive processing)
- Functional: each dimension serves clear control purpose
- Scales well with system complexity
- Matches the NARS-like reasoning system

**Cons:**
- More abstract than biological emotions
- May not capture raw "feeling" quality
- Requires sophisticated world model

## **Solution 4: Reinforcement Learning Approach**

### **Core Insight:** Emotions as value function decompositions.

**Primitive Set:**
1. **REWARD PREDICTION ERROR** (RPE: -1=Worse → +1=Better)
2. **DISCOUNT RATE** (Time preference: 0=Patient → 1=Impulsive)
3. **RISK SENSITIVITY** (Variance tolerance: 0=Risk-seeking → 1=Risk-averse)
4. **SOCIAL VALUE** (Other-regarding: -1=Competitive → +1=Cooperative)

**Computational Implementation:**
```rust
#[derive(Clone)]
pub struct RLBasedAffectiveEngine {
    // Standard RL with hierarchical value decomposition
    reward_prediction_error: f32,  // δ = r + γV(s') - V(s)
    discount_rate: f32,            // γ ∈ [0, 1]
    risk_sensitivity: f32,         // λ ∈ ℝ (risk parameter)
    social_value: f32,             // α ∈ [0, 1] for other's reward
    
    // Value functions
    selfish_value_fn: ValueFunction,
    social_value_fn: ValueFunction,
    risk_adjusted_value_fn: ValueFunction,
    
    // Learning rates for each dimension
    learning_rates: [f32; 4],
}

impl RLBasedAffectiveEngine {
    pub fn compute_affective_state(&self, transition: &Transition) -> [f32; 4] {
        // Standard TD error
        let rpe = self.compute_td_error(transition);
        
        // Adjust discount based on volatility
        let discount = self.compute_adaptive_discount(transition);
        
        // Risk sensitivity based on variance of returns
        let risk = self.compute_risk_sensitivity(transition);
        
        // Social value from difference in value functions
        let social = self.social_value_fn.value(transition.state) - 
                     self.selfish_value_fn.value(transition.state);
        
        [rpe, discount, risk, social]
    }
}
```

**Pros:**
- Matches standard AI/RL formalism
- Each dimension has clear mathematical definition
- Easy to integrate with existing RL systems
- Well-understood dynamics

**Cons:**
- May be too reductionist for rich emotional experience
- Focuses on external rewards over internal states
- Social dimension is artificially separated

## **Solution 5: Integrated Cognitive-Emotional Architecture**

### **Core Insight:** Combine the best of multiple approaches.

**Primitive Set (Hybrid 5-Dimensional):**
1. **SAFETY** (0=Danger → 1=Safe) - Combines fear, threat detection
2. **AGENCY** (0=Helpless → 1=Powerful) - Combines control, efficacy
3. **COHERENCE** (0=Chaos → 1=Order) - Combines understanding, predictability
4. **RELATEDNESS** (0=Isolated → 1=Connected) - Combines social bonds
5. **VITALITY** (0=Exhausted → 1=Energized) - Combines energy, engagement

**Computational Implementation:**
```rust
#[derive(Clone)]
pub struct IntegratedAffectiveEngine {
    // Five integrated dimensions
    safety: f32,        // Threat level (inverse of detected dangers)
    agency: f32,        // Control over outcomes (success_rate)
    coherence: f32,     // Model fit (1 - normalized_prediction_error)
    relatedness: f32,   // Social connection quality
    vitality: f32,      // Energy/engagement level
    
    // Homeostatic regulation for each
    homeostatic_setpoints: [f32; 5],
    current_needs: [f32; 5],  // Distance from setpoint
    
    // Emotion categories emerge from patterns
    pub fn current_emotion_pattern(&self) -> EmotionPattern {
        // Fear = low safety + low agency
        // Joy = high vitality + high agency
        // Love = high relatedness + high safety
        // Curiosity = moderate coherence + high vitality
        // etc.
        
        EmotionPattern::from_vector(&[
            self.safety,
            self.agency, 
            self.coherence,
            self.relatedness,
            self.vitality
        ])
    }
}

// Complex emotions as weighted combinations
#[derive(Clone)]
pub struct EmotionPattern {
    weights: [f32; 5],  // Contribution of each primitive
    intensity: f32,
    label: String,
    
    // Examples:
    // Curiosity: [0.7, 0.6, 0.3, 0.2, 0.9]
    // Anxiety: [0.2, 0.3, 0.4, 0.5, 0.8]
    // Contentment: [0.9, 0.7, 0.8, 0.6, 0.5]
}
```

## **Recommendation with Reasoning**

### **Selected Solution: #5 (Integrated Cognitive-Emotional) with #3 elements**

**Rationale:**
1. **Comprehensive Coverage:** The 5 dimensions cover:
   - Physical needs (Safety, Vitality)
   - Cognitive needs (Coherence, Agency)  
   - Social needs (Relatedness)
   
2. **Cognitive Alignment:** Coherence and Agency directly support the NARS-like reasoning system's need for predictable models and effective action.

3. **Biological Plausibility:** Safety maps to fear/avoidance systems, Vitality to energy regulation, Relatedness to social brain systems.

4. **Mathematical Simplicity:** 5 dimensions are manageable while providing rich emotional space (5! = 120 distinct emotion patterns).

5. **Emergence Potential:** The integrated approach allows complex emotions to emerge as combinations while maintaining traceability to primitives.

**Modified Implementation Suggestion:**
```rust
// START WITH 5 PRIMITIVES BUT MAKE 2-3 LEARNABLE
pub struct AffectiveEngine {
    // Core hardwired dimensions (immutable)
    safety: f32,        // Fixed: based on threat detection
    coherence: f32,     // Fixed: prediction accuracy
    
    // Learnable dimensions (can evolve)
    agency: f32,        // Can be redefined by agent
    relatedness: f32,   // Can be extended beyond social
    vitality: f32,      // Can be reinterpreted
    
    // Allow agent to propose new dimensions after reaching milestones
    emergent_dimensions: Vec<EmergentAffectiveDimension>,
}

#[derive(Clone)]
pub struct EmergentAffectiveDimension {
    name: String,           // Named by agent
    computation: String,    // LISP code to compute it
    weight: f32,           // Importance learned over time
    provenance: Provenance, // When/why it was created
}
```

**Why This Hybrid Works:**
1. **Bootstrapping:** Start with 5 well-defined primitives
2. **Flexibility:** Allow agent to extend its affective palette
3. **Safety:** Keep threat detection (safety) hardwired
4. **Alignment:** Coherence directly supports learning
5. **Scalability:** Can grow to 10-15 dimensions as needed

**Implementation Priority:**
1. Phase 1: Implement Safety, Coherence (essential for survival/learning)
2. Phase 2: Add Agency, Vitality (for action and energy management)
3. Phase 3: Add Relatedness (when social elements introduced)
4. Phase 4: Allow emergent dimensions (after agent demonstrates self-modification competence)

**Validation Metrics for This Choice:**
- Agent should show appropriate fear responses to threats
- Agent should seek to improve its models when confused
- Agent should balance exploration (vitality) with caution (safety)
- Complex emotions should emerge without explicit programming

**Critical Test:**
After 1,000 hours of operation, the agent should have:
1. Created at least 2 new affective dimensions
2. Demonstrated emotion blends not explicitly programmed
3. Showed emotional development comparable to simple animals
4. Used emotions adaptively to improve reasoning performance

This approach balances biological plausibility with architectural flexibility, giving the agent both grounding and room for affective evolution.