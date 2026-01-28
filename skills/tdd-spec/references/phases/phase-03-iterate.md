---
phase: 3
name: iterate
next: phase-04-complete.md
---

# Phase 3: Iterate (TDD Loop)

<objective>
Execute RED-GREEN-REFACTOR cycle for each function in order.
</objective>

<prerequisite>
Phase 2 must be complete with:
- Test file created (all tests skipped)
- Component skeleton created
- function_order available
</prerequisite>

<input>
From previous phases:
- `TARGET_COMPONENT`: Component name
- `function_order`: Ordered list of functions with test cases
- `TEST_FILE_PATH`: Path to test file
- `COMPONENT_FILE_PATH`: Path to component file
- `TEST_COMMAND`: Command to run tests

Iteration state:
- `current_function_index`: Which function we're on (starts at 0)
- `functions_completed`: List of completed functions
</input>

<steps>

<step n="1" name="select_function">
Get next function from function_order.

```
current_function = function_order[current_function_index]
```

Display:
```
═══════════════════════════════════════════════════════════════
TDD Iteration: {current_function.name}
Function {current_function_index + 1} of {function_order.length}
═══════════════════════════════════════════════════════════════

Purpose: {current_function.for}
Tests:   {current_function.tests.length} cases
```
</step>

<step n="2" name="red_enable_tests">
**RED PHASE: Enable tests for this function**

Edit test file to unskip/enable tests for current function.

**By framework:**

| Framework | Change |
|-----------|--------|
| Vitest/Jest | `describe.skip('{fn}', ...)` → `describe('{fn}', ...)` |
| pytest | Remove `@pytest.mark.skip` decorator |
| Go | Remove `t.Skip()` call |
| Rust | Remove `#[ignore]` attribute |

After enabling, the test block should run.
</step>

<step n="3" name="red_verify_fail">
**RED PHASE: Verify tests FAIL**

Run tests:
```bash
{TEST_COMMAND}
```

**Expected:** Tests for current_function FAIL (skeleton throws "not implemented")

**Parse output:**
- Count failed tests for this function
- Ensure they fail for right reason (not implemented, not syntax error)

Display:
```
RED: {current_function.name}
─────────────────────────────

Tests enabled: {count}
Tests failed:  {count} ✗ (expected)

Failure reason: "TDD: Not yet implemented"

Ready to implement...
```

**If tests pass unexpectedly:**
- Warning: Function may already be implemented
- Or tests aren't actually testing this function
- Investigate before proceeding

**If syntax/import error:**
- Fix error in test file
- Re-run this step
</step>

<step n="4" name="green_implement">
**GREEN PHASE: Implement the function**

Read the test expectations and implement MINIMAL code to pass.

**TDD Discipline:**
1. Look at failing test assertions
2. Write ONLY enough code to make them pass
3. Don't anticipate future tests
4. Don't over-engineer

**Implementation checklist:**
- [ ] Follow exact signature from spec
- [ ] Handle happy path (make success tests pass)
- [ ] Handle error cases (make error tests pass)
- [ ] Handle edge cases (make edge tests pass)
- [ ] Use types from spec
- [ ] Follow conventions from spec

**Example progression:**

```typescript
// First test: "authenticates valid credentials"
// Minimal implementation:
login(credentials: Credentials): AuthResult | AuthError {
  // Just enough to pass first test
  return new AuthResult({ token: 'xxx', userId: '123' })
}

// Second test: "returns AuthError for invalid password"
// Add password check:
login(credentials: Credentials): AuthResult | AuthError {
  if (credentials.password !== 'validPassword123') {
    return new AuthError('INVALID_PASSWORD')
  }
  return new AuthResult({ token: 'xxx', userId: '123' })
}

// Continue until all tests pass...
```

Edit COMPONENT_FILE_PATH to implement the function.
</step>

<step n="5" name="green_verify_pass">
**GREEN PHASE: Verify tests PASS**

Run tests:
```bash
{TEST_COMMAND}
```

**Expected:** All tests for current_function PASS

Display:
```
GREEN: {current_function.name}
───────────────────────────────

Tests passed: {count}/{count} ✓

All tests passing. Consider refactoring...
```

**If tests still fail:**
1. Read failure message
2. Fix implementation
3. Re-run tests
4. Repeat until green

Do NOT proceed until all tests for this function pass.
</step>

<step n="6" name="refactor">
**REFACTOR PHASE: Improve while staying green**

Review the implementation and improve:

**Refactoring checklist:**
- [ ] Remove duplication
- [ ] Improve naming
- [ ] Extract helper methods (if appropriate)
- [ ] Simplify conditionals
- [ ] Add meaningful comments (only where needed)
- [ ] Ensure code follows conventions

**TDD Discipline:** Only refactor if there's clear benefit. Don't gold-plate.

After any change, run tests to verify still green:
```bash
{TEST_COMMAND}
```

If tests fail → undo change, try different approach.

Display:
```
REFACTOR: {current_function.name}
─────────────────────────────────

Changes made:
  - {description of refactoring, or "No refactoring needed"}

Tests: Still passing ✓
```
</step>

<step n="7" name="record_progress">
Record completion and check for next function.

```
functions_completed.push(current_function.name)
current_function_index += 1
```

Display:
```
Completed: {current_function.name}
──────────────────────────────────

Progress: {current_function_index}/{function_order.length} functions

{If more functions:}
  Next: {function_order[current_function_index].name}

{If all done:}
  All functions implemented!
```
</step>

</steps>

<output>
Current function implemented and tested. Ready for next function or completion.
</output>

<verify>
AI self-verification for current function:

| Step | Expected Output | Status |
|------|-----------------|--------|
| select_function | Function selected | |
| red_enable_tests | Tests enabled (unskipped) | |
| red_verify_fail | Tests fail as expected | |
| green_implement | Function implemented | |
| green_verify_pass | All tests pass | |
| refactor | Code improved (if needed) | |
| record_progress | Progress recorded | |

**TDD verification:**
- Tests failed BEFORE implementation? (true RED)
- Tests pass AFTER implementation? (true GREEN)
- Tests still pass after refactor? (safe REFACTOR)
</verify>

<checkpoint required="false">
No user checkpoint during iteration. TDD provides its own feedback loop.
</checkpoint>

<next>
**If more functions remain:**
```
current_function_index < function_order.length
```
→ Loop back to step 1 (select next function)

**If all functions complete:**
```
current_function_index >= function_order.length
```
1. Speak: "All functions implemented via TDD!"

2. Load: `phase-04-complete.md` (same folder)
</next>
