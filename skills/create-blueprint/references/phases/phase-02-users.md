---
phase: 2
name: users
next: phase-03-features.md
---

# Phase 2: Users

<objective>
Define who will use this product and what they need.
</objective>

<prerequisite>
Get SKILL_ROOT and verify previous phase complete:

```bash
SKILL_ROOT=$(python .opensdd/blueprint.state.py get-skill-root)
python .opensdd/blueprint.state.py check-phase 1
```

If exit code != 0:
- Show: "Phase 1 (Vision) must be complete first."
- STOP workflow.
</prerequisite>

<input>
Vision statement from Phase 1:
```bash
python .opensdd/blueprint.state.py get-data 1 vision
```
</input>

<steps>

<step n="1" name="identify_user_types">
AI proactively suggests user types based on the vision.

**AI Analysis:**
Based on the vision statement, identify likely user types:
- Who has the problem we're solving?
- Who would pay for/use this solution?
- Are there different user roles (admin, end-user, etc.)?
- Are there power users vs casual users?

**Propose 2-4 user types with brief descriptions.**

Use AskUserQuestionTool:
- question: "Based on your vision, I see these potential user types. Which ones are relevant?"
- options: [Generated user types with descriptions]
- multiSelect: true

Capture which user types the user confirms.
</step>

<step n="2" name="deep_dive_personas">
For each confirmed user type, apply Socratic questioning to build a rich persona.

**Socratic Questions for Each Persona:**

Ask these questions (use AskUserQuestionTool with relevant options):

1. "What does this user do BEFORE your product exists?"
   - What's their current workflow?
   - What tools do they use today?

2. "What frustrates them about current solutions?"
   - Where do they waste time?
   - What makes them curse under their breath?

3. "What would make them CHOOSE your product?"
   - What feature would be the hook?
   - What would make them switch from current solution?

4. "What would make them ABANDON your product?"
   - What would be a dealbreaker?
   - What friction would cause them to leave?

5. "How technical/experienced are they?"
   - What can we assume they know?
   - What do we need to teach them?

For each question, AI should propose likely answers and let user confirm/modify.

Document findings in structured persona format.
</step>

<step n="3" name="define_goals_and_needs">
For each persona, define concrete goals and needs.

**Structure per persona:**

| Aspect | Description |
|--------|-------------|
| **Primary Goal** | The #1 thing they want to accomplish |
| **Secondary Goals** | Other things they'd like to do |
| **Pain Points** | Specific frustrations to address |
| **Must-Haves** | Non-negotiable requirements |
| **Nice-to-Haves** | Would delight but not required |
| **Anti-Requirements** | Things they explicitly don't want |

Use AskUserQuestionTool to validate each persona's goals:
- question: "For [Persona Name], does this capture their goals correctly?"
- options:
  - label: "Yes, accurate"
    description: "Goals and needs are well captured"
  - label: "Needs adjustment"
    description: "Some goals need to be modified"
  - label: "Missing goals"
    description: "There are important goals not listed"
</step>

<step n="4" name="prioritize_users">
Determine the primary target persona.

Use AskUserQuestionTool:
- question: "Which user persona should be the PRIMARY focus?"
- options: [List of personas with brief descriptions]

Capture the primary persona. All subsequent phases should optimize for this persona first, others second.

Also ask:
- question: "Are any personas explicitly OUT of scope?"
- options: [List of personas]
- multiSelect: true
</step>

</steps>

<output>
User personas with goals, needs, and priority designation.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| identify_user_types | User types identified and confirmed | ✓ / ✗ |
| deep_dive_personas | Each persona deeply understood | ✓ / ✗ |
| define_goals_and_needs | Goals/needs documented per persona | ✓ / ✗ |
| prioritize_users | Primary persona selected | ✓ / ✗ |

If any step failed (✗):
- Return to that step and redo
- Do NOT proceed until all steps pass
</verify>

<checkpoint required="true">

**AI Quick Check (internal):**

Verify primary persona has actionable must-haves:
```
primary = get_primary_persona()
must_haves = primary.get("must_haves", [])
issues = []

if len(must_haves) < 2:
    issues.append("Primary persona has fewer than 2 must-haves defined")

if all(item is vague for item in must_haves):  # e.g., "easy to use", "works well"
    issues.append("Must-haves are too vague to guide feature decisions")
```

**If no issues found:**

Use AskUserQuestionTool:
- question: "Personas complete. Ready to discover features?"
- options:
  - label: "Continue to Features (Recommended)"
    description: "Personas are solid, proceed with confidence"
  - label: "Review personas"
    description: "Show me the persona details before continuing"
  - label: "Make changes"
    description: "I want to adjust the personas"
  - label: "Save and pause"
    description: "Continue later"

**If issues found:**

Present issues first:

"Before continuing, I found an issue with the primary persona:

⚠ [Issue description]

The Features phase uses must-haves to prioritize what to build. Without clear must-haves, prioritization will be guesswork."

Use AskUserQuestionTool:
- question: "How would you like to handle this?"
- options:
  - label: "Define must-haves (Recommended)"
    description: "Add specific must-haves for primary persona"
  - label: "Continue anyway"
    description: "Proceed to Features without clear must-haves"
  - label: "Save and pause"
    description: "Think it over, continue later"

On response:
- "Continue/Recommended": Proceed to <next>
- "Review/Make changes/Define must-haves": Return to step 3
- "Save and pause": Save state, end session
</checkpoint>

<next>
1. Save users data:
   ```bash
   python .opensdd/blueprint.state.py set-data 2 personas "<JSON of personas>"
   python .opensdd/blueprint.state.py set-data 2 primary_persona "<primary persona name>"
   python .opensdd/blueprint.state.py complete-phase 2
   ```

2. Speak to user:
   "Personas defined. Moving to discover features..."

3. Load: `phase-03-features.md` (same folder)
</next>
