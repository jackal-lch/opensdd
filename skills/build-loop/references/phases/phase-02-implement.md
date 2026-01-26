---
phase: 2
name: implement
next: phase-03-verify.md
---

# Phase 2: Implement

<objective>
AI implements the selected component following the spec.yaml definition.
</objective>

<prerequisite>
Get current component from state:

```bash
python .opensdd/build-loop.state.py status
```

Extract `current_component` from JSON output. If null:
- Show: "No component selected. Return to Phase 1."
- Load: `phase-01-select.md`
</prerequisite>

<input>
From state:
- `current_component`: The component to implement
From files:
- `.opensdd/spec.yaml`: Technical specification
- `.opensdd/blueprint.md`: Product context (if exists)
</input>

<steps>

<step n="1" name="load_component_spec">
Read spec.yaml and extract the full definition for `current_component`:

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

Also extract relevant:
- `tech_stack`: Language, framework
- `conventions`: Naming, style rules
- `structure.layers`: File paths for each layer
- `architecture.component_patterns.[component]`: Specific patterns to use
- `types`: Type definitions referenced by this component
</step>

<step n="2" name="understand_context">
Build context for implementation:

1. **Read blueprint** (if exists):
   ```bash
   cat .opensdd/blueprint.md 2>/dev/null || echo "No blueprint"
   ```
   Understand the domain context and user intent.

2. **Check existing code** in the component's layer folder:
   ```bash
   ls -la [LAYER_PATH]/ 2>/dev/null || echo "Directory not exists"
   ```
   Understand what already exists.

3. **Review consumed components**:
   For each component in `consumes`, understand its interface so you can call it correctly.
</step>

<step n="3" name="implement">
**Implement the component following spec:**

For each function in `provides`:
1. Create file in correct layer path per `structure.layers`
2. Implement function with exact signature from spec
3. Use types from spec's `types` section
4. Follow patterns from `architecture.component_patterns`
5. Handle errors per `architecture.global_patterns.error_handling`

For events in `emits`:
1. Implement event emission at appropriate points
2. Use correct payload types

For subscriptions in `subscribes`:
1. Wire up event handlers
2. Implement handler logic

**Implementation checklist:**
- [ ] File created in correct layer path
- [ ] All `provides` functions implemented with correct signatures
- [ ] Types match spec definitions
- [ ] Events emitted correctly
- [ ] Subscriptions wired up
- [ ] Error handling follows spec patterns
- [ ] Naming follows `conventions`
</step>

<step n="4" name="summarize">
Summarize what was created:

**Files created/modified:**
- [list files with paths]

**Functions implemented:**
- [list function signatures]

**Types used:**
- [list type names]

**Events:**
- Emits: [list]
- Subscribes: [list]
</step>

</steps>

<output>
Component code written to disk. Implementation summary provided.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| load_component_spec | Full component definition extracted | |
| understand_context | Context built from blueprint and existing code | |
| implement | All provides/emits/subscribes implemented | |
| summarize | Summary of created files and functions | |

**Implementation completeness check:**
- All functions in `provides` exist in code?
- All events in `emits` are fired somewhere?
- All subscriptions in `subscribes` have handlers?

If any incomplete → fix before proceeding.
</verify>

<checkpoint required="true">

**AI Quick Check:**

Review implementation against spec:
- All `provides` functions exist with correct signatures?
- File locations match `structure.layers`?
- Naming follows `conventions`?

**If issues found:**

"Implementation has gaps: [specific issue]"

Use AskUserQuestionTool:
- question: "How would you like to handle this?"
- options:
  - label: "Fix it (Recommended)"
    description: "AI fixes the implementation gaps"
  - label: "Continue anyway"
    description: "Proceed to verify phase as-is"

**If no issues:**

Use AskUserQuestionTool:
- question: "Implementation complete. Ready to verify alignment with spec?"
- options:
  - label: "Verify alignment (Recommended)"
    description: "Run extract → compare → fix loop"
  - label: "Adjust implementation"
    description: "Make changes before verifying"
  - label: "Discard and re-select"
    description: "Abandon this component, return to selection"

On user response:
- "Verify/Continue": Proceed to <next>
- "Fix/Adjust": Return to step 3
- "Discard": Load `phase-01-select.md`
</checkpoint>

<next>
After user approves implementation:

1. Mark phase in progress:
   ```bash
   python .opensdd/build-loop.state.py start-phase 2
   ```

2. Speak to user:
   "Implementation complete. Proceeding to verify alignment with spec..."

3. Load: `phase-03-verify.md` (same folder)
</next>
