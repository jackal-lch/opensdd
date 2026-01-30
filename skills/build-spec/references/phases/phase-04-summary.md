---
phase: 4
name: summary
next: null
---

# Phase 4: Summary

<objective>
Generate unified build-summary.yaml and display final results to user.
</objective>

<prerequisite>
Phase 3 must be complete with compare results.
</prerequisite>

<input>
From previous phases:
- Package build results (GREEN/BLOCKED status per package)
- Compare results (matches, drifts, missing, extras)
</input>

<steps>

<step n="1" name="generate_summary_file">
Create unified build summary at `.opensdd/results/build-summary.yaml`.

```yaml
build_summary:
  timestamp: "{ISO datetime}"
  spec_file: ".opensdd/spec.yaml"
  manifest_file: ".opensdd/packages/manifest.yaml"

  # ═══════════════════════════════════════════════
  # BUILD RESULTS
  # ═══════════════════════════════════════════════

  packages:
    total: {count}
    green: {count}
    blocked: {count}

    details:
      - id: pkg-01-types
        status: GREEN
        attempts: 1
        files_created:
          - src/types/user.ts
          - src/types/auth.ts

      - id: pkg-02-user-service
        status: GREEN
        attempts: 2
        files_created:
          - src/services/user_service.ts

      - id: pkg-03-auth
        status: BLOCKED
        attempts: 3
        blocked_reason: "Missing OAuth configuration"
        last_fix_hints:
          - issue: "OAuth provider config not found"
            suggestion: "Define OAuth settings in environment"

  # ═══════════════════════════════════════════════
  # COMPARE RESULTS
  # ═══════════════════════════════════════════════

  compare:
    matches: {count}
    drifts: {count}
    missing: {count}
    extras: {count}

    drift_details:
      - component: AuthService
        issue: "Missing logout method"

    missing_details:
      - component: PaymentService

  # ═══════════════════════════════════════════════
  # ACTION ITEMS
  # ═══════════════════════════════════════════════

  action_items:
    - priority: HIGH
      item: "Fix pkg-03-auth: Missing OAuth configuration"

    - priority: MEDIUM
      item: "Fix drift in AuthService: Add logout method"

    - priority: LOW
      item: "Implement PaymentService (missing from build)"
```

Write to `.opensdd/results/build-summary.yaml`.
</step>

<step n="2" name="display_final_summary">
Display comprehensive summary to user.

```
╔═══════════════════════════════════════════════════════════════╗
║                      BUILD COMPLETE                           ║
╚═══════════════════════════════════════════════════════════════╝

PACKAGES
────────────────────────────────────────────────────────────────
Total: {total}    GREEN: {green} ✓    BLOCKED: {blocked} ✗

  ✓ pkg-01-types (1 attempt)
  ✓ pkg-02-user-service (2 attempts)
  ✗ pkg-03-auth - BLOCKED: Missing OAuth configuration

COMPARE-SPEC
────────────────────────────────────────────────────────────────
Matches: {count} ✓    Drifts: {count} ⚠    Missing: {count} ✗

  ⚠ AuthService: Missing logout method
  ✗ PaymentService: Not implemented

ACTION ITEMS
────────────────────────────────────────────────────────────────
[HIGH]   Fix pkg-03-auth: Missing OAuth configuration
[MEDIUM] Fix drift in AuthService: Add logout method
[LOW]    Implement PaymentService

────────────────────────────────────────────────────────────────
Full report: .opensdd/results/build-summary.yaml
Package logs: .opensdd/packages/pkg-*.yaml (builds: section)
════════════════════════════════════════════════════════════════
```
</step>

</steps>

<output>
- `.opensdd/results/build-summary.yaml` generated
- Final summary displayed to user
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| generate_summary_file | build-summary.yaml created | ✓ / ✗ |
| display_final_summary | Summary displayed | ✓ / ✗ |

All steps must pass.
</verify>

<checkpoint required="true">
Final checkpoint - build complete.

Use AskUserQuestionTool:
- question: "Build complete. What would you like to do next?"
- options:
  - label: "Done"
    description: "Build finished - review results and continue manually"
  - label: "Re-run blocked packages"
    description: "Retry only the packages that were BLOCKED"
  - label: "View full report"
    description: "Display complete build-summary.yaml"

On user response:
- "Done": Complete workflow
- "Re-run blocked": Return to Phase 2 with only blocked packages
- "View report": Display full YAML then return to this checkpoint
</checkpoint>

<next>
Workflow complete. No next phase.

Speak to user:
"Build-spec complete. Results saved to .opensdd/results/build-summary.yaml"
</next>
