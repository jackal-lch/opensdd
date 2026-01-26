---
phase: 2
name: create
next: phase-03-validate.md
---

# Phase 2: Create Packages

<objective>
Generate all work packages using standardized templates. Each package is self-contained with everything needed to build that piece.
</objective>

<prerequisite>
Phase 1 must be complete with:
- `package_order`: Ordered list of package IDs
- `component_order`: Component build order
- `flow_mapping`: Component → flows mapping
- `external_integrations`: External integration details
- `issues`: Any issues found (warnings only, errors would have stopped)
</prerequisite>

<input>
From Phase 1 context + original documents.
</input>

<steps>

<step n="1" name="create_scaffold_package">
Create `pkg-00-scaffold.yaml` for project infrastructure.

**Template:** `references/templates/pkg-scaffold.yaml`

**Extract from spec:**
- `tech_stack`: language, framework, database
- `deployment`: model, target
- `conventions`: all naming conventions
- `structure`: root, layout, layers, entrypoints, tests, configs

**Include:**
- Directory creation instructions
- Config file templates (pyproject.toml, package.json, etc.)
- Deployment files (Dockerfile, docker-compose.yaml)
- Environment template (.env.example)
- README stub

**Verification criteria:**
- All directories from structure.layers exist
- Project config file exists and is valid
- Deployment files exist (if deployment defined)
- .env.example has all required variables
</step>

<step n="2" name="create_types_package">
Create `pkg-01-types.yaml` for all type definitions.

**Template:** `references/templates/pkg-types.yaml`

**Extract from spec.types:**
- ALL types (domain, input, output, error, event)
- Group by category for organization
- Include `for` description for each

**Structure types by file:**
Based on `structure.types` mapping in spec:
```yaml
# Example
files:
  - path: src/hiagent_core/types.py
    types: [Settings, SessionContext, TokenPair, ...]
  - path: src/hiagent_dashboard/types.py
    types: [AgentConfig, User, ...]
```

**Verification criteria:**
- All types from spec are included
- Each type has fields appropriate to its purpose
- No duplicate type names
- Type files in correct locations per structure.types
</step>

<step n="3" name="create_component_packages">
For each component in `component_order`, create a package.

**Template:** `references/templates/pkg-component.yaml`

**For each component:**

1. **Metadata:**
   ```yaml
   package:
     id: pkg-{NN}-{component_name}
     type: component
     build_order: {NN}
     depends_on: [pkg-ids of consumed components]
   ```

2. **Scope (from spec.components.{name}):**
   ```yaml
   scope:
     description: "{component.for}"
     component:
       name: {name}
       for: "{for}"
       layer: {layer}
       provides: [copy all provides]
       emits: [copy all emits]
       subscribes: [copy all subscribes]
   ```

3. **Context - Types (COPY full definitions, not just names):**
   ```yaml
   context:
     types:
       # Include ALL types this component uses:
       # - Input types for provides
       # - Output types for provides (including errors)
       # - Event payload types for emits
       # - Types from subscribed events
       TypeName:
         for: "copied from spec.types"
         category: domain | input | output | error | event
         fields:
           field_name: type
   ```

4. **Context - Dependencies (interfaces only):**
   ```yaml
   context:
     dependencies:
       consumed_component:
         status: implemented | pending
         interface: |
           # Provide signatures of functions this component calls
           def function_name(param: Type) -> ReturnType
   ```

5. **Context - Flows (from flow_mapping):**
   ```yaml
   context:
     flows:
       - name: "Flow name from blueprint"
         relevant_steps:
           - "Step 3: This component does X"
           - "Step 4: Handle Y"
         edge_cases:
           - condition: "What can go wrong"
             handling: "How to handle it"
   ```

6. **Context - External (if in external_integrations):**
   ```yaml
   context:
     external:
       sdk_name:
         package: "package-name"
         initialization: |
           # Copied from blueprint
         methods:
           - name: method_name
             signature: "async def method(...) -> Return"
         environment:
           - VARIABLE_NAME: "description"
   ```

