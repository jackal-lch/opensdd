---
phase: 2
name: build
next: null
---

# Phase 2: Build

<objective>
Execute build→probe→retry loop for each package sequentially.
</objective>

<prerequisite>
Phase 1 must be complete with validated manifest.
</prerequisite>

<input>
From Phase 1:
- `build_order`: List of package IDs
- `total_packages`: Count

From skill arguments:
- `review_mode`: Boolean (default: false) - pause after each package for human review
</input>

<steps>

<step n="1" name="build_loop">
For EACH package in build_order, execute the build→probe→retry loop.

```
FOR package_id IN build_order:

    # Initialize tracking
    attempt = 1
    fix_hints = null
    status = PENDING
    build_history = []

    WHILE attempt <= 3 AND status != GREEN:

        # ═══════════════════════════════════════════════
        # STEP A: BUILD
        # ═══════════════════════════════════════════════

        1. Read package file: .opensdd/packages/{package_id}.yaml
        2. Read spec file: .opensdd/spec.yaml

        3. Invoke build-agent:

           Task(
             subagent_type: "opensdd:build-agent",
             prompt: """
             ## Package Content
             {package_yaml_content}

             ## Spec Reference
             {spec_yaml_content}

             ## Fix Hints (from previous probe)
             {fix_hints or "None - first attempt"}
             """
           )

        4. Parse build result:
           - Extract `build_result.status` (SUCCESS or BLOCKED)
           - Extract `build_result.files_created` array
           - Store `component_path` = first file path from `files_created`
             Example: if files_created[0].path = "src/services/user_service.ts"
                      then component_path = "src/services/user_service.ts"

        5. If status == BLOCKED:
           - Record: blocked_reason, blocked_needs
           - status = BLOCKED
           - BREAK loop (move to next package)

        # ═══════════════════════════════════════════════
        # STEP B: PROBE
        # ═══════════════════════════════════════════════

        6. Extract from package YAML:
           - `verification` section (prerequisites, scenarios, do_not_call)
           - `package.language` for probe script generation

        7. Derive component_path for probe:
           - Use `component_path` extracted in step 4
           - This tells probe-agent where to import from

        8. Invoke probe-agent:

           Task(
             subagent_type: "opensdd:probe-agent",
             prompt: """
             ## Package Info
             - package_id: {package_id}
             - package_file: .opensdd/packages/{package_id}.yaml
             - package_language: {language}
             - component_path: {path from build result}
             - attempt_number: {current attempt number: 1, 2, or 3}

             ## Verification Section
             {verification_yaml}
             """
           )

        9. Parse probe result

        10. Handle result based on classification:

            If BLOCKED:
              - status = BLOCKED
              - BREAK loop (no retry will help)

            If GREEN:
              - status = GREEN
              - BREAK loop (done)

            If FAILED:
              - If attempt < 3:
                - attempt++
                - fix_hints = probe's fix_hints
                - CONTINUE loop (retry)
              - Else:
                - status = FAILED
                - BREAK loop (max retries reached)

    # ═══════════════════════════════════════════════
    # STEP C: FINALIZE PACKAGE STATUS
    # ═══════════════════════════════════════════════
    # Note: Probe results are recorded by probe-agent directly
    # to the package file after each probe attempt.

    13. If status still PENDING after loop: status = BLOCKED

    14. Display progress:
        ```
        [{N}/{total}] {package_id}: {status} ({attempt} attempts)
        ```

        Show files created (if any):
        ```
        Files created:
          - {path} ({lines} lines)
          ...
        ```

    15. REVIEW CHECKPOINT (if review_mode enabled):

        If `review_mode` parameter is true:

        Use AskUserQuestion tool:
        ```
        question: "Package {package_id} completed: {status}. What would you like to do?"
        header: "Review"
        options:
          - label: "Continue"
            description: "Proceed to next package"
          - label: "Retry"
            description: "Re-run build+probe for this package (resets attempt count)"
          - label: "View files"
            description: "Show contents of generated files, then ask again"
          - label: "Abort"
            description: "Stop build process entirely"
        ```

        Handle response:
        - "Continue" → proceed to next package
        - "Retry" → reset attempt=1, fix_hints=null, re-enter WHILE loop for same package
        - "View files" → Read and display each file in files_created, then ask again
        - "Abort" → exit build loop, skip to summary phase with partial results

        If `review_mode` is false (default): auto-continue to next package

    16. Continue to next package (regardless of status)

END FOR
```
</step>

<step n="2" name="build_summary">
After all packages processed, show interim summary.

```
═══════════════════════════════════════════════════════════════
BUILD PHASE COMPLETE
═══════════════════════════════════════════════════════════════

Packages:
  ✓ pkg-01-types (GREEN, 1 attempt)
  ✓ pkg-02-user-service (GREEN, 2 attempts)
  ✗ pkg-03-auth (FAILED, 3 attempts) - needs human review
  ⊘ pkg-04-sdk (BLOCKED) - missing HIAGENT_API_KEY

Summary:
  GREEN:   {count} (passed with real tests)
  FAILED:  {count} (tried but failed after 3 attempts)
  BLOCKED: {count} (can't test - missing prerequisites)

Proceeding to compare-spec verification...
═══════════════════════════════════════════════════════════════
```
</step>

</steps>

<output>
All packages processed with:
- Status for each package (GREEN, FAILED, or BLOCKED)
- Probe results recorded by probe-agent to each package file (`probe_attempts:` section)
- Fix hints for any failed packages

**Artifacts modified by probe-agent:**
- `.opensdd/packages/pkg-*.yaml` - Each file has `probe_attempts:` section appended
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| build_loop | All packages processed | ✓ / ✗ |
| probe_results_recorded | Each package file has `probe_attempts:` section (recorded by probe-agent) | ✓ / ✗ |
| build_summary | Summary displayed | ✓ / ✗ |

**CHECK: Verify probe results were recorded by probe-agent**

For each package that was probed, confirm probe_attempts exists:
```bash
grep -l "probe_attempts:" .opensdd/packages/pkg-*.yaml
```

Note: Probe results are recorded by probe-agent, not by build-spec.
If results are missing, the probe-agent invocation may have failed.

If any step failed → identify and resolve.
If all passed → proceed to compare phase.
</verify>

<checkpoint required="false">
Per-package checkpoint: Conditional on `review_mode` flag.
- If `review_mode=true`: Pause after each package for human review (step 15)
- If `review_mode=false` (default): Auto-continue to next package
</checkpoint>

<next>
Workflow complete. No next phase.

Speak to user:
"Build-spec complete.

Results:
  GREEN:   {count} packages built and verified
  BLOCKED: {count} packages need prerequisites

Package details: .opensdd/packages/pkg-*.yaml

To check overall code-spec alignment, run: /opensdd:compare"
</next>
