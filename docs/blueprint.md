# OpenSDD Blueprint

## Why OpenSDD?

Existing SDD approaches share a common failure pattern:

```
Day 1: Spec is perfect and detailed
Day 30: Spec is stale, code has drifted
Day 60: Spec is fiction, nobody trusts it
```

The spec defined at the beginning is nice, but it becomes stale during development and ultimately drifts completely from the implemented code.

**OpenSDD exists to make Spec-Driven Development actually work.**

## The Problems

### Problem 1: Over-Specification

Traditional specs try to define everything upfront:
- Every field in every interface
- Implementation logic inside functions
- Data structures down to the last property

This fails because:
1. You can't predict implementation details before coding
2. The more you specify, the more will be wrong
3. Wrong specs become ignored specs

### Problem 2: No Feedback Loop

Once code is written, there's no practical way to compare it against the spec. The spec and code live in separate worlds, drifting apart silently.

### Problem 3: AI Self-Validation

When AI writes both tests and implementation:
- AI writes tests that pass with minimal implementation
- AI writes code that passes its own tests
- Tests and code are co-designed to pass each other
- Result: placeholder implementations, fake integrations, mocked everything

**Self-validation is circular. It produces code that "passes" but doesn't work.**

### Problem 4: Context Overload

Large specs cause AI confusion:
- Signal gets diluted by noise
- AI hallucinates when context is too broad
- Accumulated context from previous components adds confusion
- AI fakes implementations when unsure, instead of asking

---

## OpenSDD Principles

### 1. Spec Defines Boundaries, Not Implementation

Spec includes:
- Components and their responsibilities
- Function signatures with purpose (not logic)
- Input/output types with purpose (not fields)
- Data ownership and event flow
- Architecture patterns and constraints

Spec does NOT include:
- Field definitions inside structs
- Implementation logic inside functions
- Internal data transformations

**AI is clever enough to implement details. It needs constraints, not instructions.**

### 2. Two Documents, Clear Separation

| Document | Purpose | Audience |
|----------|---------|----------|
| `.opensdd/blueprint.md` | What and why | AI context |
| `.opensdd/spec.yaml` | Contracts and boundaries | AI implementation |

**Blueprint** captures intent: vision, users, features, flows, constraints. It helps AI understand what we're building and why.

**Spec** defines structure: components, types, interfaces, patterns. It's the source of truth that code must match.

### 3. Spec is Source of Truth

When spec and code disagree, **code is wrong**. This is non-negotiable.

- Spec changes require explicit decisions
- Code drifts are bugs to be fixed
- AI implements from spec, not from intuition

### 4. Focused Context > Large Context

Signal-to-noise ratio matters more than raw size.

- Each build unit sees only what it needs
- Work is split into focused packages (like a tech lead assigning tasks)
- Clean slate per package prevents accumulated confusion
- Irrelevant content dilutes attention to relevant content

### 5. Builder ≠ Verifier

Self-validation leads to fakes. Separation is mandatory.

- Builder agent implements code
- Verifier agent (different model) probes the code
- Builder doesn't know how it will be verified
- Verifier doesn't know builder's shortcuts

**Different agents, different models, clean contexts = no collusion.**

### 6. Probe, Don't Assert

Traditional testing: write assertions that can be gamed.
Probing: call functions, log output, observe honestly.

- No assertions that can be written to pass
- Just call the code and log what happens
- Human judges the honest output
- Truth is revealed by execution, not by test design

### 7. BLOCK > FAKE

When information is missing:
- **Wrong**: Fake it, mock it, placeholder it
- **Right**: Report BLOCKED and stop

Incomplete is better than wrong. A blocked build can be unblocked. A fake build must be discovered, debugged, and rewritten.

### 8. Continuous Comparison via Extraction

Drift will still happen. The solution is detection, not prevention.

**The extraction approach:**
1. Extract implemented code into signatures only (YAML format)
2. Compare extracted signatures against `.opensdd/spec.yaml`
3. Identify: drift, missing, extra
4. Fix code to match spec

Both extracted and spec are contracts-only (no implementation bodies), making comparison meaningful and context-efficient.

---

