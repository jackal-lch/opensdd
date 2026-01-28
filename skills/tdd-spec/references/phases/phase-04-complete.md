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

<step n="4" name="display_summary">
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
| display_summary | Summary displayed | |

**Final checks:**
- [ ] All functions from function_order are implemented
- [ ] All tests pass (none skipped)
- [ ] Code compiles without errors
- [ ] Coverage meets threshold
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
