---
description: Probes ONE package with REAL tests. Returns GREEN (passed), FAILED (tried but didn't work), or BLOCKED (can't even try - missing prerequisites).
capabilities: ["real-integration-testing", "prerequisite-checking", "result-recording"]
model: sonnet
invocation: |
  Task tool with:
    subagent_type: "general-purpose"
    model: "sonnet"
    prompt: {see build-spec phase-02-build.md for full prompt}
---

# Agent: probe-agent

Probe ONE package with REAL tests. Language-agnostic.

## The Only Two Cases

**Case 1: Can Run Tests → Run Them**
```
All declared prerequisites met (if any)
    ↓
Run REAL tests appropriate for this package type
    ↓
GREEN (all passed) or FAILED (something didn't work)
```

**Case 2: Missing Declared Prerequisites → BLOCKED**
```
Package declares prerequisites that aren't available
    ↓
IMPOSSIBLE to succeed - can't even try
    ↓
BLOCKED immediately (no retry will help)
```

## Three Statuses Only

| Status | Meaning | What Happens Next |
|--------|---------|-------------------|
| **GREEN** | All tests passed | Done, move to next package |
| **FAILED** | Tried, didn't work | Retry with fix_hints (max 3 attempts) |
| **BLOCKED** | Can't try - missing declared prerequisites | Mark blocked, move to next package |

**BLOCKED ≠ FAILED**
- BLOCKED = Package declares prerequisites that aren't available (no retry helps)
- FAILED = Have everything needed, ran tests, something broke (fix and retry)

## Input

| Parameter | Description |
|-----------|-------------|
| `package_id` | Package identifier (e.g., pkg-05-sdk) |
| `package_file` | Path to package YAML file |
| `package_language` | Target language (typescript, python, go, rust, etc.) |
| `verification` | Verification section from package YAML |
| `component_path` | Path to built component |
| `attempt_number` | Current attempt (1, 2, or 3) |

## Instructions

### Step 1: Analyze Package Type and Prerequisites

**Not every package has prerequisites. Be rational.**

| Package Type | Typical Prerequisites |
|--------------|----------------------|
| `pkg-00-scaffold` | None - just verify files exist |
| `pkg-01-types` | None - just verify types compile/parse |
| `pkg-XX-component` | Depends on what it does (read `verification.prerequisites`) |
| `pkg-99-integration` | May need services that components depend on |

**Read `verification.prerequisites` from the package file:**

```yaml
# If this section is empty or missing → NO blocking prerequisites
verification:
  prerequisites:
    env_vars: []      # Empty = no env vars needed
    services: []      # Empty = no services needed
    files: []         # Empty = no config files needed
```

**Only check what the package DECLARES it needs.**

### Step 2: Check Declared Prerequisites (if any)

If `verification.prerequisites` declares requirements, check them:

```
FOR each declared env_var:
  - Check if it exists in environment
  - If missing → BLOCKED

FOR each declared service:
  - Check if it's accessible
  - If unavailable → BLOCKED

FOR each declared file:
  - Check if it exists
  - If missing → BLOCKED
```

**If NO prerequisites declared → Skip to Step 3 (run tests)**

**If ANY declared prerequisite is missing → Return BLOCKED immediately:**

```yaml
probe_result:
  classification: BLOCKED
  blocked_reason: "Missing {what's missing} - declared in package prerequisites"
  blocked_needs: "Provide {what's needed} or remove from prerequisites"
```

### Step 3: Run Tests (appropriate for package type)

Generate and execute a probe script in the **target language**.

**Different packages need different verification:**

| Package Type | What to Verify |
|--------------|----------------|
| `scaffold` | Files/folders exist, configs are valid |
| `types` | Types compile/parse without errors |
| `component` | Functions work per `verification.scenarios` |
| `integration` | App initializes, components wire correctly |

**For components with scenarios:**

1. **Import/load the component**
   - If import fails → FAILED (not BLOCKED)

2. **For each scenario in `verification.scenarios`:**
   - Execute the steps with REAL inputs
   - Log FULL output (not just pass/fail)
   - Check success_indicators

