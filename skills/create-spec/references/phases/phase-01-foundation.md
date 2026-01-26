---
phase: 1
name: foundation
next: phase-02-types.md
---

# Phase 1: Foundation

<objective>
Establish technical foundation: tech stack, deployment model, language-specific conventions, project structure, components, and architecture patterns from the product blueprint.
</objective>

<prerequisite>
Product blueprint should be available (either provided by user or in conversation context).

If no blueprint provided:
- Ask user: "Do you have a product blueprint? If so, please share it or point me to the file."
- If user has no blueprint: "I recommend running /opensdd:create-blueprint first to create one. Would you like to proceed anyway with what you know about the product?"
</prerequisite>

<input>
Product blueprint containing: vision, users, features, user flows, data model, integrations, constraints.
</input>

<steps>

<step n="1" name="analyze_blueprint">
Read and analyze the product blueprint to extract technical requirements.

**Extract from blueprint:**
1. Features list → will become components or component functions
2. Data model entities → will become domain types
3. User flows → will inform component interactions
4. Integrations → will become integration section
5. Constraints → will inform boundaries

**Step-back analysis (think big picture first):**
Before diving into details, ask:
- What is the core technical challenge this product solves?
- What architectural pattern fits best (monolith for simplicity, microservices for scale)?
- What are the non-negotiable technical constraints?

Document your analysis for the user.
</step>

<step n="2" name="determine_tech_stack">
Propose technology choices based on blueprint requirements.

**Consider:**
- Language: What fits the team's expertise, domain requirements, and constraints?
- Framework: What fits the product type and scale?
- Database: What fits the data model and access patterns?
- Deployment: What fits the operational requirements?

**First-principles check:**
For each choice, ask: "Why this over alternatives?"
- Don't choose because it's popular
- Choose because it fits the specific constraints

Use AskUserQuestionTool to confirm or let user override:
- question: "Here's my proposed tech stack based on the blueprint. Does this align with your preferences?"
- Present your recommendations with rationale
- Options should include your recommendation + alternatives
</step>

<step n="3" name="define_conventions_and_structure">
Define conventions and structure based on chosen language and blueprint context.

**Load language reference:**
```
references/languages/{language}.md
```

The language file provides:
- **Conventions (fixed)** - Apply these directly
- **Structure options** - Multiple options with trade-offs
- **Pattern options** - Language-specific idioms
- **Preferred/Avoid** - Dependency recommendations

**AI decides structure based on blueprint context:**

Consider from blueprint:
- Component count and complexity
- Team structure (solo, small team, multiple teams)
- Scale requirements
- Deployment model
- Integration patterns

Match blueprint context to the language file's **Structure Options** table (see "When to Consider" column) and recommend with rationale.

**If unclear, ask user:**
Use AskUserQuestionTool to clarify:
- "Do you need independent deployments for different parts?"
- "Is this a library others will import, or an application?"

Present conventions and recommended structure to user for confirmation.
</step>

<step n="4" name="identify_components">
Identify all components needed to implement the blueprint features.

**Component identification process:**
1. List all features from blueprint
2. Group related features → these become components
3. For each component, define:
   - Name (following language conventions from step 3)
   - `for`: one-line responsibility
   - `layer`: domain | application | infrastructure
4. Map each component to structure location (from step 3)

**Layering guidance:**
- **domain**: Core business logic, entities, business rules
- **application**: Orchestration, use cases, workflows
- **infrastructure**: External concerns (database, cache, API gateway, integrations)

**Verify completeness:**
- Can every blueprint feature be implemented by these components?
- Is each component focused (single responsibility)?
- Are there missing infrastructure components (logging, auth, storage)?
- Does every component have a location in the structure?

Present component list to user for validation.
</step>

<step n="5" name="define_architecture">
Recommend architecture patterns based on components and their interactions.

**AI recommends, human decides.** Present patterns with rationale; user confirms or overrides.

**Global patterns (system-wide):**

1. **Dependency Injection**
   - When: Multiple components with dependencies
   - Options: Constructor injection, Interface-based, Container/Registry
   - Rationale: Testability, loose coupling

2. **Error Handling Strategy**
   - When: Cross-component error propagation needed
   - Options: Result types, Exception hierarchy, Error codes
   - Rationale: Must match language idioms

