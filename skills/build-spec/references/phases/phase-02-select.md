---
phase: 2
name: select
next: phase-03-implement.md
---

# Phase 2: Select

<objective>
Choose the next component to implement based on dependencies and progress.
</objective>

<prerequisite>
Verify Phase 1 complete:

```bash
python .opensdd/build-spec.state.py check-phase 1
```

If exit code != 0:
- Show: "Phase 1 (Scaffold) must be complete first."
- STOP workflow.
</prerequisite>

<input>
From state:
- `all_components`: List of all components from spec.yaml
- `completed_components`: List of already implemented components
</input>

<steps>

<step n="1" name="load_progress">
Load current progress from state:

```bash
python .opensdd/build-spec.state.py status
```

From the JSON output, extract:
- `all_components`: Full list of components
- `completed_components`: Already done
- Calculate: `remaining = all_components - completed_components`

If remaining is empty:
- All components implemented
- Skip to <next> with "all done" path
</step>

<step n="2" name="analyze_dependencies">
Read spec.yaml and analyze dependencies for remaining components:

```bash
cat .opensdd/spec.yaml
```

For each remaining component, check its `consumes` field:
- List which other components it depends on
- Check if those dependencies are in `completed_components`
- Mark as "ready" if all dependencies complete (or no dependencies)
- Mark as "blocked" if dependencies not yet implemented

**Dependency analysis table:**

| Component | Depends On | Status |
|-----------|------------|--------|
| [name] | [list or "none"] | ready/blocked |
</step>

<step n="3" name="auto_select">
**Auto-select next component using this priority:**

1. **No dependencies** - Components with empty `consumes` field
2. **Dependencies complete** - Components whose `consumes` are all in `completed_components`
3. **Layer order** - Among equals, prefer: domain → application → infrastructure

**Selection logic:**
- Filter to "ready" components only
- Sort by layer (domain first, then application, then infrastructure)
- Select the first one

If no "ready" components:
- Identify circular dependency issue
- Report to user
- STOP workflow
</step>

<step n="4" name="report_selection">
Report the selected component:

```
Component Selection
───────────────────
Progress: [completed] of [total] components done

Selected: [COMPONENT_NAME]
Layer: [layer from spec]
Depends on: [consumes list or "none"]
Provides: [count] functions

Proceeding to implement...
```
</step>

</steps>

<output>
Selected component name stored in state as `current_component`.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| load_progress | Progress loaded, remaining calculated | |
| analyze_dependencies | Dependencies analyzed for all remaining | |
| auto_select | Component auto-selected | |
| report_selection | Selection reported | |

If any step not done → return and complete it.
If all done → proceed to next.
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue after selection.
</checkpoint>

<next>
**If no components remaining:**

1. Speak: "All [N] components implemented. Proceeding to review..."
2. Skip directly to: `phase-05-review.md`

**If component selected:**

1. Store in state:
   ```bash
   python .opensdd/build-spec.state.py set-current "[COMPONENT_NAME]"
   python .opensdd/build-spec.state.py start-phase 2
   ```

2. Load: `phase-03-implement.md` (same folder)
</next>
