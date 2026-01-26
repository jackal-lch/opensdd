---
name: build-loop
description: Run the OpenSDD build loop - implement components from spec, extract signatures, compare against spec, fix drift. Use when you have .opensdd/spec.yaml and want to implement code or verify spec alignment.
argument-hint: "[component-name]"
user-invocable: true
---

# Build Loop

Implement → Extract → Compare → Fix loop for spec-driven development.

## Phases

| Phase | Name | Purpose |
|-------|------|---------|
| 0 | Initialize | Check prerequisites (spec.yaml, spec-extract), set up state tracking |
| 1 | Select | Choose component to implement this session |
| 2 | Implement | AI implements selected component from spec |
| 3 | Verify | Extract → Compare → Fix loop until component matches spec |
| 4 | Review | Review all "Extra" items, decide: add to spec or remove from code |

## Flow

```
                    ┌──────────────────────────────────────────┐
                    │                                          │
  ┌────────────┐    │  ┌────────┐   ┌───────────┐   ┌────────┐ │
  │ Initialize │───→│  │ Select │──→│ Implement │──→│ Verify │ │
  │  Phase 0   │    │  │ Phase 1│   │  Phase 2  │   │ Phase 3│ │
  └────────────┘    │  └────────┘   └───────────┘   └────────┘ │
                    │       ↑                            │     │
                    │       └── more components ─────────┘     │
                    │                                          │
                    └──────────────────────────────────────────┘
                                                   │
                                      all components done
                                                   ↓
                                             ┌──────────┐
                                             │  Review  │
                                             │ Phase 4  │
                                             └──────────┘
```

## Start

<start>
Check for existing state:

```bash
test -f ".opensdd/build-loop.state.yaml" && echo "EXISTS" || echo "NOT_FOUND"
```

If output is "EXISTS":
- Run: `python .opensdd/build-loop.state.py status`
- Get `current_phase` from JSON output
- Load: `references/phases/phase-0{current_phase}-*.md`

If output is "NOT_FOUND":
- Load: `references/phases/phase-00-initialize.md`
</start>
