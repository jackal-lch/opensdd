---
phase: 3
name: implement
next: phase-04-verify.md
---

# Phase 3: Implement

<objective>
Implement the selected component following the spec.yaml definition exactly.
</objective>

<prerequisite>
`current_component` must be set from Phase 2.

If no current_component in context:
- Return to Phase 2 to select one.
</prerequisite>

<input>
From context:
- `current_component`: The component to implement

From files:
- `.opensdd/spec.yaml`: Technical specification
- `.opensdd/blueprint.md`: Product context (if exists)
</input>

<steps>

<step n="1" name="load_component_spec">
Read spec.yaml and extract full definition for `current_component`:

```bash
cat .opensdd/spec.yaml
```

Extract for the component:
- `for`: Purpose/responsibility
- `layer`: Which layer (domain/application/infrastructure)
- `provides`: Functions to implement with signatures
- `emits`: Events this component fires
- `subscribes`: Events this component listens to
- `consumes`: Other components it depends on
- `owns_data`: Data entities it manages

Also extract:
- `tech_stack`: Language, framework
- `conventions`: Naming, style rules
- `structure.layers`: File paths for each layer
- `architecture.component_patterns.[component]`: Specific patterns (if defined)
</step>

<step n="2" name="understand_context">
Build context for implementation:

1. **Read blueprint** (if exists):
   ```bash
   cat .opensdd/blueprint.md 2>/dev/null || echo "No blueprint"
   ```
   Understand domain context and user intent.

2. **Check component's layer folder**:
   ```bash
   ls -la {layer_path}/ 2>/dev/null || echo "Directory empty"
   ```
   See what already exists.

3. **Review consumed components**:
   For each component in `consumes`, read its implementation to understand imports.

4. **Review shared types**:
   Read type files. Note which are:
   - Complete (have fields)
   - Skeletons (need fields populated)
</step>

<step n="3" name="populate_types">
For type skeletons this component uses, add fields.

1. Identify types this component uses (from `provides`, `owns_data`, `emits`)
2. For each skeleton type: add fields based on function usage, blueprint context, and domain patterns
3. Create component-specific types (DTOs) in component file, not shared types file
</step>

<step n="4" name="implement_component">
Create component code following spec:

1. **Create file** in correct layer path per `structure.layers`

2. **Implement each function in `provides:`**
   - Use EXACT signature from spec
   - Import shared types
   - Create component-specific types as needed (DTOs)
   - Follow patterns from `architecture.component_patterns`
   - Handle errors per `architecture.global_patterns.error_handling`

3. **For events in `emits:`**
   - Implement event emission at appropriate points
   - Use correct payload types

4. **For subscriptions in `subscribes:`**
   - Wire up event handlers
   - Implement handler logic

**Implementation checklist:**
- [ ] File in correct layer path
- [ ] All `provides` functions with exact signatures
- [ ] Shared types imported
- [ ] Component-specific types created locally
- [ ] Events emitted correctly
- [ ] Subscriptions wired up
- [ ] Error handling follows spec patterns
- [ ] Naming follows `conventions`
- [ ] No circular imports
</step>

<step n="5" name="verify_syntax">
Verify code compiles/parses without errors. Fix any syntax errors before proceeding.
</step>

<step n="6" name="summarize">
Display implementation summary:

```
Implementation: {COMPONENT_NAME}
────────────────────────────────

Files created/modified:
  - {file path}: {description}

Functions implemented:
  - {signature 1}
  - {signature 2}

Types:
  - Populated: {types this component defined fields for}
  - Imported: {types already had fields}
  - Created: {component-specific DTOs}

Events:
  - Emits: {list}
  - Subscribes: {list}

Proceeding to verify...
```
</step>

</steps>

<output>
Component code written to disk. Ready for verification.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| load_component_spec | Full component definition extracted | |
| understand_context | Context built | |
| populate_types | Type skeletons populated | |
| implement_component | All provides/emits/subscribes implemented | |
| verify_syntax | Code compiles without errors | |
| summarize | Summary displayed | |

**Implementation completeness:**
- All functions in `provides` exist?
- All shared types have fields?
- All events emitted?
- All subscriptions handled?
- Code compiles?

If incomplete → fix before proceeding.
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue to verify.
</checkpoint>

<next>
1. Speak: "Implementation complete. Verifying against spec..."

2. Load: `phase-04-verify.md` (same folder)
</next>
