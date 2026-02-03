---
phase: 4
name: integration
next: null
---

# Phase 4: Integration

<objective>
Define external integrations and system boundaries. Perform final validation and finalize `.opensdd/spec.yaml` along with the validation script.
</objective>

<prerequisite>
Phase 3 must be complete with `.opensdd/spec.yaml` containing:
- Foundation sections (tech_stack, deployment, conventions, structure, components, architecture)
- Types section with `used` fields populated
- Components section with contracts (provides, emits, subscribes, consumes, owns_data)
</prerequisite>

<input>
Read `.opensdd/spec.yaml` for all previous phase outputs.
From blueprint: integrations list.
</input>

<steps>

<step n="1" name="define_integrations">
Define external system integrations from blueprint.

**For each external system, determine:**
- name: What external system?
- for: What purpose does this integration serve?
- direction: How does data flow?
  - **inbound**: External system calls us (webhooks, callbacks)
  - **outbound**: We call external system (APIs, services)
  - **bidirectional**: Both directions (real-time sync)
- consumed_by: Which internal components interact with this integration?

See `references/format/spec-schema.md` for structure (integrations).

If no external integrations, this section can be empty.
</step>

<step n="2" name="define_boundaries">
Define system-wide boundaries and cross-cutting concerns.

**Boundaries to define:**

1. **error_handling**: How do errors flow through the system?
   - Do components handle their own errors or bubble up?
   - Are there error boundaries (catch-all handlers)?

2. **security**: How is auth/authz handled?
   - Where is authentication performed?
   - How is authorization enforced?
   - What context is passed to components?

3. **events**: How does async communication work?
   - What message broker/event bus (if any)?
   - Delivery guarantees (at-least-once, exactly-once)?

**Best/worst case check:**
For each boundary, ask: "What happens if this fails?"
- error_handling: What if an error goes unhandled? What's the blast radius?
- security: What if auth is bypassed? What's exposed?
- events: What if events are lost or duplicated? What breaks?

If the answer reveals unacceptable risk, strengthen the boundary strategy.

Keep boundaries at strategy level, not implementation detail.

See `references/format/spec-schema.md` for structure (boundaries).
</step>

<step n="3" name="final_validation">
Perform comprehensive validation before generating output.

**Pre-mortem analysis:**
Imagine the spec is implemented and something goes wrong. What could cause it?

1. **Ambiguity check:**
   - Is any component responsibility vague?
   - Is any function purpose unclear?
   - Could any type be interpreted multiple ways?

2. **Completeness check:**
   - Can every blueprint feature be implemented with these components?
   - Are all user flows supported by the contracts?
   - Are error cases handled?

3. **Consistency check:**
   - Do all cross-references in `used` fields match actual usage?
   - Are naming conventions consistent throughout?
   - Is layer assignment consistent (domain vs application vs infra)?

4. **Drift-detection readiness:**
   - Is the spec structured for extract→compare→fix loop?
   - Are component and function names what we'd expect in code?
   - Can an AI look at extracted signatures and match to this spec?

**For each issue found:**
- Document the issue
- Apply a fix
- Verify the fix

Present validation results to user.
</step>

<step n="4" name="finalize_spec_yaml">
Finalize `.opensdd/spec.yaml` with integrations and boundaries.

**Add final sections:**
- integrations (from step 1)
- boundaries (from step 2)

Use `references/format/spec-schema.md` as the canonical structure.

The spec.yaml has been building incrementally through phases 1-3. This step completes it.
</step>

<step n="5" name="copy_validation_script">
Copy the validation script to `.opensdd/`.

**Copy spec.py from skill assets:**

1. Read: `assets/spec.py`
2. Write contents to `.opensdd/spec.py`

The script provides internal consistency checks:
- `validate` - Run all checks
- `validate --fix` - Auto-fix cross-references
- `orphans` - Find unused types
- `missing` - Find undefined types
- `refs` - Check cross-reference consistency
- `naming` - Check naming conventions
- `architecture` - Check architecture patterns
- `usages <type>` - Show where a type is used
- `deps <component>` - Show component dependencies

**Run validation:**

```bash
python .opensdd/spec.py validate
```

If issues found, fix them before completing.
</step>

</steps>

<output>
Two files in `.opensdd/`:
1. `spec.yaml` - Complete technical specification (built incrementally across phases)
2. `spec.py` - Validation script for internal consistency checks
</output>

<verify>
AI self-verification before checkpoint:

| Step | Expected Output | Status |
|------|-----------------|--------|
| define_integrations | External integrations defined | ✓ / ✗ |
| define_boundaries | System boundaries defined | ✓ / ✗ |
| final_validation | All issues found and fixed | ✓ / ✗ |
| finalize_spec_yaml | .opensdd/spec.yaml finalized | ✓ / ✗ |
| copy_validation_script | .opensdd/spec.py copied and validated | ✓ / ✗ |

If any step incomplete → return and complete it.
If all done → proceed to checkpoint.
</verify>

<checkpoint required="true">

**AI Quick Check:**

Final validation:
- `.opensdd/spec.yaml` exists and is complete?
- YAML is valid syntax?
- All sections present (tech_stack, deployment, conventions, structure, types, components, architecture, integrations, boundaries)?
- All cross-references valid?
- `.opensdd/spec.py` copied?
- `python .opensdd/spec.py validate` passes?

**If issues found:**

"Generation had issues: ⚠ [specific problem]"

Auto-attempt to fix by regenerating or running `python .opensdd/spec.py validate --fix`.
If still failing, inform user of the specific problem.

**If no issues:**

Use AskUserQuestionTool:
- question: "Technical spec generated and validated. What would you like to do next?"
- options:
  - label: "Complete (Recommended)"
    description: "Finish - spec is ready for implementation"
  - label: "Review and adjust"
    description: "Make changes to the generated spec"
  - label: "Regenerate"
    description: "Start fresh and regenerate the files"
</checkpoint>

<next>
After user approval:

1. Display summary:
   ```
   Technical Spec Complete!

   Files created:
   - .opensdd/spec.yaml    (source of truth)
   - .opensdd/spec.py      (validation script)

   Summary:
   - Tech stack: {language} + {framework} + {database}
   - Deployment: {model} on {target}
   - Types: {count} defined
   - Components: {count} defined
   - Integrations: {count} defined

   Validation commands:
   - python .opensdd/spec.py validate         # Check internal consistency
   - python .opensdd/spec.py validate --fix   # Auto-fix cross-references
   - python .opensdd/spec.py usages <type>    # Show where a type is used
   - python .opensdd/spec.py deps <component> # Show component dependencies
   ```

2. Speak to user:
   "Technical spec created successfully!

   Files:
   - .opensdd/spec.yaml: Source of truth for implementation
   - .opensdd/spec.py: Run `python .opensdd/spec.py validate` to check consistency

   Next steps:
   1. Run `/opensdd:package` to split spec into focused work packages
   2. Run `/opensdd:build` to build packages into production-ready code
   3. Run `/opensdd:compare` to verify code-spec alignment

   Optional:
   - Run `/opensdd:visualize` to generate architecture diagrams"

3. No next phase. Workflow complete.
</next>
