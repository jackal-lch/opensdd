---
phase: 0
name: initialize
next: phase-01-scaffold.md
---

# Phase 0: Initialize

<objective>
Check prerequisites before starting the build.
</objective>

<prerequisite>
No prerequisite. This is the first phase.
</prerequisite>

<input>
No input from previous phase. This phase starts fresh.
</input>

<steps>

<step n="1" name="check_spec">
Verify `.opensdd/spec.yaml` exists:

```bash
test -f ".opensdd/spec.yaml" && echo "FOUND" || echo "NOT_FOUND"
```

If output is "NOT_FOUND":
- Tell user: "No spec.yaml found. Run `/opensdd:spec` first to generate your technical specification."
- STOP workflow.

If output is "FOUND":
- Proceed to next step.
</step>

<step n="2" name="check_tool">
Verify `spec-extract` tool is installed:

```bash
which spec-extract || echo "NOT_FOUND"
```

If output is "NOT_FOUND":
- Tell user:
  ```
  spec-extract tool not found. Install with:

  macOS/Linux:
  curl -fsSL https://raw.githubusercontent.com/jackal-lch/opensdd/main/scripts/install-spec-extract.sh | bash

  Or see: https://github.com/jackal-lch/opensdd#installation
  ```
- STOP workflow.

If tool found:
- Proceed to scaffold.
</step>

</steps>

<output>
Prerequisites verified, ready to scaffold.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| check_spec | spec.yaml exists | |
| check_tool | spec-extract installed | |

If any step failed → address issue and retry.
If all passed → proceed to next.
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue after prerequisites verified.
</checkpoint>

<next>
After verification:

1. Speak: "Prerequisites verified. Starting scaffold..."

2. Load: `phase-01-scaffold.md` (same folder)
</next>
