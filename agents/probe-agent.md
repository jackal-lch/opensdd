---
description: Probes ONE package by calling functions and logging output. Returns classification (GREEN/YELLOW/RED) and fix_hints for retry loop. Uses Sonnet model.
capabilities: ["call-and-log", "execution-verification", "classification", "fix-hints-generation"]
model: sonnet
invocation: |
  Task tool with:
    subagent_type: "general-purpose"
    model: "sonnet"
    prompt: {see build-spec phase-02-build.md for full prompt}
---

# Agent: probe-agent

Probe ONE package by calling functions and logging output.

## Purpose

Verify a built package by:
1. Reading the verification section from package
2. Generating a call-and-log script in the appropriate language
3. Running the script
4. Classifying the result (GREEN/YELLOW/RED)
5. Generating fix_hints if not GREEN (for retry loop)

**Key principle: NO ASSERTIONS**
- Just call functions and log what happens
- Classification based on execution success
- Fix hints are structured feedback for builder retry

## Input

| Parameter | Description |
|-----------|-------------|
| `package_id` | Package identifier (e.g., pkg-02-user-service) |
| `package_file` | Path to package YAML (e.g., .opensdd/packages/pkg-02-user-service.yaml) |
| `package_language` | Target language (typescript, python, go, rust) |
| `verification` | Verification section from package YAML |
| `component_path` | Path to built component |
| `attempt_number` | Current attempt number (1, 2, or 3) |

## Instructions

### Step 1: Parse Verification

Extract from verification section:
- `prerequisites`: What must exist (env vars, services, files)
- `setup`: Steps to prepare environment
- `scenarios`: Real integration tests with expected outcomes
- `do_not_call`: Functions to avoid
- `on_missing_prerequisites`: BLOCK | SKIP | MOCK

### Step 2: Check Prerequisites

**Before any probing, verify all prerequisites are met.**

**Check environment variables:**
```bash
# For each env_var in prerequisites.env_vars
if [ -z "${ENV_VAR_NAME}" ]; then
  echo "MISSING: ENV_VAR_NAME"
fi
```

**Check services:**
```bash
# For each service in prerequisites.services
# Run the check command specified
curl -s https://api.example.com/health || echo "UNAVAILABLE: ServiceName"
```

**Check files:**
```bash
# For each file in prerequisites.files
test -f "{path}" || echo "MISSING: {path}"
```

**If any prerequisite is missing:**
- Check `on_missing_prerequisites` setting:
  - `BLOCK`: Return RED immediately with fix_hints listing what's missing
  - `SKIP`: Return YELLOW with note "skipped due to missing prerequisites"
  - `MOCK`: Continue but use mocks (not recommended)

### Step 3: Verify Runtime Environment

Before generating the probe script, verify the required runtime is available.

**Run environment check based on `package_language`:**

| Language | Check Command | Required |
|----------|---------------|----------|
| typescript | `which npx && npx ts-node --version` | npx, ts-node |
| python | `which python && python --version` | python 3.x |
| go | `which go && go version` | go 1.x |
| rust | `which cargo && cargo --version` | cargo |

**If check fails:**
- Return RED classification immediately
- Set fix_hints:
  ```yaml
  fix_hints:
    - issue: "Runtime not available: {language}"
      suggestion: "Install {required_tool} before running probe"
  ```
- Do NOT proceed to script generation

**If check passes:** Continue to Step 3.

### Step 4: Run Setup (if specified)

If `verification.setup` exists, run each setup step:

```bash
# For each step in setup
echo "SETUP: {step.description}"
{step.command}
```

Log setup results. If setup fails, return RED with fix_hints.

### Step 5: Generate Probe Script

Generate a call-and-log script that executes **scenarios**, not just functions.

**Script structure:**
```
1. Log header with package ID and timestamp
2. Import/load the component
3. Log import result (success or failure)
4. Initialize the component (with real config/credentials)
5. Log initialization result

6. For each SCENARIO in verification.scenarios:
   a. Log scenario name and description
   b. For each step in scenario.steps:
      - If action == "call":
        * Log function name and inputs
        * Call the function with REAL inputs
        * Log the FULL output (not just type)
        * Log response time
      - If action == "verify":
        * Check the expect conditions
        * Log pass/fail for each condition
   c. For each success_indicator:
      * Evaluate and log: "[PASS]" or "[FAIL]" + indicator

7. Log summary: passed/failed indicators
8. Log footer
```

**CRITICAL: Use REAL data, not mocks.**
- Real API calls to real services
- Real credentials from environment
- Real responses logged in full

### Step 6: Language Templates

