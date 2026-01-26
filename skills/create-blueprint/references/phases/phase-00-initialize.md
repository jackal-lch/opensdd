---
phase: 0
name: initialize
next: phase-01-vision.md
---

# Phase 0: Initialize

<objective>
Bootstrap the skill state with SKILL_ROOT.
</objective>

<prerequisite>
No prerequisite. This is the first phase.
</prerequisite>

<input>
No input from previous phase. This is the first phase.
</input>

<steps>

<step n="1" name="derive_skill_root">
Note the full path of THIS file from the Read tool output.

Example: If you read `/Users/dev/.claude/skills/create-blueprint/references/phases/phase-00-initialize.md`

Derive SKILL_ROOT by removing `/references/phases/phase-00-initialize.md`:
- SKILL_ROOT = `/Users/dev/.claude/skills/create-blueprint`
</step>

<step n="2" name="init_state">
Initialize state with the derived SKILL_ROOT:

```bash
python $SKILL_ROOT/scripts/blueprint.state.py init --skill-root "$SKILL_ROOT"
```

If exit code != 0:
- Show error message from script
- STOP workflow

If exit code == 0:
- Copy blueprint.state.py to known location:
  ```bash
  mkdir -p .opensdd && cp $SKILL_ROOT/scripts/blueprint.state.py .opensdd/blueprint.state.py
  ```
- Proceed
</step>

<step n="3" name="verify_state">
Verify state file created correctly:

```bash
test -f ".opensdd/blueprint.state.yaml" && echo "OK" || echo "FAILED"
grep 'skill_root:' .opensdd/blueprint.state.yaml
```

If verification fails, fix before proceeding.
</step>

</steps>

<output>
State initialized with SKILL_ROOT. blueprint.state.py copied to .opensdd/ for future phases.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| derive_skill_root | SKILL_ROOT path derived | ✓ / ✗ |
| init_state | State file created, blueprint.state.py copied | ✓ / ✗ |
| verify_state | "OK" and skill_root shown | ✓ / ✗ |

If any step failed (✗):
- Identify which step failed
- Return to that step and redo
- Do NOT proceed until all steps pass

If all steps passed (✓):
- Proceed to next
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue after verify passes.
</checkpoint>

<next>
1. Complete phase:
   ```bash
   python .opensdd/blueprint.state.py complete-phase 0
   ```

2. Speak to user:
   "create-blueprint initialized. Ready to begin capturing your vision."

3. Load: `phase-01-vision.md` (same folder)
</next>
