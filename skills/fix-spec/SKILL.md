---
name: fix-spec
description: Fix code-spec alignment issues from compare.report.yaml. Automatically fixes drifts and missing items, evaluates extras with strict rules, promotes to spec when justified, or deletes. Human-in-loop only as last resort. Use after `/opensdd:compare-spec` finds issues.
user-invocable: true
---

# Fix Spec

Automatically fix all code-spec alignment issues identified by compare-spec.

## Philosophy

**Three core principles guide this skill:**

### 1. Document Hierarchy: Blueprint -> Spec -> Code

```
Blueprint (product features - WHAT users need)
    ↓
Spec (technical contracts - boundaries for code)
    ↓
Code (implementation)
```

- Blueprint is the product source of truth
- Spec derives from blueprint, is the technical source of truth
- Code must match spec

### 2. Spec is the Single Source of Truth for Code

When spec and code disagree, code is wrong. This skill:
- Fixes code to match spec (drifts, missing)
- Evaluates extras with strict rules
- Promotes to spec ONLY when justified by blueprint
- Deletes code that doesn't belong
- Escalates to human ONLY when rules cannot determine the action

### 3. Spec Defines Boundaries, Not Implementation

When promoting extras to spec, we capture:
- Function signature (name, input type, output type)
- Purpose (`for:`) - WHAT it does, WHY it exists

We NEVER capture:
- Implementation details (caching, algorithms, protocols)
- Field definitions inside types
- Internal logic or HOW it works

## Phases

| Phase | Name | Purpose |
|-------|------|---------|
| 1 | Initialize | Load compare.report.yaml, validate, display plan |
| 2 | Verify | Re-verify each finding with strict methodology |
| 3 | Fix | Execute fixes: rebuild drifts, build missing, evaluate extras |
| 4 | Reconcile | Re-run compare, generate audit report |

## Input

- `.opensdd/compare.report.yaml` - Output from compare-spec
- `.opensdd/spec.yaml` - The source of truth
- `.opensdd/blueprint.md` - For evaluating extras against product intent

## Output

- Fixed code (drifts corrected, missing built)
- Updated spec.yaml (only for promoted extras, following strict rules)
- Deleted code (extras that don't belong)
- `.opensdd/fix.report.yaml` - Complete audit trail

## Key Principles

### 1. Verify Before Acting

compare-spec output might have errors. Before fixing:
- Re-check drifts: Is it truly drifted or just idiom difference?
- Re-check missing: Is it truly missing or named differently?
- Re-check extras: Is classification correct?

### 2. Strict Rules for Extras

Extras are evaluated by a decision tree with strict rules:
- `helper` -> KEEP (no spec change needed)
- `infrastructure` -> KEEP (no spec change needed)
- `test` -> KEEP (no spec change needed)
- `new_functionality` -> Evaluate visibility first, then rules

**For new_functionality, first determine visibility:**
- **INTERNAL** functions can promote directly to spec if blueprint-aligned
- **USER-FACING** features not in blueprint = SCOPE CREEP -> requires blueprint update first

### 3. Promote Only When Justified (Boundaries Only)

To promote an extra to spec:
- Must align with blueprint intent
- User-facing features MUST be in blueprint first (or added during escalation)
- Must follow spec schema exactly
- Must have clear purpose (`for:` description) - WHAT/WHY, never HOW
- Must fit existing component OR require new component with justification
- Types have purpose only, never field definitions

### 4. Human-in-Loop as Last Resort

Only escalate when:
- **Scope creep**: User-facing feature not in blueprint (requires product decision)
- Rules conflict or cannot determine action
- Low confidence in verification
- Extra could belong to spec OR be deleted with equal justification

## Approach

fix-spec follows the same principles as build-spec:

### Verification (Phase 2)
Main agent applies Chain of Verification inline - no separate agent needed since verification questions are straightforward and benefit from conversation context.

### Fixing (Phase 3)
- **Drifts**: Main agent uses Edit tool directly - simple signature changes don't need agent isolation
- **Missing**: Invokes `build-agent` (Opus) - reuses existing agent, same task
- **Extras**: Main agent follows decision tree in `extras-evaluation.md` - rule application, not AI judgment

### Probing (Phase 3)
After fixing drifts or building missing items, invokes `probe-agent` (Sonnet) to verify fixes work. Failed probes trigger retry with fix_hints (max 3 attempts), same as build-spec.

### Why No Dedicated Agents?
- **Builder ≠ Verifier** is satisfied by reusing build-agent + probe-agent
- **Clean context** comes from Task invocations, not agent definitions
- **Simplicity** - fewer agents to maintain, reuse proven patterns

## Start

<start>
Load: `references/phases/phase-01-initialize.md`
</start>
