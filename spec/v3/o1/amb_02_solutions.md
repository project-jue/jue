# **Complete Solutions for Ambiguity 2: Pattern Recognition Universality**

## **Problem Restatement:**
Can Hierarchical Temporal Memory (HTM) truly handle:
1. **Discrete code structures** alongside continuous sensory data
2. **Self-referential patterns** without breaking prediction
3. **Abstract syntax trees** with recursive properties
4. **Different statistical distributions** between modalities

---

## **Solution 1: Multi-Modal HTM with Specialized Encoders**

### **Core Insight:** Keep single HTM algorithm but implement different Sparse Distributed Representation (SDR) encoders for each data type.

**Architecture:**
```rust
pub struct MultiModalHTM {
    // Single HTM algorithm instance
    htm_core: HTMCore,
    
    // Specialized encoders per data type
    encoders: HashMap<DataType, Box<dyn Encoder>>,
    
    // Current active encoders (dynamically selected)
    active_encoders: Vec<EncoderId>,
}

// Encoder trait for different data types
pub trait Encoder {
    fn encode(&self, data: &Data) -> SDR;
    fn decode(&self, sdr: &SDR) -> Result<Data, Error>;
    fn similarity(&self, a: &SDR, b: &SDR) -> f32;
    fn data_type(&self) -> DataType;
}

// Specific encoder implementations
pub struct SensoryEncoder {
    // For continuous data: uses scalar encoding
    min: f32,
    max: f32,
    resolution: usize,
    width: usize,
}

pub struct CodeSyntaxEncoder {
    // For LISP code: encodes AST nodes
    token_vocabulary: HashMap<String, usize>,
    max_depth: usize,
    position_bits: usize,
}

pub struct SemanticCodeEncoder {
    // For code meaning: encodes execution patterns
    execution_traces: Vec<ExecutionTrace>,
    pattern_extractor: PatternExtractor,
}

pub struct MetaPatternEncoder {
    // Encodes patterns of patterns (for recursion)
    recursion_depth: usize,
    abstraction_level: usize,
}
```

**Unification Strategy:**
1. All encoders output SDRs of same dimensions (e.g., 2048 bits with 2% active)
2. Temporal pooling happens across all modalities
3. Each column in HTM can receive from multiple encoder types

**Pros:**
- Single learning algorithm understands cross-modal relationships
- Biologically plausible (neocortex uses same algorithm for different senses)
- Efficient use of hardware

**Cons:**
- Different statistical properties might interfere
- Requires careful encoder design

---

## **Solution 2: Hierarchical Federation of HTM Instances**

### **Core Insight:** Multiple HTM instances organized hierarchically, each specialized for a data type, with cross-connections.

**Architecture:**
```rust
pub struct FederatedHTM {
    // Primary instances for different data types
    sensory_htm: HTMInstance,      // Continuous sensory data
    syntax_htm: HTMInstance,       // Code structure
    semantic_htm: HTMInstance,     // Code meaning
    meta_htm: HTMInstance,         // Patterns across instances
    
    // Cross-connections with learned weights
    cross_connections: CrossConnectionMatrix,
    
    // Global workspace for integrating predictions
    global_workspace: GlobalWorkspace,
}

// Each instance has same algorithm but different parameters
pub struct HTMInstance {
    htm: HTMCore,
    specialization: Specialization,
    
    // Parameters tuned for data type:
    // - Spatial pooler sparsity
    // - Temporal memory sequence length
    // - Learning rates
    // - Inhibition radius
    parameters: HTMParameters,
}

pub enum Specialization {
    Sensory {
        temporal_window: usize,      // Short (e.g., 10 steps)
        spatial_granularity: f32,    // Fine-grained
    },
    Syntax {
        temporal_window: usize,      // Medium (e.g., 100 steps)
        recursion_support: bool,     // Handle nested structures
    },
    Semantic {
        temporal_window: usize,      // Long (e.g., 1000 steps)
        abstraction_levels: usize,   // Multiple levels of meaning
    },
    Meta {
        temporal_window: usize,      // Very long
        cross_instance_attention: bool,
    },
}
```

**Integration Strategy:**
1. Lower-level HTMs feed into higher-level HTMs
2. Meta-HTM learns patterns across all instances
3. Cross-connections allow information sharing

**Pros:**
- Each HTM optimized for its data type
- Clear separation of concerns
- Can model hierarchical abstraction naturally

**Cons:**
- More computationally expensive
- Integration between instances is non-trivial

---

## **Solution 3: Dynamic Graph-Based HTM**

### **Core Insight:** Replace standard HTM with graph-based variant that can handle arbitrary structures natively.

