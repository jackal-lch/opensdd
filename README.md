<p align="center">
  <img src="docs/assets/banner-v2.png" alt="OpenSDD - Spec-Driven Development that actually works" width="800">
</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <a href="https://claude.ai/code"><img src="https://img.shields.io/badge/Claude%20Code-Plugin-blueviolet" alt="Claude Code Plugin"></a>
</p>

<p align="center">
  <a href="#why-opensdd">Why OpenSDD?</a> •
  <a href="#how-it-works">How It Works</a> •
  <a href="#commands">Commands</a> •
  <a href="#installation">Installation</a> •
  <a href="#example-session">Example</a>
</p>

---

## Quick Start

```bash
# 1. Open Claude Code
claude

# 2. Add marketplace & install plugin
/plugin marketplace add jackal-lch/opensdd
/plugin install opensdd@opensdd-marketplace

# 3. Install spec-extract (separate terminal)
curl -fsSL https://raw.githubusercontent.com/jackal-lch/opensdd/main/scripts/install-spec-extract.sh | bash

# 4. Start building
/opensdd:blueprint
```

---

## Why OpenSDD?

<p align="center">
  <img src="docs/assets/why-opensdd.png" alt="Why OpenSDD - Traditional SDD vs OpenSDD" width="800">
</p>

**OpenSDD fixes spec drift.** It continuously verifies that your code matches the spec. When they drift apart, the code gets fixed automatically.

The spec is the source of truth. Always.

---

## How It Works

<p align="center">
  <img src="docs/assets/artifact-flow.png" alt="OpenSDD Artifact & Data Flow Architecture" width="800">
</p>

| Phase | Command | Output |
|-------|---------|--------|
| **Define** | `/opensdd:blueprint` | `.opensdd/blueprint.md` — Product vision, features, flows |
| **Define** | `/opensdd:spec` | `.opensdd/spec.yaml` — Technical contracts (source of truth) |
| **Define** | `/opensdd:visualize` | `.opensdd/spec.visual.md` — Mermaid diagrams (optional) |
| **Build** | `/opensdd:package` | `.opensdd/packages/*.yaml` — Focused work units |
| **Build** | `/opensdd:build` | `src/` — Implementation code |
| **Verify** | `/opensdd:compare` | `.opensdd/compare.report.yaml` — Alignment report |
| **Verify** | `/opensdd:fix` | Fixed code + `.opensdd/fix.report.yaml` |

### The Build Loop

<p align="center">
  <img src="docs/assets/build-loop.png" alt="OpenSDD Build Loop - Agent Separation" width="700">
</p>

- **Build Agent (Opus)** — Implements code from package spec
- **Probe Agent (Sonnet)** — Verifies with REAL tests (no mocks, no self-validation)
- **Fresh context per attempt** — No accumulated confusion
- **Different models** — Builder doesn't know how it will be verified

---

## Commands

### Core Workflow

| Command | Phases | Purpose |
|---------|--------|---------|
| `/opensdd:blueprint` | 9 | Guided product definition (vision, users, features, flows) |
| `/opensdd:spec` | 4 | Technical specification (components, types, interfaces) |
| `/opensdd:visualize` | — | Generate Mermaid diagrams from spec |
| `/opensdd:package` | 4 | Split spec into focused, self-contained packages |
| `/opensdd:build` | 2 | Build loop: implement → probe → retry (max 3 attempts) |
| `/opensdd:compare` | — | Check code-spec alignment |
| `/opensdd:fix` | 4 | Fix alignment issues (drifts, missing, extras) |

### Utilities

| Command | Purpose |
|---------|---------|
| `/opensdd:cov` | Chain of Verification — validate responses |
| `/opensdd:super-review` | First principles review with best practices |
| `/opensdd:super-implement` | Implement following best practices |
| `/opensdd:super-scan` | Scan for consistency and legacy code |

---

## Installation

### Prerequisites

- [Claude Code](https://claude.ai/code) CLI installed
- macOS, Linux, or Windows

### Install Plugin

```bash
claude
```

```
/plugin marketplace add jackal-lch/opensdd
/plugin install opensdd
```

Verify with `/opensdd:` — you should see all commands in autocomplete.

### Install spec-extract

Extracts code signatures for verification.

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/jackal-lch/opensdd/main/scripts/install-spec-extract.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/jackal-lch/opensdd/main/scripts/install-spec-extract.ps1 | iex
```

Verify: `spec-extract --version`

---

## Example Session

```
You: I want to build a task management CLI

> /opensdd:blueprint    → .opensdd/blueprint.md
> /opensdd:spec         → .opensdd/spec.yaml
> /opensdd:visualize    → .opensdd/spec.visual.md
> /opensdd:package      → .opensdd/packages/*.yaml

> /opensdd:build
  [1/5] pkg-00-scaffold:    GREEN (1 attempt)
  [2/5] pkg-01-types:       GREEN (1 attempt)
  [3/5] pkg-02-task-manager: GREEN (2 attempts)
  [4/5] pkg-03-storage:     BLOCKED (missing DATABASE_URL)
  [5/5] pkg-99-integration: GREEN (1 attempt)
  ✓ 4 GREEN, 1 BLOCKED

> /opensdd:compare
  ✓ 12 matches, 1 drift, 0 missing, 3 extras

> /opensdd:fix
  ✓ Code aligned with spec
```

### Project Structure

```
your-project/
├── .opensdd/
│   ├── blueprint.md         # Product definition
│   ├── spec.yaml            # Technical contracts (source of truth)
│   ├── spec.visual.md       # Mermaid diagrams
│   ├── packages/
│   │   ├── manifest.yaml    # Build order
│   │   └── pkg-*.yaml       # Package specs + probe results
│   ├── extracted.yaml       # Code signatures
│   ├── compare.report.yaml  # Alignment report
│   └── fix.report.yaml      # Fix audit trail
└── src/                     # Your code, verified against spec
```

---

## Supported Languages

TypeScript • Python • Rust • Go

---

## Contributing

```bash
git clone https://github.com/jackal-lch/opensdd.git
cd opensdd
claude --plugin-dir .  # Test locally
```

1. Fork → 2. Branch → 3. Commit → 4. PR

Issues & feature requests: [GitHub Issues](https://github.com/jackal-lch/opensdd/issues)

---

## License

[MIT](LICENSE)
