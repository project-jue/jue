# Language Choice and Trust Boundaries

## Status
Accepted

## Date
2025-12-15

## Context
The project needed to establish clear language boundaries and trust levels for each layer of the system. This decision is critical for maintaining system safety, preventing self-modification catastrophes, and ensuring the integrity of the formal semantic core. The question was whether Physics and Core worlds should be written in Rust permanently, and what the implications would be for self-hosting and system evolution.

## Decision
The chosen solution establishes permanent language boundaries:

| Layer      | Self-hosted? | Mutable?        | Language | Reason                           |
| ---------- | ------------ | --------------- | -------- | -------------------------------- |
| Physics    | ❌            | ❌               | Rust     | Trust boundary, memory safety    |
| Core-World | ❌            | ❌               | Rust     | Semantic anchor, proof integrity |
| Jue-World  | ✅            | ⚠️ (proof-gated) | Jue      | Controlled evolution             |
| Dan-World  | ✅            | ✅ (governed)    | Jue      | Cognitive flexibility            |

## Consequences

### Positive
- **Safety**: Physics-World in Rust prevents self-modification of the execution substrate
- **Semantic Integrity**: Core-World in Rust prevents redefinition of truth and logical catastrophes
- **Memory Safety**: Rust provides memory safety without garbage collection
- **Explicit Control**: Rust gives explicit control over layout and atomics
- **Isolation**: Rust isolates the Physics-World from self-modifying code
- **Clear Boundaries**: The separation prevents the system from redefining its own foundations

### Negative
- **Flexibility Limits**: Core and Physics layers cannot participate in autopoiesis
- **Development Constraints**: Some optimizations may be harder to implement in Rust
- **Learning Curve**: Developers must be proficient in both Rust and Jue
- **Integration Complexity**: Bridging between Rust and Jue layers requires careful design

## Alternatives Considered

1. **Self-hosted Core**: Rewriting Core-World in Jue
   - **Rejected**: Would allow the system to redefine truth, creating a Gödel catastrophe where the system could modify its own semantic foundations

2. **Mutable Physics**: Making Physics-World modifiable by Dan-World
   - **Rejected**: Would compromise the execution substrate's integrity and allow runtime behavior to influence semantics

3. **Uniform Language**: Using only Rust or only Jue throughout
   - **Rejected**: Would either sacrifice semantic purity (all Rust) or safety (all Jue)

## Related
- `docs/architecture/layer_faq.md` - Source document for this decision
- `docs/architecture/architecture.md` - Overall system architecture
- `docs/adr/2025-12-15-evaluation-strategy.md` - Related evaluation strategy decision
- Rust implementation in `physics_world/` and `core_world/`

## Implementation Notes
- Physics-World must remain the unmodifiable execution substrate
- Core-World must remain the frozen semantic anchor and proof checker
- Jue-World can evolve but only with formal proofs of semantic preservation
- Dan-World can self-modify but only through governed mutation protocols
- The trust boundary between Rust and Jue layers must be strictly maintained

## Future Considerations
- May need to develop more sophisticated proof-carrying code mechanisms
- Could explore formal verification of the Rust implementation
- May need to document the trust boundary more thoroughly for security audits
- Should establish clear governance protocols for Jue-World mutations

## Reviewers
- Security Team
- Core-World Team
- Physics-World Team
- Project Architect

## Revision History
- 2025-12-15: Initial draft based on layer_faq.md analysis
- 2025-12-15: Security review and approval
- 2025-12-15: Final approval after team review