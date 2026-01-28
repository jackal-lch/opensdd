---
name: tdd-spec
description: Implement component using true TDD (Red-Green-Refactor per function). Use at build-spec phase-03 to implement a component with tests driving development. Invoke with component name.
argument-hint: "[component_name]"
user-invocable: true
---

# TDD Spec

Implement a component using true Test-Driven Development.

## Philosophy

```
FOR EACH function in component:
    RED     → Write/enable failing test
    GREEN   → Implement minimal code to pass
    REFACTOR → Improve while staying green
```

Tests are derived from spec.yaml, but implementation follows classic TDD:
- One function at a time
- Tight feedback loop
- Minimal implementation
- Continuous refactoring

## When to Use

- At build-spec phase-03 (implements entire component via TDD)
- Standalone to TDD-implement any spec-defined component

## Input

Component name as argument: `/opensdd:tdd-spec AuthService`

## Output

- Component fully implemented
- All tests passing
- Ready for signature verification (compare-spec)

## Phases

| Phase | Name | Purpose |
|-------|------|---------|
| 1 | Analyze | Parse spec, derive test cases per function |
| 2 | Setup | Create test file (skipped) + component skeleton |
| 3 | Iterate | RED-GREEN-REFACTOR loop per function |
| 4 | Complete | Verify all green, return to caller |

## The TDD Loop (Phase 3)

```
┌─────────────────────────────────────────────────────────┐
│  Function: login                                        │
│                                                         │
│  1. ENABLE tests for login (unskip)                     │
│  2. RUN tests → verify FAIL (RED) ✗                     │
│  3. IMPLEMENT login() minimally                         │
│  4. RUN tests → verify PASS (GREEN) ✓                   │
│  5. REFACTOR login() if needed                          │
│  6. RUN tests → verify still PASS ✓                     │
│  7. NEXT function                                       │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## Start

<start>
Load: `references/phases/phase-01-analyze.md`
</start>
