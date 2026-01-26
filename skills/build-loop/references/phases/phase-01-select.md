---
phase: 1
name: select
next: phase-02-implement.md
---

# Phase 1: Select

<objective>
Choose which component to implement this session based on dependencies and progress.
</objective>

<prerequisite>
Get SKILL_ROOT and verify Phase 0 complete:

```bash
SKILL_ROOT=$(python .opensdd/build-loop.state.py get-skill-root)
python .opensdd/build-loop.state.py check-phase 0
```

If exit code != 0:
- Show: "Phase 0 must be complete first."
- STOP workflow.

If exit code == 0:
- Proceed.
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
python .opensdd/build-loop.state.py status
```

From the JSON output, extract:
- `all_components`: Full list of components
- `completed_components`: Already done
- Calculate: `remaining = all_components - completed_components`

If remaining is empty:
- All components implemented
- Skip to checkpoint with "all done" path
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

<step n="3" name="recommend">
**AI recommends next component using this priority:**

1. **No dependencies** - Components with empty `consumes` field (domain layer typically)
2. **Dependencies complete** - Components whose `consumes` are all in `completed_components`
3. **Layer order** - Among equals, prefer: domain → application → infrastructure

**Recommendation logic:**
- Filter to "ready" components only
- Sort by layer (domain first)
- Pick the first one as recommendation
- If no "ready" components, identify which dependency should be built first
</step>

<step n="4" name="present_selection">
Present the selection to user.

Show:
1. Progress: "X of Y components completed"
2. Ready components with dependencies status
3. AI recommendation with reasoning

Use AskUserQuestionTool:
- question: "Which component would you like to implement?"
- options based on ready components, with AI recommendation first marked "(Recommended)"
</step>

</steps>

<output>
Selected component name, stored in state as `current_component`.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| load_progress | Progress loaded, remaining calculated | |
| analyze_dependencies | Dependencies analyzed for all remaining | |
| recommend | AI recommendation determined | |
| present_selection | User selected a component | |

If any step not done → return and complete it.
If all done → proceed to checkpoint.
</verify>

<checkpoint required="true">

**AI Quick Check:**

- At least one component remaining to implement?
- Selected component is "ready" (dependencies satisfied)?
- Component exists in spec.yaml?

**If no components remaining:**

"All components have been implemented! Proceeding to Review phase to handle any 'Extra' items found."

→ Skip to Phase 4 (load `phase-04-review.md`)

**If components remain:**

User has selected a component. Confirm and proceed.

Use AskUserQuestionTool:
- question: "Ready to implement [selected_component]?"
- options:
  - label: "Yes, implement it (Recommended)"
    description: "Proceed to implement this component from spec"
  - label: "Choose different"
    description: "Go back and select a different component"
  - label: "Skip to Review"
    description: "Skip remaining components and review extras"

On user response:
- "Yes": Proceed to <next>
- "Choose different": Return to step 4
- "Skip to Review": Load `phase-04-review.md`
</checkpoint>

<next>
After user confirms component selection:

1. Store in state:
   ```bash
   python .opensdd/build-loop.state.py set-current "[COMPONENT_NAME]"
   python .opensdd/build-loop.state.py start-phase 1
   ```

2. Speak to user:
   "Selected component: [COMPONENT_NAME]. Proceeding to implementation..."

3. Load: `phase-02-implement.md` (same folder)
</next>
