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
             You are the probe-agent. Probe ONE package with REAL tests.

             ## Package Info
             - package_id: {package_id}
             - package_file: .opensdd/packages/{package_id}.yaml
             - package_language: {language}
             - component_path: {path from build result}
             - attempt_number: {current attempt number: 1, 2, or 3}

             ## Verification Section
             {verification_yaml}

             ## THREE STATUSES ONLY

             | Status | Meaning |
             |--------|---------|
             | GREEN | All REAL tests passed |
             | FAILED | Tried with real stuff, didn't work |
             | BLOCKED | Can't test - missing prerequisites |

             ## Instructions

             1. ANALYZE PACKAGE TYPE:
                - scaffold/types: Usually no external prerequisites
                - component: Check verification.prerequisites (may be empty)
                - integration: May need dependent services

             2. CHECK DECLARED PREREQUISITES (if any):
                - Read verification.prerequisites from package
                - If empty/missing → No blocking prerequisites, proceed to tests
                - If declared → Check each one exists
                - If ANY declared prerequisite missing → BLOCKED immediately

             3. RUN TESTS (appropriate for package type):
                - scaffold: Verify files exist, configs valid
                - types: Verify types compile/parse
                - component: Execute verification.scenarios
                - integration: Verify app initializes, components wire
                - Use target language for probe script
                - Log full output

             4. CLASSIFY:
                - All pass → GREEN
                - Any fail → FAILED (with fix_hints)

             ## RATIONAL PREREQUISITE CHECKING

             NOT every package needs env vars or services:
             - pkg-00-scaffold: Just check files exist
             - pkg-01-types: Just check types compile
             - pkg-XX-component: Only if verification.prerequisites declares them
             - pkg-99-integration: Depends on what components need

             ## ABSOLUTE RULES (ALWAYS APPLY)

             ### Rule 1: NEVER Create Fakes to Pass
             ❌ NEVER set fake env vars: `os.environ["X"] = "fake"`
             ❌ NEVER skip checks: `if not key: skip()`
             ❌ NEVER mock services: `mock.patch(...)`
             ❌ NEVER use placeholders: `"test-value"`
             → If prerequisites missing → BLOCKED

             ### Rule 2: DETECT Fakes in Built Code
             The build-agent might have created fake implementations.
             YOU MUST detect and FAIL them:

             ❌ Hardcoded returns: `return {"status": "success"}`
             ❌ Empty bodies: `pass`, `{}`, `return nil, nil`, `()`
             ❌ TODO placeholders: `NotImplementedError`, `todo!()`, `panic("not impl")`
             ❌ Wrong storage: in-memory when spec says database
             ❌ Type escapes: `as any`, `// @ts-ignore`, `interface{}` abuse

             HOW TO DETECT - MATCH TO COMPONENT TYPE:

             | Type | Verification |
             |------|--------------|
             | CRUD/Data | Create→Retrieve→Verify exists |
             | Calculation | Known inputs → correct output |
             | External API | Real call succeeds OR BLOCKED |
             | Validation | Accept valid, reject invalid |

             Key: Verify what the function ACTUALLY DOES.
             A sophisticated fake can vary output but can't fake side effects.

             ### Rule 3: GREEN = Actually Works
             GREEN does NOT mean "returns something"
             GREEN means "the code actually does what it's supposed to do"

             ## Output Format

             ```yaml
             probe_result:
               package_id: {package_id}
               classification: GREEN | FAILED | BLOCKED

               # If BLOCKED:
               blocked_reason: "Missing {declared prerequisite}"
               blocked_needs: "Provide it or remove from prerequisites"

               # If GREEN or FAILED:
               indicators:
                 passed: N
                 failed: N
               probe_log: |
                 [Full execution log]
               fix_hints: [...]  # If FAILED
             ```

             ## MANDATORY: Record results to package file
             After probing, use Edit tool to append results to {package_file}
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
No checkpoint. Auto-continue to compare phase.
</checkpoint>

<next>
Proceed immediately to Phase 3.

Load: `phase-03-compare.md` (same folder)
</next>
