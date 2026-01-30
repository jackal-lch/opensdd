---
name: build-spec
description: Builds all packages from package-spec into production-ready code. Use when you have `.opensdd/packages/manifest.yaml` and want to generate a complete, working codebase. Handles the full build loop (build → probe → retry) with automatic compare-spec verification at the end.
---

# Build Spec

Build all packages into production-ready code with automated verification.

## Phases

| Phase | Name | Purpose |
|-------|------|---------|
| 1 | Initialize | Verify prerequisites, load manifest, display build plan |
| 2 | Build | Execute build→probe→retry loop for each package |
| 3 | Compare | Run compare-spec against full codebase |
| 4 | Summary | Generate unified build-summary.yaml and display results |

## Key Principles

- **Builder ≠ Verifier**: Opus builds, Sonnet probes (different models, clean contexts)
- **Probe, Don't Assert**: Call functions, log output, no assertions
- **BLOCK > FAKE**: Missing info = BLOCKED, never placeholder
- **Fresh context per attempt**: Every Task invocation is clean
- **Fix hints not raw logs**: Structured feedback for retry
- **Record Everything**: Append `builds:` section to each package file with full probe logs

## Start

<start>
Load: `references/phases/phase-01-initialize.md`
</start>
