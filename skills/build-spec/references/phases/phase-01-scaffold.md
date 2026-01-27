---
phase: 1
name: scaffold
next: phase-02-select.md
---

# Phase 1: Scaffold

<objective>
Generate all project infrastructure from spec.yaml that is NOT component code.
</objective>

<prerequisite>
Verify Phase 0 complete:

```bash
python .opensdd/build-spec.state.py check-phase 0
```

If exit code != 0:
- Show: "Phase 0 must be complete first."
- STOP workflow.
</prerequisite>

<input>
From spec.yaml:
- `tech_stack`: Language, framework, dependencies
- `structure`: Directory layout, entrypoints, tests path
- `types`: Shared type definitions
- `deployment`: Deployment target and configuration
</input>

<steps>

<step n="1" name="parse_spec">
Load spec.yaml and identify which sections have deliverables:

```bash
cat .opensdd/spec.yaml
```

Parse and identify:
- `tech_stack` → project config file
- `structure.layers` → directories to create
- `structure.entrypoints` → entry point files
- `structure.tests` → test directory
- `types` → type definition files
- `deployment` → deployment files

For each section, note if it exists and what it contains.
</step>

<step n="2" name="scaffold_project">
**Generate project configuration based on `tech_stack.language`:**

| Language | Config File | Key Contents |
|----------|-------------|--------------|
| python | pyproject.toml | dependencies, python version, pytest config |
| node/typescript | package.json | dependencies, scripts, type config |
| rust | Cargo.toml | dependencies, features |
| go | go.mod | module name, dependencies |

**Before creating:** Check if file exists
```bash
test -f "[CONFIG_FILE]" && echo "EXISTS" || echo "NOT_FOUND"
```

If EXISTS: Skip, log "SKIPPED: [file] already exists"
If NOT_FOUND: Create file with dependencies from `tech_stack.dependencies`

**Content should include:**
- All dependencies from `tech_stack.dependencies`
- Correct language version from `tech_stack.language`
- Test framework configuration from `tech_stack.testing`
</step>

<step n="3" name="scaffold_structure">
**Create directory structure from `structure.layers`:**

For each layer in `structure.layers`:
```bash
mkdir -p [ROOT_PATH]/[LAYER_PATH]
```

Also create `__init__.py` (Python) or equivalent for the language.

**Example for Python:**
```bash
# From structure.layers: { domain: "domain/", application: "application/" }
mkdir -p src/project/domain
mkdir -p src/project/application
touch src/project/__init__.py
touch src/project/domain/__init__.py
touch src/project/application/__init__.py
```

Create .specs directory for extracted signatures:
```bash
mkdir -p .specs
```
</step>

<step n="4" name="scaffold_entrypoints">
**Create entry point stubs from `structure.entrypoints`:**

For each entrypoint in `structure.entrypoints`:

1. Check if file exists:
   ```bash
   test -f "[ENTRYPOINT_PATH]" && echo "EXISTS" || echo "NOT_FOUND"
   ```

2. If NOT_FOUND: Create a valid stub file that:
   - Imports will work (empty file or minimal valid syntax)
   - Can be run without errors (even if it does nothing)
   - Has TODO comments indicating what will be wired

**Example stubs by language:**

Python (main.py):
```python
"""API entrypoint - will be wired during component implementation."""
# TODO: Import and wire components after implementation

def main():
    """Application entry point."""
    pass

if __name__ == "__main__":
    main()
```

TypeScript (main.ts):
```typescript
// API entrypoint - will be wired during component implementation
// TODO: Import and wire components after implementation

async function main(): Promise<void> {
  // Entry point
}

main();
```
</step>

<step n="5" name="scaffold_types">
**Create type SKELETONS from `types:` section:**

**Philosophy:** Spec defines contracts and boundaries. Types in spec are declarations of
WHAT types exist and their PURPOSE. Implementation details (fields, methods) are determined
by AI during component implementation based on how components use these types.

Read all types from spec.yaml `types:` section.

Determine type file location:
1. If `structure.types` defined → use that path
2. Else if `structure.layers.domain` defined → use `{domain}/types.{ext}`
3. Else → create `types/` directory

**For each type in spec `types:`:**

1. **If type has `enum:` field** → Create complete enum (enums are fully specified)
2. **If type has `for:` description** → Create skeleton with docstring
3. **Otherwise** → Create empty skeleton with TODO

**Example scaffold generation:**

From spec:
```yaml
types:
  UserStatus:
    enum: [active, inactive, pending]
  User:
    for: "Core user entity representing authenticated users"
  OrderItem:
    for: "Line item in an order"
```

Python output:
```python
from enum import Enum
from dataclasses import dataclass
from typing import Any

# Enums are fully generated (they're contracts)
class UserStatus(Enum):
    ACTIVE = "active"
    INACTIVE = "inactive"
    PENDING = "pending"

# Domain types are skeletons - fields added during component implementation
@dataclass
class User:
    """Core user entity representing authenticated users.

    Fields will be defined when components that use User are implemented.
    See: components that have User in provides/params/returns.
    """
    pass  # Fields added by component implementation

@dataclass
class OrderItem:
    """Line item in an order.

    Fields will be defined when components that use OrderItem are implemented.
    """
    pass  # Fields added by component implementation
```

