---
name: build-spec
description: Implement everything in spec.yaml to production-ready code. Use when you have .opensdd/spec.yaml and want to generate a complete, working codebase. Handles project scaffolding, component implementation, and spec alignment verification.
user-invocable: true
---

# Build Spec

Implement everything in spec.yaml to production-ready code.

## Core Principle

```
spec.yaml -> production-ready codebase
```

Every component in spec.yaml gets implemented. compare-spec drives the workflow.

## Phases

| Phase | Name | Purpose |
|-------|------|---------|
| 0 | Initialize | Check prerequisites |
| 1 | Scaffold | Create structure, types, run baseline /opensdd:compare |
| 2 | Select | Pick next missing component from compare-result |
| 3 | Implement | Implement selected component |
| 4 | Verify | Run /opensdd:compare, fix drift if needed |
| 5 | Review | Review extras with user confirmation |

## Flow

```
┌────────────┐   ┌──────────┐   ┌─────────────────────────────────────┐
│ Initialize │──→│ Scaffold │──→│          Component Loop             │
│  Phase 0   │   │ Phase 1  │   │                                     │
└────────────┘   └──────────┘   │  SELECT ← from compare-result       │
                                │    │                                │
                                │    ↓                                │
                                │  IMPLEMENT                          │
                                │    │                                │
                                │    ↓                                │
                                │  VERIFY ← run /opensdd:compare         │
                                │    │                                │
                                │    ├─ drift → fix → re-verify       │
                                │    ├─ match + more missing → SELECT │
                                │    └─ all match → REVIEW            │
                                └─────────────────────────────────────┘
```

## State

No separate state file. **compare-result.yaml IS the state:**

| Need | Where |
|------|-------|
| What's done? | `components[x].status == match` |
| What's missing? | `components[x].status == missing` |
| What needs fixing? | `components[x].status == drift` |
| What extras? | `extras[]` |

## Start

<start>
Load: `references/phases/phase-00-initialize.md`
</start>
