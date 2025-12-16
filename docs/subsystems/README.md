# Subsystem Documentation

This directory contains documentation for individual subsystems within Project Jue.

## Purpose

Subsystem documentation provides focused, technical documentation for specific components, making it easier for developers and LLMs to understand implementation details.

## Subsystem Structure

Each subsystem has its own directory with the following structure:

```
subsystem_name/
├── README.md            # Overview and key concepts
├── architecture.md      # Internal architecture
├── api.md               # Public interfaces and contracts
├── invariants.md        # Key invariants and assumptions
├── examples.md          # Usage examples
└── testing.md           # Testing strategy and coverage
```

## Subsystem Documentation Template

```markdown
# [Subsystem Name]

## Overview
Brief description of the subsystem's purpose and scope.

## Responsibilities
- Primary responsibility 1
- Primary responsibility 2

## Key Components
- Component 1: Description
- Component 2: Description

## Architecture
High-level architecture diagram and description.

## Public API
Key interfaces, data structures, and contracts.

## Invariants
- Invariant 1: Description
- Invariant 2: Description

## Error Handling
Expected error conditions and recovery strategies.

## Performance Characteristics
Performance expectations and constraints.

## Dependencies
- Internal dependencies
- External dependencies

## Usage Examples
```rust
// Example usage code
```

## Testing Strategy
Approach to testing this subsystem.

## Common Patterns
Recommended usage patterns.

## Anti-Patterns
Patterns to avoid.
```

## Available Subsystems

| Subsystem       | Description              | Status         |
| --------------- | ------------------------ | -------------- |
| `core_world`    | Formal λ-calculus kernel | Complete       |
| `physics_world` | Deterministic VM         | In development |
| `jue_world`     | Execution engine         | Planned        |
| `dan_world`     | Cognitive layer          | Planned        |

## Documentation Standards

- Keep documentation up-to-date with code changes
- Use consistent terminology across subsystems
- Document both "what" and "why"
- Include code examples where helpful
- Link to related ADRs and specifications
- Document invariants explicitly

## Best Practices

- Update subsystem documentation when making changes
- Review documentation as part of code review
- Use diagrams to illustrate complex concepts
- Keep examples simple and focused
- Document edge cases and error conditions
- Link to related test cases