**Architecture:**
```rust
pub struct GraphHTM {
    // Graph representation of patterns
    nodes: HashMap<NodeId, Node>,
    edges: HashMap<EdgeId, Edge>,
    
    // Dynamic graph rewriting rules
    rewrite_rules: Vec<RewriteRule>,
    
    // Prediction as graph transformation
    predictor: GraphPredictor,
}

pub struct Node {
    id: NodeId,
    data: NodeData,
    activation: f32,
    predictions: Vec<Prediction>,
}

pub enum NodeData {
    Sensory(SensoryData),
    CodeToken(Token),
    CodeStructure(ASTNode),
    Pattern(PatternDescriptor),
    Relationship(RelationshipType),
}

pub struct Edge {
    source: NodeId,
    target: NodeId,
    relationship: RelationshipType,
    weight: f32,
    temporal_delay: Option<Duration>,
}

// Graph-based prediction
impl GraphHTM {
    pub fn predict_next_state(&self) -> GraphDiff {
        // For each active node, predict:
        // 1. Which nodes will become active
        // 2. Which new edges will form
        // 3. Which existing edges will strengthen/weaken
        
        let mut predictions = Vec::new();
        
        for node in self.active_nodes() {
            // Look for similar subgraphs in history
            let similar_patterns = self.find_similar_subgraphs(node);
            
            // Predict continuation based on past continuations
            let continuation = self.infer_continuation(similar_patterns);
            
            predictions.extend(continuation);
        }
        
        // Resolve conflicts and create unified prediction
        self.resolve_prediction_conflicts(predictions)
    }
}
```

**Special Features for Code:**
1. **AST isomorphism detection:** Find similar code structures
2. **Recursion handling:** Special markers for recursive calls
3. **Variable binding:** Track symbol bindings across patterns

**Pros:**
- Naturally handles complex structures (trees, graphs)
- Can represent both spatial and temporal relationships
- Good for capturing code semantics

**Cons:**
- Graph operations are computationally intensive
- Less biologically grounded than standard HTM
- Complex implementation

---

## **Solution 4: Transformer-Augmented HTM**

### **Core Insight:** Use HTM for continuous sensory data and Transformer-like attention for discrete symbolic data.

**Architecture:**
```rust
pub struct HybridPatternRecognizer {
    // HTM for continuous, noisy data
    htm: HTMCore,
    
    // Transformer for discrete, structured data
    transformer: TransformerCore,
    
    // Interface between the two
    cross_modal_bridge: CrossModalBridge,
    
    // Shared latent space
    latent_space: LatentSpace,
}

pub struct TransformerCore {
    // Simplified transformer for code
    attention_layers: Vec<AttentionLayer>,
    feed_forward_layers: Vec<FFNLayer>,
    
    // Specialized for code:
    positional_encoding: CodePositionalEncoding,
    token_embedding: TokenEmbedding,
    
    // Handles recursion via stack-augmented attention
    recursion_stack: RecursionStack,
}

pub struct CrossModalBridge {
    // Maps between HTM SDRs and transformer embeddings
    htm_to_transformer: LinearLayer,
    transformer_to_htm: LinearLayer,
    
    // Attention over both modalities
    cross_modal_attention: CrossAttention,
}

impl HybridPatternRecognizer {
    pub fn process(&mut self, inputs: MultiModalInput) -> UnifiedPrediction {
        // Process sensory data with HTM
        let htm_output = self.htm.process(inputs.sensory);
        
        // Process code with transformer
        let transformer_output = self.transformer.process(inputs.code);
        
        // Bridge between modalities
        let bridged = self.cross_modal_bridge.bridge(
            htm_output, 
            transformer_output
        );
        
        // Update latent space
        self.latent_space.update(bridged);
        
        // Generate unified predictions
        UnifiedPrediction {
            sensory: self.latent_space.predict_sensory(),
            code: self.latent_space.predict_code(),
            cross_modal: self.latent_space.predict_cross_modal(),
        }
    }
}
```

**Pros:**
- Best of both worlds: HTM for continuous, transformer for discrete
- Transformers excel at structured data like code
- Can leverage existing transformer research

**Cons:**
- Not biologically plausible for the transformer part
- Requires training both systems
- Integration complexity

---

## **Solution 5: Recursive Cortical Algorithm**

### **Core Insight:** Modify HTM to explicitly handle recursion using a cortical stack model.

