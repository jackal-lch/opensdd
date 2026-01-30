---
phase: 1
name: analyze
next: phase-02-create.md
---

# Phase 1: Analyze

<objective>
Read spec.yaml and blueprint.md. Build dependency graph. Determine package order. Define probe verification criteria. Identify potential issues.
</objective>

<prerequisite>
Verify required files exist:

```bash
test -f ".opensdd/spec.yaml" && echo "SPEC_FOUND" || echo "SPEC_NOT_FOUND"
test -f "blueprint.md" || test -f ".opensdd/blueprint.md" && echo "BLUEPRINT_FOUND" || echo "BLUEPRINT_NOT_FOUND"
```

If SPEC_NOT_FOUND:
- Tell user: "No spec.yaml found. Run `/opensdd:create-spec` first."
- STOP workflow.

If BLUEPRINT_NOT_FOUND:
- Tell user: "No blueprint.md found. Run `/opensdd:create-blueprint` first."
- STOP workflow.
</prerequisite>

<input>
- `.opensdd/spec.yaml`: Technical specification
- `blueprint.md` or `.opensdd/blueprint.md`: Product blueprint
</input>

<steps>

<step n="1" name="read_documents">
Read both documents completely.

```bash
cat .opensdd/spec.yaml
```

```bash
cat blueprint.md 2>/dev/null || cat .opensdd/blueprint.md
```

**Extract from spec.yaml:**
- `tech_stack`: Language, framework, database
- `deployment`: Model, target
- `conventions`: Naming conventions
- `structure`: Project layout, layers, entrypoints
- `components`: All components with provides, emits, subscribes, consumes
- `types`: All type definitions
- `integrations`: External systems
- `boundaries`: Cross-cutting concerns

**Extract from blueprint.md:**
- User flows (with steps and edge cases)
- Integration details (SDK packages, methods, initialization)
- Constraints
- Data model context
</step>

<step n="2" name="build_dependency_graph">
Build a dependency graph from `components.*.consumes`.

For each component:
1. Get its `consumes` list
2. Add edges: component → each consumed component

```
Example:
  sdk consumes [config]          → sdk depends on config
  auth consumes [config, database] → auth depends on config, database
  users consumes [auth, database]  → users depends on auth, database
```

**Detect circular dependencies:**
- If A consumes B and B consumes A → circular
- Report as issue, do not proceed

**Build order (topological sort):**
1. Components with no dependencies → first
2. Components whose dependencies are all resolved → next
3. Continue until all components ordered
</step>

<step n="3" name="determine_package_order">
Create complete package list in build order.

**Fixed packages:**
- `pkg-00-scaffold` (always first)
- `pkg-01-types` (after scaffold, before components)
- `pkg-99-integration` (always last)

**Component packages:**
- Assign numbers 02-98 based on dependency order
- Format: `pkg-{NN}-{component_name}`

**Example order:**
```
pkg-00-scaffold      # Infrastructure
pkg-01-types         # Shared types
pkg-02-config        # No deps
pkg-03-database      # No deps
pkg-04-events        # No deps
pkg-05-observability # Consumes config
pkg-06-auth          # Consumes config, database
pkg-07-sdk           # Consumes config
pkg-08-workers       # Consumes config, events
pkg-09-api           # Consumes config, observability, auth
pkg-10-agent_configs # Consumes database, events
pkg-11-users         # Consumes database, auth, events
pkg-12-analytics     # Consumes database, events
pkg-13-agent_testing # Consumes database, sdk, agent_configs, events
pkg-14-example       # Consumes sdk, database, events
pkg-99-integration   # Wire everything
```
</step>

<step n="4" name="map_flows_to_components">
Map blueprint flows to components they involve.

For each flow in blueprint:
1. Read the flow steps
2. Identify which components are involved
3. Record mapping: component → [flows]

This mapping is used in Phase 2 to include relevant flows in each package.

**Example:**
```yaml
flow_mapping:
  sdk:
    - "Flow 3: Integrate HiAgent Agent"
  agent_configs:
    - "Flow 4: Configure Agent via Dashboard"
  auth:
    - "Flow 4: Configure Agent via Dashboard"
    - "Flow 5: Monitor Agent Usage"
  analytics:
    - "Flow 5: Monitor Agent Usage"
```
</step>

<step n="5" name="identify_external_integrations">
Identify components that interact with external systems.

For each component:
1. Check if it appears in `integrations.*.consumed_by`
2. If yes, mark as external integration
3. Extract SDK/API details from blueprint

**External integration details needed:**
- Package name
- Initialization pattern
- Method signatures
- Environment variables

**Example:**
```yaml
external_integrations:
  sdk:
    integration: hiagent
    blueprint_section: "Appendix B: HiAgent SDK Quick Reference"
    details:
      package: "hiagent-api + hiagent-components"
      initialization: "ChatService(endpoint=..., region=...)"
      methods: ["Agent.ainit", "agent.ainvoke", "agent.astream"]
```
</step>

<step n="6" name="spot_issues">
Identify potential issues before creating packages.

**Check for:**

1. **Missing types:**
   - Types used in components but not defined in spec.types

2. **Undefined dependencies:**
   - Components in `consumes` that don't exist in spec.components

3. **Missing external details:**
   - External integrations without SDK details in blueprint

4. **Ambiguous contracts:**
   - Functions with no input/output types
   - Types with no `for` description

5. **Flow coverage:**
   - Components not covered by any user flow (may be ok for infrastructure)

**Record issues:**
```yaml
issues:
  - severity: error | warning
    category: missing_type | undefined_dep | missing_external | ambiguous | coverage
    location: "component.function or type name"
    description: "What's wrong"
    suggestion: "How to fix"
```

**If any error-severity issues:** Report to user, STOP workflow.
**If only warnings:** Continue, include in manifest.
</step>

<step n="7" name="summarize_analysis">
Display analysis summary.

```
Package-Spec Analysis
═══════════════════

Source Documents:
  Spec:      .opensdd/spec.yaml
  Blueprint: blueprint.md

Package Plan:
  Total:       {N} packages
  Scaffold:    1
  Types:       1
  Components:  {N}
  Integration: 1

Build Order:
  1. pkg-00-scaffold
  2. pkg-01-types
  3. pkg-02-{component} (depends on: none)
  4. pkg-03-{component} (depends on: pkg-02)
  ...

External Integrations:
  - {component}: {integration_name}

{If issues:}
Issues Found:
  ⚠ {warning description}
  ✗ {error description}

{If no errors:}
Analysis complete. Ready to create packages.
```
</step>

</steps>

<output>
Analysis data for Phase 2:
- `package_order`: Ordered list of package IDs
- `component_order`: Component build order
- `flow_mapping`: Component → flows mapping
- `external_integrations`: External integration details
- `issues`: Any issues found
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| read_documents | Both docs read completely | |
| build_dependency_graph | Graph built, no cycles | |
| determine_package_order | All packages numbered | |
| map_flows_to_components | Flows mapped | |
| identify_external_integrations | External details extracted | |
| spot_issues | Issues identified | |
| summarize_analysis | Summary displayed | |

**Critical checks:**
- [ ] No circular dependencies
- [ ] All components have a package number
- [ ] All external integrations have blueprint details (or flagged)
- [ ] No error-severity issues blocking proceed
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue after analysis.
</checkpoint>

<next>
After analysis complete:

1. Store analysis data in context
2. Speak: "Analysis complete. Creating {N} packages..."
3. Load: `phase-02-create.md`
</next>
