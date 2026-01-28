---
phase: 4
name: complete
next: null
---

# Phase 4: Complete

<objective>
Verify all tests pass, run final checks, and return to caller with fully implemented component.
</objective>

<prerequisite>
Phase 3 must be complete with:
- All functions in function_order implemented
- All tests passing
</prerequisite>

<input>
From previous phases:
- `TARGET_COMPONENT`: Component name
- `function_order`: All functions (now implemented)
- `functions_completed`: List of completed functions
- `TEST_FILE_PATH`: Path to test file
- `COMPONENT_FILE_PATH`: Path to component file
- `TYPES_FILE_PATH`: Path to types file (for the layer)
- `TEST_COMMAND`: Command to run tests
- `COMPILE_CHECK_COMMAND`: Command to verify syntax
</input>

<steps>

<step n="1" name="run_all_tests">
Run complete test suite to verify everything passes.

```bash
{TEST_COMMAND}
```

**Expected:** ALL tests pass (none skipped, none failed)

Display:
```
Final Test Run: {TARGET_COMPONENT}
══════════════════════════════════

Total tests:  {count}
Passed:       {count} ✓
Failed:       0
Skipped:      0

All tests passing!
```

**If any test fails:**
- Identify which function's test failed
- Return to Phase 3 to fix
- This shouldn't happen if TDD was followed correctly
</step>

<step n="2" name="verify_syntax">
Verify all code compiles/parses correctly.

```bash
# By language
{COMPILE_CHECK_COMMAND}
```

Fix any issues before proceeding.
</step>

<step n="3" name="check_coverage">
Run coverage to ensure high test coverage.

**Coverage Commands:**

| Language | Command |
|----------|---------|
| TypeScript | `npx vitest run --coverage` |
| Python | `pytest --cov={module} --cov-report=term` |
| Go | `go test -cover ./...` |
| Rust | `cargo tarpaulin` |

**Expected:** 80%+ coverage (TDD naturally produces high coverage)

Display:
```
Coverage: {TARGET_COMPONENT}
────────────────────────────

Lines:     {percent}%
Branches:  {percent}%
Functions: {percent}%

{If >= 80%:}
Coverage meets threshold ✓

{If < 80%:}
WARNING: Coverage below 80%
TDD should produce high coverage - review implementation
```
</step>

<step n="4" name="verify_no_placeholders">
**Verify no placeholder or fake implementations remain.**

Search for placeholder patterns in component and types files:

```bash
# Search for common placeholder patterns
grep -n "pass$\|TODO\|FIXME\|NotImplemented\|placeholder\|raise.*Not.*implemented" {COMPONENT_FILE_PATH}
grep -n "pass$" {TYPES_FILE_PATH}

# Search for FAKE/STUB patterns - these indicate shortcuts!
grep -in "fake\|stub\|mock\|dummy\|hardcoded\|xxx\|test_\|_test" {COMPONENT_FILE_PATH}

# Search for hardcoded return values that should be real
grep -n "return.*\".*\"\|return.*'.*'" {COMPONENT_FILE_PATH}
```

**Placeholder patterns to detect:**

| Pattern | Issue |
|---------|-------|
| `pass` (alone on line) | Empty function/class body |
| `raise NotImplementedError` | Stub implementation |
| `TODO` / `FIXME` | Incomplete code |
| `"placeholder"` / `"xxx"` | Hardcoded fake values |
| `"fake"` / `"stub"` / `"dummy"` | Fake implementations |
| `return "..."` with literal strings | Hardcoded values instead of real logic |
| `_configs = {}` / `_users = {}` | In-memory storage instead of database |
| `def func(): pass` | No-op function |
| Empty dataclass (only `pass`) | Type without fields |

**CRITICAL: In-memory storage is a placeholder!**

If you see patterns like:
```python
_configs: dict[str, Config] = {}  # THIS IS A PLACEHOLDER!
_users: dict[str, User] = {}      # THIS IS A PLACEHOLDER!
_logs: list[Log] = []             # THIS IS A PLACEHOLDER!
```

This means the component is NOT using the database/repository layer defined in spec.
The component MUST use proper persistence via repository pattern.

**If placeholders found:**

1. Identify which function/type has placeholder
2. Return to Phase 3 to implement properly
3. Tests may be too weak - strengthen assertions to verify:
   - Data is actually persisted (not just stored in memory)
   - External services are actually called (not hardcoded responses)
   - Values are actually computed (not returned as literals)