## The OpenSDD Workflow

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│   BLUEPRINT ──→ SPEC ──→ PACKAGE ──→ BUILD ──→ COMPARE          │
│                                        │                        │
│                              ┌─────────┴─────────┐              │
│                              │    Per package:   │              │
│                              │                   │              │
│                              │  Build (Opus)     │              │
│                              │       ↓           │              │
│                              │  Probe (Sonnet)   │              │
│                              │       ↓           │              │
│                              │  GREEN? ──→ Next  │              │
│                              │       ↓           │              │
│                              │  Fix hints ──→    │              │
│                              │  Retry (max 2)    │              │
│                              │       ↓           │              │
│                              │  Still fails?     │              │
│                              │  Mark BLOCKED     │              │
│                              │       ↓           │              │
│                              │  Next package     │              │
│                              └───────────────────┘              │
│                                        │                        │
│                              FINAL REPORT + COMPARE             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Fully automatic. No human in loop during build.**

### Phase 1: Blueprint

| | |
|---|---|
| **Skill** | `/opensdd:create-blueprint` |
| **Input** | User idea, context, requirements |
| **Output** | `.opensdd/blueprint.md` |

AI collaborates with user to capture:
- Vision and problem statement
- Target users and their needs
- Features (prioritized for v1)
- User flows with edge cases
- Data model (entities and relationships)
- Integrations
- Constraints (performance, security, platform)

The blueprint is comprehensive but focused on **what** and **why**, not **how**.

### Phase 2: Spec

| | |
|---|---|
| **Skill** | `/opensdd:create-spec` |
| **Input** | `.opensdd/blueprint.md` + tech decisions |
| **Output** | `.opensdd/spec.yaml` |

AI generates technical specification:
- Tech stack and conventions
- Project structure (folders, layers)
- Components with responsibilities
- Function signatures (name, purpose, in/out types)
- Type definitions (name, purpose, usage)
- Event flow (emits, subscribes)
- Architecture patterns
- Integration boundaries

The spec defines **contracts and boundaries** that code must satisfy.

### Phase 2.5: Visualize (Optional)

| | |
|---|---|
| **Skill** | `/opensdd:visualize-spec` |
| **Input** | `.opensdd/spec.yaml` |
| **Output** | `.opensdd/spec-visual.md` |

Generate Mermaid diagrams to understand system design at a glance:
- Architecture overview (components by layer)
- Component dependencies (consumes relationships)
- Event flow (emits → subscribes)
- Type map (types with component usage)

Useful for reviewing the spec before implementation or sharing with stakeholders.

### Phase 3: Package

| | |
|---|---|
| **Skill** | `/opensdd:package-spec` |
| **Input** | `.opensdd/spec.yaml` + `.opensdd/blueprint.md` |
| **Output** | `.opensdd/packages/*.yaml` |
| **Agent** | `package-agent` |

The package phase splits the spec into focused work units. Like a tech lead assigning tasks to developers:

1. **Analyze** spec and blueprint
2. **Create** packages in dependency order:
   - `pkg-00-scaffold` — Project structure, configs, infrastructure
   - `pkg-01-types` — All shared type definitions
   - `pkg-02..N-{component}` — One package per component
   - `pkg-99-integration` — Wire everything together
3. **Validate** each package is self-contained
4. **Output** manifest + individual package files

Each package contains:
- **Scope**: What to build (files, component)
- **Context**: Types and dependencies (references to spec)
- **Instructions**: Purpose, constraints, BLOCK triggers
- **Verification**: Functions to probe, safe inputs, criteria

### Phase 4: Build

| | |
|---|---|
| **Skill** | `/opensdd:build-spec` |
| **Input** | `.opensdd/packages/*.yaml` |
| **Output** | Implementation + build summary (probe logs in package files) |
| **Agents** | `build-agent` (Opus), `probe-agent` (Sonnet) |

**Fully automatic with retry loop. No human intervention required.**

For each package in order:

