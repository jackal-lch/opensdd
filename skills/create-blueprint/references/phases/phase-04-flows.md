---
phase: 4
name: flows
next: phase-05-data.md
---

# Phase 4: User Flows

<objective>
Map how users accomplish their goals in the product.
</objective>

<prerequisite>
Get SKILL_ROOT and verify previous phase complete:

```bash
SKILL_ROOT=$(python .opensdd/blueprint.state.py get-skill-root)
python .opensdd/blueprint.state.py check-phase 3
```

If exit code != 0:
- Show: "Phase 3 (Features) must be complete first."
- STOP workflow.
</prerequisite>

<input>
From previous phases:
```bash
python .opensdd/blueprint.state.py get-data 2 personas
python .opensdd/blueprint.state.py get-data 2 primary_persona
python .opensdd/blueprint.state.py get-data 3 features
```
</input>

<steps>

<step n="1" name="identify_key_journeys">
Based on personas and features, AI proposes key user flows.

**Essential Flows to Consider:**

1. **First-Time User Flow**
   - How do they discover the product?
   - What's the onboarding experience?
   - What's the "aha moment"?

2. **Core Value Flow**
   - What's the main thing users do?
   - How do they accomplish their primary goal?
   - What's the happy path?

3. **Secondary Flows**
   - Supporting activities
   - Settings/configuration
   - Help/support

4. **Return User Flow**
   - What brings them back?
   - What's the daily/weekly usage pattern?

Use AskUserQuestionTool:
- question: "I've identified these key user journeys. Which should we map in detail?"
- options: [List of flows with descriptions]
- multiSelect: true
</step>

<step n="2" name="reverse_engineer_flows">
For each selected flow, work backwards from the goal.

**Apply Reverse Engineering Technique:**

Start with: "User achieves [goal]"
Then ask: "What was the step before that?"
Continue until reaching the entry point.

Example for "User completes a purchase":
```
Goal: Order confirmed, user satisfied
  ← User clicks "Place Order"
  ← User reviews order summary
  ← User enters payment info
  ← User enters shipping address
  ← User views cart
  ← User adds items to cart
  ← User browses products
Entry: User lands on homepage
```

For each flow, document:
- Entry point (where user starts)
- Steps (in sequence)
- Exit point (goal achieved)
- Key screens/pages involved
</step>

<step n="3" name="map_decision_points">
Identify where users make choices.

For each flow, ask:
- Where do users make decisions?
- What options do they have at each point?
- What information do they need to decide?
- What happens for each choice?

**Document decision points:**

| Flow | Decision Point | Options | What Helps Decide |
|------|---------------|---------|-------------------|
| [flow] | [where] | [choices] | [info/context needed] |

Use AskUserQuestionTool to validate:
- question: "For the [flow name] flow, I identified these decision points. Anything missing?"
- options:
  - label: "Looks complete"
    description: "All key decision points captured"
  - label: "Missing decisions"
    description: "There are more choices users need to make"
  - label: "Over-complicated"
    description: "Some decisions can be simplified or removed"
</step>

<step n="4" name="identify_edge_cases">
What happens when things go wrong?

For each flow, consider:
- What if the user makes a mistake?
- What if required data is missing?
- What if an external service fails?
- What if the user changes their mind?

**Document error/edge cases:**

| Flow | Edge Case | How to Handle |
|------|-----------|---------------|
| [flow] | [what goes wrong] | [recovery path] |

Present to user:
"Here's how we'll handle edge cases. Any scenarios missing?"
</step>

<step n="5" name="validate_against_personas">
Ensure each persona has a clear path to their goals.

**Cross-reference check:**

| Persona | Primary Goal | Flow That Achieves It | Covered? |
|---------|-------------|----------------------|----------|
| [name] | [goal] | [flow name] | ✓ / ✗ |

If any persona's goal isn't covered by a flow:
- Flag it
- Create a new flow or extend an existing one

Use AskUserQuestionTool:
- question: "All personas have paths to their goals. Does this mapping look right?"
- options:
  - label: "Mapping is correct"
    description: "Each persona can accomplish their goals"
  - label: "Gaps exist"
    description: "Some personas don't have clear paths"
</step>

</steps>

<output>
User flows with decision points, edge cases, and persona mapping.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| identify_key_journeys | Key flows identified and selected | ✓ / ✗ |
| reverse_engineer_flows | Each flow mapped step-by-step | ✓ / ✗ |
| map_decision_points | Decision points documented | ✓ / ✗ |
| identify_edge_cases | Error handling defined | ✓ / ✗ |
| validate_against_personas | All personas have clear paths | ✓ / ✗ |

If any step failed (✗):
- Return to that step and redo
- Do NOT proceed until all steps pass
</verify>

<checkpoint required="true">

**AI Quick Check (internal):**

Check for orphaned features (features with no flow):
```
features = get_features()
flows = get_all_flows()
issues = []

for feature in features:
    feature_in_flow = False
    for flow in flows:
        if feature_used_in_flow(feature, flow):
            feature_in_flow = True
            break

    if not feature_in_flow:
        issues.append({
            "feature": feature.id,
            "name": feature.name,
            "message": f"Feature '{feature.name}' ({feature.id}) has no user flow. How do users access it?"
        })
```

**If no issues found:**

Use AskUserQuestionTool:
- question: "Flows mapped. All features covered. Ready for Data?"
- options:
  - label: "Continue to Data (Recommended)"
    description: "Flows are complete, proceed with confidence"
  - label: "Review flows"
    description: "Show me the flow summaries before continuing"
  - label: "Add more flows"
    description: "I want to map additional user journeys"
  - label: "Save and pause"
    description: "Continue later"

**If issues found:**

Present issues first:

"Before continuing, I found [N] feature(s) without user flows:

[For each orphaned feature:]
⚠ [feature.name] ([feature.id]) - No flow shows how users access this feature.

Features without flows can't be implemented - there's no defined path to reach them."

Use AskUserQuestionTool:
- question: "How would you like to handle this?"
- options:
  - label: "Add missing flows (Recommended)"
    description: "Create flows for orphaned features"
  - label: "Remove orphaned features"
    description: "Cut these features from scope"
  - label: "Continue anyway"
    description: "Proceed, will figure out flows during development"
  - label: "Save and pause"
    description: "Think it over, continue later"

On response:
- "Continue/Recommended (no issues)": Proceed to <next>
- "Add missing flows/Add more flows": Return to step 1
- "Remove orphaned features": Return to Phase 3 to adjust scope
- "Continue anyway": Proceed to <next> with warning noted
- "Save and pause": Save state, end session
</checkpoint>

<next>
1. Save flows data:
   ```bash
   python .opensdd/blueprint.state.py set-data 4 flows "<JSON of user flows>"
   python .opensdd/blueprint.state.py complete-phase 4
   ```

2. Speak to user:
   "User flows complete. Moving to define data model..."

3. Load: `phase-05-data.md` (same folder)
</next>
