---
phase: 0
name: initialize
next: phase-01-scaffold.md
---

# Phase 0: Initialize

<objective>
Check prerequisites exist and bootstrap state tracking for the build.
</objective>

<prerequisite>
No prerequisite. This is the first phase.
</prerequisite>

<input>
No input from previous phase. This phase starts fresh.
</input>

<steps>

<step n="1" name="derive_skill_root">
Note the full path of THIS file from the Read tool output.

Example: If you read `/path/to/opensdd/skills/build-spec/references/phases/phase-00-initialize.md`

Derive SKILL_ROOT by removing `/references/phases/phase-00-initialize.md`:
- SKILL_ROOT = `/path/to/opensdd/skills/build-spec`
</step>

<step n="2" name="check_spec">
Verify `.opensdd/spec.yaml` exists:

```bash
test -f ".opensdd/spec.yaml" && echo "FOUND" || echo "NOT_FOUND"
```

If output is "NOT_FOUND":
- Tell user: "No spec.yaml found. Run `/create-spec` first to generate your technical specification."
- STOP workflow.

If output is "FOUND":
- Proceed to next step.
</step>

<step n="3" name="check_tool">
Verify `spec-extract` tool is installed:

```bash
which spec-extract || echo "NOT_FOUND"
```

If output is "NOT_FOUND":
- Tell user: "spec-extract tool not found. Install with: `pip install spec-extract` or see opensdd documentation."
- STOP workflow.

If tool found:
- Proceed to next step.
</step>

<step n="4" name="init_state">
Initialize state with the derived SKILL_ROOT:

```bash
python $SKILL_ROOT/scripts/state.py init --skill-root "$SKILL_ROOT"
```

If exit code != 0:
- Show error message from script
- STOP workflow

If exit code == 0:
- Copy state.py to .opensdd location:
  ```bash
  cp $SKILL_ROOT/scripts/state.py .opensdd/build-spec.state.py
  ```
- Proceed
</step>

<step n="5" name="parse_spec">
Parse spec.yaml to extract metadata and component list:

```bash
python3 -c "
import yaml
import json

with open('.opensdd/spec.yaml', 'r') as f:
    spec = yaml.safe_load(f)

# Extract components
components = list(spec.get('components', {}).keys())

# Extract spec sections that have deliverables
sections = {
    'has_tech_stack': 'tech_stack' in spec,
    'has_structure': 'structure' in spec,
    'has_types': 'types' in spec and len(spec.get('types', {})) > 0,
    'has_deployment': 'deployment' in spec,
}

print(json.dumps({
    'components': components,
    'sections': sections,
    'language': spec.get('tech_stack', {}).get('language', 'unknown')
}))
"
```

Store components in state:

```bash
python .opensdd/build-spec.state.py set-components '[COMPONENTS_JSON]'
```

Replace `[COMPONENTS_JSON]` with the components array from the output.
</step>

<step n="6" name="verify_state">
Verify state file created correctly:

```bash
test -f ".opensdd/build-spec.state.yaml" && echo "OK" || echo "FAILED"
python .opensdd/build-spec.state.py status
```

If verification fails, fix before proceeding.
</step>

</steps>

<output>
State initialized with:
- SKILL_ROOT path stored
- spec.yaml verified
- All components parsed and stored
- Spec sections identified for scaffold phase
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| derive_skill_root | SKILL_ROOT path derived | |
| check_spec | spec.yaml exists | |
| check_tool | spec-extract installed | |
| init_state | State file created, state.py copied | |
| parse_spec | Components and sections stored in state | |
| verify_state | "OK" and status shown | |

If any step failed:
- Identify which step failed
- Return to that step and redo
- Do NOT proceed until all steps pass

If all steps passed:
- Proceed to next
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue after verify passes.
</checkpoint>

<next>
1. Complete phase:
   ```bash
   python .opensdd/build-spec.state.py complete-phase 0
   ```

2. Speak to user:
   "Build initialized. Found [N] components in spec.yaml. Starting scaffold..."

3. Load: `phase-01-scaffold.md` (same folder)
</next>
