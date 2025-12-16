# Architecture Decision Records (ADRs)

This directory contains Architecture Decision Records (ADRs) for Project Jue. ADRs document important architectural decisions, their context, and the reasoning behind them.

## Purpose

ADRs serve to:
- Capture the rationale behind key architectural choices
- Provide historical context for future developers
- Prevent rehashing of settled decisions
- Maintain consistency across the codebase

## ADR Structure

Each ADR follows this template:

```markdown
# [Short Title]

## Status
Proposed / Accepted / Deprecated / Superseded

## Context
The problem being addressed and any relevant background

## Decision
The chosen solution or approach

## Consequences
Positive and negative outcomes of the decision

## Alternatives Considered
Other options that were evaluated

## Related
Links to related ADRs, issues, or documentation
```

## ADR Lifecycle

1. **Proposed**: Initial draft for discussion
2. **Accepted**: Decision has been made and implemented
3. **Deprecated**: No longer relevant but kept for historical context
4. **Superseded**: Replaced by a newer decision

## Creating a New ADR

1. Copy the template from `template.md`
2. Fill in all sections with relevant information
3. Name the file using the format: `YYYY-MM-DD-title.md`
4. Submit for review and discussion
5. Update status as the decision progresses

## ADR Index

| Date       | Title                                      | Status    |
| ---------- | ------------------------------------------ | --------- |
| 2025-12-15 | Evaluation Strategy Decision               | Accepted  |
| 2025-12-15 | Language Choice and Trust Boundaries        | Accepted  |
| 2025-12-15 | Layer Separation and Interaction Model      | Accepted  |

## Best Practices

- Keep ADRs concise and focused
- Include enough context for future understanding
- Document both positive and negative consequences
- Link to related decisions and documentation
- Update status as decisions evolve
- Review ADRs periodically for relevance