---
name: create-blueprint
description: AI-guided blueprint creation for greenfield applications. Use when starting a new project, capturing product vision, or when users need help articulating what they want to build.
user-invocable: true
---

# Product Blueprint

AI-guided process to capture your product vision and create a comprehensive, implementation-ready blueprint.

## Phases

| Phase | Name | Purpose |
|-------|------|---------|
| 0 | Initialize | Set up state tracking, confirm project context |
| 1 | Vision | Capture core idea, problem, and value proposition |
| 2 | Users | Define target users, personas, and their goals |
| 3 | Features | Discover, prioritize, and scope features |
| 4 | Flows | Map key user journeys and interactions |
| 5 | Data | Define entities and relationships (non-technical) |
| 6 | Integrations | Identify external systems, APIs, dependencies |
| 7 | Constraints | Capture requirements: performance, security, compliance |
| 8 | Assembly | Compile final blueprint, validate completeness |

## Start

<start>
Check for existing state:

```bash
test -f ".opensdd/blueprint.state.yaml" && echo "EXISTS" || echo "NOT_FOUND"
```

If output is "EXISTS":
- Run: `python .opensdd/blueprint.state.py status`
- Get `current_phase` from JSON output
- Load: `references/phases/phase-0{current_phase}-*.md`

If output is "NOT_FOUND":
- Load: `references/phases/phase-00-initialize.md`
</start>
