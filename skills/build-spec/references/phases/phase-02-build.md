---
phase: 2
name: build
next: phase-03-compare.md
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
        # STEP A: BUILD (Opus)
        # ═══════════════════════════════════════════════

        1. Read package file: .opensdd/packages/{package_id}.yaml
        2. Read spec file: .opensdd/spec.yaml

        3. Invoke build-agent:

           Task(
             model: "opus",
             subagent_type: "general-purpose",
             prompt: """
             You are the build-agent. Build ONE package into production-ready code.

             ## Package Content
             {package_yaml_content}

             ## Spec Reference
             {spec_yaml_content}

             ## Fix Hints (from previous probe)
             {fix_hints or "None - first attempt"}

             ## Instructions

             Follow the package instructions to implement the code.

             Key rules:
             1. Use exact names from spec
             2. Follow language conventions
             3. Inject dependencies (don't instantiate directly)
             4. Follow flows from context for business logic
             5. Handle edge cases specified in context

             ## CRITICAL: BLOCK > FAKE

             If ANY information is missing:
             - DO NOT use placeholder implementations
             - DO NOT use in-memory storage if spec says database
             - DO NOT mock external services
             - DO NOT use `pass`, `NotImplementedError`, `TODO`
             - DO NOT write empty function bodies

             Instead: Report BLOCKED with specific missing info.

             ## Output Format

             Return result as YAML:

             ```yaml
             build_result:
               package_id: {package_id}
               status: SUCCESS | BLOCKED

               files_created:
                 - path: src/...
                   lines: N

               declarations:
                 storage: "postgresql"
                 external_apis: ["stripe"]
                 events_emitted: ["UserCreated"]

               # If BLOCKED:
               blocked_reason: "..."
               blocked_needs: "..."
             ```
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
        # STEP B: PROBE (Sonnet)
        # ═══════════════════════════════════════════════

        6. Extract from package YAML:
           - `verification` section (prerequisites, scenarios, do_not_call)
           - `package.language` for probe script generation

        7. Derive component_path for probe:
           - Use `component_path` extracted in step 4
           - This tells probe-agent where to import from

        8. Invoke probe-agent:

           Task(
             model: "sonnet",
             subagent_type: "general-purpose",
             prompt: """
             You are the probe-agent. Probe ONE package with REAL integration tests.

             ## Package Info
             - package_id: {package_id}
             - package_file: .opensdd/packages/{package_id}.yaml
             - package_language: {language}
             - component_path: {path from build result}
             - attempt_number: {current attempt number: 1, 2, or 3}

             ## Verification Section
             {verification_yaml}

             ## Instructions

             1. CHECK PREREQUISITES first:
                - Verify all env_vars are set
                - Verify all services are accessible
                - Verify all required files exist
                - If any missing: check `on_missing_prerequisites`:
                  - BLOCK: Return RED immediately
                  - SKIP: Return YELLOW with note

             2. RUN SETUP if specified:
                - Execute each setup step
                - Log results

             3. EXECUTE SCENARIOS:
                For each scenario in verification.scenarios:
                - Log scenario name and description
                - For each step:
                  - If action == "call": call function with REAL inputs
                  - Log FULL output (not just type)
                  - Log response time
                - For each success_indicator:
                  - Evaluate and log [PASS] or [FAIL]

             4. CLASSIFY based on success indicators:
                - GREEN: ALL indicators show [PASS]
                - YELLOW: SOME indicators passed
                - RED: Prerequisites missing, connection failed, majority failed

             5. If not GREEN, generate fix_hints:
                - Reference specific failed indicators
                - Include actual error messages
                - Suggest concrete fixes

             ## Output Format

             ```yaml
             probe_result:
               package_id: {package_id}
               classification: GREEN | YELLOW | RED

               indicators:
                 passed: N
                 failed: N
                 total: N

               scenarios:
                 - name: "scenario_name"
                   status: PASS | FAIL
                   indicators:
                     - "[PASS] indicator description"
                     - "[FAIL] indicator description"

               probe_log: |
                 [Full execution log with timestamps]

               fix_hints:  # Only if not GREEN
                 - issue: "What specifically failed"
                   suggestion: "How to fix it"
             ```

             ## CRITICAL

             - Use REAL credentials from environment
             - Make REAL API calls to REAL services
             - Log FULL responses, not just types
             - Classification based on concrete [PASS]/[FAIL] counts
             - NEVER call functions in do_not_call
             """
           )

        9. Parse probe result

        10. Record attempt in build_history:
           ```yaml
           - attempt: {attempt}
             build_status: {SUCCESS|BLOCKED}
             probe_classification: {GREEN|YELLOW|RED}
             probe_log: |
               {log}
             fix_hints: {hints or null}
           ```

        11. If classification == GREEN:
            - status = GREEN
            - BREAK loop

        12. Else:
            - attempt++
            - fix_hints = probe's fix_hints (for next build)

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

    15. Continue to next package (regardless of status)

END FOR
```
</step>

<step n="2" name="build_summary">
After all packages processed, show interim summary.

```
═══════════════════════════════════════════════════════════════
BUILD PHASE COMPLETE
═══════════════════════════════════════════════════════════════

Packages built:
  ✓ pkg-01-types (GREEN, 1 attempt)
  ✓ pkg-02-user-service (GREEN, 2 attempts)
  ✗ pkg-03-auth (BLOCKED, 3 attempts)
  ✓ pkg-04-api (GREEN, 1 attempt)

GREEN: {count}
BLOCKED: {count}

Proceeding to compare-spec verification...
═══════════════════════════════════════════════════════════════
```
</step>

</steps>

<output>
All packages processed with:
- Status for each package (GREEN, YELLOW, or BLOCKED)
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
No checkpoint. Auto-continue to compare phase.
</checkpoint>

<next>
Proceed immediately to Phase 3.

Load: `phase-03-compare.md` (same folder)
</next>
