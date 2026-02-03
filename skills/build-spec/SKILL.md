---
name: build-spec
description: Builds all packages from package-spec into production-ready code. Use when you have `.opensdd/packages/manifest.yaml` and want to generate a complete, working codebase. Handles the full build loop (build → probe → retry).
user-invocable: true
parameters:
  review_mode:
    type: boolean
    default: false
    description: When true, pause after each package for human review. When false (default), auto-continue to next package.
---

# Build Spec

Build all packages into production-ready code with automated verification.

## Phases

| Phase | Name | Purpose |
|-------|------|---------|
| 1 | Initialize | Verify prerequisites, load manifest, display build plan |
| 2 | Build | Execute build→probe→retry loop for each package |

After build completes, run `/opensdd:compare` to verify overall code-spec alignment.

## Usage

```
/opensdd:build              # Auto-continue (default)
/opensdd:build --review     # Pause after each package for human review
```

## Key Principles

- **Builder ≠ Verifier**: Opus builds, Sonnet probes (different models, clean contexts)
- **Probe, Don't Assert**: Call functions, log output, no assertions
- **BLOCK > FAKE**: Missing info = BLOCKED, never placeholder
- **Fresh context per attempt**: Every Task invocation is clean
- **Fix hints not raw logs**: Structured feedback for retry
- **Record Everything**: Probe-agent appends `probe_attempts:` to package file after each probe

## Start

<start>
**Parse arguments first:**
- If args contains `--review` → set `review_mode = true`
- Otherwise → set `review_mode = false` (default)

Store `review_mode` for use in Phase 2.

Load: `references/phases/phase-01-initialize.md`
</start>
