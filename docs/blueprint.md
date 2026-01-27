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

## The Problem with Current SDD

### Over-Specification

Traditional specs try to define everything upfront:
- Every field in every interface
- Implementation logic inside functions
- Data structures down to the last property

This fails because:
1. You can't predict implementation details before coding
2. The more you specify, the more will be wrong
3. Wrong specs become ignored specs

### No Feedback Loop

Once code is written, there's no practical way to compare it against the spec. The spec and code live in separate worlds, drifting apart silently.

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

### 4. Continuous Comparison via Extraction

Drift will still happen. The solution is detection, not prevention.

**The extraction approach:**
1. Extract implemented code into signatures only (YAML format)
2. Compare extracted signatures against `.opensdd/spec.yaml`
3. Identify: drift, missing, extra
4. Fix code to match spec

Both extracted and spec are contracts-only (no implementation bodies), making comparison meaningful and context-efficient.

### 5. AI Fills the Gaps

AI is smart enough to:
- Infer struct fields from type purposes
- Implement functions from signatures and context
- Detect when code drifts from spec
- Fix drift without human intervention

We don't constrain AI with detailed instructions. We constrain it with clear boundaries.

---

## The OpenSDD Workflow

### Phase 1: Blueprint

| | |
|---|---|
| **Skill** | `/create-blueprint` |
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
| **Skill** | `/create-spec` |
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

### Phase 3: Build Loop (Ralph Loop)

```
┌──────────────────────────────────────────────────┐
│                                                  │
│   ┌─────────┐    ┌─────────┐    ┌─────────┐     │
│   │IMPLEMENT│───→│ EXTRACT │───→│ COMPARE │     │
│   └─────────┘    └─────────┘    └─────────┘     │
│        ↑                              │         │
│        │         ┌─────────┐          │         │
│        └─────────│   FIX   │←─────────┘         │
│                  └─────────┘                    │
│                                                  │
└──────────────────────────────────────────────────┘
```

#### IMPLEMENT
AI implements components based on spec.yaml. One component/layer at a time.

#### EXTRACT
Run `spec-extract` tool to extract code signatures into YAML:
```bash
spec-extract ./src -o .opensdd/extracted
```
This produces signatures only - no implementation bodies.

#### COMPARE
Compare `.opensdd/extracted/*.yaml` (extracted) against `.opensdd/spec.yaml` (designed):
- **Match** — Component aligns with spec. Done.
- **Drift** — Wrong naming, signature, types. Needs fix.
- **Missing** — In spec but not in code. Needs implementation.
- **Extra** — In code but not in spec. Evaluate if needed.

#### FIX
AI fixes code to match spec. Loop continues until all components match.

---

## Spec File Structure

### spec.yaml

```yaml
tech_stack:
  language: rust
  framework: tauri
  # ...

conventions:
  type_case: PascalCase
  function_case: snake_case
  # ...

structure:
  root: src
  layout: by_layer
  layers:
    domain: src/domain
    application: src/application
    infrastructure: src/infrastructure

components:
  component_name:
    for: Purpose of this component
    layer: domain|application|infrastructure
    provides:
      - function_name:
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
      - other_component
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
      approach: result_types
    async:
      approach: async_await
  component_patterns:
    component_name:
      pattern: strategy|repository|observer|etc
      for: Why this pattern
```

### Extracted YAML (from spec-extract)

```yaml
file: src/domain/types.rs
package: types
types:
  - name: User struct
    kind: struct
    fields:
      - "id: UserId"
      - "name: String"
  - name: UserError enum
    kind: enum
    variants:
      - NotFound
      - InvalidInput
functions:
  - signature: "fn create_user(input: CreateUserInput) -> Result<User>"
methods:
  - signature: "fn validate(&self) -> Result<()>"
    receiver: User
```

Both formats are contracts-only, enabling meaningful comparison.

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
| `/create-blueprint` | Generate product blueprint from user intent |
| `/create-spec` | Generate technical spec from blueprint |
| `/build-spec` | Run build loop: implement → extract → compare → fix |

---

## Core Assumption

**AI is smart enough to generate workable code. What it needs is proper boundaries to prevent hallucination out of bounds.**

We don't constrain AI with detailed instructions. We constrain it with clear structure. The spec provides the skeleton; AI provides the body.
