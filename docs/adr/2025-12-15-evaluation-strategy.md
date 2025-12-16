# Evaluation Strategy Decision

## Status
Accepted

## Date
2025-12-15

## Context
The project needed to determine the appropriate evaluation strategy for each layer of the system. The decision impacts semantic purity, performance, and the overall architecture's coherence. The key question was whether to use call-by-value (CBV), call-by-name (CBN), or a hybrid approach across the different layers.

## Decision
The chosen solution implements a stratified evaluation strategy:

| Layer      | Evaluation Strategy    | Reason                  |
| ---------- | ---------------------- | ----------------------- |
| Core-World | Call-by-name (or need) | Maximal semantic purity |
| Jue-World  | Call-by-value          | Performance & realism   |
| Dan-World  | Event-driven / async   | Cognition, not eval     |

## Consequences

### Positive
- **Semantic Purity**: Core-World uses call-by-name, which treats arguments as expressions rather than values, aligning perfectly with β-reduction as a logical rule
- **Performance**: Jue-World uses call-by-value, which is predictable, maps cleanly to hardware, and is easier to optimize
- **Clear Separation**: The stratification maintains a clean boundary between semantic and operational concerns
- **Proof Simplicity**: Call-by-name in Core-World makes normalization and equivalence reasoning cleaner
- **Real-world Execution**: Call-by-value in Jue-World provides the performance characteristics needed for real-world applications

### Negative
- **Complexity**: Requires formal equivalence proofs to bridge the semantic gap between layers
- **Learning Curve**: Developers must understand multiple evaluation strategies
- **Debugging Complexity**: Tracing execution across layers with different strategies can be challenging

## Alternatives Considered

1. **Hybrid Core**: A hybrid CBV/CBN approach in Core-World
   - **Rejected**: Would complicate the kernel, pollute proofs with operational detail, and break the idea of a frozen semantic anchor

2. **Uniform Call-by-Value**: Using CBV throughout all layers
   - **Rejected**: Would introduce irreversible commitments in the formal kernel, making evaluation order semantic and complicating equivalence proofs

3. **Uniform Call-by-Name**: Using CBN throughout all layers
   - **Rejected**: Would sacrifice performance in the execution layer and make real-world optimization difficult

## Related
- `docs/architecture/layer_faq.md` - Source document for this decision
- `docs/architecture/architecture.md` - Overall system architecture
- Core-World implementation in `core_world/`
- Jue-World compiler and runtime in `jue_world/`

## Implementation Notes
- Core-World must implement pure λ-calculus with relational semantics
- Jue-World compiler must generate proofs showing CBV implementation preserves CBN meaning
- The bridge between layers requires formal equivalence proofs (e.g., via CPS or strictness analysis)

## Future Considerations
- May need to develop automated proof generation for the CBN→CBV equivalence
- Could explore optimization strategies that maintain proof obligations
- May need to document the proof bridge more thoroughly for developer understanding

## Reviewers
- Project Architect
- Core-World Team
- Jue-World Team

## Revision History
- 2025-12-15: Initial draft based on layer_faq.md analysis
- 2025-12-15: Final approval after team review