---
description: Perform whole-codebase semantic comparison between extracted code signatures and spec.yaml
capabilities: ["semantic-comparison", "drift-detection", "language-idiom-translation", "extras-classification"]
---

# Agent: compare-agent

Perform complete bidirectional comparison between spec.yaml and extracted code signatures.

## Purpose

Given spec.yaml (what we expect) and .opensdd/extracted.yaml (what code has), produce a complete diff:
- **match**: spec item exists in code with correct signature
- **drift**: spec item exists in code but signature differs
- **missing**: spec item has no implementation in code
- **extra**: code has item not defined in spec

## Input

| Parameter | Description |
|-----------|-------------|
| `spec_file` | Path to `.opensdd/spec.yaml` |
| `extracted_file` | Path to `.opensdd/extracted.yaml` containing all extracted code signatures |

## Instructions

### Step 1: Load Sources

**Load spec.yaml:**
```bash
cat {spec_file}
```

Parse and extract:
- `components`: map of component definitions
- `types`: shared type definitions
- `structure.root`: source root directory
- `structure.layers`: layer name → directory mapping
- `tech_stack.language`: programming language (for idiom translation)

**Load extracted file:**
```bash
cat {extracted_file}
```

The extracted.yaml contains:
- `project`: project name
- `root`: source root directory
- `extracted_at`: timestamp
- `files`: array of file specifications, each with:
  - `file`: source file path
  - `package`: module/package name
  - `imports`: list of import statements (optional)
  - `types`: array of type definitions, each with `name`, `kind`, `fields`, `methods`, `embeds`, `implements`, `variants`
  - `functions`: array of standalone functions, each with `signature`, `doc`, `uses`
  - `methods`: array of methods, each with `signature`, `doc`, `receiver`, `uses`
  - `constants`: array of constants, each with `name`, `type_name`, `value`, `doc` (optional)
  - `variables`: array of variables, each with `name`, `type_name`, `doc` (optional)
  - `errors`: array of error definitions, each with `name`, `message`, `doc` (optional)

### Step 2: Build Mappings

**Map components to extracted files:**

For each component in spec:
1. Get component's `layer` (e.g., "application")
2. Get layer directory from `structure.layers` (e.g., "services/")
3. Find matching files in `extracted.yaml` by path prefix
4. Match by naming convention:
   - ComponentName → component_name.py (Python)
   - ComponentName → ComponentName.ts (TypeScript)
   - ComponentName → component_name.rs (Rust)
   - ComponentName → component_name.go (Go)

Store mapping: `component → [list of extracted files]`

### Step 3: Compare (Bidirectional Scan)

**For each component in spec:**

For each function in component's `provides`:

1. Search mapped extracted files for matching function
2. Apply semantic matching (see Semantic Matching Rules below)
3. Classify result:

| Result | Criteria |
|--------|----------|
| **match** | Found with compatible signature (after idiom translation) |
| **drift** | Found but signature incompatible |
| **missing** | No function fulfills this intent |

4. If drift, identify:
   - `drift_type`: naming / param / return / structural
   - `spec_expects`: exact spec signature
   - `code_has`: exact extracted signature
   - `suggested_fix`: action to resolve

5. Record confidence: high / medium / low

**For types in spec:**

Same process - find matching type in extracted, classify as match/drift/missing.

**Track all matched code items** for extras identification.

### Step 4: Identify Extras

Collect all items from extracted files NOT matched in Step 3.

For each extra item, classify:

| Classification | Criteria |
|----------------|----------|
| `helper` | Called by a matched spec function (trace imports/calls) |
| `infrastructure` | Language idiom: error types, interfaces, traits, protocols, type aliases |
| `test` | Name contains test/mock/stub, or in test directory |
| `new_functionality` | Standalone business logic not used by spec functions |

Record each extra with:
- `item`: function/type name
- `signature`: full signature
- `file`: source file path
- `line`: line number
- `classification`: helper / infrastructure / test / new_functionality
- `used_by`: list of spec functions that use this (if helper)

### Step 5: Return Result

Return ONLY valid JSON (no markdown, no explanation).

**Schema:** Output must conform to the schema defined in `skills/compare-spec/references/output-schema.yaml`.

Key requirements:
- `status`: "success" or "error"
- `timestamp`: ISO 8601 format
- `summary`: counts for total_components, total_types, matches, drifts, missing, total_extras, extras_by_type
- `components`: map of component name → status, layer, matched_file, provides (with per-function details)
- `types`: map of type name → status, matched_to, spec_expects, code_has
- `extras`: array of items not in spec with kind, signature, file, line, classification, used_by

If comparison fails, return:

```json
{
  "status": "error",
  "error": "{description}",
  "details": "{additional context}"
}
```

---

## Semantic Matching Rules

### Intent Matching (Primary)

Match by PURPOSE, not just name:
- `parseConfig` ≈ `loadConfig` ≈ `readConfig` if same purpose
- Compare `for:` descriptions semantically
- Consider what function operates on and produces

### Language Idiom Translation

Normalize signatures for `tech_stack.language` before comparing:

| Spec Pattern | Python | TypeScript | Rust | Go |
|--------------|--------|------------|------|-----|
| `T \| null` | `Optional[T]` | `T \| null` | `Option<T>` | `*T` or `(T, bool)` |
| `throws Error` | `raises Exception` | `throws` | `Result<T, E>` | `(T, error)` |
| `functionName` | `function_name` | `functionName` | `function_name` | `FunctionName` |
| `async T` | `async def -> T` | `Promise<T>` | `async fn -> T` | goroutine |
| `boolean` | `bool` | `boolean` | `bool` | `bool` |

### Structural Equivalence

These are equivalent if interface matches:

| Spec Says | Code Has | Equivalent? |
|-----------|----------|-------------|
| Standalone function | Method on class/struct | Yes |
| One function | Public fn + private helpers | Yes |
| Sync function | Async with sync wrapper | Yes |
| Direct implementation | Wrapper delegating to lib | Yes |

### Confidence Levels

| Level | Criteria |
|-------|----------|
| `high` | Exact/near-exact match, clear classification |
| `medium` | Intent matches but signature differs |
| `low` | Uncertain match, needs human review |

---

## Constraints

- Do NOT modify any files
- Do NOT interact with user
- Return ONLY the JSON result
- Use semantic understanding, not string matching
- Trust spec-extract output format
