---
phase: 3
name: implement
next: phase-04-verify.md
---

# Phase 3: Implement

<objective>
Implement the selected component following the spec.yaml definition exactly.
</objective>

<prerequisite>
Get current component from state:

```bash
python .opensdd/build-spec.state.py status
```

Extract `current_component` from JSON output. If null:
- Show: "No component selected. Return to Phase 2."
- Load: `phase-02-select.md`
</prerequisite>

<input>
From state:
- `current_component`: The component to implement

From files:
- `.opensdd/spec.yaml`: Technical specification
- `.opensdd/blueprint.md`: Product context (if exists)
</input>

<steps>

<step n="1" name="load_component_spec">
Read spec.yaml and extract the full definition for `current_component`:

```bash
cat .opensdd/spec.yaml
```

Extract for the component:
- `for`: Purpose/responsibility
- `layer`: Which layer (domain/application/infrastructure)
- `provides`: Functions to implement with signatures
- `emits`: Events this component fires
- `subscribes`: Events this component listens to
- `consumes`: Other components it depends on
- `owns_data`: Data entities it manages

Also extract relevant:
- `tech_stack`: Language, framework
- `conventions`: Naming, style rules
- `structure.layers`: File paths for each layer
- `architecture.component_patterns.[component]`: Specific patterns to use
</step>

<step n="2" name="understand_context">
Build context for implementation:

1. **Read blueprint** (if exists):
   ```bash
   cat .opensdd/blueprint.md 2>/dev/null || echo "No blueprint"
   ```
   Understand the domain context and user intent.

2. **Check existing code** in the component's layer folder:
   ```bash
   ls -la [LAYER_PATH]/ 2>/dev/null || echo "Directory not exists"
   ```
   Understand what already exists from scaffold phase.

3. **Review consumed components**:
   For each component in `consumes`, read its interface so you can import and call it correctly.

4. **Review shared types**:
   Read the types file(s) from scaffold phase. Note which types are:
   - Complete (enums)
   - Skeletons (need fields populated)
</step>

<step n="3" name="populate_types">
**Populate type skeletons used by this component:**

**Philosophy:** Spec defines contracts (type names). Implementation defines details (fields).
The first component to USE a type is responsible for defining its fields.

**Process:**

1. **Identify types this component uses:**
   - Look at `provides` function signatures (params and returns)
   - Look at `owns_data` entities
   - Look at `emits` event payloads

2. **For each type used:**

   a. **Read the type file** from scaffold location

   b. **If type is a skeleton (no fields):**
      - Determine fields based on:
        - How this component's functions use it (what data they need)
        - The blueprint's domain description
        - Common domain patterns for this type of entity
      - Add fields to the type definition
      - Keep the original docstring

   c. **If type already has fields (populated by earlier component):**
      - Check if this component needs additional fields
      - If yes, add them (types can grow as more components use them)
      - If no, just import and use

3. **Create component-specific types:**
   - Input/Output DTOs for component functions (e.g., `CreateUserInput`)
   - Internal types not in spec's `types:` section
   - These go in the component's own file, NOT the shared types file

**Example - Populating User type:**

Component `UserService` provides: `get_user(id: UserId) -> User`

Read shared types file, find User is skeleton:
```python
@dataclass
class User:
    """Core user entity..."""
    pass
```

Determine fields from:
- UserService needs to return User → needs id, basic info
- Blueprint says users have email, name, status
- Domain patterns: users have created_at, updated_at

Populate:
```python
@dataclass
class User:
    """Core user entity representing authenticated users."""
    id: UserId
    email: str
    name: str
    status: UserStatus
    created_at: datetime
    updated_at: datetime
```

**Type ownership tracking:**
After populating a type, note in implementation summary which types this component defined.
Later components importing this type get the fields for free.
</step>

<step n="4" name="implement">
**Implement the component following spec:**

For each function in `provides`:
1. Create file in correct layer path per `structure.layers`
2. Implement function with EXACT signature from spec
3. Import shared types (now populated from step 3)
4. Create component-specific types as needed (DTOs, internal types)
5. Follow patterns from `architecture.component_patterns`
6. Handle errors per `architecture.global_patterns.error_handling`

For events in `emits`:
1. Implement event emission at appropriate points
2. Use correct payload types

For subscriptions in `subscribes`:
1. Wire up event handlers
2. Implement handler logic

**Implementation checklist:**
- [ ] File created in correct layer path
- [ ] All `provides` functions implemented with correct signatures
- [ ] Shared types imported from types file
- [ ] Component-specific types created locally
- [ ] Events emitted correctly
- [ ] Subscriptions wired up
- [ ] Error handling follows spec patterns
- [ ] Naming follows `conventions`
- [ ] Imports work (no circular imports)
</step>

<step n="5" name="verify_syntax">
Verify the code is syntactically valid:

**For Python:**
```bash
python -m py_compile [FILE_PATH]
```

**For TypeScript:**
```bash
npx tsc --noEmit [FILE_PATH]
```

**For Rust:**
```bash
cargo check
```

**For Go:**
```bash
go build ./...
```

If syntax errors → fix before proceeding.
</step>

<step n="6" name="summarize">
Summarize what was created:

```
Implementation Summary: [COMPONENT_NAME]
────────────────────────────────────────

Files created/modified:
- [file path]: [what it contains]

Functions implemented:
- [signature 1]
- [signature 2]
- ...

Types:
- Shared types populated: [list types this component defined fields for]
- Shared types imported: [list types already had fields]
- Component types created: [list DTOs/internal types]

Events:
- Emits: [list]
- Subscribes: [list]
```
</step>

</steps>

<output>
Component code written to disk. Implementation summary provided.
</output>

<verify>
AI self-verification:

| Step | Expected Output | Status |
|------|-----------------|--------|
| load_component_spec | Full component definition extracted | |
| understand_context | Context built from blueprint and existing code | |
| populate_types | Shared type skeletons populated with fields | |
| implement | All provides/emits/subscribes implemented | |
| verify_syntax | Code compiles/parses without errors | |
| summarize | Summary of created files, functions, and types | |

**Implementation completeness check:**
- All functions in `provides` exist in code?
- All shared types used have fields defined?
- All events in `emits` are fired somewhere?
- All subscriptions in `subscribes` have handlers?
- Code compiles without errors?

If any incomplete → fix before proceeding.
</verify>

<checkpoint required="false">
No user approval needed. Auto-continue to verify phase.
</checkpoint>

<next>
1. Mark component as implemented:
   ```bash
   python .opensdd/build-spec.state.py mark-implemented [COMPONENT_NAME]
   ```

2. Speak:
   "Implementation complete. Verifying alignment with spec..."

3. Load: `phase-04-verify.md` (same folder)
</next>
