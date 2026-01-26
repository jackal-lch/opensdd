---
phase: 1
name: compare
next: null
---

# Phase 1: Compare

<objective>
Extract code signatures from entire codebase, compare against spec.yaml, produce structured diff report.
</objective>

<prerequisite>
Verify spec.yaml exists:

```bash
test -f ".opensdd/spec.yaml" && echo "FOUND" || echo "NOT_FOUND"
```

If output is "NOT_FOUND":
- Tell user: "No spec.yaml found. Run `/opensdd:create-spec` first."
- STOP workflow.

Verify spec-extract tool is installed:

```bash
which spec-extract || echo "NOT_FOUND"
```

If output is "NOT_FOUND":
- Tell user: "spec-extract not found. Install with:
  ```bash
  curl -fsSL https://raw.githubusercontent.com/jackal-lch/opensdd/main/scripts/install-spec-extract.sh | bash
  ```
  Or see: https://github.com/jackal-lch/opensdd#installation"
- STOP workflow.
</prerequisite>

<input>
No input from previous phase. This skill runs standalone.
</input>

<steps>

<step n="1" name="load_spec">
Read and parse spec.yaml to understand what we're comparing against.

```bash
cat .opensdd/spec.yaml
```

Extract:
- `components`: Map of component names to definitions
- `types`: Shared type definitions
- `structure.root`: Source root directory (default: "src")
- `structure.layers`: Map of layer names to directory paths
- `tech_stack.language`: Programming language

Store these values for later steps.
</step>

<step n="2" name="clean_previous">
Remove old comparison artifacts to ensure fresh comparison.

```bash
rm -rf .opensdd/extracted/ .opensdd/compare.report.yaml
```

This ensures:
- No stale extracted files from renamed/deleted source files
- No stale comparison result if this run fails mid-way
</step>

<step n="3" name="extract_code">
Run spec-extract on each layer directory to get current code signatures.

For each layer in `structure.layers`:

```bash
LAYER_PATH="{structure.root}/{layer_directory}"
spec-extract "$LAYER_PATH" -o ".opensdd/extracted/{layer_name}/"
```

Collect all generated .yaml files for comparison.
</step>

<step n="4" name="compare_all">
Invoke `compare-agent` to perform complete bidirectional comparison.

Task(
  subagent_type: "opensdd:compare-agent",
  prompt: """
  ## Input
  - spec_file: .opensdd/spec.yaml
  - extracted_dir: .opensdd/extracted/
  """
)

The agent performs one complete scan that returns:
- **match**: spec item exists in code with correct signature
- **drift**: spec item exists but signature differs
- **missing**: spec item has no implementation
- **extra**: code has item not in spec (classified as helper/infrastructure/test/new_functionality)

Store the agent's JSON result for the next step.
</step>

<step n="5" name="write_result">
Write structured comparison result to `.opensdd/compare.report.yaml`.

**Create output directory if needed:**
```bash
mkdir -p .opensdd
```

Take the JSON result from compare-agent and write as YAML.

**Schema:** See `references/output-schema.yaml` for the authoritative schema definition and example.

The output must conform to that schema exactly. Key sections:
- `status`: "success" or "error"
- `timestamp`: ISO 8601
- `summary`: counts for components, types, matches, drifts, missing, extras
- `components`: per-component status with provides details
- `types`: per-type comparison results
- `extras`: code items not in spec with classification
</step>

<step n="6" name="display_summary">
Display human-readable summary in terminal.

**Format:**

```
Compare Spec Results
════════════════════

Spec: .opensdd/spec.yaml
Code: {structure.root}/

Summary:
  Components: {total} total
  ✓ Matches:  {count}
  ⚠ Drifts:   {count}
  ✗ Missing:  {count}

  Extras:     {count} ({breakdown by type})

{If drifts > 0:}
Drifts:
  {ComponentName}.{function_name}
    └─ {drift_type}: {brief description}
    └─ Fix: {suggested_fix}

{If missing > 0:}
Missing:
  {ComponentName}
    └─ No implementation found

{If new_functionality extras > 0:}
New Functionality (not in spec):
  {item_name} ({file}:{line})
    └─ Consider adding to spec or removing

Full report: .opensdd/compare.report.yaml
```

Use emoji/symbols for quick visual scanning:
- ✓ for matches
- ⚠ for drifts
- ✗ for missing
</step>

</steps>

<output>
- `.opensdd/compare.report.yaml` written with full structured diff
- Terminal summary displayed
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| load_spec | Spec parsed, components/types/structure extracted | |
| clean_previous | .opensdd/extracted/ and compare.report.yaml removed | |
| extract_code | All layer directories extracted to .opensdd/extracted/ | |
| compare_all | Agent returned full comparison result (JSON) | |
| write_result | .opensdd/compare.report.yaml written | |
| display_summary | Terminal summary shown | |

**Verification checks:**
- [ ] .opensdd/compare.report.yaml exists and is valid YAML
- [ ] All components from spec.yaml have a status
- [ ] All extras have classification
- [ ] Summary counts match detailed results

If any check fails → identify issue and fix before completing.
</verify>

<checkpoint required="false">
No user approval needed. This is a diagnostic tool - run to completion.
</checkpoint>

<next>
No next phase. Workflow complete after displaying summary.

Speak to user:
"Comparison complete. See .opensdd/compare.report.yaml for full details."
</next>
