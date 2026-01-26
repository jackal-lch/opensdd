---
phase: 2
name: types
next: phase-03-contracts.md
---

# Phase 2: Types

<objective>
Define all types needed by the system: domain entities, input types, output types, error types, and event types. Each type has a name, purpose, and tracks where it's used.
</objective>

<prerequisite>
Phase 1 must be complete with `.opensdd/spec.yaml` containing:
- tech_stack, deployment, conventions, structure, components, architecture
</prerequisite>

<input>
Read `.opensdd/spec.yaml` for foundation decisions.
Product blueprint: data model entities, user flows.
</input>

<steps>

<step n="1" name="extract_domain_types">
Extract domain types from blueprint data model and component responsibilities.

**Domain type categories:**
- Core business entities - the main "things" in the system
- Value objects - meaningful values with constraints (identifiers, money, dates)
- Aggregates - compositions of related entities

**Process:**
1. List all entities from blueprint data model
2. For each entity, define:
   - Name (PascalCase, derived from blueprint terminology)
   - `for`: what this type represents
3. Identify any implicit entities from user flows not in data model

See `references/format/spec-schema.md` for type structure.

Present domain types to user.
</step>

<step n="2" name="derive_io_types">
Derive input and output types from component responsibilities.

**For each component, ask:**
- What data does it need to receive? → Input types
- What data does it produce? → Output types

**Input types:** Data needed to perform an operation - request payloads, command objects

**Output types:** Results of operations - response payloads, aggregated data

**Naming patterns** (see `references/format/spec-schema.md` Type Categories):
- Input: `{Action}Input`, `{Action}Request`, `{Entity}Data`
- Output: `{Action}Result`, `{Action}Response`, `{Entity}Summary`

Present I/O types to user.
</step>

<step n="3" name="define_error_types">
Define error types for failure cases.

**For each component, ask:**
- What can go wrong? → Error types
- What does the caller need to know about the failure?

**Error categories:**
- Not found - requested entity doesn't exist
- Validation - input doesn't meet requirements
- Permission - not authorized for operation
- External - integration/service failures

**Naming pattern:** `{Entity}NotFound`, `{Action}Failed`, `Invalid{Entity}`

Keep error types specific enough to be actionable.
</step>

<step n="4" name="define_event_types">
Define event types for async communication.

**Ask:**
- What state changes should other components know about?
- What triggers async workflows?

**Event categories:**
- State change notifications - entity created, updated, deleted
- Workflow triggers - something completed that starts another process

**Naming pattern:** `{Entity}{Action}` using past tense (Created, Updated, Deleted, Completed)

If the system is purely synchronous, this section may be minimal or empty.
</step>

<step n="5" name="verify_no_duplicates">
Verify types are unique and well-defined.

**Chain of verification:**

1. **Generate verification questions:**
   - Are there any types with similar names that might be duplicates?
   - Are there any types with overlapping purposes?
   - Is every type used by at least one component?
   - Are type names consistent in style (all PascalCase)?

2. **Answer each question:**
   - List any similar names found → decide which to keep
   - List any overlapping purposes → merge or differentiate
   - List any unused types → remove or justify

3. **Revise based on answers:**
   - Rename, merge, or remove types as needed

Present final type list with any changes explained.
</step>

</steps>

<output>
Update `.opensdd/spec.yaml` with types section (domain, input, output, error, event).

Structure: see `references/format/spec-schema.md` (types section)

Note: `used` field starts empty - will be populated in Phase 3.
</output>

<verify>
AI self-verification before checkpoint:

| Step | Expected Output | Status |
|------|-----------------|--------|
| extract_domain_types | Domain entities defined | ✓ / ✗ |
| derive_io_types | Input/output types defined | ✓ / ✗ |
| define_error_types | Error types defined | ✓ / ✗ |
| define_event_types | Event types defined (or noted as N/A) | ✓ / ✗ |
| verify_no_duplicates | No duplicates, all types unique | ✓ / ✗ |

If any step incomplete → return and complete it.
If all done → proceed to checkpoint.
</verify>

<checkpoint required="true">

**AI Quick Check:**

Validate types completeness:
- Every blueprint data entity has a corresponding type?
- Each component has types it can use for I/O?
- Error types cover major failure modes?
- No duplicate or overlapping types?
- All names follow PascalCase convention?

**If issues found:**

"Before continuing, I noticed: ⚠ [specific issue]"

Use AskUserQuestionTool:
- question: "How would you like to handle this?"
- options:
  - label: "Fix it (Recommended)"
    description: "Address the issue before continuing"
  - label: "Continue anyway"
    description: "Proceed to Contracts phase as-is"

**If no issues:**

Use AskUserQuestionTool:
- question: "Types look complete. Ready to define component contracts?"
- options:
  - label: "Continue to Contracts (Recommended)"
    description: "Define provides, consumes, emits for each component"
  - label: "Add more types"
    description: "Define additional types"
  - label: "Revise existing types"
    description: "Modify type definitions"
</checkpoint>

<next>
After user approval:

1. Update `.opensdd/spec.yaml` with types section

2. Speak to user:
   "Types added to .opensdd/spec.yaml. Proceeding to define component contracts..."

3. Load: `phase-03-contracts.md`
</next>