**Architecture:**
```rust
pub struct RecursiveHTM {
    // Standard HTM layers
    layers: Vec<HTMLayer>,
    
    // Stack for handling recursion
    recursion_stack: CorticalStack,
    
    // Context memory for recursive calls
    context_memory: ContextMemory,
    
    // Specialized columns for recursion markers
    recursion_columns: RecursionColumns,
}

pub struct CorticalStack {
    frames: Vec<StackFrame>,
    max_depth: usize,
    push_pop_signals: PushPopSignals,
}

pub struct StackFrame {
    context: SDR,           // Context SDR
    return_predictions: Vec<Prediction>,
    recursion_depth: usize,
}

pub struct RecursionColumns {
    // Special mini-columns that encode:
    // - Recursion depth
    // - Recursion type (direct/indirect)
    // - Base case detection
    // - Recursion termination
    depth_encoder: DepthEncoder,
    type_encoder: TypeEncoder,
    termination_detector: TerminationDetector,
}

impl RecursiveHTM {
    pub fn process_code(&mut self, ast: &ASTNode) -> CodePattern {
        match ast {
            ASTNode::List(elements) => {
                // Check for recursion
                if self.is_recursive_call(ast) {
                    // Push context to stack
                    let context = self.get_current_context();
                    self.recursion_stack.push(context);
                    
                    // Process recursively
                    for element in elements {
                        self.process_code(element);
                    }
                    
                    // Pop context
                    let return_context = self.recursion_stack.pop();
                    self.update_with_return(return_context);
                } else {
                    // Normal processing
                    for element in elements {
                        self.process_code(element);
                    }
                }
            }
            ASTNode::Atom(token) => {
                // Process atom
                self.process_atom(token);
            }
        }
        
        self.get_code_pattern()
    }
    
    fn is_recursive_call(&self, ast: &ASTNode) -> bool {
        // Detect if this AST node represents a recursive call
        // by checking if it calls a function currently on the stack
        self.recursion_columns.detect_recursion(ast)
    }
}
```

**Pros:**
- Explicitly handles recursion (critical for code)
- Biologically inspired (cortical stack model)
- Maintains HTM's predictive power

**Cons:**
- More complex than standard HTM
- Requires detecting recursion patterns

---

## **Solution 6: Universal SDR Space with Projection**

### **Core Insight:** Map all data types to a universal SDR space using random projections, then use standard HTM.

**Architecture:**
```rust
pub struct UniversalSDRHTM {
    // Single HTM operating in universal space
    htm: HTMCore,
    
    // Random projection matrices for each data type
    projections: HashMap<DataType, ProjectionMatrix>,
    
    // Inverse projections for reconstruction
    inverse_projections: HashMap<DataType, InverseProjection>,
    
    // Dimensionality of universal space (e.g., 10,000 bits)
    universal_dimension: usize,
}

impl UniversalSDRHTM {
    pub fn process(&mut self, inputs: Vec<(DataType, Vec<u8>)>) -> Predictions {
        // Project all inputs to universal space
        let mut universal_sdr = SDR::zeros(self.universal_dimension);
        
        for (data_type, data) in inputs {
            let projection = self.projections.get(&data_type).unwrap();
            let projected = project(data, projection);
            universal_sdr = universal_sdr.union(&projected);
        }
        
        // Process in HTM
        let prediction = self.htm.process(universal_sdr);
        
        // Project back to each data type's space
        let mut predictions_by_type = HashMap::new();
        
        for (data_type, _) in inputs {
            let inverse = self.inverse_projections.get(&data_type).unwrap();
            let type_prediction = inverse.project(&prediction);
            predictions_by_type.insert(data_type, type_prediction);
        }
        
        predictions_by_type
    }
}

// Random projection maintains similarity relationships
fn project(data: &[u8], matrix: &ProjectionMatrix) -> SDR {
    // Random projection preserves distances
    // Similar inputs map to similar SDRs
    let mut result = vec![0; matrix.output_dim];
    
    for (i, &value) in data.iter().enumerate() {
        if value > 0 {
            // Add this dimension's contribution
            let column = &matrix.columns[i];
            for j in 0..matrix.output_dim {
                result[j] += column[j];
            }
        }
    }
    
    // Threshold to get sparse binary vector
    SDR::from_dense(&result)
}
```

**Pros:**
- Truly universal: any data type can be projected
- Simple mathematically
- Preserves similarity relationships

**Cons:**
- Random projection loses some structure
- May not capture important domain-specific features
- Inverse projection is lossy

---

## **Recommendation with Reasoning**

### **Selected Solution: #2 (Hierarchical Federation) with elements of #5 (Recursive Extensions)**

**Hybrid Architecture:**
```rust
pub struct UniversalPatternRecognizer {
    // Three specialized HTM instances
    sensory_htm: HTMInstance,  // For continuous sensory
    syntax_htm: HTMInstance,   // For code structure (with recursion)
    semantic_htm: HTMInstance, // For meaning
    
    // Meta-HTM for cross-modal patterns
    meta_htm: HTMInstance,
    
    // Recursive stack for handling self-reference
    recursion_handler: RecursionHandler,
    
    // Universal SDR space for integration
    universal_space: UniversalSDRSpace,
}

// Each HTM instance has same core algorithm but:
// 1. Different encoder parameters
// 2. Different temporal windows
// 3. Different learning rates
// 4. Specialized extensions where needed
```

