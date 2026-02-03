---
phase: 8
name: assembly
next: null
---

# Phase 8: Assembly

<objective>
Compile everything into the final blueprint document.
</objective>

<prerequisite>
Get SKILL_ROOT and verify previous phase complete:

```bash
SKILL_ROOT=$(python .opensdd/blueprint.state.py get-skill-root)
python .opensdd/blueprint.state.py check-phase 7
```

If exit code != 0:
- Show: "Phase 7 (Constraints) must be complete first."
- STOP workflow.
</prerequisite>

<input>
All data from phases 1-7:
```bash
python .opensdd/blueprint.state.py get-data 1 vision
python .opensdd/blueprint.state.py get-data 2 personas
python .opensdd/blueprint.state.py get-data 2 primary_persona
python .opensdd/blueprint.state.py get-data 3 features
python .opensdd/blueprint.state.py get-data 4 flows
python .opensdd/blueprint.state.py get-data 5 entities
python .opensdd/blueprint.state.py get-data 5 relationships
python .opensdd/blueprint.state.py get-data 6 integrations
python .opensdd/blueprint.state.py get-data 7 constraints
```
</input>

<steps>

<step n="1" name="gather_all_outputs">
Pull all data from state and organize for assembly.

**Retrieve from state:**
- Phase 1: Vision (one-liner, problem, value, success criteria)
- Phase 2: Personas (all personas, primary designation)
- Phase 3: Features (all features with priorities)
- Phase 4: Flows (user journeys, decision points, edge cases)
- Phase 5: Data (entities, relationships, attributes, lifecycle)
- Phase 6: Integrations (external services, providers, requirements)
- Phase 7: Constraints (non-functional requirements)

If any data is missing, flag it and determine if we need to go back to that phase.
</step>

<step n="2" name="apply_chain_of_verification">
Self-check the blueprint for internal consistency.

**Verification Questions:**

1. **Vision ↔ Features Alignment**
   - Does every feature serve the stated vision?
   - Are there vision elements not addressed by features?

2. **Features ↔ Personas Alignment**
   - Does every feature serve at least one persona?
   - Does the primary persona have features for their primary goal?

3. **Features ↔ Flows Alignment**
   - Is every feature represented in at least one flow?
   - Are there flows that use features not in scope?

4. **Flows ↔ Data Alignment**
   - Do flows reference entities that exist in the data model?
   - Are there entities not used by any flow?

5. **Data ↔ Integrations Alignment**
   - Do integrations provide/consume the data they need?
   - Are there data requirements without integration support?

6. **Constraints ↔ Everything Alignment**
   - Are constraints realistic for the scope?
   - Do any constraints conflict with each other?

**For each misalignment found:**
- Flag the issue
- Determine if it's critical or minor
- Propose resolution

Use AskUserQuestionTool if critical issues found:
- question: "I found [X] inconsistencies in the blueprint. How should we handle them?"
- options:
  - label: "Fix them now"
    description: "Address issues before finalizing"
  - label: "Note as open questions"
    description: "Document but proceed with blueprint"
  - label: "Ignore these"
    description: "These are acceptable gaps"
</step>

<step n="3" name="apply_recursive_summarization">
Create an executive summary that captures the essence.

**Summarization Process:**

1. Start with everything
2. Remove details, keep insights
3. Reduce to core essence
4. Test: "If someone reads only this, will they understand what we're building?"

**Executive Summary Structure:**

- **One sentence:** What is this product?
- **One paragraph:** What problem does it solve and for whom?
- **Key numbers:** How many features, entities, integrations, constraints?
- **Core focus:** What's the one thing this product must nail?

Review the summary:
- Is it compelling?
- Does it accurately represent the full blueprint?
- Would a stakeholder understand the project from this alone?
</step>

<step n="4" name="generate_blueprint">
Write the final blueprint document.

**Blueprint Format:**

```markdown
# Product Blueprint: {Project Name}

*Generated: {date}*

## Executive Summary

{One paragraph distillation from step 3}

**Key Numbers:**
- {X} user personas
- {Y} features
- {Z} data entities
- {N} integrations

---

## 1. Vision

### One-Liner
{pitch}

### Problem Statement
{what we're solving and for whom}

### Value Proposition
{why this solution matters}

### Success Criteria
{how we'll know it worked}

---

## 2. Target Users

{For each persona:}

### {Persona Name} {(Primary) if applicable}

**Description:** {who they are}

**Goals:**
- Primary: {main goal}
- Secondary: {other goals}

**Pain Points:**
- {frustration 1}
- {frustration 2}

**Must-Haves:** {non-negotiables}

**Nice-to-Haves:** {delighters}

---

## 3. Features

| ID | Feature | Description | Priority | Serves Persona |
|----|---------|-------------|----------|----------------|
| F1 | {name} | {description} | {P1/P2/P3} | {persona} |
| F2 | {name} | {description} | {P1/P2/P3} | {persona} |
...

---

## 4. User Flows

### {Flow Name}

**Purpose:** {what user accomplishes}

**Entry:** {where user starts}

**Steps:**
1. {step}
2. {step}
3. {step}

**Exit:** {success state}

**Decision Points:**
- At step {X}: {choices}

**Edge Cases:**
- {what could go wrong}: {how to handle}

{Repeat for each flow}

---

## 5. Data Model

### Entities

{For each entity:}

**{Entity Name}**
- {attribute}: {description}
- {attribute}: {description}

### Relationships

```
{Entity A} ──{relationship}──> {Entity B}
{Entity C} ──{relationship}──> {Entity D}
```

### Lifecycle

| Entity | Created | Updated | Deleted |
|--------|---------|---------|---------|
| {name} | {when} | {when} | {when} |

---

## 6. Integrations

| Service | Category | Purpose | Provider | Criticality |
|---------|----------|---------|----------|-------------|
| {name} | {category} | {why needed} | {provider} | Must/Should |

### Integration Details

{For each integration:}

**{Service Name}**
- **Data In:** {what we send}
- **Data Out:** {what we receive}
- **Trigger:** {when it's called}

---

## 7. Constraints

| Category | Requirement | Rationale |
|----------|-------------|-----------|
| Performance | {requirement} | {why} |
| Security | {requirement} | {why} |
| Platform | {requirement} | {why} |
| Accessibility | {requirement} | {why} |
| Localization | {requirement} | {why} |

---

## 8. Open Questions

{List any items needing future clarification}

- [ ] {question 1}
- [ ] {question 2}

---

*Blueprint generated by create-blueprint skill*
```

