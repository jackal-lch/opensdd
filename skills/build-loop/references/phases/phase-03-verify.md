---
phase: 3
name: verify
next: phase-01-select.md
---

# Phase 3: Verify

<objective>
Run Extract → Compare → Fix loop until component matches spec. Record any "Extra" items for later review.
</objective>

<prerequisite>
Get current component from state:

```bash
python .opensdd/build-loop.state.py status
```

Extract `current_component`. If null:
- Show: "No component to verify."
- Load: `phase-01-select.md`
</prerequisite>

<input>
From state:
- `current_component`: Component being verified
From files:
- `.opensdd/spec.yaml`: Source of truth
- Implemented code in layer folder
</input>

<steps>

<step n="1-2" name="extract_and_compare">
**Invoke verify-compare agent for semantic comparison:**

<subagent agent="verify-compare">
  <input>
    <param name="component_name" from="state:current_component"/>
    <param name="component_path" from="spec:structure.layers[component.layer]"/>
  </input>
</subagent>

The agent performs:
1. Runs `spec-extract` on component code
2. Loads spec.yaml and extracted YAML
3. Matches by **intent first**, not just names
4. Applies **language idiom translation** (e.g., `T | null` → `Option<T>` for Rust)
5. Checks **structural equivalence** (method vs function, etc.)
6. Returns JSON with comparisons, confidence levels, and suggested fixes

**On agent result:**

- If `status == "error"`: Show error, ask user how to proceed
- If `summary.drifts > 0` or `summary.missing > 0`: Proceed to step 3
- If only extras (`summary.extras.test > 0` or `summary.extras.new_functionality > 0`): Proceed to step 5
- If clean (only matches + helpers + infrastructure): Proceed to checkpoint
</step>

<step n="3" name="show_report">
**Display semantic drift report to user:**

```
Component: [COMPONENT_NAME]
Language: [language from result]

Summary:
- Matches: X
- Drifts: Y
- Missing: Z
- Extras: W (helpers: H, new: N)

Comparisons:
┌─────────────┬────────┬────────────┬────────────┐
│ Spec Item   │ Status │ Matched To │ Confidence │
├─────────────┼────────┼────────────┼────────────┤
│ [item]      │ [status]│ [matched] │ [conf]     │
└─────────────┴────────┴────────────┴────────────┘

Drift Details:
For each drift in comparisons where status == "drift":

  [spec_item] → [matched_to]
  ├─ Type: [drift_type]
  ├─ Spec expects: [spec_expects]
  ├─ Code has: [code_has]
  ├─ Difference: [difference]
  └─ Suggested fix: [suggested_fix]

Missing Items:
For each comparison where status == "missing":

  [spec_item]
  └─ Spec expects: [spec_expects]

Low Confidence Matches (needs human review):
For each comparison where confidence == "low":

  [spec_item] → [matched_to]
  └─ Reason: AI is uncertain about this match
```

**If any confidence == "low":**

Use AskUserQuestionTool:
- question: "Some matches have low confidence. How should I proceed?"
- options:
  - label: "Trust AI judgment (Recommended)"
    description: "Proceed with AI's classification"
  - label: "Review each uncertain match"
    description: "I'll verify each low-confidence match manually"
</step>

<step n="4" name="fix_drift">
**Fix drifts and missing items using suggested fixes:**

For each comparison where `status == "drift"`:

1. **Read the suggested_fix** from `details.suggested_fix`
2. **Apply the fix based on drift_type:**

   | Drift Type | Action |
   |------------|--------|
   | `naming` | Rename function/parameter/type to match spec |
   | `param` | Change parameter names or types |
   | `return` | Change return type |
   | `structural` | Refactor structure (e.g., extract method to function) |

3. **Update any callers** if signature changed

For each comparison where `status == "missing"`:

1. **Read spec_expects** from `details.spec_expects`
2. **Implement the missing function/type** following patterns from Phase 2
3. **Use the `for:` description from spec** to understand intent