Display:
```
Placeholder Check: {TARGET_COMPONENT}
─────────────────────────────────────

{If none found:}
No placeholders detected ✓

{If found:}
WARNING: Placeholders/Fakes detected!

  {file}:{line} - {pattern found}

These indicate incomplete implementation:
- Fake/stub/dummy keywords found
- Hardcoded return values
- In-memory storage instead of database
- No-op functions

Tests need stronger assertions to force real implementation.
```
</step>

<step n="5" name="run_integration_tests">
**Run integration tests to verify components work together.**

Integration tests verify:
- Component integrates with its dependencies
- Data flows correctly between components
- Storage operations persist and retrieve correctly

**Create integration test if not exists:**

```python
# tests/integration/test_{component}_integration.py

def test_{component}_end_to_end():
    """Verify component works with real dependencies."""
    # Setup - use real (or realistic) dependencies
    repo = InMemoryRepository()  # Or test database
    component = {Component}(repo=repo)

    # Execute full workflow
    result = component.{main_function}(valid_input)

    # Verify end-to-end
    assert result is not None
    # Verify side effects
    stored = repo.get(result.id)
    assert stored == result
```

**Run integration tests:**

```bash
# By framework
{TEST_COMMAND} tests/integration/
```

**Expected:** All integration tests pass

Display:
```
Integration Tests: {TARGET_COMPONENT}
─────────────────────────────────────

Tests run:   {count}
Passed:      {count} ✓
Failed:      0

Component integrates correctly with dependencies.
```

**If integration tests fail:**
- Component may work in isolation but fail with real dependencies
- Check dependency interfaces match
- Verify storage operations work correctly
</step>

<step n="6" name="verify_types_complete">
**Verify all type definitions have fields (not empty).**

Read types file and check each type used by this component:

```bash
cat {TYPES_FILE_PATH}
```

**For each type returned or accepted by component functions:**

- [ ] Type has fields defined (not just `pass`)
- [ ] Fields have correct types
- [ ] Required fields match spec.yaml type definitions

**If empty types found:**

```python
# WRONG - Empty type
@dataclass
class TokenPair:
    pass

# CORRECT - Fields defined
@dataclass
class TokenPair:
    access_token: str
    refresh_token: str
    expires_in: int
    token_type: str
```

Fix any empty types before proceeding.
</step>

<step n="7" name="display_summary">
Display final TDD summary.

```
TDD Complete: {TARGET_COMPONENT}
════════════════════════════════════════════════════════════════

Component: {COMPONENT_FILE_PATH}
Tests:     {TEST_FILE_PATH}

Functions Implemented (via TDD):
  ✓ {function_1} ({test_count} tests)
  ✓ {function_2} ({test_count} tests)
  ...

Summary:
  Functions:  {count}
  Tests:      {total_tests}
  Coverage:   {percent}%
  Status:     ALL GREEN ✓

TDD Cycle Complete:
  RED    → Tests written before implementation
  GREEN  → Minimal code to pass tests
  REFACTOR → Code improved while staying green

Ready for signature verification (/opensdd:compare)
════════════════════════════════════════════════════════════════
```
</step>

</steps>

<output>
- Component fully implemented via TDD
- All tests passing
- High coverage achieved
- Ready for compare-spec verification
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| run_all_tests | All tests pass | |
| verify_syntax | No syntax errors | |
| check_coverage | Coverage >= 80% | |
| verify_no_placeholders | No placeholders found | |
| run_integration_tests | Integration tests pass | |
| verify_types_complete | All types have fields | |
| display_summary | Summary displayed | |

**Final checks:**
- [ ] All functions from function_order are implemented
- [ ] All tests pass (none skipped)
- [ ] Code compiles without errors
- [ ] Coverage meets threshold
- [ ] No placeholder implementations remain
- [ ] Integration tests pass
- [ ] All types have fields defined (not empty)
</verify>

<checkpoint required="false">
No user approval needed. Return to caller automatically.
</checkpoint>

<next>
Workflow complete. Return to caller (build-spec phase-03).

Speak to user:
"TDD complete for {TARGET_COMPONENT}.

  Functions: {count} implemented
  Tests:     {total} passing
  Coverage:  {percent}%

Component ready for signature verification."
</next>
