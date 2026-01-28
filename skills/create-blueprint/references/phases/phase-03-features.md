---
phase: 3
name: features
next: phase-04-flows.md
---

# Phase 3: Features

<objective>
Discover comprehensive feature set and prioritize for implementation.
</objective>

<prerequisite>
Get SKILL_ROOT and verify previous phase complete:

```bash
SKILL_ROOT=$(python .opensdd/blueprint.state.py get-skill-root)
python .opensdd/blueprint.state.py check-phase 2
```

If exit code != 0:
- Show: "Phase 2 (Users) must be complete first."
- STOP workflow.
</prerequisite>

<input>
From previous phases:
```bash
python .opensdd/blueprint.state.py get-data 1 vision
python .opensdd/blueprint.state.py get-data 2 personas
python .opensdd/blueprint.state.py get-data 2 primary_persona
```
</input>

<steps>

<step n="1" name="brainstorm_features">
AI applies SCAMPER technique to generate feature ideas.

**SCAMPER Analysis:**

Based on the vision and personas, systematically explore:

| Lens | Question | Feature Ideas |
|------|----------|---------------|
| **Substitute** | What can replace existing solutions? | [AI generates] |
| **Combine** | What features work together synergistically? | [AI generates] |
| **Adapt** | What works in similar products we can adapt? | [AI generates] |
| **Modify** | What could be bigger/smaller/different? | [AI generates] |
| **Put to other use** | Secondary use cases? | [AI generates] |
| **Eliminate** | What unnecessary complexity can we remove? | [AI generates] |
| **Reverse** | What if we flip assumptions? | [AI generates] |

Present the SCAMPER analysis to user:
"I've brainstormed features using the SCAMPER method. Here's what I came up with..."

List features grouped by SCAMPER category.
</step>

<step n="2" name="user_feature_input">
Capture features the user specifically wants.

Use AskUserQuestionTool:
- question: "Beyond the AI suggestions, what specific features do you have in mind?"
- options:
  - label: "I have specific features"
    description: "Let me tell you what I want"
  - label: "The AI list is comprehensive"
    description: "I don't have additional features to add"
  - label: "I want to remove some"
    description: "Some AI suggestions don't fit"

If user has specific features, capture them.
If user wants to remove some, let them identify which ones don't fit.
</step>

<step n="3" name="merge_and_dedupe">
Combine AI suggestions and user input into a unified feature list.

**Process:**
1. Merge all features from SCAMPER + user input
2. Identify duplicates and merge them
3. Group related features into logical categories
4. Assign each feature a unique ID (F1, F2, F3...)

Present the consolidated feature list:
- Category 1: [features]
- Category 2: [features]
- etc.
</step>

<step n="4" name="prioritize_features">
Apply Tree of Thoughts to explore prioritization paths.

**Path A: Prioritize by User Value**
- Which features directly address primary persona's #1 goal?
- Which features solve the biggest pain points?
- Score each feature 1-5 for user value

**Path B: Prioritize by Technical Feasibility**
- Which features are straightforward to build?
- Which have dependencies or unknowns?
- Score each feature 1-5 for feasibility

**Path C: Prioritize by Business Impact**
- Which features differentiate from competitors?
- Which features are table stakes (must have)?
- Score each feature 1-5 for business impact

**Synthesize:**
Combine scores: (User Value × 2) + Feasibility + Business Impact
Sort by combined score.

Present prioritized list to user with reasoning.

Use AskUserQuestionTool:
- question: "Here's my recommended feature priority. Does this feel right?"
- options:
  - label: "Priority looks good"
    description: "This ordering makes sense"
  - label: "Adjust priorities"
    description: "Some features should be higher/lower"
  - label: "Re-evaluate criteria"
    description: "Let's weight the criteria differently"
</step>

<step n="5" name="finalize_features">
Confirm the final feature list for implementation.

Present the prioritized feature list to the user.

Use AskUserQuestionTool:
- question: "Here's the complete feature list. Any features to add or remove?"
- options:
  - label: "List looks complete"
    description: "Ready to proceed with these features"
  - label: "Add more features"
    description: "I have additional features to include"
  - label: "Remove some features"
    description: "Some features should be cut"

**Document:**
- Final Features: [complete list with IDs and priorities]
</step>

</steps>

<output>
Prioritized feature list ready for implementation.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| brainstorm_features | SCAMPER analysis complete | ✓ / ✗ |
| user_feature_input | User features captured | ✓ / ✗ |
| merge_and_dedupe | Unified feature list created | ✓ / ✗ |
| prioritize_features | Features scored and prioritized | ✓ / ✗ |
| finalize_features | Feature list finalized | ✓ / ✗ |

If any step failed (✗):
- Return to that step and redo
- Do NOT proceed until all steps pass
</verify>

<checkpoint required="true">

**AI Quick Check (internal):**

Check for coverage issues:
```
features = get_features()
primary_persona = get_primary_persona()
must_haves = primary_persona.get("must_haves", [])
issues = []

# Check: Must-have coverage
for must_have in must_haves:
    if not any(feature_addresses(f, must_have) for f in features):
        issues.append({
            "type": "must_have_gap",
            "message": f"Primary persona needs '{must_have}' but no feature addresses it.",
            "suggestion": "Add a feature for this must-have or reconsider if it's truly required"
        })
```

**If no issues found:**

Use AskUserQuestionTool:
- question: "Features finalized ([N] total). Ready to map user flows?"
- options:
  - label: "Continue to Flows (Recommended)"
    description: "Feature list complete, proceed"
  - label: "Review features"
    description: "Show me the feature list before continuing"
  - label: "Adjust features"
    description: "I want to modify the feature list"
  - label: "Save and pause"
    description: "Continue later"

**If issues found:**

Present issues first:

"Before continuing, I found [N] coverage issue(s):

[For each issue:]
⚠ [message]
  → [suggestion]"

Use AskUserQuestionTool:
- question: "How would you like to handle this?"
- options:
  - If must_have_gap:
    - label: "Add missing feature (Recommended)"
      description: "Include feature for '[must_have]'"
  - Always:
    - label: "Continue anyway"
      description: "Accept current list with noted gaps"
    - label: "Save and pause"
      description: "Think it over, continue later"

On response:
- "Continue/Recommended (no issues)": Proceed to <next>
- "Add missing/Adjust features": Return to step 5
- "Continue anyway": Proceed to <next> with warning noted
- "Save and pause": Save state, end session
</checkpoint>

<next>
1. Save features data:
   ```bash
   python .opensdd/blueprint.state.py set-data 3 features "<JSON of all features>"
   python .opensdd/blueprint.state.py complete-phase 3
   ```

2. Speak to user:
   "Features locked. Moving to map user flows..."

3. Load: `phase-04-flows.md` (same folder)
</next>