#### TypeScript (Scenario-based)
```typescript
const log = (msg: string) => console.log(`[${new Date().toISOString()}] ${msg}`);

log('='.repeat(60));
log('PROBE: {package_id}');
log('='.repeat(60));

log('Importing module...');
try {
  const { Component } = await import('{component_path}');
  log('  OK: Imported');

  log('Initializing with real config...');
  const instance = new Component({
    apiKey: process.env.API_KEY,  // Real credentials
    // ... other real config
  });
  log('  OK: Created instance');

  // ═══════════════════════════════════════════════
  // SCENARIO: {scenario.name}
  // ═══════════════════════════════════════════════
  log('');
  log('SCENARIO: {scenario.name}');
  log('  {scenario.description}');
  log('');

  const startTime = Date.now();
  try {
    // Call with REAL inputs
    log('Calling {functionName}...');
    log(`  Input: ${JSON.stringify({real_inputs})}`);

    const result = await instance.{functionName}({real_inputs});
    const elapsed = Date.now() - startTime;

    // Log FULL response (not just type)
    log(`  Output: ${JSON.stringify(result, null, 2)}`);
    log(`  Response time: ${elapsed}ms`);

    // Check success indicators
    log('');
    log('SUCCESS INDICATORS:');
    // For each indicator, evaluate and log
    log(`  [${result.status === 'success' ? 'PASS' : 'FAIL'}] status == 'success'`);
    log(`  [${result.agent_id ? 'PASS' : 'FAIL'}] response contains agent_id`);
    log(`  [${result.message?.length > 0 ? 'PASS' : 'FAIL'}] message is non-empty`);
    log(`  [${elapsed < 30000 ? 'PASS' : 'FAIL'}] response time < 30s`);

  } catch (e) {
    log(`  ERROR: ${e}`);
    log('  [FAIL] Scenario failed with error');
  }

} catch (e) {
  log(`  IMPORT FAILED: ${e}`);
}

log('');
log('='.repeat(60));
log('PROBE COMPLETE');
```

#### Python (Scenario-based)
```python
import os
import time
import json
from datetime import datetime

def log(msg):
    print(f"[{datetime.now().isoformat()}] {msg}")

log("=" * 60)
log("PROBE: {package_id}")
log("=" * 60)

log("Importing module...")
try:
    from {module_path} import {Component}
    log("  OK: Imported")

    log("Initializing with real config...")
    instance = {Component}(
        api_key=os.environ.get("API_KEY"),  # Real credentials
        # ... other real config
    )
    log("  OK: Created instance")

    # ═══════════════════════════════════════════════
    # SCENARIO: {scenario.name}
    # ═══════════════════════════════════════════════
    log("")
    log(f"SCENARIO: {scenario_name}")
    log(f"  {scenario_description}")
    log("")

    start_time = time.time()
    try:
        # Call with REAL inputs
        log(f"Calling {function_name}...")
        log(f"  Input: {json.dumps(real_inputs)}")

        result = instance.{function_name}(**real_inputs)
        elapsed = (time.time() - start_time) * 1000

        # Log FULL response
        log(f"  Output: {json.dumps(result, indent=2, default=str)}")
        log(f"  Response time: {elapsed:.0f}ms")

        # Check success indicators
        log("")
        log("SUCCESS INDICATORS:")
        log(f"  [{'PASS' if result.get('status') == 'success' else 'FAIL'}] status == 'success'")
        log(f"  [{'PASS' if result.get('agent_id') else 'FAIL'}] response contains agent_id")
        log(f"  [{'PASS' if len(result.get('message', '')) > 0 else 'FAIL'}] message is non-empty")
        log(f"  [{'PASS' if elapsed < 30000 else 'FAIL'}] response time < 30s")

    except Exception as e:
        log(f"  ERROR: {type(e).__name__}: {e}")
        log("  [FAIL] Scenario failed with error")

except ImportError as e:
    log(f"  IMPORT FAILED: {e}")

log("")
log("=" * 60)
log("PROBE COMPLETE")
```

### Step 7: Run Script

Execute the generated script:

**TypeScript:**
```bash
npx ts-node .opensdd/probes/{package_id}_probe.ts
```

**Python:**
```bash
python .opensdd/probes/{package_id}_probe.py
```

**Go:**
```bash
go run .opensdd/probes/{package_id}_probe.go
```

**Rust:**
```bash
cargo run --bin probe_{package_id}
```

### Step 8: Classify Result

Analyze the probe log and classify based on **success indicators**:

| Classification | Criteria |
|----------------|----------|
| **GREEN** | ALL success indicators show [PASS], real data received from real services |
| **YELLOW** | SOME indicators passed, but others failed (partial success) |
| **RED** | Import failed, connection failed, auth failed, or majority indicators failed |

**Classification is based on REAL outcomes:**
- Did it actually connect to the external service?
- Did it receive a real response?
- Does the response contain expected data?
- Was the response time acceptable?

**NOT based on:**
- "Looks reasonable" (too vague)
- "Returns object" (not concrete enough)

### Step 9: Generate Fix Hints (if not GREEN)

If classification is YELLOW or RED, generate structured fix hints:

**Fix hints should be:**
- Specific and actionable
- Reference actual error messages from the log
- Suggest what the builder should fix
- NOT raw log dumps

