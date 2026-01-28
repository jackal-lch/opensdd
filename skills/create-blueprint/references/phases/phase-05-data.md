---
phase: 5
name: data
next: phase-06-integrations.md
---

# Phase 5: Data

<objective>
Define what information the product manages (non-technical representation).
</objective>

<prerequisite>
Get SKILL_ROOT and verify previous phase complete:

```bash
SKILL_ROOT=$(python .opensdd/blueprint.state.py get-skill-root)
python .opensdd/blueprint.state.py check-phase 4
```

If exit code != 0:
- Show: "Phase 4 (Flows) must be complete first."
- STOP workflow.
</prerequisite>

<input>
From previous phases:
```bash
python .opensdd/blueprint.state.py get-data 3 features
python .opensdd/blueprint.state.py get-data 4 flows
```
</input>

<steps>

<step n="1" name="extract_entities">
AI analyzes features and flows to propose data entities.

**Analysis Process:**

Scan through:
- Features: What things do features operate on?
- Flows: What information passes between steps?
- Personas: What do users create/manage/view?

**Common Entity Patterns:**

| If the product... | You likely need... |
|-------------------|-------------------|
| Has users | User, Profile, Account |
| Has content | Post, Article, Document, Media |
| Has commerce | Product, Order, Payment, Cart |
| Has communication | Message, Notification, Thread |
| Has organization | Team, Organization, Role |
| Has scheduling | Event, Appointment, Calendar |
| Has tracking | Activity, Log, Metric |

Present proposed entities:
"Based on your features and flows, I've identified these core entities..."

List each entity with:
- Name
- Description (what it represents)
- Why it's needed (which features use it)
</step>

<step n="2" name="apply_first_principles">
For each entity, strip assumptions and define what's actually needed.

**First Principles Questions:**

For each entity, ask:
1. "Is this entity actually needed, or can another entity handle it?"
2. "What's the minimum data this entity must have for features to work?"
3. "What data are we tempted to add 'just in case' but don't actually need?"

**Apply Occam's Razor:**
- Prefer fewer entities with clear purposes
- Avoid entities that "might be useful later"
- Every entity should map to specific features

Use AskUserQuestionTool:
- question: "I've refined the entity list using first principles. Does this feel right?"
- options:
  - label: "Entities are correct"
    description: "This captures what we need to track"
  - label: "Missing something"
    description: "We need to track additional things"
  - label: "Too many entities"
    description: "Some of these could be combined or removed"
</step>

<step n="3" name="define_relationships">
How do entities connect to each other?

**Relationship Types (plain language):**

| Relationship | Meaning |
|--------------|---------|
| "has many" | One X can have multiple Y (User has many Orders) |
| "belongs to" | One X is part of one Y (Order belongs to User) |
| "has one" | One X has exactly one Y (User has one Profile) |
| "connects to" | X and Y are associated (User connects to Team) |

**Map relationships:**

Draw connections between entities:
```
User ──has many──> Order
User ──has one──> Profile
Order ──has many──> OrderItem
OrderItem ──belongs to──> Product
```

Present as a simple relationship list, not a technical diagram.

Use AskUserQuestionTool:
- question: "These are the relationships between entities. Anything missing or incorrect?"
- options:
  - label: "Relationships are correct"
    description: "The connections make sense"
  - label: "Missing connections"
    description: "Some entities should be related"
  - label: "Incorrect relationships"
    description: "Some connections are wrong"
</step>

<step n="4" name="identify_key_attributes">
For each entity, define critical fields.

**Focus on:**
- What information is essential for the entity to exist?
- What information is needed for features to work?
- What information do users see/input?

**Avoid:**
- Technical fields (IDs, timestamps) - those are implementation details
- Fields "we might need later"
- Over-detailed specifications

**Format per entity:**

**[Entity Name]**
- [Attribute]: [Description] - [Why needed]
- [Attribute]: [Description] - [Why needed]
...

