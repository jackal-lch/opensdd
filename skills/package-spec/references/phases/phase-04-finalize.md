---
phase: 4
name: finalize
next: null
---

# Phase 4: Finalize

<objective>
Write all package files to disk. Generate final report for human review before build.
</objective>

<prerequisite>
Phase 3 must be complete with all packages validated.
</prerequisite>

<input>
All validated packages from Phase 3.
</input>

<steps>

<step n="1" name="create_packages_directory">
Create the packages directory.

```bash
mkdir -p .opensdd/packages
```
</step>

<step n="2" name="write_all_packages">
Write each package to its YAML file.

For each package:
```bash
# Write package file
# File: .opensdd/packages/pkg-{NN}-{name}.yaml
```

**Order of writing:**
1. pkg-00-scaffold.yaml
2. pkg-01-types.yaml
3. pkg-02 through pkg-98 (component packages)
4. pkg-99-integration.yaml
5. manifest.yaml (last, includes all package references)

**Verify each file was written:**
```bash
test -f ".opensdd/packages/pkg-{NN}-{name}.yaml" && echo "OK" || echo "FAILED"
```
</step>

<step n="3" name="write_manifest">
Write the manifest file.

```yaml
# .opensdd/packages/manifest.yaml

manifest:
  generated: "{current ISO8601 timestamp}"
  source:
    spec: ".opensdd/spec.yaml"
    blueprint: "blueprint.md"

  packages:
    # List all packages with metadata
    - id: pkg-00-scaffold
      type: scaffold
      path: pkg-00-scaffold.yaml
      build_order: 0
      depends_on: []
    # ... all other packages

  issues_found:
    # Any warnings from analysis/validation
    - severity: warning
      package: pkg-XX-name
      issue: "description"

  statistics:
    total_packages: {N}
    scaffold: 1
    types: 1
    components: {N}
    integration: 1
```
</step>

<step n="4" name="generate_summary_report">
Generate human-readable summary for review.

```
╔═══════════════════════════════════════════════════════════════════════╗
║                       PACKAGE-SPEC COMPLETE                           ║
╠═══════════════════════════════════════════════════════════════════════╣
║                                                                       ║
║  Source Documents:                                                    ║
║    Spec:      .opensdd/spec.yaml                                      ║
║    Blueprint: blueprint.md                                            ║
║                                                                       ║
║  Packages Created: {N}                                                ║
║    .opensdd/packages/                                                 ║
║    ├── manifest.yaml                                                  ║
║    ├── pkg-00-scaffold.yaml                                           ║
║    ├── pkg-01-types.yaml                                              ║
║    ├── pkg-02-{name}.yaml                                             ║
║    ├── ...                                                            ║
║    └── pkg-99-integration.yaml                                        ║
║                                                                       ║
║  Build Order:                                                         ║
║    1. pkg-00-scaffold     (project infrastructure)                    ║
║    2. pkg-01-types        (shared type definitions)                   ║
║    3. pkg-02-{name}       (depends on: none)                          ║
║    4. pkg-03-{name}       (depends on: pkg-02)                        ║
║    ...                                                                ║
║    N. pkg-99-integration  (entry points and wiring)                   ║
║                                                                       ║
║  External Integrations:                                               ║
║    - pkg-{NN}-sdk: HiAgent SDK                                        ║
║                                                                       ║
{If warnings:}
║  ⚠ Warnings:                                                          ║
║    - {package}: {issue}                                               ║
║                                                                       ║
{End if}
║  Next Steps:                                                          ║
║    1. Review packages (optional): cat .opensdd/packages/pkg-XX.yaml   ║
║    2. Run build: /opensdd:build                                  ║
║                                                                       ║
╚═══════════════════════════════════════════════════════════════════════╝
```
</step>

<step n="5" name="checkpoint_human_review">
**CHECKPOINT: Human review before build**

Use AskUserQuestionTool:
- question: "Packages created. Review before building?"
- options:
  - label: "Start build (Recommended)"
    description: "Proceed to build all packages"
  - label: "Review packages first"
    description: "I'll review the package files before building"
  - label: "Stop here"
    description: "Don't proceed to build yet"

**On "Start build":**
- Proceed to build-spec (caller will handle)

**On "Review packages first":**
- Tell user: "Package files are in .opensdd/packages/. Run /opensdd:build when ready."
- END workflow

**On "Stop here":**
- Tell user: "Packages saved. Run /opensdd:build when ready to build."
- END workflow
</step>

</steps>

<output>
Files written:
- `.opensdd/packages/manifest.yaml`
- `.opensdd/packages/pkg-00-scaffold.yaml`
- `.opensdd/packages/pkg-01-types.yaml`
- `.opensdd/packages/pkg-{NN}-{component}.yaml` (for each component)
- `.opensdd/packages/pkg-99-integration.yaml`
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| create_packages_directory | .opensdd/packages/ exists | |
| write_all_packages | All package files written | |
| write_manifest | manifest.yaml written | |
| generate_summary_report | Report displayed | |
| checkpoint_human_review | User decision received | |

**Final checks:**
```bash
# Verify all files exist
ls -la .opensdd/packages/

# Count packages
ls .opensdd/packages/pkg-*.yaml | wc -l

# Verify manifest
cat .opensdd/packages/manifest.yaml | head -20
```
</verify>

<checkpoint required="true">
Human approval required before proceeding to build.
See step 5 for checkpoint implementation.
</checkpoint>

<next>
Based on user choice:

**"Start build":**
- Speak: "Starting build..."
- Return to caller to invoke build-spec

**"Review packages first" or "Stop here":**
- Speak: "Packages ready at .opensdd/packages/. Run /opensdd:build when ready."
- END workflow
</next>
