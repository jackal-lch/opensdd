---
name: compare-spec
description: Compare entire codebase against spec.yaml and report differences (matches, drifts, missing, extras). Use when checking code-spec alignment before changes, after spec modifications, or as diagnostic.
user-invocable: true
---

# Compare Spec

Compare your codebase against spec.yaml to see what's different.

## Purpose

The fundamental operation for spec-driven development: **what does the code have vs what does the spec expect?**

Returns structured diff:
- **matches**: Code fulfills spec exactly
- **drifts**: Code exists but signature differs from spec
- **missing**: In spec, not in code (needs implementation)
- **extras**: In code, not in spec (needs evaluation)

## When to Use

- Before making changes: "What's the current state?"
- After modifying spec.yaml: "What needs to change in code?"
- As diagnostic: "Is my code aligned with spec?"
- In CI/CD: Validate code-spec alignment

## Output

- `.opensdd/compare-result.yaml` - Full structured report
- Terminal summary - Quick overview of differences

## Start

<start>
Load: `references/phases/phase-01-compare.md`
</start>
