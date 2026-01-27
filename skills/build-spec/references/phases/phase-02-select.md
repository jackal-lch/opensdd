---
phase: 2
name: select
next: phase-03-implement.md
---

# Phase 2: Select

<objective>
Pick the next component to implement based on compare-result.yaml and dependencies.
</objective>

<prerequisite>
`.opensdd/compare-result.yaml` must exist (created by Phase 1 scaffold or previous Phase 4 verify).

```bash
test -f ".opensdd/compare-result.yaml" && echo "FOUND" || echo "NOT_FOUND"
```

If NOT_FOUND:
- Something went wrong. Return to Phase 1.
</prerequisite>

<input>
From files:
- `.opensdd/compare-result.yaml`: Current state of spec vs code
- `.opensdd/spec.yaml`: Component dependencies
</input>

<steps>

<step n="1" name="read_compare_result">
Load current state from compare-result.yaml:

```bash
cat .opensdd/compare-result.yaml
```

**Schema:** See `skills/compare-spec/references/output-schema.yaml` for compare-result.yaml structure.

Extract:
- `summary.matches`: How many done
- `summary.missing`: How many need implementation
- `components`: Map of component name → status
</step>

<step n="2" name="find_missing">
Identify components with `status: missing`:

```
Missing components:
  - {component_name}
  - {component_name}
  ...
```

If no missing components:
- All components implemented!
- Skip to Phase 5 (Review)
</step>

<step n="3" name="analyze_dependencies">
For each missing component, check dependencies from spec.yaml:

Read spec.yaml and for each missing component:
- Get `consumes:` field (list of components it depends on)
- Check if each dependency has `status: match` in compare-result.yaml
- Mark component as "ready" if all deps are done (or has no deps)
- Mark component as "blocked" if any dep is missing/drift

Build ready list:

| Component | Dependencies | Status |
|-----------|--------------|--------|
| {name} | {consumes or "none"} | ready/blocked |
</step>

<step n="4" name="select_next">
Select next component using priority:

1. **Ready components only** (all dependencies satisfied)
2. **Layer order**: domain → application → infrastructure
3. **First in layer** if multiple ready

If no ready components:
- Circular dependency detected
- Report issue to user
- STOP workflow

Store selected component name for Phase 3.
</step>

<step n="5" name="report_selection">
Display selection:

```
Component Selection
───────────────────
Progress: {matches} of {total} components done

Selected: {COMPONENT_NAME}
Layer: {layer from spec}
Depends on: {consumes list or "none"}
Provides: {count of provides functions}

Proceeding to implement...
```
</step>

</steps>

<output>
Selected component name (stored in conversation context for Phase 3).
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| read_compare_result | State loaded | |
| find_missing | Missing list identified | |
| analyze_dependencies | Dependencies analyzed | |
| select_next | Component selected | |
| report_selection | Selection reported | |

If any step not done → return and complete it.
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue after selection.
</checkpoint>

<next>
**If no missing components:**
1. Speak: "All {N} components implemented! Proceeding to review..."
2. Load: `phase-05-review.md`

**If component selected:**
1. Store `current_component = {COMPONENT_NAME}` in context
2. Load: `phase-03-implement.md` (same folder)
</next>
