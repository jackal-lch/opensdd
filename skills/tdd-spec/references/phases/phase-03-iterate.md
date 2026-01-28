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
- `TYPES_FILE_PATH`: Path to types file (update as needed during implementation)
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

Implement code that **achieves the `for:` description** and passes all tests.

**CRITICAL: "Minimal" means NO OVER-ENGINEERING, not PLACEHOLDERS**

```
WRONG interpretation of "minimal":
  - Hardcoded return values
  - Stub implementations that just return the right type
  - Empty type definitions with `pass`

CORRECT interpretation of "minimal":
  - Real logic that achieves the `for:` purpose
  - No premature abstractions
  - No features beyond what's tested
  - No speculative error handling
```

**Implementation Requirements:**

1. **Function MUST achieve its `for:` description**
   - If `for:` says "Authenticate credentials" → actually verify credentials
   - If `for:` says "Create user" → actually persist to storage
   - If `for:` says "Generate token" → actually generate a valid token

2. **Type fields MUST be defined when used**
   - When implementing a function that returns `TokenPair`:
     - Define `TokenPair` fields: `access_token`, `refresh_token`, `expires_in`
   - When implementing a function that takes `CreateUserInput`:
     - Define `CreateUserInput` fields: `email`, `name`, `password`
   - Update type definitions in the types file as you implement

3. **Storage operations MUST persist data**
   - Don't just return objects - store them
   - Use the repository/storage layer defined in spec
   - Verify data can be retrieved after creation

**Implementation checklist:**
- [ ] Function achieves its `for:` description (not just returns right type)
- [ ] Follow exact signature from spec
- [ ] Define type fields when implementing functions that use them
- [ ] Persist data for create/update operations
- [ ] Validate input for validation functions
- [ ] Generate real values (tokens, IDs) not hardcoded strings
- [ ] Handle error cases with appropriate error types
- [ ] Follow conventions from spec

**Example - CORRECT implementation:**

```python
# for: "Authenticate user credentials and create session"
def login(self, credentials: Credentials) -> TokenPair | AuthError:
    # 1. Actually validate credentials against storage
    user = self.user_repo.find_by_email(credentials.email)
    if not user:
        return AuthError(code="USER_NOT_FOUND")

    if not verify_password(credentials.password, user.password_hash):
        return AuthError(code="INVALID_PASSWORD")

    # 2. Actually create a session
    session = Session(
        user_id=user.id,
        created_at=datetime.now(),
        expires_at=datetime.now() + timedelta(hours=24)
    )
    self.session_repo.save(session)

    # 3. Actually generate tokens
    access_token = generate_jwt(user_id=user.id, expires_in=3600)
    refresh_token = generate_refresh_token()

    # 4. Return fully populated type
    return TokenPair(
        access_token=access_token,
        refresh_token=refresh_token,
        expires_in=3600,
        token_type="Bearer"
    )
```

**Update types as you implement:**

When you need a field on a type that's not defined, add it:

```python
# Before (scaffold stub):
@dataclass
class TokenPair:
    pass

# After (during implementation):
@dataclass
class TokenPair:
    access_token: str
    refresh_token: str
    expires_in: int
    token_type: str
```

Edit COMPONENT_FILE_PATH to implement the function.
Edit types file to define type fields as needed.
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
