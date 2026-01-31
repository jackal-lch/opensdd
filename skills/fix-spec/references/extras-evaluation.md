# Extras Evaluation Decision Tree

SINGLE SOURCE OF TRUTH for evaluating `new_functionality` extras.

Referenced by:
- `skills/fix-spec/references/phases/phase-03-fix.md` (consumer)

## Philosophy

**Three core principles:**

1. **Blueprint is the product source of truth** - Defines WHAT users need
2. **Spec is the technical source of truth** - Defines contracts/boundaries for code
3. **Spec defines boundaries, not implementation** - Only WHAT and WHY, never HOW

**Document hierarchy:**
```
Blueprint (product features)
    ↓
Spec (technical contracts)
    ↓
Code (implementation)
```

When promoting extras:
- **User-facing features** not in blueprint = SCOPE CREEP → Requires blueprint update first
- **Internal functions** aligned with blueprint = OK → Can promote directly to spec
- **Implementation details** = NEVER captured in spec

## Classification Overview

| Classification | Meaning | Action |
|----------------|---------|--------|
| `helper` | Used by spec functions | KEEP (no spec change) |
| `infrastructure` | Language idioms, cross-cutting | KEEP (no spec change) |
| `test` | Test utilities | KEEP (no spec change) |
| `new_functionality` | Standalone business logic | EVALUATE (this doc) |

## Decision Tree for new_functionality

```
EXTRA (new_functionality)
         │
         ▼
┌─────────────────────────────────────────────────────────────┐
│ STEP 1: Visibility Check                                    │
│                                                             │
│ Is this USER-FACING or INTERNAL?                            │
│                                                             │
│ USER-FACING indicators:                                     │
│ - Exposed via public API/routes                             │
│ - Called from UI/client code                                │
│ - Has user-visible behavior or output                       │
│ - Represents a product feature                              │
│                                                             │
│ INTERNAL indicators:                                        │
│ - Private/unexported function                               │
│ - Only called by other internal code                        │
│ - Implementation detail or optimization                     │
│ - No external visibility                                    │
└─────────────────────────────────────────────────────────────┘
         │
         ├── INTERNAL ───────────────────────────────────────────┐
         │   -> Continue to Step 2 (can promote directly to spec) │
         │                                                        │
         └── USER-FACING ────────────────────────────────────────┤
                  │                                               │
                  ▼                                               │
         ┌─────────────────────────────────────────────────────┐ │
         │ STEP 1b: Blueprint Alignment (USER-FACING only)     │ │
         │                                                     │ │
         │ Is this feature in blueprint.md?                    │ │
         └─────────────────────────────────────────────────────┘ │
                  │                                               │
                  ├── YES (in blueprint) ─────────────────────────┤
                  │   -> Continue to Step 2                       │
                  │   -> Can promote to spec (feature is approved)│
                  │                                               │
                  └── NO (not in blueprint) ──────────────────────┤
                           │                                      │
                           ▼                                      │
                  ┌─────────────────────────────────────────────┐ │
                  │ SCOPE CREEP DETECTED                        │ │
                  │                                             │ │
                  │ User-facing feature not in blueprint.       │ │
                  │ This is a PRODUCT decision, not technical.  │ │
                  │                                             │ │
                  │ -> ESCALATE to human with special options:  │ │
                  │    A) "Add to product" - Update blueprint   │ │
                  │       FIRST, then promote to spec           │ │
                  │    B) "Delete" - Remove from code           │ │
                  │    C) "Keep informal" - Leave without spec  │ │
                  │       (technical debt, not recommended)     │ │
                  └─────────────────────────────────────────────┘ │
                                                                  │
         ┌────────────────────────────────────────────────────────┘
         │
         ▼
┌─────────────────────────────────────────────────────────────┐
│ STEP 2: Blueprint Alignment Check (for INTERNAL or          │
│         approved USER-FACING)                               │
│                                                             │
│ Read blueprint.md and answer:                               │
│ - Does this function serve a documented feature?            │
│ - Is it mentioned in any user flow?                         │
│ - Does it align with the product vision?                    │
└─────────────────────────────────────────────────────────────┘
         │
         ├── YES (explicitly mentioned) ──────────────────────┐
         │   Confidence: HIGH                                  │
         │   Decision: PROMOTE to spec                         │
         │                                                     │
         ├── RELATED (serves documented feature) ─────────────┤
         │   Confidence: MEDIUM                                │
         │   Decision: PROMOTE candidate                       │
         │                                                     │
         ├── NO (not related) ────────────────────────────────┤
         │                                                     │
         ▼                                                     │
┌─────────────────────────────────────────────────────────────┐
│ STEP 3: Infrastructure Check                                │
│                                                             │
│ Is this cross-cutting infrastructure?                       │
│                                                             │
│ Indicators:                                                 │
│ - Logging utilities (log_*, logger, etc.)                   │
│ - Configuration helpers (get_config, load_settings)         │
│ - Monitoring/metrics (track_*, measure_*, metrics_*)        │
│ - Caching utilities (cache_*, memoize)                      │
│ - Error handlers (handle_error, error_boundary)             │
│ - File location: utils/, lib/, common/, shared/             │
└─────────────────────────────────────────────────────────────┘
         │
         ├── YES ─────────────────────────────────────────────┐
         │   Reclassify: INFRASTRUCTURE                        │
         │   Decision: KEEP                                    │
         │                                                     │
         ├── NO ──────────────────────────────────────────────┤
         │                                                     │
         ▼                                                     │
┌─────────────────────────────────────────────────────────────┐
│ STEP 4: Dependency Analysis                                 │
│                                                             │
│ Check what uses this function:                              │
│                                                             │
│ grep -r "{function_name}(" src/                             │
│ grep -r "import.*{function_name}" src/                      │
└─────────────────────────────────────────────────────────────┘
         │
         ├── Not imported anywhere ───────────────────────────┐
         │   Decision: DELETE (safe, no dependents)            │
         │                                                     │
         ├── Only imported by other extras ───────────────────┤
         │   Decision: DELETE (cascade)                        │
         │   Note: Delete dependents too                       │
         │                                                     │
         ├── Imported by spec code ───────────────────────────┤
         │   Reclassify: HELPER                                │
         │   Decision: KEEP                                    │
         │                                                     │
         ▼                                                     │
┌─────────────────────────────────────────────────────────────┐
│ STEP 5: Final Decision                                      │
│                                                             │
│ Based on all evidence:                                      │
└─────────────────────────────────────────────────────────────┘
         │
         ├── PROMOTE candidate from Step 2 (high/medium) ─────┐
         │   -> Execute promotion to spec.yaml                 │
         │                                                     │
         ├── DELETE from Step 4 (no dependents) ──────────────┤
         │   -> Remove from code                               │
         │                                                     │
         ├── KEEP (reclassified) ─────────────────────────────┤
         │   -> No action needed                               │
         │                                                     │
         └── UNCERTAIN (conflicting signals) ─────────────────┤
             -> ESCALATE to human                              │
             -> Present options: Promote / Delete / Keep       │
```

