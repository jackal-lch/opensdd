---
phase: 4
name: reconcile
next: null
---

# Phase 4: Reconcile

<objective>
Verify all fixes worked by re-running compare and generate a complete audit report.
</objective>

<prerequisite>
Phase 3 must be complete (or skipped if no fixes needed).
</prerequisite>

<input>
From Phase 3 (or Phase 1 if skipped):
- `drift_fixes`: Results of drift fixes
- `missing_builds`: Results of missing builds
- `extras_decisions`: Final decisions for all extras
- `promotions`: Spec additions made
- `deletions`: Code removed
</input>

<steps>

<step n="1" name="run_final_compare">
Re-run compare-spec to verify fixes.

1. **Invoke compare-spec skill:**

   Use the Skill tool to run compare-spec:
   ```
   Skill(skill: "opensdd:compare-spec")
   ```

   This will:
   - Extract signatures from fixed codebase using spec-extract
   - Compare against spec.yaml
   - Write new `.opensdd/compare.report.yaml`
   - Display summary

2. **Parse new compare result:**
   - Read `.opensdd/compare.report.yaml`
   - Extract new counts for drifts, missing, extras
   - Compare with Phase 1 initial state
</step>

<step n="2" name="verify_fixes">
Compare before and after to verify success.

**Calculate fix success:**

```yaml
fix_verification:
  drifts:
    before: {count from Phase 1}
    after: {count from new compare}
    fixed: {before - after}
    remaining: {after}

  missing:
    before: {count from Phase 1}
    after: {count from new compare}
    built: {before - after}
    remaining: {after}

  extras_new_functionality:
    before: {count from Phase 1}
    after: {count from new compare}
    resolved: {before - after}
    remaining: {after}

  promotions:
    count: {number of items promoted to spec}
    verified: {are they now in spec?}

  deletions:
    count: {number of items deleted}
    verified: {are they gone from code?}
```

**Identify remaining issues:**

If any drifts/missing remain:
- List each remaining item
- Note why it wasn't fixed (failed, blocked, etc.)
</step>

<step n="3" name="generate_report">
Create comprehensive audit report at `.opensdd/fix.report.yaml`.

**Report schema:**
```yaml
fix_report:
  metadata:
    timestamp: "ISO-8601"
    spec_file: ".opensdd/spec.yaml"
    compare_result: ".opensdd/compare.report.yaml"
    duration_seconds: N

  # ═══════════════════════════════════════════════════════════
  # INITIAL STATE (from Phase 1)
  # ═══════════════════════════════════════════════════════════

  initial_state:
    summary:
      matches: N
      drifts: N
      missing: N
      extras_total: N
      extras_new_functionality: N

    drifts:
      - component: ComponentName
        function: functionName
        drift_type: naming
        spec_expects: "signature"
        code_has: "signature"

    missing:
      - component: ComponentName
        function: functionName

    extras_to_evaluate:
      - item: functionName
        file: "path"

  # ═══════════════════════════════════════════════════════════
  # ACTIONS TAKEN (from Phase 3)
  # ═══════════════════════════════════════════════════════════

  actions:
    drifts_fixed:
      count: N
      details:
        - component: ComponentName
          function: functionName
          action: "renamed from X to Y"
          status: SUCCESS

    drifts_failed:
      count: N
      details:
        - component: ComponentName
          function: functionName
          error: "why it failed"

    missing_built:
      count: N
      details:
        - component: ComponentName
          function: functionName
          file: "path/to/new/file.ext"
          status: SUCCESS

    missing_blocked:
      count: N
      details:
        - component: ComponentName
          function: functionName
          blocked_reason: "what was missing"

    extras_promoted:
      count: N
      details:
        - item: functionName
          visibility: USER_FACING | INTERNAL
          blueprint_verified: true | false
          promoted_to: "components.X.provides.Y"
          spec_entry:
            for: "purpose"
            input: Type
            output: Type

    extras_deleted:
      count: N
      details:
        - item: functionName
          file: "path/to/file.ext"
          reason: "not in blueprint, no dependencies"

    extras_kept:
      count: N
      details:
        - item: functionName
          classification: helper | infrastructure
          reason: "used by spec functions"

    escalations_resolved:
      count: N
      details:
        - item: functionName
          visibility: USER_FACING | INTERNAL
          escalation_type: SCOPE_CREEP | UNCERTAIN
          user_decision: ADD_TO_PRODUCT | PROMOTE | DELETE | KEEP_INFORMAL | KEEP
          blueprint_updated: true | false | null

  # ═══════════════════════════════════════════════════════════
  # FINAL STATE (from Phase 4 compare)
  # ═══════════════════════════════════════════════════════════

  final_state:
    summary:
      matches: N
      drifts: N
      missing: N
      extras_total: N

    remaining_issues:
      - type: drift | missing | blocked
        component: ComponentName
        function: functionName
        reason: "why not resolved"

  # ═══════════════════════════════════════════════════════════
  # SPEC CHANGES MADE
  # ═══════════════════════════════════════════════════════════

  spec_changes:
    functions_added:
      - component: ComponentName
        function: functionName
        for: "purpose"

    types_added:
      - type: TypeName
        for: "purpose"

  # ═══════════════════════════════════════════════════════════
  # OVERALL STATUS
  # ═══════════════════════════════════════════════════════════

  status: CLEAN | PARTIAL | ISSUES_REMAIN

  status_summary: |
    Brief description of final state.
    E.g., "All issues resolved. Code now matches spec."
    Or: "2 items blocked, require manual attention."
```

