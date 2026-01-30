---
description: Creates focused work packages from spec.yaml and blueprint.md. Acts as tech lead, splitting work into self-contained, verifiable units.
capabilities: ["dependency-analysis", "package-creation", "probe-definition"]
---

# Agent: package-agent

Create focused work packages from spec.yaml and blueprint.md.

## Purpose

Act as a "tech lead" that:
1. Analyzes spec and blueprint
2. Builds dependency graph
3. Creates packages in dependency order
4. Defines probe verification for each package
5. Spots issues before any code is written

## Input

| Parameter | Description |
|-----------|-------------|
| `spec_file` | Path to `.opensdd/spec.yaml` |
| `blueprint_file` | Path to `blueprint.md` or `.opensdd/blueprint.md` |

## Instructions

### Step 1: Load Documents

Read both documents completely.

**From spec.yaml extract:**
- `tech_stack`: Language, framework, database
- `structure`: Project layout, layers
- `components`: All components with provides, emits, subscribes, consumes
- `types`: All type definitions
- `conventions`: Naming conventions

**From blueprint.md extract:**
- User flows (steps and edge cases)
- Integration details
- Constraints

### Step 2: Build Dependency Graph

Build graph from `components.*.consumes`:

```
Example:
  auth consumes [config, database] → auth depends on config, database
  users consumes [auth, database]  → users depends on auth, database
```

**Detect circular dependencies** - report and stop if found.

**Determine build order** via topological sort.

### Step 3: Create Packages

Create packages in this order:
1. `pkg-00-scaffold` - Project infrastructure
2. `pkg-01-types` - All type definitions
3. `pkg-02..N-{component}` - Components in dependency order
4. `pkg-99-integration` - Entry points and wiring

For each package, use the appropriate template from:
- `skills/package-spec/references/templates/pkg-scaffold.yaml`
- `skills/package-spec/references/templates/pkg-types.yaml`
- `skills/package-spec/references/templates/pkg-component.yaml`
- `skills/package-spec/references/templates/pkg-integration.yaml`

### Step 4: Define Probe Verification

**CRITICAL: Verification must be CONCRETE and REAL, not abstract.**

For each package, define verification that produces observable, real-world proof:

```yaml
verification:
  # What must exist before probing
  prerequisites:
    env_vars:
      - name: HIAGENT_API_KEY
        purpose: "Authenticate with HiAgent platform"
        example: "sk-..."
    services:
      - name: "HiAgent Platform"
        check: "curl -s https://api.hiagent.ai/health returns 200"

  # Real integration scenarios with concrete outcomes
  scenarios:
    - name: "real_agent_query"
      description: |
        Connect to HiAgent platform using real credentials,
        send an actual query, receive actual agent response.

      steps:
        - action: "call"
          function: "sendQuery"
          inputs:
            query: "What is 2+2?"
            agent_id: "test-agent-001"

        - action: "verify"
          expect: |
            Response contains:
            - agent_id matching input
            - non-empty response_text
            - status == "success"
            - response_time < 30s

      success_indicators:
        - "Successfully authenticated with platform"
        - "Query sent and response received"
        - "Response text is coherent and relevant"
        - "No connection timeouts or auth errors"

  do_not_call:
    - function: "deleteAgent"
      reason: "Destructive - removes agent permanently"
```

**Key principles:**
- **REAL connections**: Test actual external services, not mocks
- **CONCRETE outcomes**: "response.status == 'success'" not "returns object"
- **OBSERVABLE proof**: Log actual API responses, actual data
- **Prerequisites first**: Specify what credentials/services are needed
- **Success indicators**: What specifically proves it works?

**BAD verification (too abstract):**
```yaml
criteria:
  - "returns object with expected fields"
  - "handles errors appropriately"
```

**GOOD verification (concrete and real):**
```yaml
scenarios:
  - name: "connect_and_query"
    success_indicators:
      - "HTTP 200 from HiAgent API"
      - "Response JSON contains agent_id and message"
      - "Response.message is non-empty string"
      - "Round-trip time < 5 seconds"
```

### Step 5: Create Manifest

Create `manifest.yaml` listing all packages:

```yaml
manifest:
  generated: "{timestamp}"
  source:
    spec: ".opensdd/spec.yaml"
    blueprint: "blueprint.md"

  packages:
    - id: pkg-00-scaffold
      type: scaffold
      path: pkg-00-scaffold.yaml
      build_order: 0
      depends_on: []
    # ...

  statistics:
    total_packages: {N}
```

### Step 6: Write Files

Write all files to `.opensdd/packages/`:
- `manifest.yaml`
- `pkg-00-scaffold.yaml`
- `pkg-01-types.yaml`
- `pkg-{NN}-{component}.yaml` for each component
- `pkg-99-integration.yaml`

## Output

All package files written to `.opensdd/packages/`.

Display summary:
```
Package-Spec Complete
---------------------
Packages created: {N}
  pkg-00-scaffold
  pkg-01-types
  pkg-02-{component}
  ...
  pkg-99-integration

Files written to: .opensdd/packages/
Next: Run /opensdd:build-spec
```

## Constraints

- Every package must be self-contained
- Types have PURPOSE (for), AI infers FIELDS
- Use spec references (ref: spec.types.X), not copied implementations
- Define probe verification, not tests
- BLOCK triggers must be explicit
