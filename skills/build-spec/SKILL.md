---
name: build-spec
description: Implement everything in spec.yaml to production-ready code. Use when you have .opensdd/spec.yaml and want to generate a complete, working codebase. Handles project scaffolding, component implementation, and spec alignment verification.
user-invocable: true
---

# Build Spec

Implement EVERYTHING in spec.yaml to production-ready code.

## Core Principle

```
spec.yaml → production-ready codebase
```

Every section in spec.yaml that has physical deliverables gets implemented.

## Phases

| Phase | Name | Purpose |
|-------|------|---------|
| 0 | Initialize | Check prerequisites (spec.yaml, spec-extract), set up state |
| 1 | Scaffold | Generate project infrastructure (non-component deliverables) |
| 2 | Select | Choose next component to implement |
| 3 | Implement | Implement selected component from spec |
| 4 | Verify | Extract → Compare → Fix until aligned with spec |
| 5 | Review | Review "Extra" items found during build |

## Flow

```
┌────────────┐   ┌──────────┐   ┌──────────────────────────────────┐
│ Initialize │──→│ Scaffold │──→│    Component Loop (2-4)          │
│  Phase 0   │   │ Phase 1  │   │  Select → Implement → Verify     │
└────────────┘   └──────────┘   │       ↑________________|         │
                                │       (repeat for each)          │
                                └──────────────────────────────────┘
                                                    │
                                       all components done
                                                    ↓
                                              ┌──────────┐
                                              │  Review  │
                                              │ Phase 5  │
                                              └──────────┘
```

## What Gets Implemented

| Spec Section | Phase | Deliverables |
|--------------|-------|--------------|
| `tech_stack` | Scaffold | Project config (pyproject.toml, package.json, etc.) |
| `structure.layers` | Scaffold | Directory tree |
| `structure.entrypoints` | Scaffold | Entry point files |
| `structure.tests` | Scaffold | Test directory structure |
| `types` | Scaffold | Shared type definitions |
| `deployment` | Scaffold | Deployment files (Dockerfile, k8s, etc.) |
| `components` | Component Loop | All component code with contracts |

## Start

<start>
Check for existing state:

```bash
test -f ".opensdd/build-spec.state.yaml" && echo "EXISTS" || echo "NOT_FOUND"
```

If output is "EXISTS":
- Run: `python .opensdd/build-spec.state.py status`
- Get `current_phase` from JSON output
- Load the corresponding phase file:
  | current_phase | Load |
  |---------------|------|
  | 0 | `references/phases/phase-00-initialize.md` |
  | 1 | `references/phases/phase-01-scaffold.md` |
  | 2 | `references/phases/phase-02-select.md` |
  | 3 | `references/phases/phase-03-implement.md` |
  | 4 | `references/phases/phase-04-verify.md` |
  | 5 | `references/phases/phase-05-review.md` |

If output is "NOT_FOUND":
- Load: `references/phases/phase-00-initialize.md`
</start>