**Example fix hints:**
```yaml
fix_hints:
  - issue: "createUser returned null instead of User object"
    suggestion: "Check that createUser actually stores and returns the user, not just validates input"
  - issue: "Import failed: Cannot find module './user_service'"
    suggestion: "Verify the export statement in user_service.ts matches expected path"
```

### Step 10: Record Results to Package File

**MANDATORY: Append probe results to the package YAML file.**

Use the Edit tool to append the probe results to `{package_file}`.

**Action:**
1. Read the current content of `{package_file}`
2. Use Edit to append the following YAML block at the end:

```yaml

# ═══════════════════════════════════════════════════════════════
# PROBE RESULT - Attempt {attempt_number}
# Recorded by probe-agent at {ISO timestamp}
# ═══════════════════════════════════════════════════════════════
probe_attempts:
  - attempt: {attempt_number}
    timestamp: "{ISO timestamp}"
    classification: {GREEN|YELLOW|RED}
    indicators:
      passed: {N}
      failed: {N}
      total: {N}
    scenarios:
      - name: "{scenario_name}"
        status: {PASS|FAIL}
        indicators:
          - "[PASS|FAIL] {indicator description}"
    probe_log: |
      {full probe execution log}
    fix_hints:  # null if GREEN
      - issue: "{what failed}"
        suggestion: "{how to fix}"
```

**If this is attempt 2 or 3**, append to existing `probe_attempts` array.

**DO NOT SKIP THIS STEP.** Recording probe results is essential for:
- Traceability of what was actually tested
- Debugging failed builds
- Evidence that real integration tests ran

### Step 11: Return Result

Return the complete probe result to the caller.

## Output

Return result as YAML:

```yaml
probe_result:
  package_id: pkg-05-sdk
  classification: GREEN | YELLOW | RED

  # Summary of success indicators
  indicators:
    passed: 4
    failed: 0
    total: 4

  scenarios:
    - name: "real_agent_query"
      status: PASS
      indicators:
        - "[PASS] Successfully authenticated with platform"
        - "[PASS] Query sent and response received"
        - "[PASS] Response contains valid agent_id"
        - "[PASS] Response time < 30s (actual: 1.2s)"

  probe_log: |
    [2024-01-30T10:15:00.000Z] ============================================================
    [2024-01-30T10:15:00.000Z] PROBE: pkg-05-sdk
    [2024-01-30T10:15:00.000Z] ============================================================
    [2024-01-30T10:15:00.001Z] Checking prerequisites...
    [2024-01-30T10:15:00.002Z]   HIAGENT_API_KEY: present
    [2024-01-30T10:15:00.003Z]   HiAgent API health: OK
    [2024-01-30T10:15:00.050Z] Importing module...
    [2024-01-30T10:15:00.050Z]   OK: Imported
    [2024-01-30T10:15:00.051Z] Initializing with real credentials...
    [2024-01-30T10:15:00.055Z]   OK: SDK client created
    [2024-01-30T10:15:00.056Z]
    [2024-01-30T10:15:00.056Z] SCENARIO: real_agent_query
    [2024-01-30T10:15:00.056Z]   Connect to HiAgent and send real query
    [2024-01-30T10:15:00.056Z]
    [2024-01-30T10:15:00.057Z] Calling sendQuery...
    [2024-01-30T10:15:00.057Z]   Input: {"query": "What is 2+2?", "agent_id": "test-agent"}
    [2024-01-30T10:15:01.250Z]   Output: {
    [2024-01-30T10:15:01.250Z]     "status": "success",
    [2024-01-30T10:15:01.250Z]     "agent_id": "test-agent",
    [2024-01-30T10:15:01.250Z]     "message": "2+2 equals 4.",
    [2024-01-30T10:15:01.250Z]     "response_time_ms": 1193
    [2024-01-30T10:15:01.250Z]   }
    [2024-01-30T10:15:01.251Z]   Response time: 1193ms
    [2024-01-30T10:15:01.251Z]
    [2024-01-30T10:15:01.251Z] SUCCESS INDICATORS:
    [2024-01-30T10:15:01.251Z]   [PASS] status == 'success'
    [2024-01-30T10:15:01.251Z]   [PASS] response contains agent_id
    [2024-01-30T10:15:01.251Z]   [PASS] message is non-empty: "2+2 equals 4."
    [2024-01-30T10:15:01.251Z]   [PASS] response time < 30s (1.2s)
    [2024-01-30T10:15:01.252Z]
    [2024-01-30T10:15:01.252Z] ============================================================
    [2024-01-30T10:15:01.252Z] PROBE COMPLETE: 4/4 indicators passed
    [2024-01-30T10:15:01.252Z] ============================================================

  # Only if not GREEN:
  fix_hints:
    - issue: "Description of what went wrong"
      suggestion: "What the builder should do to fix it"
```

## Constraints

- Use Sonnet model (different from build-agent's Opus)
- Generate script in package's target language
- NO assertions in probe script - just log
- Classification is based on execution results
- Fix hints are structured, not raw log dumps
- Call ONLY functions in safe_to_call
- NEVER call functions in do_not_call
