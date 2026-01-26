# OpenSDD Agents

Subagents for isolating repetitive analysis from the main conversation context.

## Available Agents

| Agent | Use Case | Skill/Phase |
|-------|----------|-------------|
| [verify-compare](./verify-compare.md) | Semantic extract + compare | build-loop / Phase 3 |

## Why Only One Agent?

After analysis, we found that most phase steps benefit from staying in the main conversation because:

1. **Interactive workflow** - Users need to see progress and approve each step
2. **Tool trust** - `spec-extract` already handles language-specific parsing
3. **Simplicity** - The phases work well as-is without added complexity

The **verify-compare** agent is the exception because:
- The extract+compare cycle runs **multiple times** per component (loop until zero drift)
- Each iteration adds significant context that isn't needed after producing the report
- Isolating this prevents conversation bloat during the fix loop
- **Semantic comparison requires AI reasoning** that benefits from focused context

## verify-compare Agent

**Purpose**: Run steps 1-2 of build-loop Phase 3 with semantic understanding.

### What Makes It Different from Simple Matching

The agent doesn't just compare strings. It uses AI intelligence to:

| Capability | What It Does |
|------------|--------------|
| **Intent matching** | Matches functions by purpose, not just name (`parseConfig` ≈ `loadConfig`) |
| **Language idiom translation** | `User \| null` in spec = `Option<User>` in Rust = `*User` in Go |
| **Structural equivalence** | Method on class can fulfill standalone function spec |
| **Confidence levels** | Reports certainty: high / medium / low |
| **Actionable drifts** | Each drift includes `suggested_fix` for the fix step |
| **Categorized extras** | Classifies as helper / infrastructure / test / new_functionality |

### Flow

```
Main Agent                          Subagent
    │                                   │
    ├─── spawn with component ─────────►│
    │                                   ├── spec-extract
    │                                   ├── load spec + extracted
    │                                   ├── semantic comparison
    │                                   │   ├── intent matching
    │                                   │   ├── idiom translation
    │                                   │   └── structural equivalence
    │◄── JSON drift report ────────────┤
    │    (with confidence + suggested fixes)
    │
    ├── present to user (step 3)
    ├── fix drifts using suggested_fix (step 4)
    ├── record extras by category (step 5)
    │
    ├─── spawn again (re-verify) ──────►│
    │                                   ├── spec-extract
    │◄── JSON drift report ────────────┤
    │
    └── loop until zero drift
```

### Example Output

```json
{
  "status": "success",
  "component": "ConfigLoader",
  "language": "rust",
  "summary": {
    "matches": 2,
    "drifts": 1,
    "missing": 0,
    "extras": { "helpers": 1, "infrastructure": 0, "test": 0, "new_functionality": 0 }
  },
  "comparisons": [
    {
      "spec_item": "parseConfig",
      "status": "drift",
      "matched_to": "load_configuration",
      "confidence": "high",
      "details": {
        "drift_type": "naming",
        "spec_expects": "parseConfig(path: string): Config",
        "code_has": "load_configuration(file_path: String): Config",
        "difference": "Function and param names differ from spec",
        "suggested_fix": "Rename to parse_config (Rust convention for parseConfig)"
      }
    }
  ],
  "extras": [
    {
      "item": "validate_path",
      "signature": "validate_path(path: String) -> Result<(), ConfigError>",
      "file": "src/config.rs",
      "line": 45,
      "classification": "helper",
      "used_by": ["parseConfig"],
      "recommendation": "keep_internal"
    }
  ]
}
```

### What the Agent Does

1. Runs `spec-extract` on component path
2. Reads extracted YAML and spec.yaml
3. Builds intent map from spec's `for:` fields
4. Matches extracted to spec by intent (not just name)
5. Applies language idiom translation
6. Checks structural equivalence
7. Classifies: Match / Drift / Missing
8. For drifts: identifies type and suggests fix
9. For extras: traces usage to classify necessity
10. Returns JSON report

### What the Main Agent Handles

- Present report to user
- Fix drifts (using `suggested_fix` from report)
- Record extras to state (by category)
- Decide whether to re-verify

## Design Principles

1. **Semantic over syntactic** - Match by intent, not strings
2. **Language aware** - Idiom translation for TypeScript, Go, Rust, Python
3. **Actionable output** - Every drift has a suggested fix
4. **Confidence reporting** - Uncertain matches flagged for human review
5. **Categorized extras** - Helpers vs infrastructure vs test vs new functionality
6. **Mirror the phase** - Does exactly what steps 1-2 need, no more
7. **Trust the tools** - `spec-extract` handles language-specific parsing

## Usage from Phase Files

```xml
<step n="1-2" name="extract_and_compare">
<subagent agent="verify-compare">
  <input>
    <param name="component_name" from="state:current_component"/>
    <param name="component_path" from="spec:structure"/>
  </input>
</subagent>

On result:
- If status == "error": show error, ask user how to proceed
- If drifts > 0 or missing > 0: proceed to step 3 (show report)
  - Use comparisons[].details.suggested_fix for step 4 (fix)
- If only extras: proceed to step 5 (record by category)
- If clean (matches only): proceed to checkpoint
</step>
```

## Adding New Agents

Before adding an agent, ask:

1. **Does this step run multiple times?** If not, keep in main conversation
2. **Does the user need to see the process?** If yes, keep in main conversation
3. **Is significant context generated that's discarded after?** If yes, consider agent
4. **Does the phase already work well?** If yes, don't add complexity
5. **Does the step require AI reasoning that benefits from focused context?** If yes, consider agent

Most of the time, the answer is: **don't add an agent**.