**Do NOT modify extras** - these will be reviewed in Phase 4.

After fixing, note what was changed:

```
Fixed:
- [spec_item]: [action taken]
- [spec_item]: [action taken]

Implemented:
- [spec_item]: [created at file:line]
```
</step>

<step n="5" name="record_extras">
**Record extras in state by category:**

For each extra in agent result:

```bash
python .opensdd/build-loop.state.py add-extra \
  --component "[COMPONENT_NAME]" \
  --item "[extra.item]" \
  --signature "[extra.signature]" \
  --file "[extra.file]" \
  --line [extra.line] \
  --classification "[extra.classification]" \
  --used-by "[extra.used_by as comma-separated]" \
  --recommendation "[extra.recommendation]"
```

**Summary by category:**

| Classification | Count | Action |
|---------------|-------|--------|
| `helper` | X | Auto-keep (used by spec functions) |
| `infrastructure` | Y | Auto-keep (language requirements) |
| `test` | Z | Review in Phase 4 (likely remove) |
| `new_functionality` | W | Review in Phase 4 (may add to spec) |

Only `test` and `new_functionality` extras need user decision in Phase 4.
</step>

<step n="6" name="re_verify">
**Re-run verification via agent:**

<subagent agent="verify-compare">
  <input>
    <param name="component_name" from="state:current_component"/>
    <param name="component_path" from="spec:structure.layers[component.layer]"/>
  </input>
</subagent>

**Check result:**

- If `summary.drifts > 0` or `summary.missing > 0`:
  - Show: "Still have [N] drifts and [M] missing. Continuing fix loop..."
  - Return to step 4

- If `summary.drifts == 0` and `summary.missing == 0`:
  - Component is aligned
  - Proceed to checkpoint
</step>

</steps>

<output>
Component verified and aligned with spec. Extras recorded for Phase 4 review.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| extract_and_compare | Agent returned JSON result | |
| show_report | Report displayed with confidence levels | |
| fix_drift | All drifts fixed using suggested_fix | |
| record_extras | Extras saved to state with classification | |
| re_verify | Zero drift, zero missing | |

**Final alignment check:**
- Zero drifts?
- Zero missing?
- All extras recorded with classification?

If not aligned → continue fix loop.
If aligned → proceed to checkpoint.
</verify>

<checkpoint required="true">

**AI Quick Check:**

Verify final state:
- Component has zero drift from spec?
- Component has zero missing items?
- All extras recorded in state with category?

**Present to user:**

```
Component [COMPONENT_NAME] verified.

Final Status:
- Matches: X (all high confidence)
- Extras: Y
  - Helpers: H (auto-kept)
  - Infrastructure: I (auto-kept)
  - Test/debug: T (needs review in Phase 4)
  - New functionality: N (needs review in Phase 4)

Progress: [completed] of [total] components done.
```

Use AskUserQuestionTool:
- question: "Component aligned with spec. What's next?"
- options:
  - label: "Next component (Recommended)"
    description: "Mark complete and implement next component"
  - label: "Review extras now"
    description: "Jump to Phase 4 to review new functionality extras"
  - label: "Re-implement this component"
    description: "Start over with this component"

On user response:
- "Next component": Proceed to <next> (back to Phase 1)
- "Review extras now": Load `phase-04-review.md`
- "Re-implement": Load `phase-02-implement.md`
</checkpoint>

<next>
After user chooses "Next component":

1. Mark component complete:
   ```bash
   python .opensdd/build-loop.state.py complete-component
   python .opensdd/build-loop.state.py complete-phase 3
   ```

2. Check if all components done:
   ```bash
   python .opensdd/build-loop.state.py status
   ```

   Compare `completed_components` length to `all_components` length.

3. If all done:
   - Speak: "All components implemented and verified! Proceeding to review extras..."
   - Load: `phase-04-review.md`

4. If more remain:
   - Speak: "Component complete. [X] of [Y] done. Selecting next component..."
   - Load: `phase-01-select.md` (loop back)
</next>
