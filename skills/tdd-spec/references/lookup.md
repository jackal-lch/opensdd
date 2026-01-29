# TDD Lookup Tables

Single source of truth for all lookup data. Phase files reference these by section name.

---

## § Test Commands

| Language | TEST_COMMAND | COMPILE_CHECK |
|----------|--------------|---------------|
| typescript | `npx vitest run` | `npx tsc --noEmit` |
| python | `pytest` | `python -m py_compile {file}` |
| go | `go test ./...` | `go build ./...` |
| rust | `cargo test` | `cargo check` |

---

## § Skip Syntax

How to skip/unskip tests by framework:

| Framework | Skip | Unskip |
|-----------|------|--------|
| Vitest/Jest | `describe.skip(...)` | `describe(...)` |
| pytest | `@pytest.mark.skip` | Remove decorator |
| Go | `t.Skip()` at start | Remove `t.Skip()` |
| Rust | `#[ignore]` | Remove attribute |

---

## § File Patterns

### Test Files

| Language | Pattern | Example |
|----------|---------|---------|
| typescript | `{tests}/{component}.test.ts` | `tests/auth-service.test.ts` |
| python | `{tests}/test_{component}.py` | `tests/test_auth_service.py` |
| go | `{layer}/{component}_test.go` | `domain/auth_service_test.go` |
| rust | `tests/{component}_test.rs` | `tests/auth_service_test.rs` |

### Component Files

| Language | Pattern | Example |
|----------|---------|---------|
| typescript | `{layer}/{component}.ts` | `src/domain/auth-service.ts` |
| python | `{layer}/{component}.py` | `src/domain/auth_service.py` |
| go | `{layer}/{component}.go` | `domain/auth_service.go` |
| rust | `{layer}/{component}.rs` | `src/domain/auth_service.rs` |

### Types Files

| Language | Pattern | Example |
|----------|---------|---------|
| typescript | `{layer}/types.ts` | `src/domain/types.ts` |
| python | `{layer}/types.py` | `src/domain/types.py` |
| go | `{layer}/types.go` | `domain/types.go` |
| rust | `{layer}/types.rs` | `src/domain/types.rs` |

---

## § Report Formats

### RED Report

Print after running tests (expecting failure):

```
RED REPORT: {function_name}
═══════════════════════════════════════
Command: {TEST_COMMAND}
Tests targeting this function: {N}
─────────────────────────────────────────
ACTUAL RESULTS FROM OUTPUT:
  Failed:  {N}
  Passed:  {N}
  Skipped: {N}
─────────────────────────────────────────
Failure messages (copy from output):
  {paste actual messages}
═══════════════════════════════════════
```

### GREEN Report

Print after running tests (expecting pass):

```
GREEN REPORT: {function_name}
═══════════════════════════════════════
Command: {TEST_COMMAND}
─────────────────────────────────────────
ACTUAL RESULTS FROM OUTPUT:
  Passed:  {N}
  Failed:  {N}
─────────────────────────────────────────
{If failures, copy messages}
═══════════════════════════════════════
```

### REFACTOR Report

Print after refactoring:

```
REFACTOR: {function_name}
═══════════════════════════════════════
Changes: {list or "None needed"}
Tests:   {PASS/FAIL from actual output}
═══════════════════════════════════════
```

### Completion Report

Print after each function is done:

```
✓ COMPLETED: {function_name}
  Progress: {completed}/{total} functions ({blocked} blocked)
  Next: {next_function_name or "All done!"}
```

### Blocked Function Report

Print when a function cannot be implemented:

```
⚠️  BLOCKED: {function_name}
═══════════════════════════════════════
Reason: {reason}
Missing Information:
  • {missing_item_1}
  • {missing_item_2}
Suggested Fix: {fix}
═══════════════════════════════════════
Skipping to next function...
```

### Final Blocked Summary

Print at end of Phase 4 if any blocked:

```
BLOCKED FUNCTIONS SUMMARY
═══════════════════════════════════════
Total Blocked: {count}

{function_name}:
  Reason: {reason}
  Missing: {list}
  Fix: {suggested_fix}

{repeat for each}
═══════════════════════════════════════
```

---

## § Layer Classification

| Component Layer | Default Test Type |
|-----------------|-------------------|
| domain | unit |
| application | integration |
| infrastructure | integration |

### Override to Unit When:
- Function is pure (no side effects)
- Function only uses other domain components

### Override to Integration When:
- Function interacts with external services
- Function reads/writes to database
- Function makes HTTP calls

---

## § Coverage Thresholds

| Code Type | Target |
|-----------|--------|
| Critical business logic | 100% |
| Public APIs | 90%+ |
| General code | 80%+ |
| Generated code | Exclude |