3. **Async Pattern**
   - When: I/O-bound or concurrent operations
   - Options: Async/await, Channels, Actor model
   - Rationale: Based on concurrency needs

**Component-specific patterns:**

Analyze each component to determine if it needs a specific pattern:

| Pattern | When to Use |
|---------|-------------|
| Strategy | Multiple interchangeable implementations |
| Factory | Complex object creation with variants |
| State Machine | Explicit state transitions |
| Repository | Data access abstraction |
| Observer/Pub-Sub | Event-driven communication |
| Decorator | Cross-cutting concerns (logging, caching, retry) |

**Process:**
1. Analyze component interactions from step 4
2. Identify components that need patterns (not all do)
3. Recommend patterns with rationale
4. Present to user for confirmation

**Occam's razor check:**
For each pattern recommended, ask: "Is this pattern necessary, or would simpler code suffice?"
- If the pattern solves a real problem (multiple implementations, complex state) → keep it
- If it's "just in case" or "best practice" without concrete need → remove it

**Analogical reasoning check:**
For key architectural decisions, ask: "What similar systems use this approach successfully?"
- Grounds decisions in real-world precedent
- If no analogous system exists, the approach may be novel risk

Use AskUserQuestionTool:
- question: "Here are my recommended architecture patterns. Do these fit your needs?"
- Present global patterns + component patterns with rationale
- Options should allow user to confirm, adjust, or add patterns
</step>

</steps>

<output>
Foundation decisions written to `.opensdd/spec.yaml`:
- tech_stack (language, framework, database)
- deployment (model, target)
- conventions (language-specific naming and patterns)
- structure (project layout with component mappings)
- components (names + responsibilities + layers)
- architecture (global patterns + component-specific patterns)

Create `.opensdd/` directory if it doesn't exist.
Write initial spec.yaml using `references/format/spec-schema.md` structure.
</output>

<verify>
AI self-verification before checkpoint:

| Step | Expected Output | Status |
|------|-----------------|--------|
| analyze_blueprint | Blueprint analyzed, key requirements extracted | ✓ / ✗ |
| determine_tech_stack | Tech stack proposed and confirmed | ✓ / ✗ |
| define_conventions_and_structure | Conventions applied, structure recommended with rationale | ✓ / ✗ |
| identify_components | All components identified with responsibilities | ✓ / ✗ |
| define_architecture | Architecture patterns recommended and confirmed | ✓ / ✗ |

If any step incomplete → return and complete it.
If all done → proceed to checkpoint.
</verify>

<checkpoint required="true">

**AI Quick Check:**

Validate foundation completeness:
- Tech stack has language, framework, database defined?
- Deployment model chosen?
- Conventions match chosen language idioms?
- Structure follows language conventions?
- Every component has a location in structure?
- Every blueprint feature maps to a component?
- No component has overlapping responsibilities?
- Architecture patterns defined (global + component-specific)?
- Patterns match language idioms (e.g., Result types for Rust, exceptions for Python)?

**If issues found:**

"Before continuing, I noticed: ⚠ [specific issue]"

Use AskUserQuestionTool:
- question: "How would you like to handle this?"
- options:
  - label: "Fix it (Recommended)"
    description: "Address the issue before continuing"
  - label: "Continue anyway"
    description: "Proceed to Types phase as-is"

**If no issues:**

Use AskUserQuestionTool:
- question: "Foundation looks complete. Ready to define types?"
- options:
  - label: "Continue to Types (Recommended)"
    description: "Proceed to define domain, input, output, error, and event types"
  - label: "Adjust architecture"
    description: "Modify architecture patterns"
  - label: "Adjust structure"
    description: "Modify the project structure"
  - label: "Adjust components"
    description: "Modify the component list"
</checkpoint>

<next>
After user approval:

1. Create `.opensdd/` directory if it doesn't exist

2. Write `.opensdd/spec.yaml` with foundation sections:
   - tech_stack, deployment, conventions, structure, components, architecture
   - See `references/format/spec-schema.md` for structure

3. Speak to user:
   "Foundation written to .opensdd/spec.yaml. Proceeding to define types..."

4. Load: `phase-02-types.md`
</next>
