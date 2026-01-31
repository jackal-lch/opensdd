---
name: compare-spec
description: Compare entire codebase against spec.yaml and report differences (matches, drifts, missing, extras). Use when checking code-spec alignment before changes, after spec modifications, after build-spec completes, or as diagnostic.
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

## Usage Modes

### Standalone (User Invoked)

Run `/opensdd:compare-spec` to check alignment anytime:
- Before making changes: "What's the current state?"
- After modifying spec.yaml: "What needs to change in code?"
- As diagnostic: "Is my code aligned with spec?"
- In CI/CD: Validate code-spec alignment

### After Build

Run manually after `build-spec` completes to verify overall alignment:
1. build-spec builds and probes all packages
2. User runs `/opensdd:compare-spec` to check alignment
3. Results saved to `.opensdd/compare.report.yaml`

## Output

- `.opensdd/compare.report.yaml` - Full structured report
- Terminal summary - Quick overview of differences

## Start

<start>
Load: `references/phases/phase-01-compare.md`
</start>
