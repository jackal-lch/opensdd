---
phase: 3
name: contracts
next: phase-04-integration.md
---

# Phase 3: Contracts

<objective>
Define contracts for each component: what functions it provides, what events it emits/subscribes to, what it consumes, and what data it owns. Populate the `used` field in types to create cross-references.
</objective>

<prerequisite>
Phase 2 must be complete with `.opensdd/spec.yaml` containing:
- Foundation sections (tech_stack, deployment, conventions, structure, components, architecture)
- Types section (domain, input, output, error, event)
</prerequisite>

<input>
Read `.opensdd/spec.yaml` for foundation and types.
</input>

<steps>

<step n="1" name="define_provides">
For each component, define what functions it provides.

**For each component:**
1. What operations does it expose?
2. What does each operation need (input type)?
3. What does each operation return (output type | error type)?

**Guidelines:**
- Function names should be verbs (action-oriented)
- Input should reference a type from Phase 2 (not inline definition)
- Output should include success AND error possibilities using `|`

See `references/format/spec-schema.md` for structure (components.provides).

Define provides for all components before proceeding.
</step>

<step n="2" name="define_events">
For each component, define events it emits and subscribes to.

**Emits:** What state changes should this component broadcast?

**Subscribes:** What events from other components does this react to?

See `references/format/spec-schema.md` for structure (components.emits, components.subscribes).

If component has no async behavior, emits and subscribes can be empty arrays.
</step>

<step n="3" name="define_dependencies">
For each component, define what other components it consumes and what data it owns.

**Consumes:** What other internal components does this depend on?

**Owns_data:** What types is this component the source of truth for?

**Guidelines:**
- Only list direct dependencies, not transitive
- Data ownership should be exclusive (one owner per type)

See `references/format/spec-schema.md` for structure (components.consumes, components.owns_data).
</step>

<step n="4" name="populate_used_field">
Cross-reference: populate the `used` field in each type.

**For each type, track where it appears:**
- As input to a function: `{component}.{function}`
- As output from a function: `{component}.{function}`
- As event payload: `{component}.emits`
- As subscription: `{component}.subscribes`

This creates bidirectional traceability for drift detection.

See `references/format/spec-schema.md` for structure (types.used).
</step>

<step n="5" name="verify_contracts">
Stress-test the contracts for completeness and consistency.

**Devil's advocate check:**

1. **Attack the contracts:**
   - Is any component doing too much? (violates single responsibility)
   - Is any component too thin? (could be merged)
   - Are there circular dependencies?
   - Is any type defined but never used?
   - Is any type used but never defined?

2. **For each weakness found:**
   - Explain the issue
   - Propose a fix
   - Apply the fix

3. **Verify cross-references:**
   - Every entry in a type's `used` field must correspond to actual usage
   - Every function's input/output must be in that type's `used` field

Present any issues found and fixes applied.
</step>

</steps>

<output>
Update `.opensdd/spec.yaml`:
- Add contracts to components (provides, emits, subscribes, consumes, owns_data)
- Populate `used` fields in types section

See `references/format/spec-schema.md` for structure.
</output>

<verify>
AI self-verification before checkpoint:

| Step | Expected Output | Status |
|------|-----------------|--------|
| define_provides | All components have provides defined | ✓ / ✗ |
| define_events | Events defined (or marked N/A) | ✓ / ✗ |
| define_dependencies | Consumes and owns_data defined | ✓ / ✗ |
| populate_used_field | All types have accurate used fields | ✓ / ✗ |
| verify_contracts | Contracts stress-tested, issues fixed | ✓ / ✗ |

If any step incomplete → return and complete it.
If all done → proceed to checkpoint.
</verify>

<checkpoint required="true">

**AI Quick Check:**

Validate contracts completeness:
- Every component has at least one function in provides?
- All input/output types exist in types section?
- All used fields accurately reflect actual usage?
- No circular dependencies between components?
- Data ownership is exclusive (no type owned by multiple components)?
- No orphan types (defined but unused)?

**If issues found:**

"Before continuing, I noticed: ⚠ [specific issue]"

Use AskUserQuestionTool:
- question: "How would you like to handle this?"
- options:
  - label: "Fix it (Recommended)"
    description: "Address the issue before continuing"
  - label: "Continue anyway"
    description: "Proceed to Integration phase as-is"

**If no issues:**

Use AskUserQuestionTool:
- question: "Contracts look complete. Ready to finalize with integrations and boundaries?"
- options:
  - label: "Continue to Integration (Recommended)"
    description: "Define external integrations and boundaries, generate final YAML"
  - label: "Adjust contracts"
    description: "Modify component contracts"
  - label: "Add/remove components"
    description: "Change the component list"
</checkpoint>

<next>
After user approval:

1. Update `.opensdd/spec.yaml` with contracts and populated `used` fields

2. Speak to user:
   "Contracts added to .opensdd/spec.yaml. Proceeding to integrations and finalization..."

3. Load: `phase-04-integration.md`
</next>