## Visibility Classification

### User-Facing Indicators

| Indicator | Example |
|-----------|---------|
| Public API endpoint | `@app.route("/api/users")` |
| Exported from public module | `export function getUser()` |
| Called by UI/client | Referenced in frontend code |
| Has user documentation | Mentioned in API docs |
| Returns user-visible data | User profile, order status |
| Triggers user notification | Email, SMS, push |

### Internal Indicators

| Indicator | Example |
|-----------|---------|
| Private/unexported | `def _helper()`, `function _internal()` |
| In internal module | `src/internal/`, `src/lib/` |
| Only called by other functions | No direct external calls |
| Implementation optimization | Caching, batching, pooling |
| Data transformation | Internal mapping, formatting |

## Scope Creep Handling

When a USER-FACING feature is found that's NOT in blueprint:

**This is SCOPE CREEP** - a product decision was made during implementation without updating the product definition.

**Options presented to human:**

| Option | Description | Action |
|--------|-------------|--------|
| "Add to product" | This is a valuable feature, make it official | 1. Human updates blueprint.md<br>2. Then promote to spec.yaml |
| "Delete" | This shouldn't exist | Remove from code |
| "Keep informal" | Leave for now, decide later | Keep in code without spec entry (tech debt) |

**Why require blueprint first for user-facing features?**

1. Blueprint is the product source of truth
2. Adding to spec without blueprint = technical decision overriding product decision
3. Future spec regeneration would miss it
4. Keeps blueprint and spec in sync

## Promotion Rules

When promoting an extra to spec, MUST follow these rules:

### Rule 1: Schema Compliance

Addition must conform to spec.yaml schema:

```yaml
# Adding to existing component
components:
  {ComponentName}:
    provides:
      - {function_name}:
          for: "{WHAT it does - boundary only, no HOW}"
          input: {InputType}
          output: {OutputType} | {ErrorType}

# Adding new type (if needed)
types:
  {TypeName}:
    for: "{purpose - what this type represents}"
    used:
      - {component}.{function}
```

**Remember:** Only boundaries and contracts. No implementation details, no field definitions.

### Rule 2: Boundaries Only (CRITICAL)

**Spec defines WHAT and WHY, never HOW.**

The `for:` field captures:
- WHAT the function does (the boundary/contract)
- WHY it exists (the business purpose)

The `for:` field NEVER captures:
- HOW it works (implementation details)
- Algorithms, caching strategies, protocols
- Performance characteristics
- Internal data structures

**Examples:**

| Code Implementation | BAD `for:` (has implementation) | GOOD `for:` (boundary only) |
|---------------------|--------------------------------|----------------------------|
| Uses Redis cache | "retrieves user with cached fallback" | "retrieves user by ID" |
| Validates per RFC 5322 | "validates email according to RFC 5322" | "validates email format" |
| Uses bcrypt hashing | "hashes password with bcrypt" | "hashes password for storage" |
| Retries 3 times | "sends email with retry logic" | "sends email to user" |
| Batches in groups of 100 | "processes users in batches of 100" | "processes all users" |

