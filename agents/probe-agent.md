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
| `package_language` | Target language (typescript, python, go, rust) |
| `verification` | Verification section from package YAML |
| `component_path` | Path to built component |

## Instructions

### Step 1: Parse Verification

Extract from verification section:
- `safe_to_call`: Functions to call with their inputs
- `do_not_call`: Functions to avoid (side effects)
- `criteria`: What to look for in output (informational)

### Step 2: Verify Runtime Environment

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

### Step 3: Generate Probe Script

Generate a call-and-log script in the package language.

**Script structure:**
```
1. Log header with package ID and timestamp
2. Import/load the component
3. Log import result (success or failure)
4. Initialize the component
5. Log initialization result
6. For each safe_to_call function:
   a. Log the function name
   b. Log the input
   c. Call the function
   d. Log the output
   e. Log the type
   f. Log fields/attributes if object
   g. Log any errors
7. Log footer
```

### Step 4: Language Templates

#### TypeScript
```typescript
const log = (msg: string) => console.log(`[${new Date().toISOString()}] ${msg}`);

log('='.repeat(60));
log('PROBE: {package_id}');
log('='.repeat(60));

log('Importing module...');
try {
  const { Component } = await import('{component_path}');
  log('  OK: Imported');

  log('Initializing...');
  const instance = new Component();
  log('  OK: Created instance');

  // For each safe_to_call
  log('Calling {functionName}...');
  log(`  Input: ${JSON.stringify({inputs})}`);
  try {
    const result = await instance.{functionName}({inputs});
    log(`  Output: ${JSON.stringify(result)}`);
    log(`  Type: ${typeof result}`);
    if (result && typeof result === 'object') {
      log(`  Fields: ${Object.keys(result).join(', ')}`);
    }
  } catch (e) {
    log(`  ERROR: ${e}`);
  }
} catch (e) {
  log(`  IMPORT FAILED: ${e}`);
}

log('='.repeat(60));
log('PROBE COMPLETE');
```

#### Python
```python
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

    log("Initializing...")
    instance = {Component}()
    log("  OK: Created instance")

    # For each safe_to_call
    log("Calling {function_name}...")
    log(f"  Input: {inputs}")
    try:
        result = instance.{function_name}(**{inputs})
        log(f"  Output: {repr(result)}")
        log(f"  Type: {type(result).__name__}")
        if hasattr(result, '__dict__'):
            log(f"  Fields: {list(vars(result).keys())}")
    except Exception as e:
        log(f"  ERROR: {type(e).__name__}: {e}")

except ImportError as e:
    log(f"  IMPORT FAILED: {e}")

log("=" * 60)
log("PROBE COMPLETE")
```

#### Go
```go
package main

import (
    "fmt"
    "time"
    "encoding/json"
)

func log(msg string) {
    fmt.Printf("[%s] %s\n", time.Now().Format(time.RFC3339), msg)
}

func main() {
    log("============================================================")
    log("PROBE: {package_id}")
    log("============================================================")

    // Import and initialize (language-specific)
    // Call functions and log results
}
```

#### Rust
```rust
use chrono::Utc;

fn log(msg: &str) {
    println!("[{}] {}", Utc::now().to_rfc3339(), msg);
}

fn main() {
    log("============================================================");
    log("PROBE: {package_id}");
    log("============================================================");

    // Import and initialize (language-specific)
    // Call functions and log results
}
```

### Step 5: Run Script

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

### Step 6: Classify Result

Analyze the probe log and classify:

| Classification | Criteria |
|----------------|----------|
| **GREEN** | All functions imported and executed successfully, output looks reasonable |
| **YELLOW** | Functions executed but output unexpected (nulls, wrong types, missing fields) |
| **RED** | Import failed, functions crashed, or critical errors |

### Step 7: Generate Fix Hints (if not GREEN)

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

### Step 8: Return Result

Return the complete probe result.

## Output

Return result as YAML:

```yaml
probe_result:
  package_id: pkg-02-user-service
  classification: GREEN | YELLOW | RED

  probe_log: |
    [2024-01-30T10:15:00.000Z] ============================================================
    [2024-01-30T10:15:00.000Z] PROBE: pkg-02-user-service
    [2024-01-30T10:15:00.000Z] ============================================================
    [2024-01-30T10:15:00.001Z] Importing module...
    [2024-01-30T10:15:00.050Z]   OK: Imported
    [2024-01-30T10:15:00.051Z] Initializing...
    [2024-01-30T10:15:00.055Z]   OK: Created instance
    [2024-01-30T10:15:00.056Z] Calling createUser...
    [2024-01-30T10:15:00.056Z]   Input: {"email": "test@example.com", "name": "Test"}
    [2024-01-30T10:15:00.100Z]   Output: {"id": "abc-123", "email": "test@example.com", "name": "Test"}
    [2024-01-30T10:15:00.100Z]   Type: object
    [2024-01-30T10:15:00.100Z]   Fields: id, email, name, createdAt
    [2024-01-30T10:15:00.101Z] ============================================================
    [2024-01-30T10:15:00.101Z] PROBE COMPLETE

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
