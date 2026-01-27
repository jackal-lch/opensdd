---
phase: 4
name: verify
next: phase-02-select.md
---

# Phase 4: Verify

<objective>
Run Extract → Compare → Fix loop until component matches spec exactly. Record any "Extra" items for later review.
</objective>

<prerequisite>
Get current component from state:

```bash
python .opensdd/build-spec.state.py status
```

Extract `current_component`. If null:
- Show: "No component to verify."
- Load: `phase-02-select.md`
</prerequisite>

<input>
From state:
- `current_component`: Component being verified

From files:
- `.opensdd/spec.yaml`: Source of truth
- Implemented code in layer folder
</input>

<steps>

<step n="1" name="extract">
**Run spec-extract on the implemented component code:**

Determine component path from spec:
```bash
# Read component's layer from spec, then get path from structure.layers
COMPONENT_PATH=$(python3 -c "
import yaml
with open('.opensdd/spec.yaml', 'r') as f:
    spec = yaml.safe_load(f)
component = spec['components']['[COMPONENT_NAME]']
layer = component.get('layer', 'domain')
root = spec.get('structure', {}).get('root', 'src')
layers = spec.get('structure', {}).get('layers', {})
print(f\"{root}/{layers.get(layer, layer)}\")
")
```

Run extraction:
```bash
spec-extract "$COMPONENT_PATH" -o ".specs/[COMPONENT_NAME].extracted.yaml"
```

If spec-extract fails:
- Show error
- Check if path is correct
- Retry with corrected path

Verify extraction succeeded:
```bash
test -f ".specs/[COMPONENT_NAME].extracted.yaml" && echo "OK" || echo "FAILED"
cat ".specs/[COMPONENT_NAME].extracted.yaml"
```
</step>

<step n="2" name="compare">
**Compare extracted YAML against spec.yaml using verify-compare agent:**

<subagent agent="verify-compare">
  <input>
    <param name="extracted_file">.specs/[COMPONENT_NAME].extracted.yaml</param>
    <param name="spec_file">.opensdd/spec.yaml</param>
    <param name="component_name">[COMPONENT_NAME]</param>
  </input>
</subagent>

The agent performs:
1. Loads extracted YAML (what code actually has)
2. Loads spec.yaml component section (what spec expects)
3. Matches by **intent first**, not just names
4. Applies **language idiom translation** (e.g., `T | null` → `Option<T>`)
5. Checks **structural equivalence**
6. Returns JSON with comparisons, confidence levels, suggested fixes

**Expected agent output:**
```json
{
  "status": "success",
  "summary": {
    "matches": N,
    "drifts": N,
    "missing": N,
    "extras": { "helper": N, "infrastructure": N, "test": N, "new_functionality": N }
  },
  "comparisons": [...],
  "extras": [...]
}
```
</step>

<step n="3" name="show_report">
**Display comparison report:**

```
Verification Report: [COMPONENT_NAME]
─────────────────────────────────────

Summary:
- Matches: X
- Drifts: Y
- Missing: Z
- Extras: W (helpers: H, infrastructure: I, new: N)

Comparisons:
┌─────────────┬────────┬────────────┬────────────┐
│ Spec Item   │ Status │ Matched To │ Confidence │
├─────────────┼────────┼────────────┼────────────┤
│ [item]      │ [status]│ [matched] │ [conf]     │
└─────────────┴────────┴────────────┴────────────┘
```

If drifts > 0, show drift details:
```
Drift Details:
  [spec_item] → [matched_to]
  ├─ Type: [drift_type]
  ├─ Spec expects: [spec_expects]
  ├─ Code has: [code_has]
  └─ Suggested fix: [suggested_fix]
```

If missing > 0, show missing items:
```
Missing Items:
  [spec_item]
  └─ Spec expects: [spec_expects]
```
</step>

<step n="4" name="fix_drift">
**Fix drifts and missing items using suggested fixes:**

For each comparison where `status == "drift"`:

1. Read `suggested_fix` from agent result
2. Apply fix based on drift_type:

   | Drift Type | Action |
   |------------|--------|
   | `naming` | Rename function/parameter/type to match spec |
   | `param` | Change parameter names or types |
   | `return` | Change return type |
   | `structural` | Refactor structure (e.g., method → function) |

3. Update any callers if signature changed

For each comparison where `status == "missing"`:

1. Read `spec_expects` from agent result
2. Implement the missing function/type following spec
3. Use the `for:` description from spec to understand intent

**Do NOT modify extras** - these will be reviewed in Phase 5.

After fixing, note changes:
```
Fixed:
- [spec_item]: [action taken]

Implemented:
- [spec_item]: [created at file:line]
```
</step>

<step n="5" name="record_extras">
**Record extras in state by category:**

For each extra in agent result:

```bash
python .opensdd/build-spec.state.py add-extra \
  --component "[COMPONENT_NAME]" \
  --item "[extra.item]" \
  --signature "[extra.signature]" \
  --file "[extra.file]" \
  --line [extra.line] \
  --classification "[extra.classification]" \
  --used-by "[extra.used_by as comma-separated]" \
  --recommendation "[extra.recommendation]"
```

Categories:
| Classification | Action |
|---------------|--------|
| `helper` | Auto-keep (used by spec functions) |
| `infrastructure` | Auto-keep (language requirements) |
| `test` | Review in Phase 5 |
| `new_functionality` | Review in Phase 5 |
</step>

<step n="6" name="re_extract">
**Re-run extraction after fixes:**

```bash
spec-extract "$COMPONENT_PATH" -o ".specs/[COMPONENT_NAME].extracted.yaml"
```
</step>

<step n="7" name="re_compare">
**Re-run comparison via agent:**

<subagent agent="verify-compare">
  <input>
    <param name="extracted_file">.specs/[COMPONENT_NAME].extracted.yaml</param>
    <param name="spec_file">.opensdd/spec.yaml</param>
    <param name="component_name">[COMPONENT_NAME]</param>
  </input>
</subagent>

**Check result:**

If `summary.drifts > 0` or `summary.missing > 0`:
- Show: "Still have [N] drifts and [M] missing. Continuing fix loop..."
- Return to step 4

If `summary.drifts == 0` and `summary.missing == 0`:
- Component is aligned
- Proceed to verify output
</step>

</steps>

<output>
Component verified and aligned with spec. Extras recorded for Phase 5 review.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| extract | .specs/[component].extracted.yaml created | |
| compare | Agent returned comparison result | |
| show_report | Report displayed | |
| fix_drift | All drifts fixed | |
| record_extras | Extras saved to state | |
| re_extract | Extraction re-run after fixes | |
| re_compare | Zero drift, zero missing | |

**Final alignment check:**
- Zero drifts?
- Zero missing?
- All extras recorded with classification?

If not aligned → continue fix loop (return to step 4).
If aligned → proceed to next.
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue after verification passes.
</checkpoint>

<next>
1. Mark component complete:
   ```bash
   python .opensdd/build-spec.state.py complete-component
   python .opensdd/build-spec.state.py complete-phase 4
   ```

2. Check if all components done:
   ```bash
   python .opensdd/build-spec.state.py status
   ```

   Compare `completed_components` length to `all_components` length.

3. **If all done:**
   - Show: "All components implemented and verified!"
   - Load: `phase-05-review.md`

4. **If more remain:**
   - Show: "Component complete. [X] of [Y] done."
   - Load: `phase-02-select.md` (loop back)
</next>
