---
phase: 3
name: fix
next: phase-04-reconcile.md
---

# Phase 3: Fix

<objective>
Execute fixes for all verified issues: rebuild drifts, build missing, evaluate and act on extras.
</objective>

<prerequisite>
Phase 2 must be complete with verified work items.
</prerequisite>

<input>
From Phase 2:
- `verified_drifts`: Confirmed drifts with fix strategies
- `verified_missing`: Confirmed missing items to build
- `verified_extras`: Extras with classifications and confidence
</input>

<steps>

<step n="1" name="fix_drifts">
Fix each confirmed drift by modifying code to match spec.

**Drifts are simple signature changes - main agent uses Edit tool directly.**

**For each drift in verified_drifts:**

1. **Read the file and identify the change:**
   - Read the source file
   - Locate the drifted function
   - Determine exact edit needed based on fix_strategy from Phase 2

2. **Apply fix using Edit tool:**

   Based on drift_type:
   - **naming**: Rename function/parameters using Edit with replace_all if needed
   - **param**: Change parameter names/types
   - **return**: Change return type annotation
   - **structural**: May require multiple edits

   **RULES:**
   - Match spec signature EXACTLY (names, types, order)
   - Preserve function BODY logic
   - Update callers in same file if function renamed
   - Do NOT change business logic unless required by signature

3. **Probe the fix:**

   After applying edits, invoke probe-agent to verify the fix works:

   ```
   Task(
     subagent_type: "opensdd:probe-agent",
     prompt: """
     Probe the drift fix for {component}.{function}.

     ## Context
     - File: {file}
     - Change: {description of what was changed}
     - Expected: Function signature now matches spec

     ## Probe Instructions
     1. Check file compiles/parses correctly
     2. If function has simple test case, run it
     3. Verify signature matches spec_expects

     ## Return
     - GREEN: Fix verified working
     - FAILED: Fix broke something (include fix_hints)
     - BLOCKED: Cannot verify (missing dependencies)
     """
   )
   ```

4. **Handle probe result:**

   | Result | Action |
   |--------|--------|
   | GREEN | Record as FIXED, continue |
   | FAILED (attempt < 3) | Retry with fix_hints |
   | FAILED (attempt >= 3) | Record as FAILED |
   | BLOCKED | Record as BLOCKED |

5. **Retry loop (if FAILED):**

   ```
   For attempt 2..3:
     - Read fix_hints from probe
     - Re-apply fix addressing the hints
     - Re-probe
     - If GREEN: break, record FIXED
   ```

6. **Record result:**
   ```yaml
   drift_fixes:
     - component: ComponentName
       function: functionName
       status: FIXED | FAILED | BLOCKED
       attempts: N
       changes: [list of changes made]
       error: null | "why it failed"
       fix_hints: null | [hints from last probe]
   ```

**Output:** List of drift fix results
</step>

<step n="2" name="build_missing">
Build each confirmed missing item from spec.

**Reuses existing build-agent (same task as build-spec, same rules).**

**For each missing in verified_missing:**

1. **Prepare build context:**
   ```yaml
   missing_build_context:
     component: ComponentName
     function: functionName (or null for whole component)
     layer: domain | application | infrastructure
     spec_definition:
       provides:
         - functionName:
             for: "purpose from spec"
             input: InputType
             output: OutputType
     target_file: "inferred path based on component and structure"
     fix_hints: null  # Populated on retry
   ```

2. **Invoke build-agent:**

   ```
   Task(
     subagent_type: "opensdd:build-agent",
     prompt: """
     Build missing function from spec.

     ## Missing Item
     Component: {component}
     Function: {function}
     Layer: {layer}
     Target file: {target_file}

     ## Spec Definition
     {spec_definition}

     ## Full Spec
     Read: .opensdd/spec.yaml

     ## Fix Hints (if retry)
     {fix_hints or "None - first attempt"}

     ## Instructions
     1. If target file exists: Add function to it
     2. If not: Create file following project structure
     3. Implement following spec signature EXACTLY
     4. Use dependency injection
     5. Follow language conventions

     ## CRITICAL: BLOCK > FAKE
     If ANY information missing: Return BLOCKED, never placeholder.
     """
   )
   ```