**Derivation process:**
1. Read function name - what verb + noun implies
2. Read docstring/comments - extract the WHAT, ignore the HOW
3. If still unclear, ask: "What contract does this fulfill?"

### Rule 3: Type Consistency (Boundaries Only)

All types referenced must:
- Already exist in spec.types, OR
- Be added with proper `for:` and `used:`

**CRITICAL: Types have PURPOSE, not FIELDS**

When adding a new type to spec:
```yaml
# CORRECT - purpose only
types:
  WelcomeEmailResult:
    for: "outcome of sending welcome email"
    used:
      - Notifications.send_welcome_email

# WRONG - includes field definitions (implementation!)
types:
  WelcomeEmailResult:
    for: "outcome of sending welcome email"
    fields:                    # <- NEVER DO THIS
      success: bool
      message_id: str
      sent_at: datetime
```

The AI implementing the code will infer appropriate fields from the purpose.

### Rule 4: Component Fit

Function must logically belong to the component:
- Aligns with component's `for:` responsibility
- Matches component's layer (domain/application/infrastructure)
- Uses component's dependencies

If no existing component fits:
- Requires new component (rare, needs strong justification)
- New component needs blueprint alignment

## Deletion Rules

When deleting an extra:

### Rule 1: Verify No Dependents

Must confirm:
- Not imported anywhere in src/
- Not called by any function
- Not exported in index files

### Rule 2: Clean Removal

- Remove function from file
- Remove any now-unused imports
- If file becomes empty, delete file
- Update any re-export files (index.ts, __init__.py)

### Rule 3: Record Reason

Always record why deleted:
- "Not in blueprint, no dependencies"
- "Only used by other deleted extras"
- "User decision via escalation"

## Escalation Criteria

Escalate to human when:

1. **Scope creep**: User-facing feature not in blueprint
2. **Conflicting signals**: Blueprint says related, but no imports
3. **Multiple valid options**: Could reasonably promote OR delete
4. **Low confidence**: Uncertain classification after all checks
5. **Architectural impact**: Promoting would significantly change spec
6. **Cross-cutting concern**: Affects multiple components

## Examples

### Example 1: Internal Function - Promote Directly

```
Extra: _calculate_discount(order: Order, rules: list) -> float
File: src/services/pricing.py
Visibility: INTERNAL (underscore prefix, only called by apply_pricing)

Step 1: INTERNAL -> Skip user-facing check, continue

Step 2: Blueprint mentions "dynamic pricing" in features
  -> RELATED (serves documented feature)
  -> PROMOTE candidate (medium confidence)

Step 3: N/A (already decided)

Step 4: Execute promotion
  -> Add to components.Pricing.provides
  -> for: "calculates discount for order"

  NOTE: We say "calculates discount" (WHAT)
        NOT "calculates using rule engine with caching" (HOW)
```

### Example 2: User-Facing in Blueprint - Promote

```
Extra: send_welcome_email(user_id: str) -> bool
File: src/services/notifications.py
Visibility: USER-FACING (sends email to user)

Step 1: USER-FACING -> Check blueprint

Step 1b: Blueprint mentions "welcome email on signup" in user flow
  -> YES, in blueprint
  -> Continue (approved)

Step 2: Explicitly mentioned
  -> PROMOTE (high confidence)

Step 3: N/A

Step 4: Execute promotion
  -> Add to components.Notifications.provides
  -> for: "sends welcome email to user"
```

### Example 3: User-Facing NOT in Blueprint - Scope Creep

```
Extra: export_user_data(user_id: str, format: str) -> bytes
File: src/api/routes/users.py
Visibility: USER-FACING (public API endpoint)

Step 1: USER-FACING -> Check blueprint

Step 1b: Blueprint does NOT mention data export feature
  -> NO, not in blueprint
  -> SCOPE CREEP DETECTED

  ESCALATE with options:
  A) "Add to product" - Human adds to blueprint, then spec
  B) "Delete" - Remove the endpoint
  C) "Keep informal" - Leave as tech debt
```

### Example 4: Delete - Not Used

```
Extra: legacy_import_users(file_path: str) -> int
File: src/utils/migration.py
Visibility: INTERNAL

Step 1: INTERNAL -> Continue

Step 2: Not mentioned in blueprint
  -> NO

Step 3: Not infrastructure
  -> NO

Step 4: grep finds no imports
  -> Not imported anywhere
  -> DELETE (safe)

Step 5: Execute deletion
  -> Remove function from migration.py
  -> Reason: "Not in blueprint, no dependencies"
```

### Example 5: Reclassify as Helper

```
Extra: validate_email_format(email: str) -> bool
File: src/utils/validators.py
Visibility: INTERNAL

Step 1: INTERNAL -> Continue

Step 2: Not explicitly in blueprint
  -> NO

Step 3: Not infrastructure
  -> NO

Step 4: grep finds imports
  -> Used by UserService.create_user (spec function)
  -> Reclassify: HELPER
  -> KEEP

Step 5: No action needed
  -> Already in code, supports spec function
```
