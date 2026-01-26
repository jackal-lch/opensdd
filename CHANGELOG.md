# Changelog

All notable changes to OpenSDD will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-02-01

### Added

- **Commands** (Core Workflow)
  - `/opensdd:blueprint` - 9-phase guided product definition
    - Phase 0: Initialize (state tracking setup)
    - Phases 1-8: Vision, users, features, flows, data, integrations, constraints, assembly
    - State tracking via `blueprint.state.py` for session resume
  - `/opensdd:spec` - 4-phase technical specification generator
    - Foundation, types, contracts, integration phases
    - Language-specific guides (TypeScript, Python, Rust, Go)
    - Validation via `spec.py` script
  - `/opensdd:visualize` - Mermaid diagram generator
    - Architecture overview (C4-style container diagram)
    - Component dependencies (consumes relationships)
    - Event flow (emit/subscribe)
    - Type map (class diagram with usage)
  - `/opensdd:package` - 4-phase package splitting
    - Analyze, create, validate, finalize phases
    - Four package types: scaffold (00), types (01), component (02-98), integration (99)
    - Each package is self-contained with scope, context, instructions, verification
  - `/opensdd:build` - 2-phase automated build loop
    - Phase 1: Initialize (verify prerequisites, load manifest)
    - Phase 2: Build→Probe→Retry loop (max 3 attempts per package)
    - Optional `--review` mode for human checkpoint per package
  - `/opensdd:compare` - Code-spec alignment checker
    - Semantic comparison with language idiom translation
    - Reports: matches, drifts, missing, extras (classified)
    - Uses `spec-extract` tool + compare-agent
  - `/opensdd:fix` - 4-phase alignment fix
    - Initialize, verify, fix, reconcile phases
    - Automatic drift fixing via Edit tool
    - Missing items built via build-agent
    - Extras evaluated via decision tree
    - Scope creep detection for user-facing features not in blueprint

- **Agents**
  - `build-agent` (Opus) - Builds ONE package into production-ready code
    - BLOCK > FAKE principle: never placeholder, always report blocked
    - Accepts fix_hints from probe for retry attempts
    - Detailed anti-fake patterns per language
  - `probe-agent` (Sonnet) - Probes ONE package with REAL tests
    - Three statuses: GREEN (passed), FAILED (retry), BLOCKED (skip)
    - Verifies side effects (create → retrieve → verify)
    - Detects fake implementations
    - Records probe_attempts to package file
  - `compare-agent` - Whole-codebase semantic comparison
    - Intent matching by purpose, not just name
    - Language idiom translation (Optional, Result types, naming conventions)
    - Extras classification (helper, infrastructure, test, new_functionality)

- **Tools**
  - `spec-extract` - Rust CLI for extracting code signatures using tree-sitter
    - Supports TypeScript, Python, Rust, Go
    - Outputs YAML/JSON format
    - Generates index files for project-wide extraction

- **Commands** (Utilities)
  - `/opensdd:cov` - Chain of Verification for response validation
  - `/opensdd:super-review` - First principles review with industry best practices research
  - `/opensdd:super-implement` - Implementation following best practices without backward compatibility
  - `/opensdd:super-scan` - Comprehensive scan for consistency, legacy code, and conflicts

- **Documentation**
  - Comprehensive README with installation and usage instructions
  - Blueprint philosophy document explaining the OpenSDD approach
  - Spec schema reference

### Core Principles

- **Document Hierarchy**: Blueprint → Spec → Code
- **Spec Defines Boundaries**: WHAT/WHY, never implementation HOW
- **Builder ≠ Verifier**: Opus builds, Sonnet probes (different models, clean contexts)
- **Probe, Don't Assert**: Call functions, log output, no assertions to game
- **BLOCK > FAKE**: Missing info = BLOCKED, never placeholder
- **Focused Context > Large Context**: Each package sees only what it needs

### Technical Details

- State management via Python scripts for phase tracking
- Cross-platform install scripts for spec-extract (macOS, Linux, Windows)
- Clean context per agent invocation (fresh Task each time)
- fix_hints flow provides structured feedback for retries
- GitHub Actions workflow for automated releases
