---
phase: 1
name: initialize
next: phase-02-verify.md
---

# Phase 1: Initialize

<objective>
Load compare.report.yaml, validate findings, and display the fix plan.
</objective>

<prerequisite>
None. This is the first phase.
</prerequisite>

<input>
No input from previous phase.
</input>

<steps>

<step n="1" name="verify_prerequisites">
Check that all required files exist.

**Required files:**
```bash
# Check compare.report exists
test -f .opensdd/compare.report.yaml && echo "compare.report: OK" || echo "compare.report: MISSING"

# Check spec exists
test -f .opensdd/spec.yaml && echo "spec: OK" || echo "spec: MISSING"

# Check blueprint exists (needed for extras evaluation)
BLUEPRINT=""
if [ -f ".opensdd/blueprint.md" ]; then
  echo "blueprint: OK (in .opensdd)"
  BLUEPRINT=".opensdd/blueprint.md"
elif [ -f "blueprint.md" ]; then
  echo "blueprint: OK (in root)"
  BLUEPRINT="blueprint.md"
else
  echo "blueprint: MISSING (optional but recommended)"
fi
```

If compare.report.yaml is missing:
- Show: "Missing: .opensdd/compare.report.yaml. Run /opensdd:compare-spec first."
- STOP workflow.

If spec.yaml is missing:
- Show: "Missing: .opensdd/spec.yaml. Run /opensdd:create-spec first."
- STOP workflow.

If blueprint.md is missing:
- Show warning: "Warning: blueprint.md not found. Extras evaluation will have limited context."
- Continue (blueprint is optional but recommended).
</step>

<step n="2" name="load_compare_result">
Read and parse compare.report.yaml.

1. Read `.opensdd/compare.report.yaml`
2. Validate status field:
   - If `status: error` -> Show error and STOP
   - If `status: success` -> Continue

3. Extract summary:
   ```yaml
   summary:
     total_components: N
     total_types: N
     matches: N
     drifts: N
     missing: N
     total_extras: N
     extras_by_type:
       helper: N
       infrastructure: N
       test: N
       new_functionality: N
   ```

4. Extract detailed findings:
   - `components`: Map of component name -> status, provides details
   - `types`: Map of type name -> status
   - `extras`: Array of extra items with classification
</step>

<step n="3" name="categorize_work">
Categorize findings into work items.

**Build work lists:**

```yaml
work_items:
  drifts: []
    # Items where code exists but differs from spec
    # Each item:
    #   - component: ComponentName
    #   - function: functionName
    #   - drift_type: naming|param|return|structural
    #   - spec_expects: "signature from spec"
    #   - code_has: "signature from code"
    #   - matched_file: "path/to/file.ext"
    #   - suggested_fix: "action to resolve"

  missing: []
    # Items in spec with no code implementation
    # Each item:
    #   - component: ComponentName
    #   - function: functionName (or null for whole component)
    #   - spec_expects: "signature from spec"

  extras:
    keep: []
      # helper, infrastructure, test - no action needed
      # Each item:
      #   - item: functionName
      #   - classification: helper|infrastructure|test
      #   - file: "path/to/file.ext"
      #   - used_by: [list of spec functions if helper]

    evaluate: []
      # new_functionality - needs evaluation
      # Each item:
      #   - item: functionName
      #   - signature: "full signature"
      #   - file: "path/to/file.ext"
      #   - line: N
```

**Extraction process:**

For drifts:
- Scan `components` for items with `status: drift`
- For each component, scan `provides` for functions with `status: drift`
- Extract drift details (drift_type, spec_expects, code_has, suggested_fix)

For missing:
- Scan `components` for items with `status: missing`
- Scan component `provides` for functions with `status: missing`
- Extract spec_expects from each

For extras:
- Scan `extras` array
- Group by classification
- helper/infrastructure/test -> keep list (no action needed)
- new_functionality -> evaluate list (needs decision)

Count totals for each category.
</step>

<step n="4" name="display_plan">
Show the fix plan to user.

**Display format:**
```
===============================================================
FIX-SPEC PLAN
===============================================================

From compare.report.yaml ({timestamp}):
  Matches:  {count} (no action needed)
  Drifts:   {count} (will fix code to match spec)
  Missing:  {count} (will build from spec)
  Extras:   {count} (will evaluate)

Extras breakdown:
  helper:            {count} -> KEEP (supports spec functions)
  infrastructure:    {count} -> KEEP (language requirements)
  test:              {count} -> KEEP (test utilities)
  new_functionality: {count} -> EVALUATE (decision tree)

===============================================================

Phase 1: Initialize   <- CURRENT (complete)
Phase 2: Verify       - Re-verify each finding
Phase 3: Fix          - Execute fixes
Phase 4: Reconcile    - Final compare, report

===============================================================
```

**Check if work exists:**

If drifts == 0 AND missing == 0 AND new_functionality == 0:
- Show: "Nothing to fix! Code matches spec."
- Show: "Extras (helper/infrastructure/test): {count} - automatically kept"
- Skip to Phase 4 for summary only.

If work exists:
- Continue to Phase 2.
</step>

</steps>

<output>
Loaded and categorized work items:
- `work_items.drifts`: List of drifted items to verify and fix
- `work_items.missing`: List of missing items to verify and build
- `work_items.extras.keep`: List of extras to keep (no action)
- `work_items.extras.evaluate`: List of extras needing evaluation

Ready for verification phase.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| verify_prerequisites | Required files exist | Done / Not Done |
| load_compare_report | compare.report parsed successfully | Done / Not Done |
| categorize_work | Work items categorized into lists | Done / Not Done |
| display_plan | Plan displayed to user | Done / Not Done |

If any step not done -> return and complete it.
If all done -> proceed to next phase.
</verify>

<checkpoint required="false">
No checkpoint. Auto-continue based on work item counts.

If no work to do (clean state):
- Skip to Phase 4 (Reconcile) to generate summary report only.

If work exists:
- Auto-continue to Phase 2 (Verify).
</checkpoint>

<next>
Based on work items:

If work_items.drifts + work_items.missing + work_items.extras.evaluate == 0:
- Speak: "Code is clean! Generating summary report..."
- Load: `phase-04-reconcile.md`

Otherwise:
- Speak: "Found {N} items to process. Proceeding to verification..."
- Load: `phase-02-verify.md`
</next>