```
┌─────────────────────────────────────────────────────────────────┐
│  ATTEMPT 1                                                      │
│                                                                 │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  BUILD-AGENT (Task: Opus, fresh context)                  │ │
│  │  - Reads ONE package + spec + existing code               │ │
│  │  - Writes implementation                                  │ │
│  │  - Outputs code + declarations                            │ │
│  │  - BLOCKS if missing info (never fakes)                   │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              ↓                                  │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  PROBE-AGENT (Task: Sonnet, fresh context)                │ │
│  │  - Checks DECLARED prerequisites (if any)                 │ │
│  │  - If declared prerequisite missing → BLOCKED             │ │
│  │  - Otherwise → Runs tests appropriate for package type    │ │
│  │  - Classifies: GREEN / FAILED / BLOCKED                   │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              ↓                                  │
│                         GREEN? ────────────────→ Done ✓         │
│                         BLOCKED? ──────────────→ Next pkg       │
│                              ↓ (FAILED)                         │
│  ┌───────────────────────────────────────────────────────────┐ │
│  │  PROBE generates fix hints:                               │ │
│  │  - What failed and why                                    │ │
│  │  - Suggested fixes                                        │ │
│  │  - Attention points for retry                             │ │
│  └───────────────────────────────────────────────────────────┘ │
│                              ↓                                  │
└──────────────────────────────┼──────────────────────────────────┘
                               ↓
┌─────────────────────────────────────────────────────────────────┐
│  ATTEMPT 2 (automatic retry)                                    │
│                                                                 │
│  BUILD-AGENT (Task: Opus, FRESH context) receives:              │
│    - Original package spec                                      │
│    - Fix hints from probe (NOT raw probe log)                   │
│                              ↓                                  │
│  PROBE-AGENT (Task: Sonnet, fresh context)                      │
│                              ↓                                  │
│                         GREEN? ────────────────→ Done ✓         │
│                              ↓ (FAILED)                         │
│                    Generate new fix hints                       │
│                              ↓                                  │
└──────────────────────────────┼──────────────────────────────────┘
                               ↓
┌─────────────────────────────────────────────────────────────────┐
│  ATTEMPT 3 (final retry)                                        │
│                                                                 │
│  BUILD-AGENT (Task: Opus, FRESH context) receives:              │
│    - Original package spec                                      │
│    - Fix hints from attempt 2                                   │
│                              ↓                                  │
│  PROBE-AGENT (Task: Sonnet, fresh context)                      │
│                              ↓                                  │
│                         GREEN? ────────────────→ Done ✓         │
│                              ↓ (FAILED after 3 attempts)        │
│                    Mark package as FAILED                       │
│                    Continue to next package                     │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Key Rules:**
- **Fresh context per attempt**: Every Task invocation starts clean
- **Fix hints, not raw logs**: Probe provides structured feedback, not full log
- **Max 3 attempts**: Initial build + 2 retries
- **FAILED if exhausted**: After 3 attempts, mark FAILED (needs human review)
- **BLOCKED if can't test**: Missing prerequisites = BLOCKED immediately (no retry)
- **All results in package file**: Probe results appended to package YAML

**Three Statuses Only:**
- **GREEN**: All REAL tests passed → Done
- **FAILED**: Tried with real stuff, didn't work → Retry (max 3), then human review
- **BLOCKED**: Can't even try (missing prerequisites) → Move to next package

**BLOCKED ≠ FAILED:**
- BLOCKED = Missing info from spec/blueprint (no amount of retrying helps)
- FAILED = Have everything, ran real tests, something broke (fix and retry)

**After all packages complete:** Run compare and generate final report.

```
┌─────────────────────────────────────────────────────────────────┐
│  POST-BUILD: COMPARE PHASE (automatic)                          │
│                                                                 │
│  1. Run spec-extract on entire codebase                         │
│     → Extract all function signatures, types                    │
│                                                                 │
│  2. Run compare-spec-agent                                      │
│     → Compare extracted vs spec.yaml                            │
│     → Identify: matches, drifts, missing, extras                │
│                                                                 │
│  3. Generate unified build-summary.yaml                         │
│     → Probe results (per package)                               │
│     → Compare results (overall)                                 │
│     → Action items for human review                             │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Build complete. Human reviews final report and decides next steps.**

### Phase 5: Compare (Standalone)

| | |
|---|---|
| **Skill** | `/opensdd:compare-spec` |
| **Input** | Codebase + `.opensdd/spec.yaml` |
| **Output** | Comparison report |

Also runs automatically at end of `/build`. Can be run standalone anytime for:
- Checking after manual fixes
- Maintenance and drift detection
- Diagnostics before spec changes

Reports:
- **Match** — Component aligns with spec
- **Drift** — Wrong naming, signature, types
- **Missing** — In spec but not in code
- **Extra** — In code but not in spec (classified as helper/infrastructure/test/new_functionality)

---

## Package Structure

Each package is a self-contained work unit with build history:

```yaml
# .opensdd/packages/pkg-{NN}-{name}.yaml

package:
  id: pkg-{NN}-{name}
  type: scaffold | types | component | integration
  language: typescript | python | go | rust
  build_order: {NN}
  depends_on: [pkg-ids]

scope:
  description: "One-line description"
  files:
    - path/to/file.ext

context:
  types:
    - ref: spec.types.User
    - ref: spec.types.Session
  dependencies:
    - ref: spec.components.Database

instructions:
  purpose: |
    What this package should achieve
  constraints:
    - "MUST use real database, not in-memory"
    - "MUST emit events via EventBus"
  on_missing_info: BLOCK
  never_fake:
    - "No placeholder implementations"
    - "No in-memory storage if spec says database"
    - "No mocked external services"

verification:
  safe_to_call:
    - name: createUser
      inputs: { email: "test@example.com", name: "Test" }
    - name: getUser
      inputs: { userId: "test-123" }
  do_not_call:
    - sendEmail      # Side effect
    - chargeCard     # Side effect
  criteria:
    - "createUser returns object with id, email, name"
    - "getUser returns User or null/None"

# === BUILD HISTORY (appended by build-spec, never overwritten) ===
builds:
  - attempt: 1
    timestamp: "2024-01-30T10:15:00Z"
    build_result:
      status: SUCCESS
      files_created:
        - path: src/services/user_service.py
          lines: 85
      declarations:
        storage: postgresql
        external_apis: []
        events_emitted: [UserCreated]
    probe_log: |
      [2024-01-30T10:15:30] ============================================================
      [2024-01-30T10:15:30] PROBE: pkg-02-user-service
      [2024-01-30T10:15:30] ============================================================
      [2024-01-30T10:15:30] Importing module...
      [2024-01-30T10:15:30]   OK: Imported UserService
      [2024-01-30T10:15:31] Calling createUser...
      [2024-01-30T10:15:31]   Input: {"email": "test@example.com", "name": "Test"}
      [2024-01-30T10:15:31]   Output: User(id='abc-123', email='test@example.com')
      [2024-01-30T10:15:31]   Type: User
      [2024-01-30T10:15:31] ============================================================
      [2024-01-30T10:15:31] PROBE COMPLETE
      [2024-01-30T10:15:31] ============================================================
    probe_status: GREEN

  - attempt: 2  # Example of a retry
    timestamp: "2024-01-30T10:20:00Z"
    fix_hints_received:
      - issue: "getUser raised TypeError"
        suggestion: "Ensure userId parameter is passed correctly"
    build_result:
      status: SUCCESS
      files_created:
        - path: src/services/user_service.py
          lines: 90
    probe_log: |
      [2024-01-30T10:20:30] ...
    probe_status: GREEN

# Final status after all attempts
final_status: GREEN | FAILED | BLOCKED
blocked_reason: null  # or "Failed after 3 attempts: {summary}"
```

**Single file per package = complete traceability.** Every build attempt, probe log, and fix is recorded in one place.

---

## Probe Script (Language-Agnostic)

The probe agent generates a call-and-log script in the appropriate language:

**Core pattern (same in all languages):**
```
1. Import/load the component
2. Initialize
3. For each safe_to_call function:
   - Log the input
   - Call the function
   - Log the output, type, fields
   - Log any errors
4. No assertions — just honest logging
```

**Example output:**
```
[2024-01-30T10:15:00] ============================================================
[2024-01-30T10:15:00] PROBE: pkg-02-user-service
[2024-01-30T10:15:00] ============================================================

[2024-01-30T10:15:00] Importing module...
[2024-01-30T10:15:00]   OK: Imported UserService

[2024-01-30T10:15:00] Initializing...
[2024-01-30T10:15:00]   OK: Created instance

[2024-01-30T10:15:01] Calling createUser...
[2024-01-30T10:15:01]   Input: {"email": "test@example.com", "name": "Test"}
[2024-01-30T10:15:01]   Output: User(id='abc-123', email='test@example.com')
[2024-01-30T10:15:01]   Type: User
[2024-01-30T10:15:01]   Fields: ['id', 'email', 'name', 'created_at']

[2024-01-30T10:15:01] Calling getUser...
[2024-01-30T10:15:01]   Input: {"userId": "test-123"}
[2024-01-30T10:15:01]   Output: None
[2024-01-30T10:15:01]   WARNING: Returned None

[2024-01-30T10:15:01] ============================================================
[2024-01-30T10:15:01] PROBE COMPLETE
[2024-01-30T10:15:01] ============================================================
```

