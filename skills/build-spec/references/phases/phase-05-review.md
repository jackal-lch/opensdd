---
phase: 5
name: review
next: null
---

# Phase 5: Review

<objective>
Review extras found during build. Apply criteria and get user confirmation.
</objective>

<prerequisite>
All components must have `status: match` in compare-result.yaml.

```bash
cat .opensdd/compare-result.yaml | grep -c "status: missing" || echo "0"
```

If count > 0:
- Not all components done. Return to Phase 2.
</prerequisite>

<input>
From files:
- `.opensdd/compare-result.yaml`: Final comparison with extras
- `.opensdd/spec.yaml`: Current spec (may be updated)
</input>

<steps>

<step n="1" name="load_extras">
Read extras from compare-result.yaml:

```bash
cat .opensdd/compare-result.yaml
```

**Schema:** See `skills/compare-spec/references/output-schema.yaml` for compare-result.yaml structure.

Extract `extras:` array. Each extra has:
- `item`: function/type name
- `kind`: function | type | method
- `signature`: full signature
- `file`: source file path
- `line`: line number
- `classification`: helper | infrastructure | test | new_functionality
- `used_by`: list of spec functions that use this

If no extras:
- Speak: "No extra items found. Build complete!"
- Skip to final summary (step 5)
</step>

<step n="2" name="categorize_extras">
Group extras by classification:

```
Extras Found: {total count}
══════════════════════════

Auto-kept (no action needed):
  Helpers (used by spec functions): {count}
    - {item}: used by {used_by}
  Infrastructure (language requirements): {count}
    - {item}

Needs decision:
  Test/debug code: {count}
    - {item}
  New functionality: {count}
    - {item}: {standalone or used by X}
```
</step>

<step n="3" name="apply_criteria">
Apply criteria to determine recommendations:

| Classification | used_by | Recommendation |
|----------------|---------|----------------|
| `helper` | has consumers | **Keep** - needed by spec functions |
| `infrastructure` | any | **Keep** - language requirements |
| `test` | any | **Recommend delete** - test/debug code |
| `new_functionality` | has consumers | **Keep internal** - helper that wasn't detected |
| `new_functionality` | standalone | **Recommend: add to spec OR delete** |

Build recommendations:
```
Recommendations
───────────────

KEEP (auto):
  - {item}: {reason}

DELETE (recommend):
  - {item}: {reason}

ADD TO SPEC (recommend):
  - {item}: Standalone business logic, consider adding to spec

KEEP INTERNAL (recommend):
  - {item}: Used by spec functions, keep as internal helper
```
</step>

<step n="4" name="get_user_confirmation">
**CHECKPOINT: User decision required**

Present recommendations and ask for confirmation:

Use AskUserQuestionTool:
- question: "Review complete. {N} extras analyzed. Confirm recommendations?"
- options:
  - label: "Confirm recommendations"
    description: "Apply: keep {X}, delete {Y}, add {Z} to spec"
  - label: "Review individually"
    description: "Walk through each extra with me"
  - label: "Keep all"
    description: "Don't delete anything, flag for future review"

**On "Confirm recommendations":**
- Delete items marked for deletion
- Add spec entries for items to add to spec
- Continue to step 5

**On "Review individually":**
- For each extra needing decision:
  - Present details
  - Ask user: Keep / Delete / Add to spec
- After all reviewed, continue to step 5

**On "Keep all":**
- Skip deletions
- Log: "All extras kept. Review later if needed."
- Continue to step 5
</step>

<step n="5" name="final_summary">
Generate final build summary:

```
Build Complete!
═══════════════

Components: {N}/{N} implemented and verified

Files created:
  {Layer}: {count} files
    - {file}: {component}
  ...

Types:
  - Enums: {count}
  - Domain types: {count}

Extras resolved:
  - Kept: {count}
  - Deleted: {count}
  - Added to spec: {count}

Spec-Code Alignment: 100%

To re-run after spec changes: /opensdd:build
```
</step>

</steps>

<output>
Build complete. All components implemented, verified, and extras resolved.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| load_extras | Extras loaded from compare-result | |
| categorize_extras | Extras grouped by classification | |
| apply_criteria | Recommendations generated | |
| get_user_confirmation | User decision received | |
| final_summary | Summary displayed | |

All steps must complete before workflow ends.
</verify>

<checkpoint required="true">
**User approval required for extras handling.**

See step 4 for checkpoint implementation.
</checkpoint>

<next>
After user confirmation and summary:

1. Speak:
   "Build complete! Your code is fully aligned with spec.yaml.

   To run again after spec changes: /opensdd:build"

2. No next phase. Workflow complete.
</next>
