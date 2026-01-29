---
phase: 3
name: iterate
next: phase-04-complete.md
---

# Phase 3: Iterate (TDD Loop)

Execute RED → GREEN → REFACTOR for each function in dependency order.

**Before starting:** Read `rules.md § Absolute Rule`

---

## Prerequisites

From Phase 2:
- Test file exists (all tests skipped)
- Component skeleton exists (functions throw "not implemented")
- `function_order` list ready
- `potentially_blocked` list (may be empty) from Phase 1 CoV
- `blocked_functions` initialized as empty list

---

## For Each Function

Repeat Steps 1-11 for each function in `function_order`.

---

### Step 1: Select Function

Get `current_function = function_order[current_function_index]`

Print:
```
┌─────────────────────────────────────────────────────────────┐
│ TDD: {function_name}                                        │
│ Function {index + 1} of {total}                             │
├─────────────────────────────────────────────────────────────┤
│ Purpose: {for description from spec}                        │
│ Tests:   {count} cases                                      │
└─────────────────────────────────────────────────────────────┘
```

---

### Step 2: Check Implementability

Before enabling tests, verify function is implementable.

→ See: `rules.md § Not Implementable Detection`

**Ask:**
1. Can I write the implementation body with info available?
2. What would the implementation look like?

**If NOT IMPLEMENTABLE:**

```yaml
blocked_functions.append({
  name: current_function.name,
  reason: "{specific reason}",
  missing: ["{list what's missing}"],
  phase: "iterate"
})
```

Print:
```
⚠️  BLOCKED: {function_name}
─────────────────────────────────────────
Reason: {reason}
Missing:
  - {missing_item_1}
  - {missing_item_2}

Skipping to next function...
─────────────────────────────────────────
```

→ Skip to Step 11 (Record & Continue)

**If IMPLEMENTABLE:** → Continue to Step 3 (RED — Enable Tests)

---

### Step 3: RED — Enable Tests

Edit test file. Enable tests for this function only.

→ Syntax: `lookup.md § Skip Syntax`

---

### Step 4: RED — Run & Report

Run: `{TEST_COMMAND}` (from `lookup.md § Test Commands`)

Print report using format from `lookup.md § Report Formats → RED Report`

**Do not proceed yet.**

---

### Step 5: RED — Verify

Answer questions from `rules.md § Verification Questions → RED Phase`

**Proceed only when all conditions met.**

If not met: Fix issue → Go back to Step 4.

---

### Step 6: GREEN — Implement

Implement function with REAL logic.

→ Requirements: `rules.md § Implementation Requirements`
→ What to avoid: `rules.md § Fakes vs Real`

**Remember:** `rules.md § Absolute Rule`

---

### Step 7: GREEN — Run & Report

Run: `{TEST_COMMAND}`

Print report using format from `lookup.md § Report Formats → GREEN Report`

**Do not proceed yet.**

---

### Step 8: GREEN — Verify

Answer questions from `rules.md § Verification Questions → GREEN Phase`

**Proceed only when all conditions met.**

If tests fail: Fix implementation → Go back to Step 7.

---

### Step 9: Reality Check

Answer questions from `rules.md § Reality Checks`

**If any check fails:**
1. Test is too weak
2. Strengthen the test (see `rules.md § When to Strengthen Tests`)
3. Re-implement
4. Go back to Step 7

---

### Step 10: REFACTOR (Optional)

Improve code while keeping tests green.

→ Guidelines: `rules.md § Refactoring Rules`

After changes: Run tests → Print `lookup.md § Report Formats → REFACTOR Report`

---

### Step 11: Record & Continue

```python
if current_function was blocked:
    blocked_functions.append(current_function)
else:
    functions_completed.append(current_function.name)

current_function_index += 1
```

Print using `lookup.md § Report Formats → Completion Report`

**Progress shows blocked count:**
```
✓ COMPLETED: {function_name}
  Progress: {completed}/{total} functions ({blocked} blocked)
  Next: {next_function_name or "All done!"}
```

**If more functions:** Go to Step 1 with next function.

**If all done:** Proceed to Phase 4.

---

## Summary

```
FOR EACH function:
  Step 1:  Select
  Step 2:  Check Implementability
           → If NOT IMPLEMENTABLE: skip to Step 11
  Step 3:  Enable tests (RED)
  Step 4:  Run & print RED REPORT
  Step 5:  Verify RED (answer questions)
  Step 6:  Implement (REAL logic)
  Step 7:  Run & print GREEN REPORT
  Step 8:  Verify GREEN (answer questions)
  Step 9:  Reality Check (4 questions)
  Step 10: Refactor (optional)
  Step 11: Record progress (track blocked count)
NEXT function
```

---

<next>
**If more functions remain:** Loop to Step 1

**If all functions complete:** Load `phase-04-complete.md`
</next>
