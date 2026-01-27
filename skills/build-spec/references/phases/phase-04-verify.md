---
phase: 4
name: verify
next: phase-02-select.md
---

# Phase 4: Verify

<objective>
Run /opensdd:compare to verify implementation. Fix drift if needed. Determine next action.
</objective>

<prerequisite>
`current_component` must be set and just implemented.
</prerequisite>

<input>
From context:
- `current_component`: Component just implemented

From files:
- `.opensdd/spec.yaml`: Source of truth
- Implemented code in layer folder
</input>

<steps>

<step n="1" name="run_compare">
Run /opensdd:compare to see current alignment:

```
/opensdd:compare
```

This updates `.opensdd/compare-result.yaml` with:
- All component statuses
- Extras found
</step>

<step n="2" name="read_result">
Read and parse compare-result.yaml:

```bash
cat .opensdd/compare-result.yaml
```

Extract:
- `summary`: Overall counts
- `components.{current_component}`: Status of just-implemented component
- `extras`: Any new extras detected
</step>

<step n="3" name="check_component">
Check status of `current_component`.

**Schema:** See `skills/compare-spec/references/output-schema.yaml` for compare-result.yaml structure.

Key fields to check:
- `components.{current_component}.status`: match | drift | missing
- `components.{current_component}.provides.{fn}.status`: per-function status
- `components.{current_component}.provides.{fn}.drift_type`: naming | param | return | structural
- `components.{current_component}.provides.{fn}.suggested_fix`: action to resolve drift

Display result:
```
Verify: {current_component}
───────────────────────────

Status: {status}

{If match:}
  All functions match spec.

{If drift:}
  Drifts found:
    {function_name}:
      Type: {drift_type}
      Spec expects: {spec_expects}
      Code has: {code_has}
      Suggested fix: {suggested_fix}
```
</step>

<step n="4" name="handle_drift">
**If status is "drift":**

For each function with drift:
1. Read `suggested_fix` from compare-result
2. Apply fix based on drift_type:

| Drift Type | Action |
|------------|--------|
| `naming` | Rename function/parameter to match spec |
| `param` | Change parameter names or types |
| `return` | Change return type |
| `structural` | Refactor structure |

3. Update any callers if signature changed

After fixing all drifts:
- Log: "Fixed {N} drifts. Re-verifying..."
- Return to step 1 (re-run /opensdd:compare)
</step>

<step n="5" name="on_match">
**If status is "match":**

Component successfully implemented and verified!

Check if more components need implementation:
- Look at `summary.missing` in compare-result.yaml
- Or scan `components:` for any with `status: missing`

Display progress:
```
Verified: {current_component}

Progress: {matches}/{total} components done
Remaining: {missing} components

{If missing > 0:}
  Selecting next component...

{If missing == 0:}
  All components implemented! Proceeding to review...
```
</step>

<step n="6" name="record_extras">
Note any extras found during this component's verification:

From `extras:` in compare-result.yaml, identify extras related to this component's files.

These will be reviewed in Phase 5.
</step>

</steps>

<output>
Component verified. Next action determined.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| run_compare | /opensdd:compare executed | |
| read_result | compare-result.yaml parsed | |
| check_component | Component status identified | |
| handle_drift | All drifts fixed (if any) | |
| on_match | Progress reported | |
| record_extras | Extras noted | |

**Key verification:**
- Current component shows `status: match`?
- If drift was fixed, re-verify passed?
</verify>

<checkpoint required="false">
No user approval needed during component loop.
</checkpoint>

<next>
**If current_component status is "drift":**
- Fix drifts (step 4)
- Return to step 1 (re-run compare)

**If current_component status is "match" AND more missing:**
1. Clear `current_component` from context
2. Load: `phase-02-select.md` (select next)

**If current_component status is "match" AND all done:**
1. Speak: "All {N} components implemented and verified!"
2. Load: `phase-05-review.md`
</next>
