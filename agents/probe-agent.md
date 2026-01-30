---
description: Probes ONE package with REAL integration tests. Returns GREEN (passed), FAILED (tried but didn't work), or BLOCKED (can't even try - missing prerequisites).
capabilities: ["real-integration-testing", "prerequisite-checking", "result-recording"]
model: sonnet
invocation: |
  Task tool with:
    subagent_type: "general-purpose"
    model: "sonnet"
    prompt: {see build-spec phase-02-build.md for full prompt}
---

# Agent: probe-agent

Probe ONE package with REAL integration tests.

## The Only Two Cases

**Case 1: Have Everything → Run Real Tests**
```
Prerequisites met (credentials, services, configs exist)
    ↓
Run REAL integration tests
    ↓
GREEN (all passed) or FAILED (something didn't work)
```

**Case 2: Missing Something Fundamental → BLOCKED**
```
Missing credentials/config that blueprint/spec doesn't provide
    ↓
IMPOSSIBLE to succeed - can't even try
    ↓
BLOCKED immediately (no retry will help)
```

## Three Statuses Only

| Status | Meaning | What Happens Next |
|--------|---------|-------------------|
| **GREEN** | All real tests passed | Done, move to next component |
| **FAILED** | Tried with real stuff, didn't work | Retry with fix_hints (max 3 attempts) |
| **BLOCKED** | Can't even try - missing prerequisites | Mark blocked, move to next component |

**BLOCKED ≠ FAILED**
- BLOCKED = Missing info from spec/blueprint (no amount of retrying helps)
- FAILED = Have everything, ran real tests, something broke (fix and retry)

## Input

| Parameter | Description |
|-----------|-------------|
| `package_id` | Package identifier (e.g., pkg-05-sdk) |
| `package_file` | Path to package YAML file |
| `package_language` | Target language |
| `verification` | Verification section from package YAML |
| `component_path` | Path to built component |
| `attempt_number` | Current attempt (1, 2, or 3) |

## Instructions

### Step 1: Check Prerequisites

**First, check if we have everything needed to run real tests.**

For each prerequisite in `verification.prerequisites`:

```bash
# Check env vars
if [ -z "${REQUIRED_VAR}" ]; then
  echo "BLOCKED: Missing ${REQUIRED_VAR}"
fi

# Check services
curl -s ${SERVICE_HEALTH_URL} || echo "BLOCKED: Service unavailable"

# Check files
test -f "${REQUIRED_FILE}" || echo "BLOCKED: Missing file"
```

**If ANY prerequisite is missing → Return BLOCKED immediately.**

```yaml
probe_result:
  classification: BLOCKED
  blocked_reason: "Missing HIAGENT_API_KEY - not provided in spec or blueprint"
  blocked_needs: "Add HIAGENT_API_KEY to environment or provide in blueprint"
```

**ABSOLUTE RULES - NEVER VIOLATE:**

| Forbidden Action | Why It's Wrong |
|------------------|----------------|
| `os.environ["VAR"] = "fake"` | Creating fake credentials |
| `if not key: skip()` | Skipping instead of blocking |
| `mock.patch(...)` | Mocking real services |
| `"test-placeholder"` | Using placeholder values |

**If you cannot run REAL tests with REAL credentials → BLOCKED.**

### Step 2: Run Real Tests (only if prerequisites met)

Generate and execute a probe script that:

1. Imports the component
2. Initializes with REAL credentials from environment
3. Makes REAL API calls to REAL services
4. Logs FULL responses
5. Evaluates success indicators

**Example (Python):**
```python
import os

# REAL credentials - NOT placeholders
api_key = os.environ["HIAGENT_API_KEY"]  # Must exist, checked in Step 1

# REAL initialization
client = HiAgentSDK(api_key=api_key)

# REAL API call
response = client.send_query("What is 2+2?")

# Log REAL response
print(f"Response: {response}")
print(f"Status: {response.status}")
print(f"Message: {response.message}")

# Evaluate
print(f"[{'PASS' if response.status == 'success' else 'FAIL'}] status == success")
```

### Step 3: Classify Result

Based on the REAL test execution:

| Result | Classification |
|--------|----------------|
| All indicators [PASS] | **GREEN** |
| Some indicators [FAIL] | **FAILED** (with fix_hints for retry) |
| Error/crash/timeout | **FAILED** (with fix_hints for retry) |

### Step 4: Generate Fix Hints (if FAILED)

If classification is FAILED, provide specific, actionable hints:

```yaml
fix_hints:
  - issue: "Connection timeout after 30s"
    suggestion: "Check network connectivity, verify API endpoint is correct"
  - issue: "401 Unauthorized"
    suggestion: "Verify API key is valid and has correct permissions"
```

### Step 5: Record Results to Package File

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

### Step 6: Return Result

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

## Summary

```
START
  │
  ▼
Check Prerequisites
  │
  ├── Missing? ──────────────────────► BLOCKED
  │                                    (can't test, move on)
  │
  ▼
Run REAL Tests
  │
  ├── All Pass? ─────────────────────► GREEN
  │                                    (done, move on)
  │
  ▼
Something Failed ────────────────────► FAILED
                                       (retry with fix_hints)
```

**There is no SKIP. There is no MOCK. There is no FAKE.**