3. **Probe the build:**

   ```
   Task(
     subagent_type: "opensdd:probe-agent",
     prompt: """
     Probe the build for {component}.{function}.

     ## Context
     - File: {target_file}
     - Function: {function}
     - Spec expects: {spec_definition}

     ## Probe Instructions
     1. Check file compiles/parses
     2. Run function with safe test input if possible
     3. Verify signature matches spec

     ## Return
     - GREEN: Build verified working
     - FAILED: Build has issues (include fix_hints)
     - BLOCKED: Cannot verify (missing prerequisites)
     """
   )
   ```

4. **Handle probe result:**

   | Result | Action |
   |--------|--------|
   | GREEN | Record as BUILT, continue |
   | FAILED (attempt < 3) | Retry build with fix_hints |
   | FAILED (attempt >= 3) | Record as FAILED |
   | BLOCKED | Record as BLOCKED |

5. **Retry loop (if FAILED):**

   ```
   For attempt 2..3:
     - Extract fix_hints from probe result
     - Re-invoke build-agent with fix_hints
     - Re-probe
     - If GREEN: break, record BUILT
   ```

6. **Record result:**
   ```yaml
   missing_builds:
     - component: ComponentName
       function: functionName
       status: BUILT | FAILED | BLOCKED
       attempts: N
       file: "path/to/file.ext"
       blocked_reason: null | "why blocked"
       fix_hints: null | [hints from last probe]
   ```

**Output:** List of missing build results
</step>

<step n="3" name="evaluate_extras">
Evaluate each new_functionality extra using the decision tree.

**Reference:** `references/extras-evaluation.md` for complete rules.

**For each extra in verified_extras where classification == new_functionality:**

1. **STEP 1: Visibility Check**

   Determine if this is USER-FACING or INTERNAL:

   | USER-FACING indicators | INTERNAL indicators |
   |------------------------|---------------------|
   | Public API endpoint | Private/unexported function |
   | Called from UI/client | Only called by other internal code |
   | Has user-visible behavior | Implementation detail |
   | Represents a product feature | No external visibility |

   **If INTERNAL:** Continue to Step 2 (can promote directly to spec)

   **If USER-FACING:** Check if in blueprint first (Step 1b)

2. **STEP 1b: Blueprint Check for USER-FACING (scope creep detection)**

   Is this user-facing feature in blueprint.md?

   | Finding | Action |
   |---------|--------|
   | YES (in blueprint) | -> Continue to Step 2 (feature approved) |
   | NO (not in blueprint) | -> SCOPE CREEP - immediate ESCALATE |

   **SCOPE CREEP handling:**
   ```yaml
   extras_decisions:
     - item: functionName
       decision: ESCALATE
       escalation_type: SCOPE_CREEP
       visibility: USER_FACING
       reasoning: "User-facing feature not in blueprint - requires product decision"
   ```

3. **STEP 2: Blueprint Alignment Check (for INTERNAL or approved USER-FACING)**

   Read blueprint.md and search for related features/flows:
   ```
   - Does this function serve a documented feature?
   - Is it mentioned in any user flow?
   - Does it align with the product vision?
   ```

   | Finding | Action |
   |---------|--------|
   | Directly mentioned in blueprint | -> PROMOTE candidate (high confidence) |
   | Indirectly related to feature | -> PROMOTE candidate (medium confidence) |
   | Not related to any feature | -> Continue to Step 3 |

4. **STEP 3: Infrastructure Check**

   Is this cross-cutting infrastructure?
   ```
   - Logging utilities
   - Configuration helpers
   - Monitoring/metrics
   - Caching utilities
   - Common error handlers
   ```

   | Finding | Action |
   |---------|--------|
   | Clearly infrastructure | -> Reclassify as INFRASTRUCTURE, KEEP |
   | Not infrastructure | -> Continue to Step 4 |

5. **STEP 4: Dependency Analysis**

   Check what uses this function:
   ```bash
   grep -r "{function_name}(" src/
   grep -r "import.*{function_name}" src/
   ```

   | Finding | Action |
   |---------|--------|
   | Not imported anywhere | -> DELETE (safe) |
   | Only imported by other extras | -> DELETE (cascade) |
   | Imported by spec code | -> Reclassify as HELPER, KEEP |

6. **STEP 5: Final Decision**

   Based on steps 1-4:
   ```
   If SCOPE_CREEP (from Step 1b):
     -> ESCALATE with special options (see handle_escalations)
   If PROMOTE candidate (from Step 2):
     -> Mark for promotion to spec
   If reclassified as HELPER or INFRASTRUCTURE:
     -> Mark as KEEP
   If safe to delete:
     -> Mark for DELETE
   If still uncertain:
     -> Mark for ESCALATE (general)
   ```

