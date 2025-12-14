

---

# Dan-World Folder Structure and Development Rules

## 1. Core Principles

1. **Modularity by Cognitive Function**

   * Each folder represents a cognitive module or agent category (perception, memory, evaluation, coherence, etc.)
   * Inspired by Minsky’s Society of Mind: “modules within modules, each responsible for a micro-function”

2. **Layered Hierarchy**

   * Three conceptual layers, mirrored in code:

     1. **Micro-kernels / primitive agents** – minimal reasoning units (atomic)
     2. **Functional agents** – combine micro-kernels into coherent roles
     3. **Macro-modules / emergent systems** – global workspace interactions, policies, evolution

3. **Proof & Validation Tracking**

   * Each module maintains **proof obligations, verification functions, and behavioral invariants**
   * Supports gradual formalization of actions and self-modification

4. **Belief Facet Theory Integration**

   * Modules can maintain **facet metadata** for their beliefs, goals, or perceptions
   * Folder naming and structure should accommodate **mapping across cognitive dimensions**: perception, affect, memory, planning, evaluation, self-modeling

5. **Research-Inspired Categorization**

   * NARS-like adaptive reasoning: `non_axiomatic` folder for heuristic / probabilistic modules
   * Neuroscience-inspired: `neural_substrate` folder for synapse-like structures, plasticity rules
   * SOC/Minsky-style: `society_of_agents` folder for agent orchestration and competition

---

## 2. Recommended Top-Level Folder Layout

```
dan_world/
├── agents/                     # All individual agents and micro-kernels
│   ├── perception/             # Sensor, feature detection, surprise detectors
│   ├── memory/                 # Episodic, semantic, pattern abstraction
│   ├── evaluation/             # Goal-driven assessment, cost/resource evaluation
│   ├── coherence/              # Self-models, conflict resolution, belief integration
│   ├── affect/                 # Emotional/affective state simulation
│   ├── meta/                   # Meta-agents, optional, disabled initially
│   └── templates/              # Agent boilerplate & interface definitions
│
├── society_of_agents/           # Orchestration & global workspace
│   ├── workspace/              # Event loop, proposal handling, salience scoring
│   ├── mutation/               # Self-modification & trust levels
│   ├── conflict_resolution/    # Arbitration, voting, emergent governance
│   └── synchronization/        # Cross-module/world coherence
│
├── non_axiomatic/               # NARS-inspired reasoning components
│   ├── probabilistic_inference/
│   ├── novelty_detection/
│   ├── resource_allocation/
│   └── learning_rules/
│
├── neural_substrate/            # Low-level neuroscience-inspired structures
│   ├── neurons/                # Micro-units, firing rules, thresholds
│   ├── synapses/               # Weighted connections, plasticity
│   ├── neurotransmitters/      # Modulatory signals, stress/cortisol models
│   └── network_topology/       # Graph connectivity, small-world, etc.
│
├── belief_facets/               # Encodes Belief Facet Theory models
│   ├── facets/                  # Facet definitions per cognitive domain
│   ├── facet_mappings/          # Inter-module mappings & projections
│   └── evaluation_metrics/      # Confidence, trust, and weighting of facets
│
├── utils/                       # General helpers (logging, serialization)
├── tests/                       # Unit and integration tests
├── docs/                        # Design documents, research notes, references
└── config/                      # Parameter files, thresholds, and weights
```

---

## 3. Naming and Implementation Rules

1. **Folder Names**

   * Use **singular nouns** for clarity: `perception`, `memory`, `affect`
   * Avoid generic names like `misc` or `stuff`
   * Reflect **cognitive or functional role**, not implementation detail

2. **File Naming**

   * Agent files: `<agent_name>.jue`

     * e.g., `novelty_detector.jue`, `episodic_memory.jue`
   * Module orchestration: `<function>_module.jue`

     * e.g., `workspace_module.jue`, `mutation_module.jue`
   * Tests: `<target>_test.jue` or `<target>_test.rs` for Rust primitives

3. **Agent Template Conventions**

   * All agents inherit a **standard template** from `agents/templates/agent_template.jue`
   * Required fields:

     * `name`
     * `category` (from starter set)
     * `state`
     * `proposal-function`
     * `update-function`
     * Optional: `salience-function`, `serialize-state`, `validate`

4. **Cross-Module Interfaces**

   * All interactions go through **workspace-provided channels** or `society_of_agents` coordination
   * Direct agent-to-agent mutation **forbidden**; must propose to workspace

5. **Documentation Requirement**

   * Each module should include:

     1. Purpose & inspiration (Minsky, NARS, neuroscience)
     2. Inputs and outputs
     3. Dependencies
     4. Facet mapping (if relevant)
     5. Known ambiguities / TODOs

6. **Facet Integration**

   * Every module optionally registers **belief facets** it influences
   * Naming convention: `facet_<domain>_<aspect>.json`

     * e.g., `facet_perception_salience.json`, `facet_affect_confidence.json`

---

## 4. Opinions & Design Justifications

1. **Separation of Layers**

   * Micro-kernels are atomic and testable
   * Functional agents provide coherent behavior
   * Society-of-agents provides emergent governance
   * Prevents uncontrolled entanglement while supporting emergent complexity

2. **Extensibility**

   * Any new research-inspired module (e.g., additional NARS reasoning rules or neuroscience-inspired networks) can be added without refactoring core workspace

3. **LLM-Agent Friendly**

   * Clear folder names + file naming + template standard allows **LLMs to auto-generate new agents** with minimal human intervention

4. **Belief Facet Theory Mapping**

   * By keeping facets modular, different cognitive dimensions (perception, affect, memory, coherence) can be studied independently and recombined

5. **Testing & Verification**

   * Starter agents + bad agent templates ensure **sanity of workspace and module governance**
   * Tests can be organized to reflect research alignment:

     * e.g., `tests/perception/novelty_detector_test.jue` or `tests/coherence/conflict_detector_test.jue`

---

