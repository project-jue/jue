# Changelog

All notable changes to Project Jue will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

### Core-World
- Ongoing refinement of βη-reduction engine
- Extended proof checker capabilities

### Physics-World
- Capability system implementation (in progress)
- Enhanced actor scheduler

### Integration
- Cross-layer verification bridge

---

## [0.2.0] - 2024-XX-XX (V2 Architecture)

### Added

#### Specification V2
- **Capability-based architecture** across all layers
- [Core V2 Spec](spec/v2/core/core_spec_v2.0.md) - Updated with capability boundary acknowledgment
- [Physics V2 Spec](spec/v2/physics/physics_spec_v2.0.md) - New capability-enforced runtime
- [Jue V2 Spec](spec/v2/jue/jue_spec_v2.0.md) - Capability-aware compilation
- [Dan V2 Spec](spec/v2/dan/dan_spec_v2.0.md) - Capability-centric cognition
- [Cross-Layer Integration](spec/v2/integration.md) - Complete integration architecture

#### Core-World
- Explicit capability boundary documentation
- Standardized CoreExpr serialization format
- Performance optimizations for large term normalization

#### Physics-World
- **Capability System**: Core mechanism for managing privileged operations
- **Enhanced Actor Model**: Actors carry capability sets and request logs
- **Capability-Aware Instructions**: New opcodes (`HasCap`, `RequestCap`, etc.)
- **Scheduler as Authority**: Mediates all capability grants/revocations
- **Comptime Integration**: Sandboxed compile-time execution

#### Jue-World (Specification)
- Capability declarations in type signatures
- Unified type system with capability requirements
- Macro system with capability inheritance
- Error handling with capability-gated inspection

#### Dan-World (Specification)
- **Capability Negotiation Protocols**: Structured capability requests
- **Consensus Mechanisms**: Democratic voting for dangerous capabilities
- **Learning from Violations**: Capability violations as learning signals
- **Social Capability Management**: Multi-agent capability reasoning

### Changed
- Architectural focus shifted to capability-based security model
- Trust tier system now integrates with capability grants
- Self-modification requires capability acquisition

### Documentation
- Added comprehensive [README.md](README.md)
- Added architecture validation reports
- Expanded layer-specific documentation

---

## [0.1.0] - 2024-XX-XX (Foundation)

### Added

#### Core-World
- Pure λ-calculus implementation with De Bruijn indices
- βη-reduction engine with leftmost-outermost (normal order) strategy
- `CoreExpr` types: `Var`, `Lam`, `App`
- Substitution with variable capture avoidance
- Normalization to βη-normal form
- Proof checker foundation for equivalence verification
- Comprehensive test suite (65+ tests)

#### Physics-World
- VM architecture foundation (`VmState`, `OpCode` interpreter)
- Arena-based memory allocator (`ObjectArena`)
- Actor scheduler with round-robin execution
- Structured error system (`ResourceExhaustion`, `IllegalOp`)
- Type system foundation (`JueValue`, `JueType`)
- Test suite (34+ tests)

#### Integration
- Bridge between Core-World and Physics-World
- Cross-layer test infrastructure

#### Specifications V1
- [Core-World V1 Spec](spec/v1/core_spec_v1.0.md)
- [Physics-World V1 Spec](spec/v1/physics_spec_v1.0.md)
- [Physics-World V1.5 Spec](spec/v1/physics_spec_v1.5.md) (VM refinements)
- [Jue-World V1 Spec](spec/v1/jue_spec_v1.0.md)
- [Dan-World V1 Spec](spec/v1/dan_spec_v1.0.md)

#### Documentation
- System introduction and overview
- Engineering reference manual
- Implementation checklist
- Layer-specific documentation (Core, Physics, Jue, Dan)
- Architecture documents
- Testing plans and summaries
- Cheatsheets (De Bruijn indices, Logic, Homoiconicity)

### Technical Details

#### Core-World Implementation
```rust
// Core expression types
pub enum CoreExpr {
    Var(usize),           // De Bruijn index
    Lam(Box<CoreExpr>),   // Lambda abstraction
    App(Box<CoreExpr>, Box<CoreExpr>),  // Application
}
```

#### Physics-World Foundation
```rust
// VM execution model
pub struct VmState {
    stack: Vec<JueValue>,
    heap: ObjectArena,
    step_count: u64,
    step_limit: u64,
}
```

---

## Version History Summary

| Version | Focus                       | Status         |
| ------- | --------------------------- | -------------- |
| 0.2.0   | V2 Capability Architecture  | In Development |
| 0.1.0   | Foundation (Core + Physics) | Complete       |

---

## Roadmap

### Phase 1: Foundation ✅
- Core-World βη-reduction engine
- Physics-World VM architecture
- V1 specifications

### Phase 2: V2 Architecture 🔄
- V2 specifications with capabilities
- Capability system implementation
- Enhanced Physics-World

### Phase 3: Bridge Implementation
- Jue-World compiler frontend
- Trust-tier compilation pipeline
- Cross-layer verification

### Phase 4: Emergence
- Dan-World cognitive modules
- Self-modification protocols
- Multi-agent systems

---

[Unreleased]: https://github.com/project-jue/jue/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/project-jue/jue/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/project-jue/jue/releases/tag/v0.1.0