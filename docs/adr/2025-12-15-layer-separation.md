# Layer Separation and Interaction Model

## Status
Accepted

## Date
2025-12-15

## Context
The project needed to clarify the relationship between Physics-World and Core-World, particularly why they don't interact directly. This architectural decision is crucial for maintaining the separation between operational execution and semantic meaning. The confusion arose from the expectation that the execution layer should understand the semantic layer.

## Decision
The chosen solution maintains strict layer separation with Jue-World as the exclusive bridge:

```
Core-World
  defines meaning of programs

Jue-World
  claims: "this bytecode implements that meaning"

Physics
  blindly executes bytecode
```

## Consequences

### Positive
- **Semantic Purity**: Core-World remains purely semantic with no operational concerns
- **Execution Integrity**: Physics-World remains purely operational with no semantic understanding
- **Clear Responsibilities**: Each layer has a single, well-defined purpose
- **Proof Boundary**: Jue-World serves as the formal bridge with proof obligations
- **Safety**: Prevents operational behavior from influencing semantics
- **Maintainability**: Clean stratification makes the system easier to understand and modify

### Negative
- **Complexity**: Requires formal proof infrastructure in Jue-World
- **Performance Overhead**: Proof generation and verification add computational cost
- **Development Complexity**: Developers must understand the multi-layer architecture
- **Debugging Challenges**: Tracing issues across layers can be more difficult

## Alternatives Considered

1. **Direct Interaction**: Allowing Physics-World to directly execute Core expressions
   - **Rejected**: Would collapse the clean stratification, making semantics depend on memory layout and operational behavior
   - Would create situations where "you would no longer know whether something is *true* or just *happened to run that way*"

2. **Semantic Physics**: Making Physics-World understand Core semantics
   - **Rejected**: Would compromise the execution layer's simplicity and introduce semantic complexity where it's not needed

3. **Unified Layer**: Merging Core and Physics layers
   - **Rejected**: Would destroy the separation of concerns and make the system much harder to reason about formally

## Related
- `docs/architecture/layer_faq.md` - Source document for this decision
- `docs/architecture/architecture.md` - Overall system architecture
- `docs/adr/2025-12-15-evaluation-strategy.md` - Related evaluation strategy decision
- `docs/adr/2025-12-15-language-choice.md` - Related language choice decision
- Core-World implementation in `core_world/`
- Jue-World compiler in `jue_world/`
- Physics-World VM in `physics_world/`

## Implementation Notes
- Core-World must remain non-executable and purely relational
- Physics-World must remain semantically unaware and purely operational
- Jue-World must implement formal equivalence proofs between Core semantics and Physics bytecode
- The proof infrastructure must be robust enough to handle all translation cases
- Each layer must maintain its conceptual integrity without leaking concerns

## Future Considerations
- May need to develop more efficient proof mechanisms for common translation patterns
- Could explore automated proof generation for Jue-World compilation
- Should establish clear documentation of the layer boundaries for new developers
- May need to create validation tools to ensure layer separation is maintained

## Reviewers
- Architecture Team
- Core-World Team
- Jue-World Team
- Physics-World Team
- Security Team

## Revision History
- 2025-12-15: Initial draft based on layer_faq.md analysis
- 2025-12-15: Architecture review and approval
- 2025-12-15: Security review and final approval