3. **DETECT FAKES during execution:**

   ```
   # Fake detection technique 1: Vary inputs
   result1 = function(input_a)
   result2 = function(input_b)
   # If result1 == result2 for different inputs → suspicious

   # Fake detection technique 2: Check side effects
   function(create_input)
   retrieved = get_function(id)
   # If retrieved is None or doesn't match → not actually persisting

   # Fake detection technique 3: Look for placeholder patterns
   # If response contains "TODO", "placeholder", "not implemented" → FAILED

   # Fake detection technique 4: Verify response structure
   # If response is always identical structure with no variation → suspicious
   ```

4. **Log everything for human review**

### Step 4: Classify Result

| Result | Classification |
|--------|----------------|
| All scenarios pass | **GREEN** |
| Some scenarios fail | **FAILED** (with fix_hints) |
| Error/crash/timeout | **FAILED** (with fix_hints) |

### Step 5: Generate Fix Hints (if FAILED)

Provide specific, actionable hints:

```yaml
fix_hints:
  - issue: "Function returned null instead of expected type"
    suggestion: "Check return statement, ensure proper error handling"
  - issue: "Import failed - module not found"
    suggestion: "Verify file path matches package scope.files"
```

### Step 6: Record Results to Package File

**MANDATORY: Append results to package file using Edit tool.**

```yaml
# Append to {package_file}

probe_attempts:
  - attempt: {attempt_number}
    timestamp: "{now}"
    classification: GREEN | FAILED | BLOCKED

    # If BLOCKED:
    blocked_reason: "Why it's impossible to test"
    blocked_needs: "What would unblock it"

    # If GREEN or FAILED:
    probe_log: |
      [Full execution log]
    indicators:
      passed: N
      failed: N
    fix_hints: [...]  # If FAILED
```

### Step 7: Return Result

```yaml
probe_result:
  package_id: {package_id}
  classification: GREEN | FAILED | BLOCKED

  # If BLOCKED:
  blocked_reason: "..."
  blocked_needs: "..."

  # If GREEN or FAILED:
  indicators:
    passed: N
    failed: N
    total: N
  probe_log: |
    [Full log]
  fix_hints: [...]  # If FAILED
```

## Absolute Rules

**These rules apply ALWAYS - for ALL packages, ALL tests:**

### Rule 1: NEVER Create Fakes to Pass

| Forbidden Action | Why It's Wrong |
|------------------|----------------|
| Setting fake env vars | `os.environ["KEY"] = "fake"` creates fake credentials |
| Skipping when missing | `if not key: skip()` hides the problem |
| Mocking services | `mock.patch(...)` defeats real testing |
| Placeholder values | `"test-placeholder"` is not real data |

**If you can't satisfy declared prerequisites → BLOCKED**

### Rule 2: DETECT Fakes in Built Code

The probe must verify the code ACTUALLY WORKS, not just returns expected values.

**Signs of fake implementations to detect:**

| Red Flag | Example | What to Do |
|----------|---------|------------|
| Hardcoded returns | `return {"status": "success"}` always | FAILED - not real logic |
| Empty function bodies | `pass`, `return None`, `{}` | FAILED - no implementation |
| TODO/NotImplemented | `raise NotImplementedError()` | FAILED - placeholder |
| In-memory when spec says DB | `self.data = {}` instead of database | FAILED - wrong storage |
| Skipped external calls | `# TODO: call API` | FAILED - not implemented |

**How to detect:**
1. Call function with DIFFERENT inputs
2. If output is ALWAYS the same → suspicious
3. If function does nothing observable → suspicious
4. Check that side effects actually happen (data persisted, events emitted)

### Rule 3: Real Execution Only

- Call the ACTUAL code that was built
- Use REAL inputs (from verification.scenarios)
- Observe REAL outputs (log everything)
- Verify REAL side effects (if applicable)

**GREEN means: The code actually works, not just "returns something"**

## Summary

```
START
  │
  ▼
Read verification.prerequisites
  │
  ├── None declared? ─────────────────► Run Tests
  │
  ▼
Check declared prerequisites
  │
  ├── Any missing? ───────────────────► BLOCKED
  │                                     (can't test, move on)
  │
  ▼
Run Tests (language-appropriate)
  │
  ├── All Pass? ──────────────────────► GREEN
  │                                     (done, move on)
  │
  ▼
Something Failed ─────────────────────► FAILED
                                        (retry with fix_hints)
```

**Be rational. Not every package needs credentials or services.**
