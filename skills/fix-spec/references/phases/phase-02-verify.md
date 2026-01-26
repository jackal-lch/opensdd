---
phase: 2
name: verify
next: phase-03-fix.md
---

# Phase 2: Verify

<objective>
Re-verify each finding from compare-spec to catch errors before taking action.
</objective>

<prerequisite>
Phase 1 must be complete with work items categorized.
</prerequisite>

<input>
From Phase 1:
- `work_items.drifts`: List of drifted items
- `work_items.missing`: List of missing items
- `work_items.extras.evaluate`: List of new_functionality extras
</input>

<steps>

<step n="1" name="verify_drifts">
Re-verify each drift to confirm it's truly drifted.

**For each drift in work_items.drifts:**

1. **Read actual code:**
   - Read the file at `matched_file`
   - Find the function/method with the drifted name
   - Extract the actual signature

2. **Read spec definition:**
   - Load spec.yaml
   - Find the component and function
   - Extract the expected signature

3. **Apply Chain of Verification (CoV):**

   Ask these questions:

   Q1: Is this a true drift or naming convention difference?
   - Check: snake_case vs camelCase (Python vs TypeScript)
   - Check: get_user vs getUser vs GetUser
   - If just convention -> Reclassify as MATCH

   Q2: Is this a language idiom difference?
   - Check: Optional[T] vs T | null vs Option<T>
   - Check: Result<T, E> vs throws vs (T, error)
   - If equivalent -> Reclassify as MATCH

   Q3: Is the signature actually different?
   - Compare parameter names
   - Compare parameter types (after idiom translation)
   - Compare return types (after idiom translation)
   - If truly different -> Confirm as DRIFT

   Q4: What's the minimal fix?
   - Rename function?
   - Change parameter names?
   - Change parameter types?
   - Change return type?
   - Record fix_strategy

4. **Record verification result:**
   ```yaml
   verified_drifts:
     - component: ComponentName
       function: functionName
       status: CONFIRMED | RECLASSIFIED_MATCH
       confidence: high | medium | low
       fix_strategy: "rename function" | "change params" | etc
       details: "specific explanation"
   ```

**Output:** List of confirmed drifts with fix strategies
</step>

<step n="2" name="verify_missing">
Re-verify each missing item to confirm it's truly missing.

**For each missing in work_items.missing:**

1. **Search codebase for similar functions:**
   ```bash
   # Search for function name variations
   grep -r "def {function_name}" src/
   grep -r "function {function_name}" src/
   grep -r "fn {function_name}" src/

   # Search for similar names (fuzzy)
   grep -ri "{partial_name}" src/
   ```

2. **Check alternative structures:**
   - Could it be a method on a class instead of standalone function?
   - Could it be in a different file than expected?
   - Could it have a different but equivalent name?

3. **Apply Chain of Verification (CoV):**

   Q1: Could this exist under a different name?
   - Search for synonyms (create vs new vs add)
   - Search for abbreviations (auth vs authentication)
   - If found -> Reclassify as MATCH or DRIFT

   Q2: Could this be a method instead of function?
   - Search for class with component name
   - Check if method exists on class
   - If found -> Reclassify as MATCH or DRIFT

   Q3: Is the component structured differently?
   - Check if component is split across files
   - Check if function is in different module
   - If found -> Reclassify as MATCH or DRIFT

   Q4: Is it truly missing?
   - Exhausted all search options
   - No reasonable alternative found
   - Confirm as MISSING

4. **Record verification result:**
   ```yaml
   verified_missing:
     - component: ComponentName
       function: functionName
       status: CONFIRMED | RECLASSIFIED_MATCH | RECLASSIFIED_DRIFT
       confidence: high | medium | low
       search_results: "what was searched and found"
       details: "explanation"
   ```

**Output:** List of confirmed missing items
</step>

<step n="3" name="verify_extras">
Re-verify each new_functionality extra to confirm classification.

**For each extra in work_items.extras.evaluate:**

1. **Trace call graph:**
   ```bash
   # Find where this function is called
   grep -r "{function_name}(" src/

   # Check imports
   grep -r "from.*import.*{function_name}" src/
   grep -r "import.*{function_name}" src/
   ```