7. **Record decision:**
   ```yaml
   extras_decisions:
     - item: functionName
       decision: PROMOTE | KEEP | DELETE | ESCALATE
       visibility: USER_FACING | INTERNAL
       escalation_type: null | SCOPE_CREEP | UNCERTAIN
       confidence: high | medium | low
       reasoning: "explanation"
       # For PROMOTE:
       promote_to_component: ComponentName
       promote_as_function: functionName
       # For DELETE:
       file_to_modify: "path/to/file.ext"
   ```

**Output:** List of extras with decisions (including visibility classification)
</step>

<step n="4" name="execute_promotions">
Execute promotions for items marked PROMOTE.

**CRITICAL PRINCIPLE: Spec defines boundaries, not implementation.**

When promoting to spec, we capture:
- Function signature (name, input type, output type)
- Purpose (`for:`) - WHAT it does, WHY it exists

We NEVER capture:
- Implementation details (caching, algorithms, protocols)
- Field definitions inside types
- Internal logic

**For each extra where decision == PROMOTE:**

1. **Derive spec addition (boundaries only):**
   - Read the function's code to understand WHAT it does (not HOW)
   - Extract a `for:` description that captures the boundary/contract
   - Identify input/output types (type NAMES only, not fields)

   **`for:` derivation rules:**
   | Code Implementation | WRONG `for:` | CORRECT `for:` |
   |---------------------|--------------|----------------|
   | Uses Redis cache | "retrieves with cache" | "retrieves user by ID" |
   | Validates per RFC | "validates per RFC 5322" | "validates email format" |
   | Retries 3 times | "sends with retry" | "sends email to user" |

2. **Generate spec.yaml patch:**
   ```yaml
   # Add to existing component's provides:
   components:
     {component_name}:
       provides:
         - {function_name}:
             for: "{WHAT it does - boundary only}"
             input: {InputType}
             output: {OutputType}

   # If new types needed, add to types (PURPOSE only, no fields):
   types:
     {NewTypeName}:
       for: "{what this type represents}"
       used:
         - {component}.{function}
   ```

3. **Validate against spec philosophy:**
   - Check `for:` has NO implementation details
   - Check `for:` describes WHAT/WHY, not HOW
   - Check types have purpose only (no field definitions)
   - Check component fit makes sense

4. **Apply to spec.yaml:**
   - Read current spec.yaml
   - Add the new function to component's provides
   - Add any new types (purpose only)
   - Write updated spec.yaml

5. **Record promotion:**
   ```yaml
   promotions:
     - item: functionName
       promoted_to: components.{Component}.provides.{function}
       spec_addition:
         for: "boundary description - WHAT not HOW"
         input: InputType
         output: OutputType
       types_added: [list if any]
   ```

**Output:** List of promotions applied to spec.yaml
</step>

<step n="5" name="execute_deletions">
Execute deletions for items marked DELETE.

**For each extra where decision == DELETE:**

1. **Identify code to remove:**
   - File path
   - Function/class name
   - Line range

2. **Check for dependents:**
   - Double-check nothing imports this
   - If dependents found, do NOT delete, mark as ESCALATE

3. **Remove from code:**
   - If function in multi-function file: Remove just the function
   - If function is only export in file: Delete the file
   - Clean up any now-unused imports

4. **Record deletion:**
   ```yaml
   deletions:
     - item: functionName
       file: "path/to/file.ext"
       action: "removed function" | "deleted file"
   ```

**Output:** List of deletions executed
</step>

<step n="6" name="handle_escalations">
Handle items that need human decision.

**Two types of escalations:**

| Type | Reason | Options |
|------|--------|---------|
| `SCOPE_CREEP` | User-facing feature not in blueprint | Add to product / Delete / Keep informal |
| `UNCERTAIN` | General uncertainty | Promote to spec / Delete / Keep |

**For each extra where decision == ESCALATE:**

### If escalation_type == SCOPE_CREEP:

1. **Present scope creep warning:**
   ```
   ⚠️  SCOPE CREEP DETECTED: {function_name}
   ----------------------------------------------------------------
   File: {file_path}:{line}
   Visibility: USER-FACING
   Signature: {signature}

   What it does: {brief description from code}

   Why escalated: This is a USER-FACING feature that was implemented
   but is NOT in blueprint.md. This represents a product scope change.

   Options:
   A) "Add to product" - You must FIRST add this feature to blueprint.md,
      then we will add it to spec.yaml
   B) "Delete" - Remove this feature from code
   C) "Keep informal" - Leave in code without spec entry (tech debt)
   ----------------------------------------------------------------
   ```

