<p align="center">
  <img src="docs/assets/banner-v2.png" alt="OpenSDD - Spec-Driven Development that actually works" width="800">
</p>

<p align="center">
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-yellow.svg" alt="License: MIT"></a>
  <a href="https://claude.ai/code"><img src="https://img.shields.io/badge/Claude%20Code-Plugin-blueviolet" alt="Claude Code Plugin"></a>
</p>

---

## Quick Start

**1. Open Claude Code**
```bash
claude
```

**2. Add the marketplace**
```
/plugin marketplace add jackal-lch/opensdd
```

**3. Install the plugin**
```
/plugin install opensdd@opensdd-marketplace
```

**4. Install spec-extract** (in a separate terminal)
```bash
curl -fsSL https://raw.githubusercontent.com/jackal-lch/opensdd/main/scripts/install-spec-extract.sh | bash
```

**5. Start building**
```
/opensdd:blueprint
```

---

## Table of Contents

- [Why OpenSDD?](#why-opensdd)
- [How It Works](#how-it-works)
- [Commands](#commands)
- [Installation](#installation)
- [Example Session](#example-session)
- [Supported Languages](#supported-languages)
- [Contributing](#contributing)
- [License](#license)

---

## Why OpenSDD?

Traditional specs become fiction:

```
Day 1:  Spec is perfect
Day 30: Spec is stale
Day 60: Nobody trusts it
```

**OpenSDD fixes this.** It continuously verifies that your code matches the spec. When they drift apart, the code gets fixed automatically.

The spec is the source of truth. Always.

---

## How It Works

```
/opensdd:blueprint → /opensdd:spec → /opensdd:package → /opensdd:build
                           ↓
                  /opensdd:visualize (optional)
                           ↓
                  /opensdd:compare → /opensdd:fix
```

| Command | What It Does |
|---------|--------------|
| `/opensdd:blueprint` | Define your product (vision, users, features, flows) - 9 phases |
| `/opensdd:spec` | Generate technical contracts (components, types, interfaces) - 4 phases |
| `/opensdd:visualize` | Generate Mermaid diagrams to understand system design |
| `/opensdd:package` | Split spec into focused work packages - 4 phases |
| `/opensdd:build` | Implement and verify code (build → probe → retry loop) |
| `/opensdd:compare` | Check code-spec alignment (matches, drifts, missing, extras) |
| `/opensdd:fix` | Automatically fix alignment issues from compare report |

The build loop for each package:

```
┌──────────────────────────────────────────┐
│   Build (Opus) → Probe (Sonnet)          │
│        ↑              │                  │
│        │         GREEN? → Done           │
│        │         BLOCKED? → Next pkg     │
│        │              │                  │
│        └── fix_hints ─┘ (max 3 attempts) │
└──────────────────────────────────────────┘
```

1. **Build** — Opus implements code from package spec
2. **Probe** — Sonnet verifies with REAL tests (no mocks)
3. **Retry** — Failed probes provide fix_hints for retry
4. **Compare** — After build, check overall alignment
5. **Fix** — Drifts fixed, missing built, extras evaluated

---

## Commands

### Core Workflow

| Command | Purpose |
|---------|---------|
| `/opensdd:blueprint` | 9-phase guided product definition |
| `/opensdd:spec` | 4-phase technical specification |
| `/opensdd:visualize` | Generate Mermaid diagrams from spec |
| `/opensdd:package` | 4-phase package splitting for focused builds |
| `/opensdd:build` | 2-phase build loop (initialize, build→probe→retry) |
| `/opensdd:compare` | Check code-spec alignment (matches, drifts, missing, extras) |
| `/opensdd:fix` | 4-phase alignment fix (drifts, missing, extras evaluation) |

### Utility Commands

| Command | Purpose |
|---------|---------|
| `/opensdd:cov` | Chain of Verification - validate and enhance responses |
| `/opensdd:super-review` | First principles review with industry best practices research |
| `/opensdd:super-implement` | Implementation following best practices without backward compatibility |
| `/opensdd:super-scan` | Comprehensive scan for consistency, legacy code, and conflicts |

---

## Installation

### Prerequisites

- [Claude Code](https://claude.ai/code) CLI installed
- macOS, Linux, or Windows

### Step 1: Install the Plugin

Open Claude Code and run these commands:

```
claude
```

Then inside Claude Code:

```
/plugin marketplace add jackal-lch/opensdd
/plugin install opensdd
```

To verify installation:

```
/opensdd:
```

You should see `blueprint`, `spec`, `visualize`, `package`, `build`, `compare`, `fix`, `cov`, `super-review`, `super-implement`, and `super-scan` in the autocomplete.

### Step 2: Install spec-extract

The `spec-extract` tool extracts code signatures for verification.

**macOS / Linux:**
```bash
curl -fsSL https://raw.githubusercontent.com/jackal-lch/opensdd/main/scripts/install-spec-extract.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://raw.githubusercontent.com/jackal-lch/opensdd/main/scripts/install-spec-extract.ps1 | iex
```

### Step 3: Verify Installation

```bash
spec-extract --version
```

---

## Example Session

```
You: I want to build a task management CLI

> /opensdd:blueprint
[Interactive: 9 phases - vision, users, features, flows, data, integrations, constraints, assembly]
✓ Created .opensdd/blueprint.md

> /opensdd:spec
[Interactive: 4 phases - foundation, types, contracts, integration]
✓ Created .opensdd/spec.yaml

> /opensdd:visualize
✓ Created .opensdd/spec.visual.md (Mermaid diagrams)

> /opensdd:package
[4 phases - analyze, create, validate, finalize]
✓ Created .opensdd/packages/manifest.yaml + pkg-*.yaml

> /opensdd:build
[1/5] pkg-00-scaffold: GREEN (1 attempt)
[2/5] pkg-01-types: GREEN (1 attempt)
[3/5] pkg-02-task-manager: GREEN (2 attempts)
[4/5] pkg-03-storage: BLOCKED (missing DATABASE_URL)
[5/5] pkg-99-integration: GREEN (1 attempt)
✓ Build complete: 4 GREEN, 1 BLOCKED

> /opensdd:compare
✓ 12 matches, 1 drift, 0 missing, 3 extras
  ⚠ Drift: TaskManager.createTask return type
  Report: .opensdd/compare.report.yaml

> /opensdd:fix
  → Fixed drift: updated return type annotation
  → Kept 2 helpers, 1 infrastructure
✓ Code aligned with spec
```

**Generated project structure:**

```
your-project/
├── .opensdd/
│   ├── blueprint.md        # Product definition (via /blueprint)
│   ├── blueprint.state.yaml # State tracking for blueprint phases
│   ├── spec.yaml           # Technical contracts (via /spec)
│   ├── spec.visual.md      # Mermaid diagrams (via /visualize)
│   ├── packages/           # Build packages (via /package)
│   │   ├── manifest.yaml
│   │   └── pkg-*.yaml      # Each has probe_attempts after build
│   ├── compare.report.yaml # Code-spec alignment (via /compare)
│   ├── fix.report.yaml     # Fix audit trail (via /fix)
│   └── extracted/          # Code signatures (temp)
└── src/
    └── ...                 # Your code, verified against spec
```

---

## Supported Languages

| Language | Status |
|----------|--------|
| TypeScript | Full support |
| Python | Full support |
| Rust | Full support |
| Go | Full support |

---

## Contributing

Contributions are welcome! Here's how to get started:

### Development Setup

```bash
git clone https://github.com/jackal-lch/opensdd.git
cd opensdd

# Test the plugin locally
claude --plugin-dir .
```

### Building spec-extract

```bash
cd tools/spec-extract
cargo build --release
```

### Submitting Changes

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/my-feature`)
3. Commit your changes
4. Push to your fork
5. Open a Pull Request

### Reporting Issues

Found a bug or have a feature request? [Open an issue](https://github.com/jackal-lch/opensdd/issues).

---

## License

[MIT](LICENSE)
