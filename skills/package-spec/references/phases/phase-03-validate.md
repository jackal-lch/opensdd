---
phase: 3
name: validate
next: phase-04-finalize.md
---

# Phase 3: Validate Packages

<objective>
Validate each package for completeness using the standardized checklist. Ensure all packages are self-contained and buildable.
</objective>

<prerequisite>
Phase 2 must be complete with all packages created in context.
</prerequisite>

<input>
All package data from Phase 2.
</input>

<steps>

<step n="1" name="validate_each_package">
For each package, run the completeness checklist.

**Checklist Template:**

```yaml
validation:
  package_id: pkg-{NN}-{name}

  metadata:
    - check: "id follows pattern pkg-{NN}-{name}"
      passed: true | false
    - check: "type is valid (scaffold|types|component|integration)"
      passed: true | false
    - check: "build_order matches position"
      passed: true | false
    - check: "depends_on lists all required packages"
      passed: true | false

  scope:
    - check: "description is clear and specific"
      passed: true | false
    - check: "type-specific fields are complete"
      passed: true | false
      details: "missing: [list] | complete"

  context:
    - check: "all referenced types are defined in context.types"
      passed: true | false
      details: "missing: [list] | all present"
    - check: "all dependency interfaces are included"
      passed: true | false
      details: "missing: [list] | all present"
    - check: "relevant flows extracted (if applicable)"
      passed: true | false
    - check: "external details included (if applicable)"
      passed: true | false

  instructions:
    - check: "approach provides clear guidance"
      passed: true | false
    - check: "constraints are explicit"
      passed: true | false
    - check: "if_blocked conditions defined"
      passed: true | false

  verification:
    - check: "safe_to_call functions defined"
      passed: true | false
    - check: "do_not_call lists side-effect functions"
      passed: true | false
    - check: "criteria describe expected probe output"
      passed: true | false

  output:
    - check: "file paths match project structure"
      passed: true | false
    - check: "result format matches standard schema"
      passed: true | false

  overall:
    passed: true | false
    issues: [list of failed checks]
```
</step>

<step n="2" name="check_type_completeness">
Verify types are properly copied (not just referenced).

For each component package:
1. List all types used in scope.provides (input and output)
2. Check each type exists in context.types with FIELDS defined
3. Check fields are appropriate for the type's purpose

**Common issues:**
- Type name present but no fields
- Type referenced but not in context.types
- Fields don't match type's `for` description

```yaml
type_validation:
  package: pkg-03-sdk
  types_used:
    - InvokeAgentInput:
        in_context: true
        has_fields: true
    - InvokeAgentResult:
        in_context: true
        has_fields: true
    - AgentInvocationFailed:
        in_context: true
        has_fields: false  # ISSUE!
```
</step>

<step n="3" name="check_dependency_interfaces">
Verify dependency interfaces are complete.

For each component package:
1. List all components in `depends_on`
2. Check each has an interface definition in context.dependencies
3. Check interface includes all functions this component calls

**Example validation:**
```yaml
dependency_validation:
  package: pkg-11-users
  dependencies:
    auth:
      in_context: true
      interface_defined: true
      functions_needed: [verify_token, hash_password]
      functions_provided: [verify_token, hash_password]
      complete: true
    database:
      in_context: true
      interface_defined: true
      functions_needed: [get_session]
      functions_provided: [get_session]
      complete: true
```
</step>

<step n="4" name="check_cross_package_consistency">
Verify consistency across packages.

**Same types must have same definitions:**
```yaml
# If AgentConfig appears in pkg-10-agent_configs and pkg-13-agent_testing
# Both must have identical field definitions
```

**Check:**
1. List all types that appear in multiple packages
2. Compare their definitions
3. Flag any differences

**Dependency status consistency:**
```yaml
# If pkg-05 depends on pkg-02
# pkg-05.context.dependencies.config.status should be "implemented"
# (because pkg-02 comes before pkg-05)
```
</step>

<step n="5" name="check_external_completeness">
For packages with external integrations, verify details are sufficient.

**Required for external integrations:**
- Package name (e.g., "hiagent-api")
- Initialization code snippet
- Method signatures being used
- Environment variables needed

**Validate:**
```yaml
external_validation:
  package: pkg-07-sdk
  integration: hiagent

  checks:
    - package_name: "hiagent-api + hiagent-components"
      present: true
    - initialization:
        present: true
        includes_endpoint: true
        includes_region: true
    - methods:
        present: true
        count: 3
        all_have_signatures: true
    - environment:
        present: true
        variables: [HIAGENT_TOP_ENDPOINT, HIAGENT_APP_BASE_URL]
```
</step>

<step n="6" name="compile_validation_report">
Compile all validation results into a report.

```
Package Validation Report
═════════════════════════

Total Packages: {N}
Passed:         {N}
Failed:         {N}

{For each package:}
┌─────────────────────────────────────────────────────────────┐
│ pkg-{NN}-{name}                                      {PASS|FAIL} │
├─────────────────────────────────────────────────────────────┤
│ Metadata:     ✓                                              │
│ Scope:        ✓                                              │
│ Context:      ✗ Missing type fields for AgentTimeout         │
│ Instructions: ✓                                              │
│ Verification: ✓                                              │
│ Output:       ✓                                              │
└─────────────────────────────────────────────────────────────┘

{If any failures:}
Issues to Fix:
  1. pkg-03-sdk: context.types.AgentTimeout missing fields
  2. pkg-07-auth: context.dependencies.database missing interface
  ...

{If all pass:}
All packages validated successfully!
```
</step>

<step n="7" name="fix_issues">
If validation found issues, fix them before proceeding.

For each failed check:
1. Identify what's missing
2. Go back to source (spec.yaml or blueprint.md)
3. Extract missing information
4. Update package

**If information is genuinely missing from source:**
- Add to manifest.issues as warning
- Document what's missing in package.instructions.if_blocked
- Proceed (subagent will mark as BLOCKED during build)

Re-run validation after fixes.
</step>

</steps>

<output>
- All packages validated (or fixed)
- Validation report generated
- Any unfixable issues documented in manifest
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| validate_each_package | All packages checked | |
| check_type_completeness | Types have fields | |
| check_dependency_interfaces | Interfaces complete | |
| check_cross_package_consistency | No conflicts | |
| check_external_completeness | External details present | |
| compile_validation_report | Report generated | |
| fix_issues | All fixable issues resolved | |

**Critical checks:**
- [ ] All packages pass metadata validation
- [ ] All types have field definitions
- [ ] All dependencies have interface definitions
- [ ] Cross-package types are consistent
- [ ] External integrations have sufficient details
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue after validation passes.
</checkpoint>

<next>
**If validation fails and cannot be fixed:**
- Report issues to user
- STOP workflow

**If validation passes:**
1. Speak: "Validation complete. Writing {N} packages..."
2. Load: `phase-04-finalize.md`
</next>