Write to `.opensdd/fix.report.yaml`.
</step>

<step n="4" name="display_summary">
Display final results to user.

**Display format:**
```
===============================================================
FIX-SPEC COMPLETE
===============================================================

SUMMARY
────────────────────────────────────────────────────────────────
                    Before      After       Change
Drifts:              {N}    ->   {N}        {-N fixed}
Missing:             {N}    ->   {N}        {-N built}
Extras (evaluate):   {N}    ->   {N}        {-N resolved}

ACTIONS TAKEN
────────────────────────────────────────────────────────────────
Drifts fixed:        {count}
Missing built:       {count}
Extras promoted:     {count}
Extras deleted:      {count}
Extras kept:         {count}

SPEC CHANGES
────────────────────────────────────────────────────────────────
Functions added:     {count}
Types added:         {count}

FINAL STATUS: {CLEAN | PARTIAL | ISSUES_REMAIN}
────────────────────────────────────────────────────────────────
{status_summary}

Full report: .opensdd/fix.report.yaml
===============================================================
```

**If ISSUES_REMAIN:**

Show remaining issues:
```
REMAINING ISSUES (require manual attention)
────────────────────────────────────────────────────────────────
[BLOCKED] ComponentName.functionName
  Reason: Missing database schema

[FAILED] AnotherComponent.otherFunction
  Reason: Could not rename, multiple callers
────────────────────────────────────────────────────────────────
```
</step>

</steps>

<output>
- `.opensdd/fix.report.yaml` - Complete audit trail
- Updated `.opensdd/spec.yaml` - With promotions (if any)
- Fixed codebase - Drifts corrected, missing built, extras handled

Workflow complete.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| run_final_compare | New compare.report obtained | Done / Not Done |
| verify_fixes | Before/after comparison complete | Done / Not Done |
| generate_report | fix.report.yaml created | Done / Not Done |
| display_summary | Summary displayed to user | Done / Not Done |

If any step not done -> return and complete it.
If all done -> proceed to checkpoint.
</verify>

<checkpoint required="true">

Use AskUserQuestionTool:
- question: "Fix-spec complete. What would you like to do?"
- header: "Complete"
- options:
  - label: "Done"
    description: "Finish - review fix.report.yaml for details"
  - label: "View full report"
    description: "Display the complete fix.report.yaml"
  - label: "Re-run compare"
    description: "Run compare-spec again to double-check"
  - label: "Re-run fix-spec"
    description: "Start over if issues remain"

On user response:
- "Done": Complete workflow
- "View full report": Display fix.report.yaml, then return to this checkpoint
- "Re-run compare": Invoke compare-spec skill, then return
- "Re-run fix-spec": Restart from Phase 1
</checkpoint>

<next>
After user selects "Done":

Speak:
"Fix-spec complete.

Results saved to .opensdd/fix.report.yaml

{If CLEAN:}
  All issues resolved. Code now matches spec.

{If PARTIAL or ISSUES_REMAIN:}
  Some issues require manual attention. See fix.report.yaml for details."

Workflow complete. No next phase.
</next>