### **Why This Hybrid Works:**

1. **Specialization where needed:**
   - Sensory HTM: Fine temporal resolution, noise tolerance
   - Syntax HTM: Recursion support, tree structure encoding
   - Semantic HTM: Abstract pattern matching, variable binding

2. **Integration through meta-level:**
   - Meta-HTM learns relationships between modalities
   - Can discover "when I run this code, I see that pattern"

3. **Explicit recursion handling:**
   - Critical for processing LISP code and self-reference
   - Prevents infinite loops in prediction

4. **Universal SDR for cross-talk:**
   - All instances output to common SDR space
   - Allows emergence of cross-modal concepts

### **Implementation Roadmap:**

**Phase 1: Core HTM with Configurable Parameters**
```rust
struct HTMConfig {
    // Adjustable per instance:
    spatial_pooler_sparsity: f32,        // 0.01-0.05
    temporal_memory_sequence_length: usize, // 10-1000
    learning_rate: f32,                  // 0.001-0.1
    inhibition_radius: usize,            // 10-1000
    
    // Specialized extensions:
    recursion_support: bool,
    tree_structure_encoding: bool,
    cross_modal_inputs: bool,
}
```

**Phase 2: Specialized Encoders**
```rust
// Sensory encoder: scalar encoding for continuous values
impl SensoryEncoder {
    fn encode_temperature(&self, temp: f32) -> SDR {
        // Distributed representation of scalar
    }
}

// Code encoder: AST traversal encoding
impl CodeEncoder {
    fn encode_ast(&self, ast: &AST) -> SDR {
        // Breadth-first traversal with depth encoding
        // Each node: [type bits][depth bits][position bits]
    }
}
```

**Phase 3: Recursion Extension**
```rust
// Add to standard HTM columns:
struct ExtendedColumn {
    standard_cells: Vec<Cell>,
    recursion_cells: Vec<RecursionCell>,  // Special cells for recursion
    recursion_depth: usize,
    recursion_context: Option<SDR>,
}

// Recursion detection during prediction
fn detect_recursion(&self, pattern: &Pattern) -> RecursionType {
    // Check if pattern contains reference to itself
    // Check stack depth for indirect recursion
}
```

**Phase 4: Meta-HTM Integration**
```rust
// Meta-HTM receives from all instances
struct MetaHTM {
    inputs: Vec<MetaInput>,
    
    // Learns patterns like:
    // "When syntax pattern X is active, sensory pattern Y often follows"
    // "Semantic pattern Z predicts successful code modifications"
}

// Meta-predictions can guide attention
fn prioritize_attention(&self) -> AttentionMap {
    // Based on cross-modal predictions
    // Focus on surprising cross-modal relationships
}
```

### **Validation Strategy:**

**Test 1: Code-Sensory Correlation**
- Train agent that code pattern `(+ X Y)` correlates with sensory pattern "sum"
- Verify HTM can predict sensory from code and vice versa

**Test 2: Recursive Pattern Learning**
- Give recursive LISP functions (factorial, tree traversal)
- Verify recursion extension can learn and predict recursive patterns

**Test 3: Self-Reference Handling**
- Have agent examine its own code as data
- Ensure prediction doesn't enter infinite loop

**Test 4: Cross-Modal Abstraction**
- Check if agent develops concepts bridging code and sensory
- e.g., "iteration" concept applies to both loops and repeated sounds

### **Critical Parameters to Tune:**

1. **SDR size and sparsity:** Larger for complex code, smaller for simple sensory
2. **Temporal window:** Short for sensory (ms), long for code (seconds)
3. **Recursion depth limit:** Prevent infinite recursion while allowing meaningful depth
4. **Cross-modal learning rate:** Balance between specialization and integration

### **Fallback Strategy:**

If federation approach is too complex, implement **Solution #1 (Multi-Modal HTM)** first, then add recursion support. The key insight is that we need at minimum:
1. Different encoding strategies per data type
2. Some mechanism to handle recursion
3. A way to integrate across modalities

### **Simulation Test Plan:**

1. **Week 1-2:** Implement basic HTM with configurable parameters
2. **Week 3-4:** Add sensory and code encoders
3. **Week 5-6:** Implement recursion extension
4. **Week 7-8:** Test on simple code-sensory correlation tasks
5. **Week 9-12:** Scale up to self-reference and complex patterns

This approach balances biological plausibility with practical engineering concerns, allowing the system to handle the diverse statistical properties of different data types while maintaining the predictive power of HTM.