Example:
**Order**
- Items: What products and quantities - For checkout
- Total: Order value - For display and payment
- Status: Where in process - For tracking
- Customer: Who placed it - For fulfillment
</step>

<step n="5" name="data_lifecycle">
How is data created, updated, and deleted?

For each entity:

| Entity | Created When | Updated When | Deleted When |
|--------|-------------|--------------|--------------|
| User | Registration | Profile edit | Account deletion |
| Order | Checkout | Status changes | Never (archived) |

**Consider:**
- Who can create/modify/delete each entity?
- Are there entities that should never be deleted (just archived)?
- What happens to related entities when one is deleted?

Use AskUserQuestionTool:
- question: "This is the data lifecycle. Does this match how you expect data to behave?"
- options:
  - label: "Lifecycle is correct"
    description: "Data creation/updates/deletion makes sense"
  - label: "Needs adjustment"
    description: "Some lifecycle rules should change"
</step>

</steps>

<output>
Data model with entities, relationships, key attributes, and lifecycle rules.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| extract_entities | Entities identified from features/flows | ✓ / ✗ |
| apply_first_principles | Entities refined to essentials | ✓ / ✗ |
| define_relationships | Entity connections mapped | ✓ / ✗ |
| identify_key_attributes | Critical attributes defined | ✓ / ✗ |
| data_lifecycle | Creation/update/deletion rules set | ✓ / ✗ |

If any step failed (✗):
- Return to that step and redo
- Do NOT proceed until all steps pass
</verify>

<checkpoint required="true">

**AI Quick Check (internal):**

Check for data gaps (flows needing data that no entity provides):
```
flows = get_all_flows()
entities = get_all_entities()
issues = []

for flow in flows:
    data_needs = extract_data_needs(flow)  # What data does this flow read/write?
    for need in data_needs:
        if not any(entity_provides(e, need) for e in entities):
            issues.append({
                "flow": flow.name,
                "data_need": need,
                "message": f"Flow '{flow.name}' needs '{need}' but no entity stores it."
            })
```

**If no issues found:**

Use AskUserQuestionTool:
- question: "Data model complete. All flows have data support. Ready for Integrations?"
- options:
  - label: "Continue to Integrations (Recommended)"
    description: "Data model is sufficient, proceed with confidence"
  - label: "Review data model"
    description: "Show me entities and relationships before continuing"
  - label: "Refine data model"
    description: "I want to adjust entities or relationships"
  - label: "Save and pause"
    description: "Continue later"

**If issues found:**

Present issues first:

"Before continuing, I found [N] data gap(s):

[For each gap:]
⚠ Flow '[flow.name]' needs '[data_need]' but no entity stores it.

Missing data means the flow can't work as designed."

Use AskUserQuestionTool:
- question: "How would you like to handle this?"
- options:
  - label: "Add missing data (Recommended)"
    description: "Add attribute or entity for '[data_need]'"
  - label: "Simplify flow"
    description: "Remove the data requirement from the flow"
  - label: "Continue anyway"
    description: "Proceed, will figure out data during development"
  - label: "Save and pause"
    description: "Think it over, continue later"

On response:
- "Continue/Recommended (no issues)": Proceed to <next>
- "Add missing data/Refine data model": Return to step 4
- "Simplify flow": Return to Phase 4 to adjust flow
- "Continue anyway": Proceed to <next> with warning noted
- "Save and pause": Save state, end session
</checkpoint>

<next>
1. Save data model:
   ```bash
   python .opensdd/blueprint.state.py set-data 5 entities "<JSON of entities>"
   python .opensdd/blueprint.state.py set-data 5 relationships "<JSON of relationships>"
   python .opensdd/blueprint.state.py complete-phase 5
   ```

2. Speak to user:
   "Data model complete. Moving to identify integrations..."

3. Load: `phase-06-integrations.md` (same folder)
</next>