2. **Check if used by spec functions:**
   - For each caller found, check if caller is a spec function
   - If called by spec function -> Reclassify as HELPER

3. **Check if infrastructure:**
   - Is it an error type? (ends with Error, Exception)
   - Is it a base class or interface?
   - Is it a type alias or enum?
   - Is it config/logging/monitoring?
   - If yes -> Reclassify as INFRASTRUCTURE

4. **Apply Chain of Verification (CoV):**

   Q1: Is this truly standalone or used by spec functions?
   - If used by spec code -> Helper, not new_functionality
   - If only used by other extras -> Still new_functionality

   Q2: Could this be infrastructure not detected?
   - Check file location (utils/, lib/, common/)
   - Check naming patterns
   - If infrastructure -> Reclassify

   Q3: Is classification confident?
   - Multiple evidence points -> High confidence
   - Single evidence point -> Medium confidence
   - Uncertain -> Low confidence (may need escalation)

5. **Record verification result:**
   ```yaml
   verified_extras:
     - item: functionName
       original_classification: new_functionality
       verified_classification: CONFIRMED | HELPER | INFRASTRUCTURE
       confidence: high | medium | low
       used_by: [list of callers]
       details: "explanation"
   ```

**Output:** List of extras with verified classifications
</step>

<step n="4" name="display_verified_plan">
Show refined work list after verification.

**Display format:**
```
===============================================================
VERIFICATION COMPLETE
===============================================================

Drifts: {original_count} -> {confirmed_count} confirmed
  Reclassified as match: {count}

Missing: {original_count} -> {confirmed_count} confirmed
  Found under different name: {count}

Extras (new_functionality): {original_count} -> {confirmed_count} remaining
  Reclassified as helper: {count}
  Reclassified as infrastructure: {count}

Confidence levels:
  High:   {count} items (will auto-process)
  Medium: {count} items (will auto-process with extra checks)
  Low:    {count} items (may escalate to human)

===============================================================
```

**Prepare for Phase 3:**
- Confirmed drifts with fix strategies
- Confirmed missing items
- Confirmed new_functionality extras (for evaluation)
- Low confidence items flagged for potential escalation
</step>

</steps>

<output>
Verified work items:
- `verified_drifts`: Confirmed drifts with fix strategies
- `verified_missing`: Confirmed missing items to build
- `verified_extras`: Extras with final classifications and confidence

Ready for fix phase.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| verify_drifts | All drifts verified | Done / Not Done |
| verify_missing | All missing items verified | Done / Not Done |
| verify_extras | All new_functionality extras verified | Done / Not Done |
| display_verified_plan | Verified plan displayed | Done / Not Done |

If any step not done -> return and complete it.
If all done -> proceed to checkpoint.
</verify>

<checkpoint required="true">

**AI Quick Check:**

Before proceeding to fixes, verify:
- All items have verification status?
- Confidence levels assigned?
- No unresolved ambiguities?

**If issues found:**

Speak: "Before proceeding, I found: [specific issue]"

Use AskUserQuestionTool:
- question: "How would you like to handle this?"
- header: "Verify Issue"
- options:
  - label: "Re-verify (Recommended)"
    description: "Re-run verification on problematic items"
  - label: "Continue anyway"
    description: "Proceed to fixes with current verification"
  - label: "Skip to report"
    description: "Skip fixes, generate report only"

**If no issues:**

Use AskUserQuestionTool:
- question: "Verification complete. {N} items confirmed. Proceed to fixes?"
- header: "Proceed"
- options:
  - label: "Proceed to fixes (Recommended)"
    description: "Continue to Phase 3 to apply fixes"
  - label: "Review specific items"
    description: "Deep dive on items before proceeding"
  - label: "Skip to report"
    description: "Skip fixes, generate report only"

On user response:
- "Proceed/Continue": Load Phase 3
- "Re-verify/Review": Return to relevant step
- "Skip to report": Load Phase 4
</checkpoint>

<next>
After user approval:

1. Speak: "Proceeding to apply fixes..."

2. Load: `phase-03-fix.md` (same folder)
</next>
