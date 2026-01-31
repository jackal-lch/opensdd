---
phase: 1
name: initialize
next: phase-02-build.md
---

# Phase 1: Initialize

<objective>
Verify all prerequisites exist and load the build plan.
</objective>

<prerequisite>
None. This is the first phase.
</prerequisite>

<input>
No input from previous phase.

From skill arguments:
- `review_mode`: Boolean (default: false) - pause after each package for human review
</input>

<steps>

<step n="1" name="verify_prerequisites">
Check that all required files exist.

**Required files:**
1. `.opensdd/spec.yaml` - The specification
2. `.opensdd/packages/manifest.yaml` - Package manifest with build order
3. At least one package file in `.opensdd/packages/`

**Verification:**
```bash
# Check spec exists
test -f .opensdd/spec.yaml && echo "spec: OK" || echo "spec: MISSING"

# Check manifest exists
test -f .opensdd/packages/manifest.yaml && echo "manifest: OK" || echo "manifest: MISSING"

# Count package files
PKGS=$(ls .opensdd/packages/pkg-*.yaml 2>/dev/null | wc -l)
echo "packages: $PKGS"
```

If any check fails:
- Show clear error: "Missing: [file]. Run /opensdd:package-spec first."
- STOP workflow.

If all checks pass:
- Proceed to next step.
</step>

<step n="2" name="load_manifest">
Read and validate the package manifest.

1. Read `.opensdd/packages/manifest.yaml`
2. Extract:
   - `build_order`: List of package IDs in order
   - `total_packages`: Count
3. Validate each package file exists

**Expected manifest format:**
```yaml
manifest:
  spec_version: "1.0"
  generated_at: "..."

build_order:
  - pkg-01-types
  - pkg-02-user-service
  - pkg-03-auth
  ...
```

If any referenced package file is missing:
- Show: "Package file missing: pkg-{NN}-{name}.yaml"
- STOP workflow.
</step>

<step n="3" name="display_plan">
Show the build plan to user.

**Display format:**
```
═══════════════════════════════════════════════════════════════
BUILD PLAN
═══════════════════════════════════════════════════════════════

Packages to build: {total_packages}
Build order:
  1. pkg-01-types
  2. pkg-02-user-service
  3. pkg-03-auth
  ...

Each package will:
  1. Build (Opus) - implement code
  2. Probe (Sonnet) - verify execution
  3. Retry (up to 3 attempts if needed)

Review mode: {ON if review_mode else OFF}
  {if ON: "You will be prompted after each package to review/continue/abort"}
  {if OFF: "Auto-continue to next package (use --review to enable review mode)"}

═══════════════════════════════════════════════════════════════
Starting build...
═══════════════════════════════════════════════════════════════
```

**No confirmation needed** - proceed automatically.
</step>

</steps>

<output>
Loaded manifest with package list and build order, ready for build phase.

Pass to Phase 2:
- `build_order`: List of package IDs
- `total_packages`: Count
- `review_mode`: Boolean flag for per-package review
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| verify_prerequisites | All files exist | ✓ / ✗ |
| load_manifest | Manifest loaded, all packages validated | ✓ / ✗ |
| display_plan | Plan displayed to user | ✓ / ✗ |

If any step failed → STOP and show error.
If all passed → proceed to next phase.
</verify>

<checkpoint required="false">
No checkpoint. Auto-continue to build phase.
</checkpoint>

<next>
Proceed immediately to Phase 2.

Load: `phase-02-build.md` (same folder)
</next>