**Write to file:**

Save the blueprint to `.opensdd/blueprint.md`:

```bash
mkdir -p .opensdd
```

Write the blueprint content to `.opensdd/blueprint.md`.
</step>

<step n="5" name="identify_open_questions">
Flag anything that needs future clarification.

**Review for gaps:**
- Any "TBD" or placeholder values?
- Any decisions deferred?
- Any assumptions that need validation?
- Any dependencies on external factors?

**Document as open questions in the blueprint.**
</step>

</steps>

<output>
Complete blueprint document saved as `.opensdd/blueprint.md`
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| gather_all_outputs | All phase data retrieved | ✓ / ✗ |
| apply_chain_of_verification | Internal consistency checked | ✓ / ✗ |
| apply_recursive_summarization | Executive summary created | ✓ / ✗ |
| generate_blueprint | Blueprint file written | ✓ / ✗ |
| identify_open_questions | Open questions documented | ✓ / ✗ |

If any step failed (✗):
- Return to that step and redo
- Do NOT proceed until all steps pass
</verify>

<checkpoint required="true">

**AI Quick Check (internal):**

Run comprehensive cross-reference check:
```
issues = []

# Vision ↔ Features: Every vision element should have supporting features
vision = get_vision()
features = get_features()
for element in vision.key_elements:
    if not any(feature_supports(f, element) for f in features):
        issues.append(f"Vision element '{element}' has no supporting feature")

# Features ↔ Personas: Every feature should serve at least one persona
personas = get_personas()
for feature in features:
    if not any(feature_serves(feature, p) for p in personas):
        issues.append(f"Feature '{feature.name}' doesn't serve any persona")

# Features ↔ Flows: Every feature should be in at least one flow
flows = get_flows()
for feature in features:
    if not any(feature_in_flow(feature, f) for f in flows):
        issues.append(f"Feature '{feature.name}' has no user flow")

# Flows ↔ Data: Every flow's data needs should be met
entities = get_entities()
for flow in flows:
    for need in flow.data_needs:
        if not any(entity_provides(e, need) for e in entities):
            issues.append(f"Flow '{flow.name}' needs '{need}' but no entity provides it")

# Constraints: Check for any unresolved conflicts
constraints = get_constraints()
for conflict in find_conflicts(constraints):
    issues.append(f"Unresolved constraint conflict: {conflict}")
```

**If no issues found:**

Use AskUserQuestionTool:
- question: "Blueprint complete. All cross-references check out. Ready to export?"
- options:
  - label: "Export and finish (Recommended)"
    description: "Blueprint is consistent, save the final document"
  - label: "Review full blueprint"
    description: "Show me the complete document first"
  - label: "Make adjustments"
    description: "I want to modify some sections"
  - label: "Save and pause"
    description: "Continue later"

**If issues found:**

Present issues first:

"Before finalizing, I found [N] consistency issue(s):

[For each issue:]
⚠ [issue description]

These gaps may cause problems during implementation."

Use AskUserQuestionTool:
- question: "How would you like to handle these?"
- options:
  - label: "Fix issues (Recommended)"
    description: "Return to relevant phase to address gaps"
  - label: "Note as open questions"
    description: "Add to Open Questions section, export anyway"
  - label: "Export anyway"
    description: "Accept gaps, finalize blueprint as-is"
  - label: "Save and pause"
    description: "Think it over, continue later"

On response:
- "Export/Recommended (no issues)": Proceed to <next>
- "Review full blueprint": Display blueprint, then ask again
- "Fix issues/Make adjustments": Ask which phase, return to that phase
- "Note as open questions": Add issues to Open Questions, proceed to <next>
- "Export anyway": Proceed to <next> with gaps accepted
- "Save and pause": Save state, end session
</checkpoint>

<next>
1. Complete phase and skill:
   ```bash
   python .opensdd/blueprint.state.py complete-phase 8
   ```

2. Speak to user:
   "Blueprint complete!

   Your blueprint has been saved to: `.opensdd/blueprint.md`

   This document contains everything needed to begin development:
   - Vision and success criteria
   - User personas and their goals
   - Prioritized feature list
   - User flows with decision points
   - Data model with entities and relationships
   - Integration requirements
   - Non-functional constraints

   Next steps:
   1. Review the blueprint with stakeholders
   2. Address any open questions
   3. Run `/opensdd:spec` to generate the technical specification

   Good luck with your project!"

3. No next phase. Workflow complete.
</next>
