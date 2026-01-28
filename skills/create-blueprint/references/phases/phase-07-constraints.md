---
phase: 7
name: constraints
next: phase-08-assembly.md
---

# Phase 7: Constraints

<objective>
Capture non-functional requirements and constraints.
</objective>

<prerequisite>
Get SKILL_ROOT and verify previous phase complete:

```bash
SKILL_ROOT=$(python .opensdd/blueprint.state.py get-skill-root)
python .opensdd/blueprint.state.py check-phase 6
```

If exit code != 0:
- Show: "Phase 6 (Integrations) must be complete first."
- STOP workflow.
</prerequisite>

<input>
Full context from all previous phases available in state.
</input>

<steps>

<step n="1" name="probe_constraints">
AI systematically probes for constraints across key categories.

**Constraint Categories:**

1. **Performance**

Use AskUserQuestionTool:
- question: "What are your performance expectations?"
- options:
  - label: "Standard web app"
    description: "Pages load in 2-3 seconds, handles hundreds of users"
  - label: "Fast and responsive"
    description: "Sub-second responses, handles thousands of users"
  - label: "High performance"
    description: "Near-instant responses, handles large scale"
  - label: "Specific requirements"
    description: "I have exact numbers in mind"

2. **Security**

Use AskUserQuestionTool:
- question: "What security considerations apply?"
- options:
  - label: "Standard security"
    description: "Basic auth, HTTPS, input validation"
  - label: "Sensitive data"
    description: "PII, financial data, encryption at rest"
  - label: "Compliance required"
    description: "HIPAA, SOC2, GDPR, PCI-DSS, etc."
  - label: "Enterprise security"
    description: "SSO, audit logs, role-based access"
- multiSelect: true

3. **Platforms**

Use AskUserQuestionTool:
- question: "What platforms must be supported?"
- options:
  - label: "Web only"
    description: "Desktop and mobile browsers"
  - label: "Web + mobile apps"
    description: "Browsers plus iOS and Android apps"
  - label: "Mobile-first"
    description: "Primarily mobile apps, web secondary"
  - label: "Desktop application"
    description: "Native desktop app required"
- multiSelect: true

4. **Accessibility**

Use AskUserQuestionTool:
- question: "What accessibility standards should we target?"
- options:
  - label: "Basic accessibility"
    description: "Semantic HTML, keyboard navigation"
  - label: "WCAG 2.1 AA"
    description: "Industry standard accessibility compliance"
  - label: "WCAG 2.1 AAA"
    description: "Highest level of accessibility"
  - label: "Not a priority"
    description: "Address accessibility post-launch"

5. **Localization**

Use AskUserQuestionTool:
- question: "Will this need to support multiple languages?"
- options:
  - label: "English only"
    description: "Single language"
  - label: "Plan for i18n"
    description: "Build with translation support, launch in English"
  - label: "Multiple languages at launch"
    description: "Need specific languages from day one"
</step>

<step n="2" name="apply_pre_mortem">
Imagine the product failed and work backwards to prevent it.

**Pre-Mortem Exercise:**

Ask: "Imagine it's 6 months after launch and the product failed. What went wrong?"

**Prompt user through scenarios:**

Use AskUserQuestionTool:
- question: "Which failure scenario worries you most?"
- options:
  - label: "Performance issues"
    description: "It was too slow and users left"
  - label: "Security breach"
    description: "User data was compromised"
  - label: "Poor user experience"
    description: "Users found it confusing and gave up"
  - label: "Technical debt"
    description: "We couldn't iterate fast enough"
  - label: "Wrong features"
    description: "We built what users didn't actually need"

For their top concern, drill deeper:
- "What specifically would cause this failure?"
- "What can we do now to prevent it?"

**Convert fears into constraints:**
- Fear: "Too slow" → Constraint: "Page load < 2s on 3G"
- Fear: "Data breach" → Constraint: "Encrypt PII, regular security audits"
</step>

<step n="3" name="apply_devils_advocate">
Stress test the constraints we've captured.

**Challenge each constraint:**

For each major constraint, ask:
- "What if we're wrong about this constraint?"
- "What's the cost of meeting this constraint?"
- "What if requirements change?"

**Specific stress tests:**

Use AskUserQuestionTool:
- question: "What if you get 10x more users than expected?"
- options:
  - label: "We'd handle it"
    description: "Architecture should scale automatically"
  - label: "We'd need to adapt"
    description: "Some manual scaling would be needed"
  - label: "That's a good problem"
    description: "We'll address it when it happens"

