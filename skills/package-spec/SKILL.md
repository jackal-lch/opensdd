---
name: package-spec
description: Split spec.yaml + blueprint.md into focused work packages. Creates self-contained, verifiable work units for clean-context builds.
user-invocable: true
---

# Package Spec

Split spec.yaml and blueprint.md into focused, self-contained work packages.

## Purpose

Act as a "tech lead" that:
1. Reads the full spec and blueprint
2. Analyzes dependencies between components
3. Creates work packages in dependency order
4. Defines probe verification for each package
5. Spots issues before any code is written

## Philosophy

From OpenSDD Blueprint:
- **Focused Context > Large Context**: Each package sees only what it needs
- **Builder ≠ Verifier**: Package defines what to probe, not how to pass
- **BLOCK > FAKE**: Package specifies when to block, not when to fake

## Output

```
.opensdd/packages/
├── manifest.yaml           # Index of all packages with build order
├── pkg-00-scaffold.yaml    # Project infrastructure
├── pkg-01-types.yaml       # Shared type definitions
├── pkg-02-{component}.yaml # Component packages (in dependency order)
├── ...
└── pkg-99-integration.yaml # Entry points and wiring
```

## Package Types

| Type | Purpose | Build Order |
|------|---------|-------------|
| `scaffold` | Project structure, configs, infrastructure | 00 |
| `types` | All shared type definitions | 01 |
| `component` | One component with full context | 02-98 |
| `integration` | Entry points, routers, wiring | 99 |

## Phases

| Phase | Name | Purpose |
|-------|------|---------|
| 1 | Analyze | Read docs, build dependency graph, determine order |
| 2 | Create | Generate all packages using templates |
| 3 | Validate | Verify completeness and probe-ability |
| 4 | Finalize | Write packages, generate manifest |

## Package Structure

Each package contains:

```yaml
package:
  id: pkg-{NN}-{name}
  type: scaffold | types | component | integration
  language: typescript | python | go | rust
  build_order: {NN}
  depends_on: [pkg-ids]

scope:
  description: "One-line description"
  files: [list of files to create]

context:
  types:
    - ref: spec.types.TypeName
  dependencies:
    - ref: spec.components.ComponentName

instructions:
  purpose: |
    What this package should achieve
  constraints: [list]
  on_missing_info: BLOCK
  never_fake: [list]

verification:
  safe_to_call:
    - name: functionName
      inputs: { param: value }
  do_not_call: [list]
  criteria: [what to look for in probe output]
```

## Key Principle: Probe-Based Verification

Packages do NOT define tests. They define:
- **safe_to_call**: Functions to call during probing
- **inputs**: Safe test inputs for each function
- **criteria**: What the probe output should reveal

The probe-agent will:
1. Call these functions
2. Log inputs, outputs, types, errors
3. Human judges the log

## Start

<start>
Load: `references/phases/phase-01-analyze.md`
</start>
