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
           - `verification` section (safe_to_call, do_not_call, criteria)
           - `package.language` for probe script generation

        7. Derive component_path for probe:
           - Use `component_path` extracted in step 4
           - This tells probe-agent where to import from

        8. Invoke probe-agent:

           Task(
             model: "sonnet",
             subagent_type: "general-purpose",
             prompt: """
             You are the probe-agent. Probe ONE package by calling functions and logging output.

             ## Package Info
             - package_id: {package_id}
             - package_language: {language}
             - component_path: {path from build result}

             ## Verification Section
             {verification_yaml}

             ## Instructions

             1. Parse the verification section:
                - safe_to_call: Functions to call with their inputs
                - do_not_call: Functions to avoid (side effects)
                - criteria: What to look for (informational)

             2. Generate a call-and-log script in {language}

             3. Run the script

             4. Analyze the output and classify:
                - GREEN: All functions executed, output looks correct
                - YELLOW: Functions executed but output unexpected
                - RED: Import failed, functions crashed, or major issues

             5. If not GREEN, generate fix_hints:
                - Specific, actionable feedback
                - Reference actual error messages
                - Suggest what the builder should fix

             ## Output Format

             Return result as YAML:

             ```yaml
             probe_result:
               package_id: {package_id}
               classification: GREEN | YELLOW | RED

               probe_log: |
                 [Raw execution output here]

               fix_hints:  # Only if not GREEN
                 - issue: "Function X returned null instead of User object"
                   suggestion: "Check that createUser actually stores and returns the user"
                 - issue: "Import failed with 'Cannot find module'"
                   suggestion: "Verify export statement in index.ts"
             ```

             ## CRITICAL

             - NO assertions - just log
             - Call ONLY functions in safe_to_call
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
    # STEP C: UPDATE PACKAGE FILE
    # ═══════════════════════════════════════════════

    13. If status still PENDING after loop: status = BLOCKED

    14. Append build history to package YAML:

        Read current package file, append:
        ```yaml
        # ... existing package content ...

        builds:
          final_status: {GREEN|BLOCKED}
          attempts: {attempt count}
          history:
            {build_history}
        ```

    15. Display progress:
        ```
        [{N}/{total}] {package_id}: {status} ({attempt} attempts)
        ```

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
- Build history recorded in each package file
- Status for each package (GREEN or BLOCKED)
- Fix hints for any BLOCKED packages
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| build_loop | All packages processed | ✓ / ✗ |
| build_summary | Summary displayed | ✓ / ✗ |

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