TypeScript output:
```typescript
// Enums are fully generated
export enum UserStatus {
  ACTIVE = "active",
  INACTIVE = "inactive",
  PENDING = "pending",
}

// Domain types are skeletons - fields added during component implementation
/**
 * Core user entity representing authenticated users.
 * Fields will be defined when components that use User are implemented.
 */
export interface User {
  // Fields added by component implementation
}

/**
 * Line item in an order.
 * Fields will be defined when components that use OrderItem are implemented.
 */
export interface OrderItem {
  // Fields added by component implementation
}
```

**Type status determination:** During component implementation (Phase 3), AI determines
which types are skeletons vs complete by reading the type files:
- Skeleton: Has `pass` (Python) or empty body `{}` (TypeScript) or similar
- Complete: Has actual field definitions

This is intentionally not tracked in state - AI reads and decides at implementation time.
</step>

<step n="6" name="scaffold_deployment">
**Create deployment files based on `deployment.target`:**

If `deployment` section not in spec → skip this step.

| Target | Files to Generate |
|--------|-------------------|
| docker | Dockerfile, docker-compose.yaml, .env.example |
| kubernetes | Dockerfile, k8s/ directory with manifests |
| serverless | serverless.yml or equivalent |
| systemd | systemd service unit file |
| (other) | README with deployment notes |

**Before creating each file:** Check if exists, skip if so.

**Dockerfile should:**
- Use appropriate base image for language
- Copy dependency file and install
- Copy source code
- Set entrypoint from `structure.entrypoints`

**.env.example should:**
- List all config variables from `tech_stack.config` or component configs
- Use placeholder values
</step>

<step n="7" name="scaffold_tests">
**Create test directory structure from `structure.tests`:**

If `structure.tests` not defined → skip this step.

1. Create test directory:
   ```bash
   mkdir -p [TEST_PATH]
   ```

2. Create test config/fixtures file:
   - Python: `conftest.py`
   - Node: `jest.config.js` or `vitest.config.ts`
   - Rust: (handled by Cargo)
   - Go: (handled by go test)

3. Create placeholder test file to verify setup works.
</step>

<step n="8" name="update_gitignore">
**Add OpenSDD patterns to .gitignore:**

Check if .gitignore exists:
```bash
test -f ".gitignore" && echo "EXISTS" || echo "NOT_FOUND"
```

If NOT_FOUND: Create new .gitignore
If EXISTS: Append to existing (avoid duplicates)

**Patterns to add:**

```gitignore
# OpenSDD - build artifacts (regeneratable)
.opensdd/*.state.yaml
.opensdd/*.state.py
.specs/
```

**What stays committed:**
- `.opensdd/blueprint.md` - Product definition
- `.opensdd/spec.yaml` - Source of truth (contracts)

**What gets ignored:**
- `.opensdd/*.state.yaml` - Session state (temporary)
- `.opensdd/*.state.py` - Copied script (temporary)
- `.specs/` - Extracted signatures (regenerated by spec-extract)

Example implementation:
```bash
# Check if patterns already exist
if ! grep -q "# OpenSDD" .gitignore 2>/dev/null; then
  cat >> .gitignore << 'EOF'

# OpenSDD - build artifacts (regeneratable)
.opensdd/*.state.yaml
.opensdd/*.state.py
.specs/
EOF
  echo "Added OpenSDD patterns to .gitignore"
else
  echo "OpenSDD patterns already in .gitignore"
fi
```
</step>

<step n="9" name="verify_scaffold">
**Report what was created vs skipped:**

```bash
echo "=== Scaffold Summary ==="
echo ""
echo "Created:"
# List all files that were created

echo ""
echo "Skipped (already existed):"
# List all files that were skipped

echo ""
echo "Directory structure:"
find [ROOT_PATH] -type d | head -20
```

Verify critical files exist:
- Project config file
- Root directory structure
- At least one entrypoint stub

If any critical file missing → fix before proceeding.
</step>

</steps>

<output>
Project infrastructure scaffolded:
- Project config file created/verified
- Directory structure created
- Entry point stubs created
- Shared types created
- Deployment files created (if specified)
- Test structure created (if specified)
- .gitignore updated with OpenSDD patterns
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| parse_spec | Spec sections identified | |
| scaffold_project | Project config exists | |
| scaffold_structure | Directories created | |
| scaffold_entrypoints | Entry point files exist | |
| scaffold_types | Type definitions created | |
| scaffold_deployment | Deployment files created (if applicable) | |
| scaffold_tests | Test structure exists (if applicable) | |
| update_gitignore | OpenSDD patterns added to .gitignore | |
| verify_scaffold | Summary shown, critical files verified | |

If any step failed:
- Identify which step failed
- Return to that step and redo
- Do NOT proceed until all steps pass

If all steps passed:
- Proceed to next
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue after verify passes.
</checkpoint>

<next>
1. Complete phase:
   ```bash
   python .opensdd/build-spec.state.py complete-phase 1
   ```

2. Speak to user:
   "Scaffold complete. Project infrastructure created. Starting component implementation..."

3. Load: `phase-02-select.md` (same folder)
</next>