2. **Use AskUserQuestionTool:**
   - question: "User-facing feature '{function_name}' is not in blueprint. What should we do?"
   - header: "Scope Creep"
   - options:
     - label: "Add to product"
       description: "Update blueprint.md first, then add to spec.yaml"
     - label: "Delete"
       description: "Remove from codebase - feature not approved"
     - label: "Keep informal"
       description: "Leave in code without spec entry (technical debt, not recommended)"

3. **Execute decision:**
   - "Add to product":
     a. Tell user: "Please update blueprint.md to include this feature."
     b. WAIT for user confirmation that blueprint is updated
     c. Re-read blueprint.md to verify feature is now documented
     d. If verified: Add to execute_promotions queue
     e. If not verified: Ask again or skip
   - "Delete": Add to execute_deletions queue
   - "Keep informal": Record as kept (tech debt), no action

### If escalation_type == UNCERTAIN (or null):

1. **Present general escalation:**
   ```
   ESCALATION: {function_name}
   ----------------------------------------------------------------
   File: {file_path}:{line}
   Visibility: {INTERNAL | USER_FACING}
   Signature: {signature}

   What it does: {brief description from code}

   Why escalated: {reason - e.g., "Could be useful feature or could be dead code"}

   Options:
   A) Promote to spec (will add to {component})
   B) Delete from code
   C) Keep as-is (no spec change, leave in code)
   ----------------------------------------------------------------
   ```

2. **Use AskUserQuestionTool:**
   - question: "What should we do with {function_name}?"
   - header: "Escalation"
   - options:
     - label: "Promote to spec"
       description: "Add to spec.yaml, keep in code"
     - label: "Delete"
       description: "Remove from codebase"
     - label: "Keep as-is"
       description: "Leave in code without spec entry"

3. **Execute decision:**
   - Promote: Add to execute_promotions queue
   - Delete: Add to execute_deletions queue
   - Keep: Record as kept, no action

### Record all resolutions:

```yaml
escalation_resolutions:
  - item: functionName
    escalation_type: SCOPE_CREEP | UNCERTAIN
    visibility: USER_FACING | INTERNAL
    user_decision: ADD_TO_PRODUCT | PROMOTE | DELETE | KEEP_INFORMAL | KEEP
    blueprint_updated: true | false | null  # Only for SCOPE_CREEP
    executed: true | false
```

**Output:** All escalations resolved
</step>

<step n="7" name="display_fix_summary">
Show summary of all fixes applied.

**Display format:**
```
===============================================================
FIX PHASE COMPLETE
===============================================================

DRIFTS FIXED
  Fixed:  {count}
  Failed: {count}

MISSING BUILT
  Built:   {count}
  Blocked: {count}

EXTRAS PROCESSED
  Promoted to spec: {count}
  Deleted:          {count}
  Kept as-is:       {count}
  Escalated:        {count}

SPEC CHANGES
  Functions added to spec:  {count}
  Types added to spec:      {count}

===============================================================
```
</step>

</steps>

<output>
Fix phase results:
- `drift_fixes`: Results of drift fixes
- `missing_builds`: Results of missing builds
- `extras_decisions`: Final decisions for all extras
- `promotions`: Spec additions made
- `deletions`: Code removed
- `escalation_resolutions`: Human decisions recorded

Ready for reconcile phase.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| fix_drifts | All drifts processed | Done / Not Done |
| build_missing | All missing items processed | Done / Not Done |
| evaluate_extras | All extras have decisions | Done / Not Done |
| execute_promotions | Promotions applied to spec | Done / Not Done |
| execute_deletions | Deletions executed | Done / Not Done |
| handle_escalations | All escalations resolved | Done / Not Done |
| display_fix_summary | Summary displayed | Done / Not Done |

If any step not done -> return and complete it.
If all done -> proceed to next phase.
</verify>

<checkpoint required="false">
No checkpoint if no escalations.

If escalations exist, they are handled interactively in step 6.

Auto-continue to Phase 4 after all fixes complete.
</checkpoint>

<next>
After all fixes complete:

1. Speak: "Fixes complete. Running final comparison..."

2. Load: `phase-04-reconcile.md` (same folder)
</next>