**If not GREEN, probe generates structured fix hints:**

```yaml
fix_hints:
  - issue: "createUser raised TypeError: missing required argument 'email'"
    suggestion: "Ensure createUser accepts email parameter as specified in verification.safe_to_call"
  - issue: "getUser returned None for existing user"
    suggestion: "Check database query logic, ensure user lookup works correctly"

attention_points:
  - "Database connection must be initialized before use"
  - "Return type must be User | UserError, not None"
```

These fix hints (not the raw log) are passed to the retry builder.

---

## Build Summary Format

The unified build summary combines probe results (runtime) and compare results (structural):

```yaml
# .opensdd/results/build-summary.yaml

generated: "2024-01-30T12:00:00Z"
spec_file: .opensdd/spec.yaml
total_packages: 12

# ═══════════════════════════════════════════════════════════════════════════
# PROBE RESULTS (Runtime Verification - per package)
# ═══════════════════════════════════════════════════════════════════════════

probe_summary:
  GREEN: 9
  FAILED: 2
  BLOCKED: 1

packages:
  - id: pkg-00-scaffold
    final_status: GREEN
    attempts: 1

  - id: pkg-02-user-service
    final_status: FAILED
    attempts: 3
    notes: "getUser returns None for non-existent user - needs human review"

  - id: pkg-04-payment-service
    final_status: BLOCKED
    attempts: 3
    blocked_reason: "Stripe API credentials not configured"
    blocked_needs: "Set STRIPE_API_KEY environment variable"

# ═══════════════════════════════════════════════════════════════════════════
# COMPARE RESULTS (Structural Verification - overall)
# ═══════════════════════════════════════════════════════════════════════════

compare_summary:
  total_components: 8
  matches: 6
  drifts: 1
  missing: 0
  extras: 5

drifts:
  - component: UserService
    function: updateUser
    drift_type: return_type
    spec_expects: "User | UserNotFound | ValidationError"
    code_has: "User | None"
    suggested_fix: "Add proper error types instead of returning None"

missing: []

extras:
  - item: hash_password
    classification: helper        # OK - used by spec functions
    used_by: [AuthService.createUser]

  - item: legacy_import_users
    classification: new_functionality  # ⚠️ Needs evaluation
    used_by: []

# ═══════════════════════════════════════════════════════════════════════════
# ACTION ITEMS (What needs human attention)
# ═══════════════════════════════════════════════════════════════════════════

action_items:
  must_fix:
    - type: BLOCKED
      package: pkg-04-payment-service
      action: "Configure Stripe credentials and re-run build"

    - type: DRIFT
      component: UserService.updateUser
      action: "Fix return type to match spec"

  should_review:
    - type: FAILED
      package: pkg-02-user-service
      action: "Review getUser returning None - fix implementation or update spec"

    - type: EXTRA_NEW
      item: legacy_import_users
      action: "Add to spec if intentional, or remove"

# ═══════════════════════════════════════════════════════════════════════════
# OVERALL STATUS
# ═══════════════════════════════════════════════════════════════════════════

overall_status: NEEDS_ATTENTION  # SUCCESS | PARTIAL_SUCCESS | NEEDS_ATTENTION | FAILED

status_reason: |
  - 1 package BLOCKED (payment-service)
  - 1 structural drift (UserService.updateUser)
  - 1 untracked new functionality (legacy_import_users)
```

**This is the single report humans review after build completes.**

---

## Spec File Structure

### spec.yaml

```yaml
tech_stack:
  language: typescript
  framework: express
  # ...

conventions:
  type_case: PascalCase
  function_case: camelCase
  # ...

structure:
  root: src
  layout: by_layer
  layers:
    domain: src/domain
    application: src/application
    infrastructure: src/infrastructure

components:
  ComponentName:
    for: Purpose of this component
    layer: domain | application | infrastructure
    provides:
      - functionName:
          for: What this function does
          input: InputType
          output: OutputType | ErrorType
    emits:
      - EventName:
          for: When this event fires
          payload: PayloadType
    subscribes:
      - EventName: Why it listens
    consumes:
      - OtherComponent
    owns_data:
      - EntityType

types:
  TypeName:
    for: Purpose of this type
    used:
      - where it's used

architecture:
  global_patterns:
    error_handling:
      approach: result_types | exceptions
    async:
      approach: async_await | promises | callbacks
  component_patterns:
    ComponentName:
      pattern: repository | service | factory | etc
      for: Why this pattern
```

