---
name: tdd-spec
description: Implement component using true TDD (Red-Green-Refactor per function). Use at build-spec phase-03 to implement a component with tests driving development. Invoke with component name.
argument-hint: "[component_name]"
user-invocable: true
---

# TDD Spec

Implement a component using true Test-Driven Development.

## Usage

```
/opensdd:tdd-spec AuthService
```

## Workflow

```
Phase 1: Analyze  → Parse spec, derive tests, order by dependency
Phase 2: Setup    → Create test file (skipped) + component skeleton
Phase 3: Iterate  → RED → GREEN → REFACTOR per function
Phase 4: Complete → Verify all green, check for fakes
```

## Reference Files

| File | Contains |
|------|----------|
| `rules.md` | Absolute Rule, Reality Checks, Test Smells, Fakes vs Real |
| `lookup.md` | Test commands, Skip syntax, File patterns, Report formats |
| `derivation-rules.md` | How to derive tests from spec.yaml |
| `patterns/*.md` | Language-specific test code examples |

## Start

<start>
Load: `references/phases/phase-01-analyze.md`
</start>
