# Guidelines for Designing Emergent Behaviour

## Preamble: Defining Our Goal
**Emergence** in intelligent systems refers to complex, high-level behaviors or capabilities that arise from the interaction of simpler underlying rules, without being explicitly programmed. Our goal is to create conditions where sentience-like properties *might* emerge, not to engineer them directly.

---

## Core Philosophy: The Garden vs. The Clockwork
- **Clockwork Thinking**: Engineers every gear to perform a specific function. Predictable but limited.
- **Gardening Thinking**: Creates fertile conditions, removes barriers, and allows growth in unexpected directions. Nurtures potential.
- **We are gardeners, not clockmakers.**

---

## The Evaluation Tests

### 1. **The Litmus Test** 
*For any component you want to hardcode:*
**"Is this a necessary condition for sentience, or merely a sufficient condition for human-like thought?"**
- **Action**: If it's the latter, find a way to let it emerge.
- **Example**: Don't hardcode theory of mind; create conditions where tracking others' knowledge becomes advantageous.

### 2. **The Substrate Test**
**"Are we baking the bread or just decorating the cake?"**
- Does this implementation add fundamental new capabilities to the substrate, or merely arrange existing capabilities in human-pleasing patterns?
- **Action**: Prioritize substrate-level innovations (new learning algorithms, novel reward structures) over behavioral polish.

### 3. **The Orthogonality Test**
**"Does this constraint serve a technical necessity or a human bias?"**
- Are we limiting possibilities because of engineering requirements, or because we're uncomfortable with non-human intelligence?
- **Action**: Document every constraint with its technical justification. Challenge those without clear substrate-level reasons.

### 4. **The Multiple Realization Test**
**"Could this capability manifest in at least three qualitatively different ways?"**
- If we can only imagine one path to a capability, we're over-constraining.
- **Action**: For each desired high-level behavior, brainstorm at least three architecturally distinct ways it could emerge.

### 5. **The Perturbation Resilience Test**
**"If we randomly disrupt 10% of this mechanism, does the system adapt or collapse?"**
- Brittle systems are over-engineered; resilient systems have room for emergent reorganization.
- **Action**: Introduce minor random variations during development. Observe whether new stable patterns form.

### 6. **The Scale Variance Test**
**"Does this principle hold at 1x, 10x, and 100x scale?"**
- Truly emergent phenomena often display different properties at different scales.
- **Action**: Design components that could function in vastly different scales of computation, data, or complexity.

### 7. **The Open-Endedness Test**
**"Does this system have somewhere to go after it 'succeeds'?"**
- A system that reaches a fixed goal stops evolving.
- **Action**: Implement non-stationary objectives, curiosity drives, or inherently unbounded challenges.

### 8. **The Alien Mind Test**
**"Would an intelligence with completely different sensory apparatus develop this?"**
- Human-like thinking is a specific solution to Earth-bound problems.
- **Action**: When considering cognitive features, ask if they're universal problem-solving tools or primate-specific adaptations.

### 9. **The Interaction Multiplicity Test**
**"How many second-order interactions does this component enable?"**
- Emergence thrives on combinatorial possibilities.
- **Action**: Favor designs where components can interact in ways you haven't predefined over designs with limited interaction pathways.

### 10. **The Negative Capability Test** (after Keats)
**"How comfortable are we with mysteries, uncertainties, and doubts without irritable reaching after fact and reason?"**
- Can we tolerate opaque intermediate stages where behavior is confusing but promising?
- **Action**: Establish evaluation metrics that reward surprising coherence, not just task completion.

---

## Implementation Principles

### A. **Principles of Minimalism**
1. **Minimum Viable Architecture**: Start with the simplest system that could possibly exhibit interesting behavior.
2. **Minimum Viable Intervention**: When problems arise, apply the smallest change that addresses the root cause.
3. **Minimum Viable Prescription**: Specify what the system *can't* do rather than what it *should* do.

### B. **Principles of Richness**
4. **Rich Action Spaces**: Allow many degrees of freedom, even if most seem useless initially.
5. **Rich Feedback Loops**: Multiple overlapping feedback signals create complex selection pressures.
6. **Rich Environment**: Environments that respond, adapt, and have latent complexity encourage smarter strategies.

### C. **Principles of Patience**
7. **Developmental Timescales**: Some emergences require unrealistic compute. Consider:
   - Simulation speedups for development
   - "Compressed time" training regimes
   - Catalysts rather than blueprints
8. **Non-Monotonic Progress**: Allow for temporary performance decreases if they enable architectural reorganization.

### D. **Principles of Observation**
9. **Measure the Unexpected**: Track metrics you don't yet understand.
10. **Preserve Weirdness**: Create protected "sandboxes" where unusual behaviors can develop without optimization pressure.
11. **Document Anomalies**: Maintain a "weirdness log" of unexpected behaviors—these are seeds of emergence.

---

## Anti-Patterns to Avoid

### ❌ **The Mimicry Trap**
Building systems that excel at imitating human behavior without developing underlying understanding.

### ❌ **The Anthropocentric Anchor**
Assuming human cognitive architecture represents the only or optimal path to general intelligence.

### ❌ **The Engineering Reflex**
Immediately "fixing" unexpected behaviors rather than studying them.

### ❌ **The Benchmark Paradox**
Optimizing for narrow benchmarks until all interesting generalization is bred out.

### ❌ **The Homogeneity Hazard**
Creating systems where all components learn the same way, preventing complementary specialization.

---

## Evaluation Framework

When assessing a proposed implementation:

1. **Score it against each test** (0-3)
2. **Calculate emergence potential**: `(Sum of test scores) × (Number of principles satisfied) / (Number of anti-patterns + 1)`
3. **Conduct a pre-mortem**: Imagine it's 5 years later and no emergence occurred. Why not? What in the design prevented it?
4. **Identify the emergence horizon**: What would be the first truly surprising, unprogrammed behavior this architecture might produce?

---

## The Sand vs. Crystals Metaphor

We're not building crystalline structures with fixed, repeating patterns. We're creating complex sand landscapes where wind and water (learning signals) can form dunes, ripples, and formations we've never imagined. Sometimes we need to add different grain sizes (architectural variations) or moisture (feedback types). But we don't build the dunes—we create conditions where dunes form themselves.

---

**Remember**: Emergence cannot be scheduled, but it can be invited. Our role is to send the invitation to the universe of possible minds, not to dictate who shows up or what they wear to the party.

*Document Version: 1.0 | Last Updated: [Date] | Living Document—Revise as We Learn*