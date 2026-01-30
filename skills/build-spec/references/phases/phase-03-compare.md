---
phase: 3
name: compare
next: phase-04-summary.md
---

# Phase 3: Compare

<objective>
Run compare-spec to verify overall codebase alignment with spec.
</objective>

<prerequisite>
Phase 2 must be complete with all packages processed.
</prerequisite>

<input>
From Phase 2:
- All packages processed (GREEN or BLOCKED)
- Build history recorded in package files
</input>

<steps>

<step n="1" name="extract_signatures">
Extract code signatures from the entire codebase.

1. Determine source directories from spec.yaml (or default to `src/`)

2. Run signature extraction:
   ```bash
   # This would use a language-appropriate extractor
   # Output: .opensdd/results/code_signatures.yaml
   ```

   For TypeScript/JavaScript:
   - Extract exported functions, classes, types
   - Capture function signatures
   - Note dependencies and imports

   For Python:
   - Extract public functions, classes
   - Capture function signatures
   - Note imports and dependencies

3. Save to `.opensdd/results/code_signatures.yaml`

**Note:** If extraction fails, proceed with what's available. Compare will work with partial data.
</step>

<step n="2" name="run_compare">
Invoke compare-spec-agent to compare codebase against spec.

Task(
  model: "sonnet",
  subagent_type: "opensdd:compare-spec-agent",
  prompt: """
  Compare the entire codebase against spec.yaml.

  ## Files
  - Spec: .opensdd/spec.yaml
  - Code signatures: .opensdd/results/code_signatures.yaml (if available)

  ## Instructions

  1. Read the spec.yaml completely
  2. Read code signatures (or scan codebase if signatures unavailable)
  3. Compare and categorize:

     - **matches**: Components in code that match spec exactly
     - **drifts**: Components that exist but differ from spec
     - **missing**: Spec components not found in code
     - **extras**: Code components not in spec

  ## Output Format

  Return result as YAML:

  ```yaml
  compare_result:
    timestamp: "{ISO datetime}"

    summary:
      matches: {count}
      drifts: {count}
      missing: {count}
      extras: {count}

    details:
      matches:
        - component: UserService
          path: src/services/user_service.ts
        - component: CreateUserInput
          path: src/types/user.ts

      drifts:
        - component: AuthService
          path: src/services/auth.ts
          issue: "Missing logout method specified in spec"

      missing:
        - component: PaymentService
          specified_in: spec.yaml#components.PaymentService

      extras:
        - component: DebugHelper
          path: src/utils/debug.ts
          note: "Not in spec - may be intentional utility"
  ```
  """
)

Parse and store compare result.
</step>

<step n="3" name="display_compare">
Show compare results to user.

```
═══════════════════════════════════════════════════════════════
COMPARE-SPEC RESULTS
═══════════════════════════════════════════════════════════════

Matches: {count} ✓
Drifts:  {count} ⚠
Missing: {count} ✗
Extras:  {count} ?

Drifts (need attention):
  - AuthService: Missing logout method

Missing (not implemented):
  - PaymentService

═══════════════════════════════════════════════════════════════
```
</step>

</steps>

<output>
Compare results:
- matches: Components matching spec
- drifts: Components diverging from spec
- missing: Spec components not in code
- extras: Code components not in spec
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| extract_signatures | Signatures extracted (or noted unavailable) | ✓ / ✗ |
| run_compare | Compare results obtained | ✓ / ✗ |
| display_compare | Results displayed | ✓ / ✗ |

If compare fails → note the failure but continue to summary.
If all passed → proceed to summary phase.
</verify>

<checkpoint required="false">
No checkpoint. Auto-continue to summary phase.
</checkpoint>

<next>
Proceed immediately to Phase 4.

Load: `phase-04-summary.md` (same folder)
</next>