7. **Instructions:**
   ```yaml
   instructions:
     approach: |
       # Specific guidance for this component
       # Reference the flows and edge cases
       # Explain how to use dependencies

     constraints:
       - "Use database via {dependency}, not in-memory storage"
       - "Emit {event} after {action}"
       - "Handle {error_type} from {dependency}"

     if_blocked:
       - condition: "SDK package not available"
         action: "Mark SDK-dependent functions as BLOCKED"
       - condition: "Dependency returns error"
         action: "Propagate or wrap as appropriate error type"
   ```

8. **Verification (Probe-Based):**
   ```yaml
   verification:
     safe_to_call:
       # Functions to probe during verification
       - name: function_name
         inputs:
           param1: "test_value"
           param2: 123

     do_not_call:
       # Functions with side effects
       - sendEmail
       - chargePayment
       - deleteData

     criteria:
       # What to look for in probe output
       - "function returns object with expected fields"
       - "Error cases return appropriate error types"
       - "No NotImplementedError or placeholder responses"
   ```

9. **Output:**
   ```yaml
   output:
     files:
       - path: "{structure.components.{name}}/__init__.py"
         description: "Component implementation"
       - path: "tests/test_{name}.py"
         description: "Component tests"
   ```
</step>

<step n="4" name="create_integration_package">
Create `pkg-99-integration.yaml` to wire everything together.

**Template:** `references/templates/pkg-integration.yaml`

**Include:**

1. **Entry points (from structure.entrypoints):**
   - main.py: FastAPI app with all routers
   - worker.py: Background worker setup

2. **Router registration:**
   - Which components expose API routes
   - Route prefixes and tags

3. **Startup/shutdown hooks:**
   - Database connection
   - Redis connection
   - SDK initialization

4. **Dependency injection wiring:**
   - How components get their dependencies

5. **Integration tests:**
   - End-to-end flows from blueprint
   - Cross-component verification

**Verification criteria:**
- App starts without errors
- All routes registered
- Health check endpoint works
- Integration tests pass
</step>

<step n="5" name="create_manifest">
Create `manifest.yaml` to index all packages.

```yaml
manifest:
  generated: "{ISO8601 timestamp}"
  source:
    spec: ".opensdd/spec.yaml"
    blueprint: "blueprint.md"

  packages:
    - id: pkg-00-scaffold
      type: scaffold
      path: pkg-00-scaffold.yaml
      build_order: 0
      depends_on: []

    - id: pkg-01-types
      type: types
      path: pkg-01-types.yaml
      build_order: 1
      depends_on: [pkg-00-scaffold]

    # ... all component packages

    - id: pkg-99-integration
      type: integration
      path: pkg-99-integration.yaml
      build_order: 99
      depends_on: [all other package ids]

  issues_found:
    # Copy warnings from Phase 1
    - severity: warning
      package: pkg-{NN}-{name}
      issue: "description"

  statistics:
    total_packages: {N}
    scaffold: 1
    types: 1
    components: {N}
    integration: 1
```
</step>

</steps>

<output>
All package YAML files ready for validation:
- `pkg-00-scaffold.yaml`
- `pkg-01-types.yaml`
- `pkg-{NN}-{component}.yaml` for each component
- `pkg-99-integration.yaml`
- `manifest.yaml`
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| create_scaffold_package | pkg-00-scaffold.yaml created | |
| create_types_package | pkg-01-types.yaml created | |
| create_component_packages | All component packages created | |
| create_integration_package | pkg-99-integration.yaml created | |
| create_manifest | manifest.yaml created | |

**Critical checks:**
- [ ] Every component has a package
- [ ] Every package follows the template structure
- [ ] All sections present in each package
- [ ] Types are COPIED, not just referenced by name
- [ ] Dependencies include interface signatures
- [ ] Relevant flows extracted from blueprint
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue to validation.
</checkpoint>

<next>
After all packages created:

1. Store packages in context
2. Speak: "Created {N} packages. Validating completeness..."
3. Load: `phase-03-validate.md`
</next>
