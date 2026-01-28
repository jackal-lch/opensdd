---
phase: 3
name: implement
next: phase-04-verify.md
---

# Phase 3: Implement

<objective>
Implement the selected component using TDD via /opensdd:tdd-spec.
</objective>

<prerequisite>
`current_component` must be set from Phase 2.

If no current_component in context:
- Return to Phase 2 to select one.
</prerequisite>

<input>
From context:
- `current_component`: The component to implement

From files:
- `.opensdd/spec.yaml`: Technical specification
</input>

<steps>

<step n="1" name="invoke_tdd">
**Implement component using TDD**

Invoke the TDD skill to implement the component:

```
/opensdd:tdd-spec {current_component}
```

This performs true TDD:

1. **Analyze** - Parse spec, derive tests, order functions
2. **Setup** - Create test file (all skipped) + component skeleton
3. **Iterate** - For each function:
   - RED: Enable tests, verify fail
   - GREEN: Implement minimally, verify pass
   - REFACTOR: Improve while green
4. **Complete** - Verify all tests pass

Wait for TDD skill to complete.

**Expected output from TDD:**
- Component file fully implemented
- Test file with all tests passing
- Coverage verified
</step>

<step n="2" name="verify_completion">
Verify TDD completed successfully.

TDD phase-04 already verified:
- All tests pass (none skipped, none failed)
- Code compiles without errors
- Coverage >= 80%
- No placeholder implementations
- Integration tests pass
- All types have fields defined

Quick verification:
- Component file exists at expected path
- Test file exists at expected path

```bash
# Verify files exist
test -f "{component_file_path}" && echo "Component: OK" || echo "Component: MISSING"
test -f "{test_file_path}" && echo "Tests: OK" || echo "Tests: MISSING"
```

If TDD reported issues, they should already be fixed. If files missing, investigate.
</step>

<step n="3" name="summarize">
Display implementation summary:

```
Implementation: {current_component}
══════════════════════════════════

Method: TDD (Red-Green-Refactor)

Files:
  Component: {component_file_path}
  Tests:     {test_file_path}

TDD Summary:
  Functions:  {count} implemented
  Tests:      {count} passing
  Coverage:   {percent}%

Proceeding to verify signatures...
```
</step>

</steps>

<output>
Component implemented via TDD. All tests passing. Ready for signature verification.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| invoke_tdd | TDD skill completed | |
| verify_completion | Component and tests exist, all pass | |
| summarize | Summary displayed | |

**Verification:**
- [ ] TDD skill completed without errors
- [ ] Component file exists
- [ ] Test file exists
- [ ] All tests pass
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue to verify.
</checkpoint>

<next>
1. Speak: "Implementation complete via TDD. Verifying signatures..."

2. Load: `phase-04-verify.md` (same folder)
</next>
