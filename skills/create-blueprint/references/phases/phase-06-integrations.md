---
phase: 6
name: integrations
next: phase-07-constraints.md
---

# Phase 6: Integrations

<objective>
Identify external systems, APIs, and dependencies.
</objective>

<prerequisite>
Get SKILL_ROOT and verify previous phase complete:

```bash
SKILL_ROOT=$(python .opensdd/blueprint.state.py get-skill-root)
python .opensdd/blueprint.state.py check-phase 5
```

If exit code != 0:
- Show: "Phase 5 (Data) must be complete first."
- STOP workflow.
</prerequisite>

<input>
From previous phases:
```bash
python .opensdd/blueprint.state.py get-data 1 vision
python .opensdd/blueprint.state.py get-data 3 features
```
</input>

<steps>

<step n="1" name="analyze_for_integrations">
AI proactively identifies likely integration needs based on features.

**Common Integration Categories:**

| Category | When Needed | Examples |
|----------|-------------|----------|
| **Authentication** | Users need accounts | OAuth (Google, Apple), SSO, Magic links |
| **Payments** | Commerce features | Stripe, PayPal, Square |
| **Email** | Notifications, marketing | SendGrid, Mailchimp, SES |
| **Storage** | File uploads, media | S3, Cloudinary, Firebase Storage |
| **Analytics** | Usage tracking | Mixpanel, Amplitude, GA |
| **Search** | Complex queries | Algolia, Elasticsearch |
| **Maps/Location** | Geographic features | Google Maps, Mapbox |
| **Communication** | Real-time features | Twilio, SendBird, Pusher |
| **AI/ML** | Smart features | OpenAI, Claude, custom models |
| **CRM** | Customer data | Salesforce, HubSpot |

**Analysis Process:**
1. Scan features for integration indicators
2. Identify which categories apply
3. Note any features that can't work without external services

Present findings:
"Based on your features, I've identified these potential integration needs..."

List each with:
- Category
- Why needed (which feature requires it)
- Suggested approach
</step>

<step n="2" name="confirm_needs">
Let user confirm which integrations apply.

Use AskUserQuestionTool:
- question: "Which of these integrations do you actually need?"
- options: [List of identified integrations]
- multiSelect: true

Also ask:
- question: "Any integrations I missed?"
- options:
  - label: "List is complete"
    description: "All needed integrations are identified"
  - label: "Need additional integrations"
    description: "There are more external services required"

If user mentions additional integrations, capture them.
</step>

<step n="3" name="specify_requirements">
For each confirmed integration, define requirements.

**For each integration, ask:**

1. **What data flows in/out?**
   - What do we send to this service?
   - What do we receive back?

2. **What triggers the integration?**
   - User action?
   - Scheduled job?
   - Event-based?

3. **Provider preferences?**

Use AskUserQuestionTool:
- question: "For [integration], do you have a preferred provider?"
- options:
  - label: "[Popular option 1] (Recommended)"
    description: "[Why it's good]"
  - label: "[Popular option 2]"
    description: "[Why it's good]"
  - label: "No preference"
    description: "AI can recommend based on requirements"
  - label: "Specific provider in mind"
    description: "I have a particular service I want to use"

**Document per integration:**

| Aspect | Details |
|--------|---------|
| Category | [e.g., Payments] |
| Provider | [e.g., Stripe] |
| Data In | [what we send] |
| Data Out | [what we receive] |
| Trigger | [when it's called] |
| Criticality | [Must work / Nice to have] |
</step>

</steps>

<output>
Integration specifications with providers, data flows, and triggers.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| analyze_for_integrations | Integration needs identified | ✓ / ✗ |
| confirm_needs | User confirmed which apply | ✓ / ✗ |
| specify_requirements | Each integration detailed | ✓ / ✗ |

If any step failed (✗):
- Return to that step and redo
- Do NOT proceed until all steps pass
</verify>

<checkpoint required="true">

**AI Quick Check (internal):**

Check for integration risks (critical integrations without failure handling):
```
integrations = get_all_integrations()
issues = []

for integration in integrations:
    if integration.criticality == "must" or integration.criticality == "critical":
        if not integration.has_fallback_defined:
            issues.append({
                "integration": integration.name,
                "provider": integration.provider,
                "message": f"'{integration.name}' ({integration.provider}) is critical but no fallback defined.",
                "question": "What happens if this service is down?"
            })
```

**If no integrations at all:**

Use AskUserQuestionTool:
- question: "No external integrations needed - app is self-contained. Ready for Constraints?"
- options:
  - label: "Continue to Constraints (Recommended)"
    description: "Standalone app means fewer dependencies, more reliability"
  - label: "Actually, we need integrations"
    description: "I realized we need external services"
  - label: "Save and pause"
    description: "Continue later"

**If integrations exist but no issues:**

Use AskUserQuestionTool:
- question: "Integrations defined with fallbacks. Ready for Constraints?"
- options:
  - label: "Continue to Constraints (Recommended)"
    description: "Integration risks are addressed, proceed with confidence"
  - label: "Review integrations"
    description: "Show me integration details before continuing"
  - label: "Add more integrations"
    description: "I realized we need additional services"
  - label: "Save and pause"
    description: "Continue later"

**If issues found:**

Present issues first:

"Before continuing, I found [N] integration risk(s):

[For each risk:]
⚠ [integration.name] ([integration.provider]) is critical but has no failure plan.
  → What happens if [provider] is down? Users can't [affected functionality].

Critical integrations without fallbacks are single points of failure."

Use AskUserQuestionTool:
- question: "How would you like to handle this?"
- options:
  - label: "Define fallbacks (Recommended)"
    description: "Specify what happens when [integration] fails"
  - label: "Downgrade criticality"
    description: "Mark as 'nice-to-have' instead of critical"
  - label: "Continue anyway"
    description: "Accept the risk, proceed to Constraints"
  - label: "Save and pause"
    description: "Think it over, continue later"

On response:
- "Continue/Recommended": Proceed to <next>
- "Define fallbacks/Add more": Return to step 3
- "Downgrade criticality": Update integration, then proceed to <next>
- "Continue anyway": Proceed to <next> with warning noted
- "Save and pause": Save state, end session
</checkpoint>

<next>
1. Save integrations data:
   ```bash
   python .opensdd/blueprint.state.py set-data 6 integrations "<JSON of integrations>"
   python .opensdd/blueprint.state.py complete-phase 6
   ```

2. Speak to user:
   "Integrations defined. Moving to capture constraints..."

3. Load: `phase-07-constraints.md` (same folder)
</next>
