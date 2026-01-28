---
phase: 1
name: vision
next: phase-02-users.md
---

# Phase 1: Vision

<objective>
Capture the core idea with enough depth to guide all subsequent phases.
</objective>

<prerequisite>
Get SKILL_ROOT and verify previous phase complete:

```bash
SKILL_ROOT=$(python .opensdd/blueprint.state.py get-skill-root)
python .opensdd/blueprint.state.py check-phase 0
```

If exit code != 0:
- Show: "Phase 0 must be complete first."
- STOP workflow.

If exit code == 0:
- Proceed.
</prerequisite>

<input>
Project name from Phase 0 (if captured), otherwise start fresh.
</input>

<steps>

<step n="1" name="capture_raw_vision">
Invite user to share their vision freely.

Use AskUserQuestionTool:
- question: "What do you want to build? Share your idea - it can be rough, partial, or just a spark of an idea."
- options:
  - label: "I have a clear idea"
    description: "I know what I want to build and can describe it"
  - label: "I have a rough concept"
    description: "I have a general direction but details are fuzzy"
  - label: "I have a problem to solve"
    description: "I know the problem but not the solution yet"
  - label: "I'm exploring"
    description: "Help me figure out what to build"

Based on their choice, adapt your questioning approach:
- Clear idea: Ask them to describe it in detail
- Rough concept: Ask probing questions to sharpen the concept
- Problem to solve: Focus on understanding the problem deeply first
- Exploring: Guide them through discovery questions

Capture everything they share without judgment.
</step>

<step n="2" name="step_back_big_picture">
Before diving into details, think big picture.

**Apply Step-Back Technique:**

Ask yourself and guide user through:
1. "What's the broader context this product exists in?"
   - What market or domain is this?
   - What trends or shifts make this relevant now?

2. "What's the 3-year vision, not just the initial launch?"
   - Where could this go if successful?
   - What's the ultimate destination?

3. "Who else is trying to solve this?"
   - What existing solutions exist?
   - Why aren't they good enough?

Use AskUserQuestionTool to gather this context:
- question: "Let's zoom out. What's the bigger picture for this product?"
- options:
  - label: "It's solving a new problem"
    description: "Nothing good exists for this yet"
  - label: "It's a better solution"
    description: "Existing solutions are lacking"
  - label: "It's a different approach"
    description: "Same problem, novel angle"
  - label: "It's for a specific audience"
    description: "General solutions don't fit my users"
</step>

<step n="3" name="drill_to_core_problem">
Understand the fundamental problem being solved.

**Apply Five Whys Technique:**

Start with: "Why does this problem exist?"
Then keep asking "Why?" until reaching the root cause.

Example:
- "Users can't track their expenses" → Why?
- "Existing apps are too complex" → Why does that matter?
- "People give up after a few days" → Why?
- "The friction outweighs the benefit" → Why?
- "They don't see immediate value" → ROOT CAUSE

Use AskUserQuestionTool iteratively:
- question: "Why does [the problem you identified] exist?"
- options: Generate 3-4 plausible reasons based on context

Continue until you reach a fundamental, actionable insight.

Document:
- Surface problem (what user first described)
- Root cause (what Five Whys revealed)
- Key insight (the "aha" that guides the solution)
</step>

<step n="4" name="synthesize_vision">
AI proposes a structured vision statement based on everything gathered.

**Generate:**

1. **One-line pitch** (< 15 words)
   - "[Product] helps [audience] [achieve goal] by [unique approach]"

2. **Problem statement** (2-3 sentences)
   - What pain exists today
   - Who feels this pain
   - Why current solutions fail

3. **Value proposition** (2-3 sentences)
   - What the product does differently
   - Why that matters
   - What users gain

4. **Success criteria** (3-5 measurable outcomes)
   - How we know it's working
   - What metrics matter

Present to user with AskUserQuestionTool:
- question: "Here's my synthesis of your vision. Does this capture what you want to build?"
- options:
  - label: "Yes, that's it"
    description: "The vision statement captures my intent"
  - label: "Close, needs tweaking"
    description: "The core is right but some details are off"
  - label: "Missing something important"
    description: "There's a key aspect not reflected here"
  - label: "Not quite right"
    description: "Let me re-explain the concept"

If not approved, iterate until user confirms.
</step>

</steps>

<output>
Structured vision statement: one-liner, problem, value prop, success criteria.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| capture_raw_vision | User's raw idea captured | ✓ / ✗ |
| step_back_big_picture | Context and bigger picture understood | ✓ / ✗ |
| drill_to_core_problem | Root cause identified via Five Whys | ✓ / ✗ |
| synthesize_vision | Structured vision statement created and approved | ✓ / ✗ |

If any step failed (✗):
- Return to that step and redo
- Do NOT proceed until all steps pass
</verify>

<checkpoint required="true">

**AI Quick Check (internal):**

Scan success criteria for vague/unmeasurable terms:
- Vague indicators: "happy", "satisfied", "good", "better", "improved", "feel"
- Measurable indicators: numbers, percentages, time limits, counts, comparisons

```
issues = []
for criterion in success_criteria:
    if contains_vague_terms(criterion) and no_metric_present(criterion):
        issues.append(criterion)
```

**If no issues found:**

Use AskUserQuestionTool:
- question: "Vision complete. Ready to define your target users?"
- options:
  - label: "Continue to Users (Recommended)"
    description: "Vision is solid, proceed with confidence"
  - label: "Review vision"
    description: "Show me the vision statement before continuing"
  - label: "Make changes"
    description: "I want to adjust the vision"
  - label: "Save and pause"
    description: "Continue later"

**If issues found:**

Present issues first:

"Before continuing, I noticed [N] success criteria may be hard to measure:

⚠ '[criterion]' - How will you know when this is achieved? Consider adding a metric.

Unmeasurable criteria make it hard to know if the product succeeded."

Use AskUserQuestionTool:
- question: "How would you like to handle this?"
- options:
  - label: "Make criteria measurable (Recommended)"
    description: "Refine success criteria with specific metrics"
  - label: "Continue anyway"
    description: "Accept as-is, proceed to Users"
  - label: "Save and pause"
    description: "Think it over, continue later"

On user response:
- "Continue/Recommended": Proceed to <next>
- "Review/Make changes/Make criteria measurable": Return to step 4
- "Save and pause": Save state, end session
</checkpoint>

<next>
1. Save vision data:
   ```bash
   python .opensdd/blueprint.state.py set-data 1 vision "<JSON of vision statement>"
   python .opensdd/blueprint.state.py complete-phase 1
   ```

2. Speak to user:
   "Vision captured. Moving to define your target users..."

3. Load: `phase-02-users.md` (same folder)
</next>