---

## Tools

| Tool | Purpose | Usage |
|------|---------|-------|
| `spec-extract` | Extract code signatures to YAML | `spec-extract ./src -o .opensdd/extracted` |

### Installing spec-extract

**macOS / Linux:**
```bash
./scripts/install-spec-extract.sh
```

**Windows:**
```powershell
.\scripts\install-spec-extract.ps1
```

---

## Skills

| Skill | Purpose |
|-------|---------|
| `/opensdd:create-blueprint` | Generate product blueprint from user intent |
| `/opensdd:create-spec` | Generate technical spec from blueprint |
| `/opensdd:visualize-spec` | Generate Mermaid diagrams from spec |
| `/opensdd:package-spec` | Split spec into focused packages |
| `/opensdd:build-spec` | Build packages: implement → probe → report |
| `/opensdd:compare-spec` | Check code-spec alignment |

---

## Agents

| Agent | Model | Purpose |
|-------|-------|---------|
| `package-agent` | — | Creates packages from spec + blueprint (tech lead role) |
| `build-agent` | Opus | Builds ONE package into implementation |
| `probe-agent` | Sonnet | Probes ONE package with call-and-log |
| `compare-agent` | — | Compares codebase against spec |

### Agent Invocation Mechanism

**CRITICAL**: Agents are invoked via the **Task tool** with explicit `model` parameter.

```yaml
# Build agent invocation
Task:
  subagent_type: "general-purpose"
  model: "opus"                    # Opus for building
  description: "Build pkg-XX"
  prompt: "{build-agent prompt}"

# Probe agent invocation
Task:
  subagent_type: "general-purpose"
  model: "sonnet"                  # Sonnet for probing (DIFFERENT model)
  description: "Probe pkg-XX"
  prompt: "{probe-agent prompt}"
```

This enforces:
1. **Clean context**: Each agent starts fresh, no accumulated state
2. **Model separation**: Opus builds, Sonnet verifies (different biases)
3. **No circular validation**: Prober has no knowledge of builder's shortcuts

---

## Key Artifacts

| Path | Purpose |
|------|---------|
| `.opensdd/blueprint.md` | Product intent (what and why) |
| `.opensdd/spec.yaml` | Technical contracts (boundaries) |
| `.opensdd/packages/manifest.yaml` | Package build order |
| `.opensdd/packages/pkg-{NN}-{name}.yaml` | Package definition + build history + probe logs |
| `.opensdd/results/build-summary.yaml` | Final build results summary |
| `.opensdd/compare-result.yaml` | Code-spec comparison report |

**Note:** Probe logs are stored in each package file (not separate files), enabling complete traceability per package.

---

## Core Assumptions

1. **AI is smart enough to generate workable code.** What it needs is proper boundaries to prevent hallucination out of bounds.

2. **AI fails at self-validation.** Builder and verifier must be separate. Different agents, different models.

3. **Focused context beats large context.** Signal-to-noise ratio matters. Each agent sees only what it needs.

4. **Honest execution beats clever testing.** Call the code, log what happens. No assertions to game.

5. **Blocking beats faking.** When unsure, stop and report BLOCKED. Never fill gaps with placeholders.

6. **Automatic with retry.** Build is fully automatic. Probe failures trigger automatic retry with fix hints. Human reviews final report, not intermediate steps.

We don't constrain AI with detailed instructions. We constrain it with clear structure. The spec provides the skeleton; AI provides the body. The probe reveals the truth.

---

## Automation Philosophy

**The build process is fully automatic.** No human intervention during the build loop.

| Phase | Human Involvement |
|-------|-------------------|
| Blueprint | Interactive (AI guides human through questions) |
| Spec | Interactive (human reviews and approves) |
| Package | Automatic (human can review output) |
| Build | **Fully automatic** (retry loop, no human needed) |
| Compare | Automatic (human reviews final report) |

**Human reviews the final report**, not intermediate probe logs. This enables:
- Unattended builds (run overnight, review in morning)
- Consistent process (no human variance in retry decisions)
- Complete traceability (all attempts recorded in package files)