- question: "What if a key integration goes down?"
- options:
  - label: "Must have fallback"
    description: "Core features can't depend on external uptime"
  - label: "Graceful degradation"
    description: "Show error, let user retry later"
  - label: "Accept the dependency"
    description: "We trust the provider's uptime"

- question: "What if requirements change mid-build?"
- options:
  - label: "Build for flexibility"
    description: "Architecture should accommodate changes"
  - label: "Scope is fixed"
    description: "Requirements are locked"
  - label: "Normal iteration"
    description: "We'll adjust as we learn"
</step>

<step n="4" name="document_constraints">
Compile all constraints into a structured list.

**Format:**

| Category | Constraint | Rationale | Priority |
|----------|------------|-----------|----------|
| Performance | [specific requirement] | [why] | Must/Should/Nice |
| Security | [specific requirement] | [why] | Must/Should/Nice |
| Platform | [specific requirement] | [why] | Must/Should/Nice |
| Accessibility | [specific requirement] | [why] | Must/Should/Nice |
| Localization | [specific requirement] | [why] | Must/Should/Nice |

Present to user:
"Here are the consolidated constraints. Does this capture all requirements?"

Use AskUserQuestionTool:
- question: "Are these constraints complete and accurate?"
- options:
  - label: "Constraints are complete"
    description: "All non-functional requirements captured"
  - label: "Missing constraints"
    description: "There are additional requirements"
  - label: "Over-constrained"
    description: "Some constraints are unnecessary"
</step>

</steps>

<output>
Documented constraints with rationale and priorities.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| probe_constraints | All categories probed | ✓ / ✗ |
| apply_pre_mortem | Failure scenarios explored | ✓ / ✗ |
| apply_devils_advocate | Constraints stress-tested | ✓ / ✗ |
| document_constraints | Constraints documented | ✓ / ✗ |

If any step failed (✗):
- Return to that step and redo
- Do NOT proceed until all steps pass
</verify>

<checkpoint required="true">

**AI Quick Check (internal):**

Check for constraint conflicts (two constraints that can't both be true):
```
constraints = get_all_constraints()
issues = []

# Known conflict patterns
conflict_patterns = [
    ("works offline", "real-time sync"),
    ("no account required", "sync across devices"),
    ("no external dependencies", "push notifications"),
    ("sub-second response", "complex animations on low-end devices"),
    ("WCAG AAA", "heavy visual design"),
]

for c1, c2 in all_pairs(constraints):
    if constraints_conflict(c1, c2, conflict_patterns):
        issues.append({
            "constraint1": c1,
            "constraint2": c2,
            "message": f"'{c1.requirement}' conflicts with '{c2.requirement}'.",
            "explanation": get_conflict_explanation(c1, c2)
        })
```

**If no issues found:**

Use AskUserQuestionTool:
- question: "Constraints captured. No conflicts detected. Ready to assemble the blueprint?"
- options:
  - label: "Continue to Assembly (Recommended)"
    description: "Constraints are consistent, proceed to final step"
  - label: "Review constraints"
    description: "Show me all constraints before continuing"
  - label: "Refine constraints"
    description: "I want to adjust some requirements"
  - label: "Save and pause"
    description: "Continue later"

**If issues found:**

Present issues first:

"Before continuing, I found [N] constraint conflict(s):

[For each conflict:]
⚠ Conflict: '[constraint1]' vs '[constraint2]'
  → [explanation - why these can't both be true]
  → You'll need to choose one or adjust both.

Conflicting constraints are impossible to satisfy - one must give."

Use AskUserQuestionTool:
- question: "How would you like to resolve this?"
- options:
  - label: "Resolve conflicts (Recommended)"
    description: "Choose which constraint takes priority"
  - label: "Keep both, will figure out"
    description: "Proceed with conflict noted as open question"
  - label: "Save and pause"
    description: "Think it over, continue later"

On response:
- "Continue/Recommended (no issues)": Proceed to <next>
- "Resolve conflicts/Refine constraints": Return to step 4
- "Keep both": Add to open questions, proceed to <next>
- "Save and pause": Save state, end session
</checkpoint>

<next>
1. Save constraints data:
   ```bash
   python .opensdd/blueprint.state.py set-data 7 constraints "<JSON of constraints>"
   python .opensdd/blueprint.state.py complete-phase 7
   ```

2. Speak to user:
   "Constraints locked. Moving to assemble the final blueprint..."

3. Load: `phase-08-assembly.md` (same folder)
</next>
