---
phase: 4
name: review
next: null
---

# Phase 4: Review

<objective>
Human-in-loop review of "Extra" items found during the build. Most extras are auto-resolved by category; only `test` and `new_functionality` need user decision.
</objective>

<prerequisite>
Load extras from state:

```bash
python .opensdd/build-loop.state.py get-extras
```

If no extras:
- Speak: "No extra items to review. Build loop complete!"
- STOP workflow (success).
</prerequisite>

<input>
From state:
- `extras`: List of all extra items found across all components (with classification)
From files:
- `.opensdd/spec.yaml`: Current spec (may be updated)
- `.opensdd/blueprint.md`: Product context
</input>

<steps>

<step n="1" name="load_and_categorize">
Load all extras from state:

```bash
python .opensdd/build-loop.state.py get-extras
```

Parse the JSON output. Group by classification:

```
Auto-Kept (no action needed):
──────────────────────────────
Helpers (used by spec functions):
  - validate_path [ConfigLoader] → used by: parseConfig
  - format_error [ErrorHandler] → used by: handleError

Infrastructure (language requirements):
  - ConfigError type [ConfigLoader]
  - UserServiceTrait [UserService]

Needs Review:
──────────────────────────────
Test/Debug Code:
  - debug_dump [ConfigLoader] → recommendation: review_for_removal

New Functionality:
  - export_metrics [MetricsService] → recommendation: review_for_spec
```

**If no `new_functionality` or `test` extras:**
- Speak: "All extras are helpers or infrastructure. Auto-kept. Build complete!"
- Skip to checkpoint with "all done" path
</step>

<step n="2" name="review_needs_decision">
**For each extra where classification == "new_functionality" or "test":**

The verify-compare agent already analyzed these. Present the analysis:

```
Extra: [item]
Component: [component]
Signature: [signature]
Location: [file:line]

Agent Analysis:
- Classification: new_functionality
- Used by: [list or "nothing in spec"]
- Recommendation: [review_for_spec | review_for_removal]

Context Check:
- Does this add value beyond spec?
- Is this a feature that should be documented?
- Or is this dead code / debugging artifact?
```

Use AskUserQuestionTool:
- question: "How should we handle: [item]?"
- options based on recommendation:

If recommendation == "review_for_spec":
  - label: "Add to spec (Recommended)"
    description: "This is valuable functionality, add to spec.yaml"
  - label: "Keep internal"
    description: "Keep but don't add to spec"
  - label: "Remove"
    description: "Delete this code"

If recommendation == "review_for_removal":
  - label: "Remove (Recommended)"
    description: "This appears to be unused/debug code"
  - label: "Keep internal"
    description: "Keep but don't add to spec"
  - label: "Add to spec"
    description: "Actually, this is important - add to spec"

Capture user decision for each item.
</step>

<step n="3" name="apply_decisions">
**Apply user decisions:**

For **"Add to spec"**:
1. Determine which component this belongs to
2. Add to appropriate section (`provides`, `types`, etc.)
3. Generate `for:` description based on what it does

```yaml
# Addition to spec.yaml for [component]:
provides:
  [item_name]:
    for: "[AI-generated description of purpose]"
    params: { [from signature] }
    returns: [from signature]
```

Show the addition to user before applying:

Use AskUserQuestionTool:
- question: "Add this to spec.yaml?"
- options:
  - label: "Yes, add it"
    description: "Add the above to spec.yaml"
  - label: "Edit first"
    description: "Let me modify the description"

For **"Remove"**:
1. Delete the code from the file
2. Remove any imports/references
3. Verify no compilation errors

```bash
# Show what will be removed
echo "Removing [item] from [file]..."
```

For **"Keep internal"**:
- No action needed
- Mark as resolved in state

After each decision applied:
```bash
python .opensdd/build-loop.state.py resolve-extra --item "[ITEM]" --decision "[add_to_spec|keep_internal|remove]"
```
</step>

<step n="4" name="summarize_auto_kept">
**Show summary of auto-kept items:**

```
Auto-Kept Items (no user action needed):
────────────────────────────────────────

Helpers (internal functions used by spec):
  ✓ validate_path → used by parseConfig
  ✓ format_error → used by handleError
  [Total: X helpers]

Infrastructure (language requirements):
  ✓ ConfigError type
  ✓ UserServiceTrait
  [Total: Y infrastructure items]

These items are necessary for spec compliance and have been kept.
```
</step>

<step n="5" name="final_verification">
**Run final full-codebase verification:**

```bash
# Extract entire codebase
spec-extract ./src -o .opensdd/extracted/full-codebase.yaml
```

Compare against full spec:
- All spec components have matching code?
- No unexpected drifts introduced during review?
- Spec and code fully aligned?

Report final status to user.
</step>

</steps>

<output>
All extras resolved. Spec and code fully aligned. Build loop complete.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| load_and_categorize | Extras grouped by classification | |
| review_needs_decision | User decided on each new_functionality/test extra | |
| apply_decisions | All decisions applied | |
| summarize_auto_kept | Summary shown | |
| final_verification | Full codebase aligns with spec | |

If any step incomplete → complete it.
If all done → proceed to checkpoint.
</verify>

<checkpoint required="true">

**AI Quick Check:**

- All `test` and `new_functionality` extras resolved by user?
- Spec.yaml updated for "Add to spec" items?
- Code removed for "Remove" items?
- Final verification shows alignment?

**Present to user:**

```
Build Loop Complete!

Summary:
────────
Components implemented: [N]

Extras resolved: [M]
  ✓ Helpers (auto-kept): [H]
  ✓ Infrastructure (auto-kept): [I]
  ✓ Test/debug (reviewed): [T]
  ✓ Added to spec: [A]
  ✓ Kept internal: [K]
  ✓ Removed: [R]

Final Status: Spec and code are fully aligned.
```

Use AskUserQuestionTool:
- question: "Build loop complete. What would you like to do?"
- options:
  - label: "Finish"
    description: "Exit build loop - all done!"
  - label: "Continue building"
    description: "Return to Select phase (if new components added to spec)"
  - label: "Run verification again"
    description: "Re-run full codebase verification"

On user response:
- "Finish": Proceed to <next>
- "Continue building": Load `phase-01-select.md`
- "Run verification again": Return to step 5
</checkpoint>

<next>
After user chooses "Finish":

1. Complete phase:
   ```bash
   python .opensdd/build-loop.state.py complete-phase 4
   ```

2. Speak to user:
   "Build loop complete! Your code is now fully aligned with spec.yaml.

   To run the build loop again later (e.g., after spec changes), use: /build-loop"

3. No next phase. Workflow complete.
</next>
