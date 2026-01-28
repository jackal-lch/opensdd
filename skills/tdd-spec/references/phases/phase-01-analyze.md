---
phase: 1
name: analyze
next: phase-02-setup.md
---

# Phase 1: Analyze

<objective>
Parse spec.yaml for target component. Derive test cases per function and determine implementation order.
</objective>

<prerequisite>
Verify spec.yaml exists:

```bash
test -f ".opensdd/spec.yaml" && echo "FOUND" || echo "NOT_FOUND"
```

If output is "NOT_FOUND":
- Tell user: "No spec.yaml found. Run `/opensdd:create-spec` first."
- STOP workflow.

Verify component name was provided:

If `$ARGUMENTS` is empty:
- Tell user: "Usage: `/opensdd:tdd-spec {component_name}`"
- STOP workflow.

Store component name:
```
TARGET_COMPONENT = $ARGUMENTS
```
</prerequisite>

<input>
- `$ARGUMENTS`: Component name to implement via TDD
- `.opensdd/spec.yaml`: Technical specification
</input>

<steps>

<step n="1" name="load_component_spec">
Read spec.yaml and extract full definition for TARGET_COMPONENT.

```bash
cat .opensdd/spec.yaml
```

Find the component in `components:` section and extract:
- `for`: Purpose/responsibility
- `layer`: Which layer (domain/application/infrastructure)
- `provides`: Functions with signatures and descriptions
- `emits`: Events this component fires
- `subscribes`: Events this component handles
- `consumes`: Other components it depends on

Also extract:
- `tech_stack.language`: Programming language
- `structure.tests`: Test directory path (default: `tests/` if not defined)
- `structure.layers`: Where to put component file
- `types`: Type definitions (for understanding input/output)
- `conventions`: Naming conventions

**Derive test commands from language:**

| Language | TEST_COMMAND | COMPILE_CHECK_COMMAND |
|----------|--------------|----------------------|
| typescript | `npx vitest run` | `npx tsc --noEmit` |
| python | `pytest` | `python -m py_compile {file}` |
| go | `go test ./...` | `go build ./...` |
| rust | `cargo test` | `cargo check` |

Store:
- `TEST_COMMAND`: Command to run tests
- `COMPILE_CHECK_COMMAND`: Command to verify syntax

If TARGET_COMPONENT not found in spec:
- Tell user: "Component '{TARGET_COMPONENT}' not found in spec.yaml"
- STOP workflow.
</step>

<step n="2" name="order_functions">
Determine implementation order for functions.

**Order by dependency:**
1. Functions with no dependencies on other functions in this component → first
2. Functions that depend on previously implemented functions → next
3. Event handlers (`subscribes`) → after core functions

**Example ordering:**
```
provides:
  - validateCredentials  # No deps → 1st
  - hashPassword         # No deps → 2nd
  - createSession        # No deps → 3rd
  - login                # Uses validate, hash, createSession → 4th
  - logout               # Uses session → 5th

subscribes:
  - UserDeleted          # Event handler → last
```

Create ordered function list:
```yaml
function_order:
  - name: validateCredentials
    tests: [list of test cases]
  - name: hashPassword
    tests: [list of test cases]
  - name: login
    tests: [list of test cases]
  ...
```
</step>

<step n="3" name="derive_test_cases">
For each function, derive test cases using derivation rules.

**Reference:** See `references/derivation-rules.md` for complete rules.

**Per function, derive:**

1. **Happy Path Tests** (from `for:` description)
2. **Error Case Tests** (from output union types)
3. **Edge Case Tests** (from input types)

**Per event in `emits`:**
- Event emission test

**Per subscription in `subscribes`:**
- Event handler test

Structure:
```yaml
function_order:
  - name: login
    for: "Authenticate user credentials and create session"
    input: Credentials
    output: AuthResult | AuthError
    tests:
      - name: "authenticates valid credentials"
        category: happy
      - name: "returns AuthError for invalid password"
        category: error
      - name: "returns AuthError for unknown user"
        category: error
      - name: "handles null credentials"
        category: edge
    events:
      - name: "emits UserLoggedIn on success"
```
</step>

<step n="4" name="summarize_analysis">
Display analysis summary:

```
TDD Analysis: {TARGET_COMPONENT}
════════════════════════════════

Component: {TARGET_COMPONENT}
Layer: {layer}
Language: {tech_stack.language}

Implementation Order:
  1. {function_name} ({test_count} tests)
  2. {function_name} ({test_count} tests)
  ...

Total:
  - Functions: {count}
  - Test Cases: {total_count}

Proceeding to setup test file and component skeleton...
```
</step>

</steps>

<output>
- `TARGET_COMPONENT`: Component name
- `function_order`: Ordered list of functions with their test cases
- `tech_stack.language`: Programming language
- `structure.tests`: Test directory
- `structure.layers.{layer}`: Component directory
- `TEST_COMMAND`: Command to run tests (derived from language)
- `COMPILE_CHECK_COMMAND`: Command to verify syntax (derived from language)
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| load_component_spec | Component definition extracted | |
| order_functions | Functions ordered by dependency | |
| derive_test_cases | All functions have test cases | |
| summarize_analysis | Summary displayed | |

**Verification checks:**
- [ ] Every function in `provides` is in function_order
- [ ] Every function has at least 1 test case
- [ ] Events and subscriptions have tests
- [ ] Order respects dependencies

If any check fails → identify gap and fix.
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue to setup phase.
</checkpoint>

<next>
1. Speak: "Analysis complete. Setting up test file and component skeleton..."

2. Pass to Phase 2:
   - TARGET_COMPONENT
   - function_order
   - tech_stack.language
   - structure.tests
   - structure.layers.{layer}
   - TEST_COMMAND
   - COMPILE_CHECK_COMMAND

3. Load: `phase-02-setup.md` (same folder)
</next>
