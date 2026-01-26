---
name: create-spec
description: Generate technical specification from product blueprint. Use when you have a product blueprint and need to define system architecture, components, types, and contracts for spec-driven development.
user-invocable: true
---

# Technical Spec

Generate a YAML technical specification that defines system boundaries, components, types, and contracts - designed for spec-driven development where AI implements and validates code against the spec.

## Philosophy

- **Shapes, not implementation**: Define boundaries and contracts, not logic or field details
- **Language-idiomatic**: Conventions and structure follow chosen language's standards
- **AI-implementable**: Spec is "just enough" for AI to implement without questions
- **Drift-detectable**: Structure enables extract→compare→fix loop
- **Cross-referenced**: Types track where they're used, enabling consistency validation

## Phases

| Phase | Name | Purpose |
|-------|------|---------|
| 1 | Foundation | Tech stack, conventions, structure, components, architecture patterns |
| 2 | Types | Define all types (domain, input, output, error, event) |
| 3 | Contracts | Component interfaces, events, dependencies |
| 4 | Integration | External systems, boundaries, generate final output |

## Output Files

| File | Purpose |
|------|---------|
| `.opensdd/spec.yaml` | Technical specification - source of truth (built incrementally) |
| `.opensdd/spec.py` | Validation script for internal consistency checks |

Note: spec.yaml is generated after each phase, building incrementally. If interrupted, partial progress is preserved.

## Start

<start>
Load: `references/phases/phase-01-foundation.md`
</start>
