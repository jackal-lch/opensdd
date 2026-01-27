---
phase: 5
name: review
next: null
---

# Phase 5: Review

<objective>
Auto-resolve "Extra" items found during the build. Helpers and infrastructure are auto-kept. Test/debug code is auto-removed. New functionality is flagged for spec update.
</objective>

<prerequisite>
Load extras from state:

```bash
python .opensdd/build-spec.state.py get-extras
```

If no extras:
- Show: "No extra items to review. Build complete!"
- STOP workflow (success).
</prerequisite>

<input>
From state:
- `extras`: List of all extra items found across all components (with classification)

From files:
- `.opensdd/spec.yaml`: Current spec
</input>

<steps>

<step n="1" name="load_and_categorize">
Load all extras from state:

```bash
python .opensdd/build-spec.state.py get-extras
```

Parse the JSON output. Group by classification:

```
Extras Summary
──────────────

Auto-Kept (no action needed):
  Helpers (used by spec functions): [count]
  Infrastructure (language requirements): [count]

Auto-Removed:
  Test/Debug code: [count]

Needs Attention:
  New functionality: [count]
```
</step>

<step n="2" name="auto_resolve_helpers">
**Auto-keep helpers and infrastructure:**

For each extra where classification == "helper" or "infrastructure":

```bash
python .opensdd/build-spec.state.py resolve-extra \
  --item "[ITEM_NAME]" \
  --decision "keep_internal"
```

These are kept because:
- Helpers: Used by spec functions, necessary for implementation
- Infrastructure: Language requirements (traits, interfaces, error types)

Log:
```
Auto-kept [N] helpers and [M] infrastructure items.
```
</step>

<step n="3" name="auto_remove_test">
**Auto-remove test/debug code:**

For each extra where classification == "test":

1. Remove the code:
   ```bash
   # Delete the function/class from the file
   # Or remove the entire file if it's only test code
   ```

2. Mark as resolved:
   ```bash
   python .opensdd/build-spec.state.py resolve-extra \
     --item "[ITEM_NAME]" \
     --decision "remove"
   ```

Log:
```
Auto-removed [N] test/debug items.
```
</step>

<step n="4" name="flag_new_functionality">
**Flag new functionality for spec update:**

For each extra where classification == "new_functionality":

1. Show the item:
   ```
   New Functionality Found:

   [item_name]
   ├─ Component: [component]
   ├─ Signature: [signature]
   ├─ File: [file:line]
   └─ Used by: [list or "standalone"]
   ```

2. Determine if it should be added to spec:
   - Is it used by multiple spec functions? → Likely should be in spec
   - Is it a standalone feature? → Definitely should be in spec
   - Is it a workaround or hack? → Maybe remove

3. **Auto-decision rule:**
   - If used by spec functions → keep_internal (it's a helper that wasn't detected)
   - If standalone (not used by any spec function) → add_to_spec

4. For items that should be added to spec, generate the spec entry:
   ```yaml
   # Addition to spec.yaml for [component]:
   provides:
     [item_name]:
       for: "[AI-generated description based on implementation]"
       params: { [from signature] }
       returns: [from signature]
   ```

5. Add to spec.yaml and mark resolved:
   ```bash
   python .opensdd/build-spec.state.py resolve-extra \
     --item "[ITEM_NAME]" \
     --decision "add_to_spec"
   ```

Log:
```
New functionality:
- Added to spec: [list]
- Kept internal: [list]
```
</step>

<step n="5" name="final_verification">
**Run final full-codebase verification:**

```bash
# Extract all components
for component in [ALL_COMPONENTS]; do
  spec-extract [COMPONENT_PATH] -o ".specs/${component}.extracted.yaml"
done
```

Run verify-compare on each component to ensure no new drifts introduced during review.

If any drifts found:
- Show: "Review introduced drifts. Fixing..."
- Fix the drifts
- Re-verify

Report final status:
```
Final Verification
──────────────────
All [N] components verified.
Zero drifts.
Spec and code are fully aligned.
```
</step>

<step n="6" name="summary">
**Generate final build summary:**

```
Build Spec Complete!
════════════════════

Components Implemented: [N]/[N] ✓
  [list each component with ✓]

Scaffold Created:
  ✓ Project config
  ✓ Directory structure
  ✓ Entry points
  ✓ Shared types
  ✓ Deployment files (if applicable)

Extras Resolved: [M]
  ✓ Helpers (auto-kept): [count]
  ✓ Infrastructure (auto-kept): [count]
  ✓ Test/debug (auto-removed): [count]
  ✓ New functionality (added to spec): [count]

Final Status: Spec and code are fully aligned.

Files Created:
  [list key files organized by layer/purpose]
```
</step>

</steps>

<output>
All extras resolved. Spec and code fully aligned. Build complete.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| load_and_categorize | Extras grouped by classification | |
| auto_resolve_helpers | Helpers/infrastructure marked kept | |
| auto_remove_test | Test code removed | |
| flag_new_functionality | New functionality handled | |
| final_verification | Full codebase verifies clean | |
| summary | Final summary generated | |

If any step incomplete → complete it.
If all done → proceed to completion.
</verify>

<checkpoint required="false">
No user approval needed. Auto-complete.
</checkpoint>

<next>
1. Complete phase:
   ```bash
   python .opensdd/build-spec.state.py complete-phase 5
   ```

2. Speak:
   "Build complete! Your code is fully aligned with spec.yaml.

   To run again (e.g., after spec changes): /build-spec"

3. No next phase. Workflow complete.
</next>
